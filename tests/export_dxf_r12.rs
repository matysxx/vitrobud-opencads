use acadrust::entities::{EntityType, Line, LwPolyline, LwVertex, Text};
use acadrust::types::{Vector2, Vector3};
use acadrust::{CadDocument, DxfReader, DxfVersion};
use OpenCADStudio::io::export_dxf_r12::export_to_bytes;
use std::io::Cursor;

const BINARY_DXF_SENTINEL: &[u8] = b"AutoCAD Binary DXF\r\n\x1a\0";

#[test]
fn exports_binary_ac1009_line_and_finite_extents() {
    let mut document = CadDocument::new();
    document
        .add_entity(EntityType::Line(Line::from_points(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(20.0, 20.0, 0.0),
        )))
        .unwrap();

    let (bytes, report) = export_to_bytes(&document).expect("export");

    assert!(bytes.starts_with(BINARY_DXF_SENTINEL));
    assert_eq!(report.entity_count, 1);
    let reread = DxfReader::from_reader(Cursor::new(bytes))
        .unwrap()
        .read()
        .unwrap();
    assert_eq!(reread.version, DxfVersion::Unknown);
    assert_eq!(reread.header.model_space_extents_min, Vector3::ZERO);
    assert_eq!(
        reread.header.model_space_extents_max,
        Vector3::new(20.0, 20.0, 0.0)
    );
}

#[test]
fn converts_bulged_lwpolyline_without_mutating_source() {
    let mut polyline = LwPolyline::new();
    polyline.vertices = vec![
        LwVertex::with_bulge(Vector2::new(0.0, 0.0), 0.5),
        LwVertex::new(Vector2::new(10.0, 0.0)),
        LwVertex::new(Vector2::new(10.0, 10.0)),
    ];
    polyline.is_closed = true;
    let mut document = CadDocument::new();
    document
        .add_entity(EntityType::LwPolyline(polyline))
        .unwrap();
    let source = document.clone();

    let (bytes, report) = export_to_bytes(&document).expect("export");

    assert_eq!(document, source);
    assert_eq!(report.converted_lwpolylines, 1);
    let reread = DxfReader::from_reader(Cursor::new(bytes))
        .unwrap()
        .read()
        .unwrap();
    let EntityType::Polyline2D(converted) = reread.entities().next().unwrap() else {
        panic!("expected POLYLINE")
    };
    assert!(converted.is_closed());
    assert_eq!(converted.vertices.len(), 3);
    assert_eq!(converted.vertices[0].bulge, 0.5);
}

#[test]
fn rejects_unsupported_entities() {
    let mut document = CadDocument::new();
    document.add_entity(EntityType::Text(Text::new())).unwrap();

    let error = export_to_bytes(&document).unwrap_err();

    assert!(error.contains("TEXT"));
}
