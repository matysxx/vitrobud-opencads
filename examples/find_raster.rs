use acadrust::entities::EntityType;
use acadrust::io::dwg::DwgReader;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap();
    let doc = DwgReader::from_file(&path)?.read()?;
    for e in doc.entities() {
        if let EntityType::RasterImage(img) = e {
            println!("file_path: {}", img.file_path);
            println!("size: {} x {}", img.size.x, img.size.y);
            if let Ok(meta) = std::fs::metadata(&img.file_path) { println!("disk size: {} bytes", meta.len()); }
        }
    }
    Ok(())
}
