//! Fail-safe machine export to binary DXF R12 (AC1009).
//!
//! This deliberately does not use the general DXF writer: the pinned writer
//! can label post-R12 structures as AC1009.  Keep this path isolated until the
//! version-aware implementation is accepted by acadifc upstream.

use acadrust::entities::{EntityCommon, EntityType};
use acadrust::io::dxf::{DxfBinaryWriter, DxfStreamWriter, DxfStreamWriterExt};
use acadrust::types::{BoundingBox3D, Vector3};
use acadrust::{CadDocument, DxfReader};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Cursor;

const SENTINEL: &[u8] = b"AutoCAD Binary DXF\r\n\x1a\0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportReport {
    pub entity_count: usize,
    pub converted_lwpolylines: usize,
    pub substituted_linetypes: usize,
}

impl ExportReport {
    pub fn summary(&self) -> String {
        format!(
            "{} entities, {} lightweight polylines converted, {} linetypes substituted",
            self.entity_count, self.converted_lwpolylines, self.substituted_linetypes
        )
    }
}

pub fn suggested_filename(source: Option<&std::path::Path>) -> String {
    let stem = source
        .and_then(std::path::Path::file_stem)
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("drawing");
    format!("{stem}_R12.dxf")
}

pub fn export_to_bytes(document: &CadDocument) -> Result<(Vec<u8>, ExportReport), String> {
    let entities: Vec<&EntityType> = document
        .entities()
        .filter(|entity| entity.common().entity_mode != Some(1))
        .filter(|entity| !matches!(entity, EntityType::Viewport(_)))
        .collect();

    let unsupported: BTreeSet<String> = entities
        .iter()
        .filter_map(|entity| incompatibility(entity))
        .collect();
    if !unsupported.is_empty() {
        return Err(format!(
            "unsupported entity types for lossless R12 export: {}",
            unsupported.into_iter().collect::<Vec<_>>().join(", ")
        ));
    }

    let bounds = combined_bounds(&entities)?;
    let layers = collect_layers(&entities);
    let converted_lwpolylines = entities
        .iter()
        .filter(|entity| matches!(entity, EntityType::LwPolyline(_)))
        .count();
    let substituted_linetypes = entities
        .iter()
        .filter(|entity| !is_r12_linetype(&entity.common().linetype))
        .count();

    let mut bytes = Vec::with_capacity((entities.len() + 32) * 192);
    {
        let mut writer = DxfBinaryWriter::new(&mut bytes).map_err(|error| error.to_string())?;
        write_header(&mut writer, bounds).map_err(|error| error.to_string())?;
        write_tables(&mut writer, &layers).map_err(|error| error.to_string())?;
        writer.write_section_start("BLOCKS").map_err(|e| e.to_string())?;
        writer.write_section_end().map_err(|e| e.to_string())?;
        writer.write_section_start("ENTITIES").map_err(|e| e.to_string())?;
        for entity in &entities {
            write_entity(&mut writer, entity).map_err(|error| error.to_string())?;
        }
        writer.write_section_end().map_err(|e| e.to_string())?;
        writer.write_eof().map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;
    }

    verify(&bytes, entities.len())?;
    Ok((
        bytes,
        ExportReport {
            entity_count: entities.len(),
            converted_lwpolylines,
            substituted_linetypes,
        },
    ))
}

pub fn export_to_file(
    document: &CadDocument,
    path: &std::path::Path,
) -> Result<ExportReport, String> {
    let (bytes, report) = export_to_bytes(document)?;
    let temp = path.with_extension(format!("dxf.ocs-r12-{}-tmp", std::process::id()));
    std::fs::write(&temp, bytes).map_err(|error| error.to_string())?;
    if let Err(error) = super::replace_save_file(&temp, path) {
        let _ = std::fs::remove_file(&temp);
        return Err(error.to_string());
    }
    Ok(report)
}

