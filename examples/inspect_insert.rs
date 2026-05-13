// Lists every Insert in the doc (after xref resolve) with sub-entity counts.
// cargo run --release --example inspect_insert -- <dwg>
use acadrust::entities::EntityType;
use acadrust::io::dwg::DwgReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap();
    let mut doc = DwgReader::from_file(&path)?.read()?;
    if let Some(base) = std::path::Path::new(&path).parent() {
        let infos = H7CAD::io::xref::resolve_xrefs(&mut doc, base);
        for i in &infos {
            println!("XREF: {} -> {:?}", i.name, i.status);
        }
    }
    println!("total entities: {}", doc.entities().count());

    let inserts: Vec<_> = doc
        .entities()
        .filter_map(|e| if let EntityType::Insert(i) = e { Some(i.clone()) } else { None })
        .collect();
    println!("total inserts: {}", inserts.len());

    let mut by_block: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for i in &inserts {
        *by_block.entry(i.block_name.clone()).or_insert(0) += 1;
    }
    let mut by_block: Vec<_> = by_block.into_iter().collect();
    by_block.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\ninserts grouped by block_name:");
    for (name, n) in &by_block {
        let sub_count = inserts.iter().find(|i| &i.block_name == name)
            .map(|i| i.explode_from_document(&doc).len())
            .unwrap_or(0);
        let nested = doc.block_records.get(name)
            .map(|br| br.entity_handles.iter()
                .filter(|h| matches!(doc.get_entity(**h), Some(EntityType::Insert(_))))
                .count())
            .unwrap_or(0);
        println!("  {:>4} × {}  (sub={}, nested-inserts={})", n, name, sub_count, nested);
    }
    Ok(())
}
