//! Shared annotative-object detection + annotation-scale resolution.
//!
//! Both the Properties panel (Annotative row / applied scale name) and the
//! tessellation bake (which scales annotative content by the current annotation
//! scale) must agree on *which* entities are annotative — so that logic lives
//! here, once. An entity is annotative if it carries a per-object annotation
//! context, the legacy annotative XDATA, or an annotative style.

use acadrust::entities::{EntityCommon, EntityType};
use acadrust::objects::{Dictionary, ObjectType};
use acadrust::{CadDocument, Handle};

/// Resolve a handle to a `Dictionary` object, if it is one.
pub fn as_dict(doc: &CadDocument, handle: Handle) -> Option<&Dictionary> {
    match doc.objects.get(&handle) {
        Some(ObjectType::Dictionary(d)) => Some(d),
        _ => None,
    }
}

/// Does a style name resolve to `name` (or to "Standard" when `name` is blank)?
fn name_matches(style_name: &str, name: &str) -> bool {
    style_name.eq_ignore_ascii_case(name)
        || (name.trim().is_empty() && style_name.eq_ignore_ascii_case("Standard"))
}

fn text_style_annotative(doc: &CadDocument, name: &str) -> bool {
    doc.text_styles
        .iter()
        .find(|s| name_matches(&s.name, name))
        .is_some_and(|s| s.annotative)
}

fn dim_style_annotative(doc: &CadDocument, name: &str) -> bool {
    doc.dim_styles
        .iter()
        .find(|s| name_matches(&s.name, name))
        .is_some_and(|s| s.annotative)
}

fn mleader_style_annotative(doc: &CadDocument, handle: Option<Handle>) -> bool {
    let Some(h) = handle else {
        return false;
    };
    doc.objects.iter().any(|(oh, o)| {
        matches!(o, ObjectType::MultiLeaderStyle(s) if *oh == h && s.is_annotative)
    })
}

fn table_style_annotative(doc: &CadDocument, handle: Option<Handle>) -> bool {
    let Some(h) = handle else {
        return false;
    };
    doc.objects
        .iter()
        .any(|(oh, o)| matches!(o, ObjectType::TableStyle(s) if *oh == h && s.annotative))
}

/// Whether an object carries a per-object annotation context — its extension
/// dictionary holds an `AcDbContextDataManager`. This catches objects that are
/// annotative by context even when their style is not.
fn has_context_manager(doc: &CadDocument, common: &EntityCommon) -> bool {
    common
        .xdictionary_handle
        .and_then(|h| as_dict(doc, h))
        .map(|d| {
            d.entries
                .iter()
                .any(|(k, _)| k.eq_ignore_ascii_case("AcDbContextDataManager"))
        })
        .unwrap_or(false)
}

/// Whether an entity participates in annotation scaling.
pub fn is_annotative(doc: &CadDocument, entity: &EntityType) -> bool {
    // Per-object annotation context (works regardless of style).
    if has_context_manager(doc, entity.common()) {
        return true;
    }
    // Legacy annotative XDATA markers.
    let xd = &entity.common().extended_data;
    if xd.get_record("AcAnnoPO").is_some() || xd.get_record("AcAnnotativeData").is_some() {
        return true;
    }
    // Annotative via the assigned style (or the entity's own flag).
    match entity {
        EntityType::Text(t) => text_style_annotative(doc, &t.style),
        EntityType::MText(t) => t.is_annotative || text_style_annotative(doc, &t.style),
        EntityType::Dimension(d) => dim_style_annotative(doc, &d.base().style_name),
        EntityType::Leader(l) => dim_style_annotative(doc, &l.dimension_style),
        EntityType::MultiLeader(ml) => {
            ml.enable_annotation_scale || mleader_style_annotative(doc, ml.style_handle)
        }
        EntityType::Table(t) => table_style_annotative(doc, t.table_style_handle),
        _ => false,
    }
}