fn incompatibility(entity: &&EntityType) -> Option<String> {
    let normal_is_z = |normal: Vector3| normal == Vector3::UNIT_Z;
    match entity {
        EntityType::Point(value) if value.thickness == 0.0 && normal_is_z(value.normal) => None,
        EntityType::Line(value) if value.thickness == 0.0 && normal_is_z(value.normal) => None,
        EntityType::Circle(value) if value.thickness == 0.0 && normal_is_z(value.normal) => None,
        EntityType::Arc(value) if value.thickness == 0.0 && normal_is_z(value.normal) => None,
        EntityType::Polyline2D(value)
            if value.thickness == 0.0 && normal_is_z(value.normal) => None,
        EntityType::LwPolyline(value)
            if value.thickness == 0.0 && normal_is_z(value.normal) => None,
        EntityType::Point(_)
        | EntityType::Line(_)
        | EntityType::Circle(_)
        | EntityType::Arc(_)
        | EntityType::Polyline2D(_)
        | EntityType::LwPolyline(_) => Some(format!(
            "{} (non-default extrusion or thickness)",
            entity.as_entity().entity_type()
        )),
        _ => Some(entity.as_entity().entity_type().to_string()),
    }
}

fn combined_bounds(entities: &[&EntityType]) -> Result<BoundingBox3D, String> {
    let mut min = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for entity in entities {
        let bounds = entity_bounds(entity);
        for value in [bounds.min.x, bounds.min.y, bounds.min.z, bounds.max.x, bounds.max.y, bounds.max.z] {
            if !value.is_finite() {
                return Err("drawing contains non-finite geometry".to_string());
            }
        }
        min.x = min.x.min(bounds.min.x);
        min.y = min.y.min(bounds.min.y);
        min.z = min.z.min(bounds.min.z);
        max.x = max.x.max(bounds.max.x);
        max.y = max.y.max(bounds.max.y);
        max.z = max.z.max(bounds.max.z);
    }
    if entities.is_empty() {
        min = Vector3::ZERO;
        max = Vector3::ZERO;
    }
    Ok(BoundingBox3D::new(min, max))
}

fn entity_bounds(entity: &EntityType) -> BoundingBox3D {
    match entity {
        EntityType::LwPolyline(polyline) => {
            let points: Vec<(f64, f64, f64)> = polyline
                .vertices
                .iter()
                .map(|vertex| (vertex.location.x, vertex.location.y, vertex.bulge))
                .collect();
            polyline_bounds(&points, polyline.is_closed, polyline.elevation)
        }
        EntityType::Polyline2D(polyline) => {
            let points: Vec<(f64, f64, f64)> = polyline
                .vertices
                .iter()
                .map(|vertex| (vertex.location.x, vertex.location.y, vertex.bulge))
                .collect();
            polyline_bounds(&points, polyline.is_closed(), polyline.elevation)
        }
        _ => entity.as_entity().bounding_box(),
    }
}

// A full-circle envelope for each bulged segment is conservative but cannot
// clip geometry when an arc crosses a quadrant.
fn polyline_bounds(points: &[(f64, f64, f64)], closed: bool, elevation: f64) -> BoundingBox3D {
    if points.is_empty() {
        return BoundingBox3D::from_point(Vector3::new(0.0, 0.0, elevation));
    }
    let mut min = Vector3::new(f64::INFINITY, f64::INFINITY, elevation);
    let mut max = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, elevation);
    for &(x, y, _) in points {
        min.x = min.x.min(x);
        min.y = min.y.min(y);
        max.x = max.x.max(x);
        max.y = max.y.max(y);
    }
    let segment_count = if closed {
        points.len()
    } else {
        points.len().saturating_sub(1)
    };
    for index in 0..segment_count {
        let (x1, y1, bulge) = points[index];
        if bulge == 0.0 {
            continue;
        }
        let (x2, y2, _) = points[(index + 1) % points.len()];
        let dx = x2 - x1;
        let dy = y2 - y1;
        let chord = dx.hypot(dy);
        if chord == 0.0 || !bulge.is_finite() {
            continue;
        }
        let offset = chord * (1.0 - bulge * bulge) / (4.0 * bulge);
        let cx = (x1 + x2) * 0.5 - dy / chord * offset;
        let cy = (y1 + y2) * 0.5 + dx / chord * offset;
        let radius = chord * (1.0 + bulge * bulge) / (4.0 * bulge.abs());
        min.x = min.x.min(cx - radius);
        min.y = min.y.min(cy - radius);
        max.x = max.x.max(cx + radius);
        max.y = max.y.max(cy + radius);
    }
    BoundingBox3D::new(min, max)
}

