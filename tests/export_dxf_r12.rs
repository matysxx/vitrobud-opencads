use acadrust::entities::{EntityType, Line, LwPolyline, LwVertex, Text};
use acadrust::types::{Vector2, Vector3};
use acadrust::CadDocument;
use OpenCADStudio::io::export_dxf_r12::export_to_bytes;

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Text(&'a str),
    I16(i16),
    Double(f64),
}

fn parse_ascii_r12(bytes: &[u8]) -> Result<Vec<(u16, Value<'_>)>, String> {
    if !bytes.ends_with(b"\r\n") {
        return Err("missing final CRLF".to_string());
    }
    for (index, byte) in bytes.iter().enumerate() {
        if (*byte == b'\n' && (index == 0 || bytes[index - 1] != b'\r'))
            || (*byte == b'\r' && bytes.get(index + 1) != Some(&b'\n'))
        {
            return Err("non-CRLF line ending".to_string());
        }
    }

    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let mut lines = text.strip_suffix("\r\n").unwrap().split("\r\n");
    let mut groups = Vec::new();
    while let Some(code) = lines.next() {
        let value = lines.next().ok_or_else(|| "missing value line".to_string())?;
        let code = code
            .trim()
            .parse::<u16>()
            .map_err(|error| error.to_string())?;
        let value = match code {
            0..=9 => Value::Text(value),
            10..=59 => Value::Double(value.trim().parse().map_err(|error: std::num::ParseFloatError| error.to_string())?),
            60..=79 => Value::I16(value.trim().parse().map_err(|error: std::num::ParseIntError| error.to_string())?),
            _ => return Err(format!("unsupported code {code}")),
        };
        groups.push((code, value));
    }
    Ok(groups)
}

fn text_records<'bytes>(groups: &[(u16, Value<'bytes>)], code: u16) -> Vec<&'bytes str> {
    groups
        .iter()
        .filter_map(|(candidate, value)| match (candidate, value) {
            (candidate, Value::Text(text)) if *candidate == code => Some(*text),
            _ => None,
        })
        .collect()
}

#[test]
fn exports_ascii_crlf_ac1009_line_and_finite_extents() {
    let mut document = CadDocument::new();
    document
        .add_entity(EntityType::Line(Line::from_points(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(20.0, 20.0, 0.0),
        )))
        .unwrap();

    let (bytes, report) = export_to_bytes(&document).expect("export");

    assert!(bytes.starts_with(b"  0\r\nSECTION\r\n"));
    assert!(!bytes.starts_with(b"AutoCAD Binary DXF"));
    assert_eq!(report.entity_count, 1);

    let groups = parse_ascii_r12(&bytes).expect("ASCII R12 parse");
    assert!(text_records(&groups, 1).contains(&"AC1009"));
    assert!(text_records(&groups, 2).contains(&"ENTITIES"));
    assert!(text_records(&groups, 0).contains(&"LINE"));
    assert_eq!(text_records(&groups, 0).last(), Some(&"EOF"));

    let extmax = groups
        .windows(4)
        .find(|window| matches!(&window[0], (9, Value::Text("$EXTMAX"))))
        .expect("EXTMAX");
    assert_eq!(extmax[1], (10, Value::Double(20.0)));
    assert_eq!(extmax[2], (20, Value::Double(20.0)));
    assert_eq!(extmax[3], (30, Value::Double(0.0)));
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
    document.add_entity(EntityType::LwPolyline(polyline)).unwrap();
    let source = document.clone();

    let (bytes, report) = export_to_bytes(&document).expect("export");

    assert_eq!(document, source);
    assert_eq!(report.converted_lwpolylines, 1);
    let groups = parse_ascii_r12(&bytes).expect("ASCII R12 parse");
    let records = text_records(&groups, 0);
    assert_eq!(records.iter().filter(|record| **record == "POLYLINE").count(), 1);
    assert_eq!(records.iter().filter(|record| **record == "VERTEX").count(), 3);
    assert_eq!(records.iter().filter(|record| **record == "SEQEND").count(), 1);
    assert!(groups.contains(&(42, Value::Double(0.5))));
    assert!(!bytes.windows(b"LWPOLYLINE".len()).any(|window| window == b"LWPOLYLINE"));
}

#[test]
fn independent_parser_rejects_binary_or_lf_only_streams() {
    assert!(parse_ascii_r12(b"AutoCAD Binary DXF\r\n\x1a\0").is_err());
    assert!(parse_ascii_r12(b"0\nEOF\n").is_err());
}

#[test]
fn rejects_unsupported_entities() {
    let mut document = CadDocument::new();
    document.add_entity(EntityType::Text(Text::new())).unwrap();

    let error = export_to_bytes(&document).unwrap_err();

    assert!(error.contains("TEXT"));
}
