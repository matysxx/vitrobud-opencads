//! DGN line-style rendering (first pass).
//!
//! Linetypes converted from MicroStation DGN store their real pattern as DGN
//! line-style objects (`AcDbLS*`), not standard `LTYPE` dashes — the standard
//! table entry is empty, so acadrust exposes the structure in
//! [`CadDocument::dgn_ls_definitions`] / `dgn_ls_components` instead. See
//! `objects/dgn_linestyle.rs` in acadrust and `~/Documents/OCS/DGN_LINESTYLE_PLAN.md`.
//!
//! The visible content is the **symbol components**, each of which references an
//! anonymous block (e.g. a pipe's end circle). This renders those blocks at the
//! host polyline's endpoints. The exact placement / scale / dash pattern live in
//! the components' leaf data-stream fields, which are not decoded yet, so this is
//! an approximation: symbols at native scale on the first and last vertices.

use acadrust::objects::DgnLsComponentType;
use acadrust::types::{Handle, Transform, Vector3};
use acadrust::{CadDocument, EntityType};
use std::collections::HashSet;

use crate::scene::model::wire_model::WireModel;

/// A symbol placement in a linetype's DGN line-style tree: the anonymous block
/// to draw and the scale divisor to draw it at (`geometry / scale`).
pub struct DgnSymbol {
    pub block: Handle,
    pub scale: f64,
}

/// Symbol placements referenced by a linetype's DGN line-style tree, in tree
/// order. Empty when the linetype is not a DGN line style.
pub fn symbol_blocks(doc: &CadDocument, lt_name: &str) -> Vec<DgnSymbol> {
    let Some(def) = doc
        .dgn_ls_definitions
        .values()
        .find(|d| d.name.eq_ignore_ascii_case(lt_name))
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    walk(doc, def.root_component, &mut out, &mut seen);
    out
}

fn walk(doc: &CadDocument, h: Handle, out: &mut Vec<DgnSymbol>, seen: &mut HashSet<Handle>) {
    if !seen.insert(h) {
        return;
    }
    let Some(c) = doc.dgn_ls_components.get(&h) else {
        return;
    };
    match c.component_type {
        DgnLsComponentType::Compound | DgnLsComponentType::Point => {
            for r in &c.refs {
                let Some(sub) = doc.dgn_ls_components.get(r) else {
                    continue;
                };
                if sub.component_type == DgnLsComponentType::Symbol {
                    if let Some(block) = sub.symbol_block() {
                        if !out.iter().any(|s| s.block == block) {
                            out.push(DgnSymbol {
                                block,
                                scale: sub.scale,
                            });
                        }
                    }
                } else {
                    walk(doc, *r, out, seen);
                }
            }
        }
        _ => {}
    }
}

/// Host entity's polyline vertices in WCS f64 (consecutive duplicates dropped).
pub fn polyline_points(e: &EntityType) -> Vec<[f64; 3]> {
    let mut v: Vec<[f64; 3]> = match e {
        EntityType::LwPolyline(p) => p
            .vertices
            .iter()
            .map(|w| [w.location.x, w.location.y, 0.0])
            .collect(),
        EntityType::Polyline2D(p) => p
            .vertices
            .iter()
            .map(|w| [w.location.x, w.location.y, 0.0])
            .collect(),
        EntityType::Line(l) => vec![
            [l.start.x, l.start.y, l.start.z],
            [l.end.x, l.end.y, l.end.z],
        ],
        _ => Vec::new(),
    };
    v.dedup();
    v
}

/// Tessellate a symbol block's entities, translated so the block origin lands at
/// `at`, in the host entity's colour. Reuses the normal entity tessellator on
/// translated clones — the symbol geometry (ellipses, lines, …) renders exactly
/// as it would anywhere else.
#[allow(clippy::too_many_arguments)]
pub fn place_block_wires(
    doc: &CadDocument,
    block: Handle,
    scale_divisor: f64,
    at: [f64; 3],
    color: [f32; 4],
    line_weight_px: f32,
    anno_scale: f32,
    world_per_pixel: Option<f32>,
    bg_color: [f32; 4],
) -> Vec<WireModel> {
    let Some(br) = doc.block_records.iter().find(|b| b.handle == block) else {
        return Vec::new();
    };
    // The symbol block's native geometry is drawn at 1 / scale_divisor (the
    // divisor is read from the symbol component's leaf data). Scale about the
    // origin, then translate the (scaled) base point to the placement point.
    let s = if scale_divisor.abs() > 1e-9 {
        1.0 / scale_divisor
    } else {
        1.0
    };
    let scale = Transform::from_scale(s);
    let offset = Vector3::new(
        at[0] - br.base_point.x * s,
        at[1] - br.base_point.y * s,
        at[2] - br.base_point.z * s,
    );
    let mut out = Vec::new();
    for eh in &br.entity_handles {
        let Some(ent) = doc.get_entity(*eh) else {
            continue;
        };
        let mut clone = ent.clone();
        clone.as_entity_mut().apply_transform(&scale);
        clone.as_entity_mut().translate(offset);
        out.extend(super::tessellate::tessellate(
            doc,
            *eh,
            &clone,
            false,
            color,
            0.0,
            [0.0; 8],
            line_weight_px,
            anno_scale,
            world_per_pixel,
            bg_color,
            false,
        ));
    }
    out
}