fn collect_layers(entities: &[&EntityType]) -> BTreeMap<String, i16> {
    let mut layers = BTreeMap::from([("0".to_string(), 7)]);
    for entity in entities {
        let common = entity.common();
        let name = if common.layer.trim().is_empty() { "0" } else { common.layer.as_str() };
        layers.entry(name.to_string()).or_insert(7);
    }
    layers
}

fn is_r12_linetype(name: &str) -> bool {
    name.is_empty()
        || name.eq_ignore_ascii_case("BYLAYER")
        || name.eq_ignore_ascii_case("BYBLOCK")
        || name.eq_ignore_ascii_case("CONTINUOUS")
}

fn write_header<W: std::io::Write>(
    writer: &mut DxfBinaryWriter<W>,
    bounds: BoundingBox3D,
) -> acadrust::Result<()> {
    writer.write_section_start("HEADER")?;
    writer.write_string(9, "$ACADVER")?;
    writer.write_string(1, "AC1009")?;
    writer.write_string(9, "$EXTMIN")?;
    writer.write_point3d(10, bounds.min)?;
    writer.write_string(9, "$EXTMAX")?;
    writer.write_point3d(10, bounds.max)?;
    writer.write_section_end()
}

fn write_tables<W: std::io::Write>(
    writer: &mut DxfBinaryWriter<W>,
    layers: &BTreeMap<String, i16>,
) -> acadrust::Result<()> {
    writer.write_section_start("TABLES")?;
    writer.write_string(0, "TABLE")?;
    writer.write_string(2, "LTYPE")?;
    writer.write_i16(70, 1)?;
    writer.write_string(0, "LTYPE")?;
    writer.write_string(2, "CONTINUOUS")?;
    writer.write_i16(70, 0)?;
    writer.write_string(3, "Solid line")?;
    writer.write_i16(72, 65)?;
    writer.write_i16(73, 0)?;
    writer.write_double(40, 0.0)?;
    writer.write_string(0, "ENDTAB")?;
    writer.write_string(0, "TABLE")?;
    writer.write_string(2, "LAYER")?;
    writer.write_i16(70, layers.len().min(i16::MAX as usize) as i16)?;
    for (name, color) in layers {
        writer.write_string(0, "LAYER")?;
        writer.write_string(2, name)?;
        writer.write_i16(70, 0)?;
        writer.write_i16(62, *color)?;
        writer.write_string(6, "CONTINUOUS")?;
    }
    writer.write_string(0, "ENDTAB")?;
    writer.write_section_end()
}

fn write_common<W: std::io::Write>(writer: &mut DxfBinaryWriter<W>, common: &EntityCommon) -> acadrust::Result<()> {
    writer.write_string(8, if common.layer.trim().is_empty() { "0" } else { &common.layer })?;
    if !matches!(common.color, acadrust::types::Color::ByLayer) {
        writer.write_color(62, common.color)?;
    }
    if is_r12_linetype(&common.linetype)
        && !common.linetype.is_empty()
        && !common.linetype.eq_ignore_ascii_case("BYLAYER")
    {
        writer.write_string(6, &common.linetype.to_ascii_uppercase())?;
    }
    if (common.linetype_scale - 1.0).abs() > f64::EPSILON {
        writer.write_double(48, common.linetype_scale)?;
    }
    Ok(())
}

