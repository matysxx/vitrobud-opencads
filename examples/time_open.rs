// Times each open phase: parse / purge / derived caches.
// Run with:  cargo run --release --example time_open -- <path-to-dwg>

use acadrust::entities::EntityType;
use std::env;
use std::path::Path;
use std::time::Instant;
use H7CAD::io::{load_file, purge_corrupt_entities};
use H7CAD::scene::{build_derived_caches, Scene};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or("usage: time_open <file>")?;

    let t0 = Instant::now();
    let mut doc = load_file(Path::new(&path))?;
    let parse_ms = t0.elapsed().as_millis();

    let t1 = Instant::now();
    let dropped = purge_corrupt_entities(&mut doc);
    let purge_ms = t1.elapsed().as_millis();

    let t2 = Instant::now();
    let caches = build_derived_caches(&doc);
    let caches_ms = t2.elapsed().as_millis();

    let mut counts = std::collections::HashMap::<&'static str, usize>::new();
    let mut total = 0usize;
    for e in doc.entities() {
        total += 1;
        let tag: &'static str = match e {
            EntityType::Line(_) => "Line",
            EntityType::Arc(_) => "Arc",
            EntityType::Circle(_) => "Circle",
            EntityType::LwPolyline(_) => "LwPolyline",
            EntityType::Polyline2D(_) => "Polyline2D",
            EntityType::Polyline3D(_) => "Polyline3D",
            EntityType::Polyline(_) => "Polyline",
            EntityType::Hatch(_) => "Hatch",
            EntityType::Solid(_) => "Solid",
            EntityType::RasterImage(_) => "RasterImage",
            EntityType::Solid3D(_) => "Solid3D",
            EntityType::Region(_) => "Region",
            EntityType::Body(_) => "Body",
            EntityType::Text(_) => "Text",
            EntityType::MText(_) => "MText",
            EntityType::Insert(_) => "Insert",
            EntityType::Dimension(_) => "Dimension",
            EntityType::Spline(_) => "Spline",
            EntityType::Ellipse(_) => "Ellipse",
            _ => "_other",
        };
        *counts.entry(tag).or_insert(0) += 1;
    }
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1));

    // Set up a Scene as the UI thread does and time first wire tessellation.
    let mut scene = Scene::new();
    scene.document = doc;
    scene.world_offset = caches.world_offset;
    scene.local_extent_max = caches.local_extent_max;
    scene.hatches = caches.hatches.clone();
    scene.images = caches.images.clone();
    scene.meshes = caches.meshes.clone();
    scene.bump_geometry();
    scene.current_layout = "Model".to_string();
    H7CAD::linetypes::populate_document(&mut scene.document);

    let t3 = Instant::now();
    let wires = scene.entity_wires();
    let wires_ms = t3.elapsed().as_millis();

    println!("file:           {}", path);
    println!("parse:          {} ms", parse_ms);
    println!("purge:          {} ms  (dropped {})", purge_ms, dropped);
    println!("derived caches: {} ms", caches_ms);
    println!("first wires:    {} ms  ({} wires)", wires_ms, wires.len());
    println!("  hatches:      {}", caches.hatches.len());
    println!("  images:       {}", caches.images.len());
    println!("  meshes:       {}", caches.meshes.len());
    println!();
    println!("entities:       {}", total);
    for (tag, n) in counts.iter().take(20) {
        println!("  {:>14}  {}", tag, n);
    }
    Ok(())
}