fn write_entity<W: std::io::Write>(writer: &mut DxfBinaryWriter<W>, entity: &EntityType) -> acadrust::Result<()> {
    match entity {
        EntityType::Point(value) => {
            writer.write_string(0, "POINT")?;
            write_common(writer, &value.common)?;
            writer.write_point3d(10, value.location)?;
        }
        EntityType::Line(value) => {
            writer.write_string(0, "LINE")?;
            write_common(writer, &value.common)?;
            writer.write_point3d(10, value.start)?;
            writer.write_point3d(11, value.end)?;
        }
        EntityType::Circle(value) => {
            writer.write_string(0, "CIRCLE")?;
            write_common(writer, &value.common)?;
            writer.write_point3d(10, value.center)?;
            writer.write_double(40, value.radius)?;
        }
        EntityType::Arc(value) => {
            writer.write_string(0, "ARC")?;
            write_common(writer, &value.common)?;
            writer.write_point3d(10, value.center)?;
            writer.write_double(40, value.radius)?;
            writer.write_double(50, value.start_angle.to_degrees())?;
            writer.write_double(51, value.end_angle.to_degrees())?;
        }
        EntityType::LwPolyline(value) => {
            writer.write_string(0, "POLYLINE")?;
            write_common(writer, &value.common)?;
            writer.write_i16(66, 1)?;
            writer.write_point3d(10, Vector3::new(0.0, 0.0, value.elevation))?;
            if value.constant_width != 0.0 {
                writer.write_double(40, value.constant_width)?;
                writer.write_double(41, value.constant_width)?;
            }
            writer.write_i16(70, if value.is_closed { 1 } else { 0 })?;
            for vertex in &value.vertices {
                writer.write_string(0, "VERTEX")?;
                write_common(writer, &value.common)?;
                writer.write_point3d(10, Vector3::new(vertex.location.x, vertex.location.y, value.elevation))?;
                writer.write_i16(70, 0)?;
                if vertex.start_width != 0.0 { writer.write_double(40, vertex.start_width)?; }
                if vertex.end_width != 0.0 { writer.write_double(41, vertex.end_width)?; }
                if vertex.bulge != 0.0 { writer.write_double(42, vertex.bulge)?; }
            }
            writer.write_string(0, "SEQEND")?;
            write_common(writer, &value.common)?;
        }
        EntityType::Polyline2D(value) => {
            writer.write_string(0, "POLYLINE")?;
            write_common(writer, &value.common)?;
            writer.write_i16(66, 1)?;
            writer.write_point3d(10, Vector3::new(0.0, 0.0, value.elevation))?;
            if value.start_width != 0.0 {
                writer.write_double(40, value.start_width)?;
            }
            if value.end_width != 0.0 {
                writer.write_double(41, value.end_width)?;
            }
            writer.write_i16(70, value.flags.bits() as i16)?;
            for vertex in &value.vertices {
                writer.write_string(0, "VERTEX")?;
                write_common(writer, &value.common)?;
                writer.write_point3d(10, vertex.location)?;
                writer.write_i16(70, vertex.flags.bits() as i16)?;
                if vertex.start_width != 0.0 { writer.write_double(40, vertex.start_width)?; }
                if vertex.end_width != 0.0 { writer.write_double(41, vertex.end_width)?; }
                if vertex.bulge != 0.0 { writer.write_double(42, vertex.bulge)?; }
            }
            writer.write_string(0, "SEQEND")?;
            write_common(writer, &value.common)?;
        }
        _ => unreachable!("unsupported entities are rejected before writing"),
    }
    Ok(())
}

fn verify(bytes: &[u8], expected_entities: usize) -> Result<(), String> {
    if !bytes.starts_with(SENTINEL) {
        return Err("R12 verification failed: missing binary DXF sentinel".to_string());
    }
    let reread = DxfReader::from_reader(Cursor::new(bytes.to_vec()))
        .and_then(DxfReader::read)
        .map_err(|error| format!("R12 verification failed: {error}"))?;
    if !bytes.windows(b"AC1009".len()).any(|window| window == b"AC1009") {
        return Err("R12 verification failed: missing AC1009 header".to_string());
    }
    for forbidden in [b"CLASSES".as_slice(), b"OBJECTS", b"LWPOLYLINE", b"BLOCK_RECORD"] {
        if bytes.windows(forbidden.len()).any(|window| window == forbidden) {
            return Err(format!(
                "R12 verification failed: forbidden {} record",
                String::from_utf8_lossy(forbidden)
            ));
        }
    }
    let actual = reread.entities().count();
    if actual != expected_entities {
        return Err(format!("R12 verification failed: expected {expected_entities} entities, read {actual}"));
    }
    Ok(())
}
