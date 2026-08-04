use acadrust::entities::{EntityType, Line, LwPolyline, LwVertex, Text};
use acadrust::types::{Vector2, Vector3};
use acadrust::CadDocument;
use OpenCADStudio::io::export_dxf_r12::export_to_bytes;

const BINARY_DXF_SENTINEL: &[u8] = b"AutoCAD Binary DXF\r\n\x1a\0";

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Text(&'a [u8]),
    I16(i16),
    Double(f64),
}

fn parse_strict_r12(bytes: &[u8]) -> Result<Vec<(u16, Value<'_>)>, String> {
    if !bytes.starts_with(BINARY_DXF_SENTINEL) {
        return Err("missing sentinel".to_string());
    }
    let mut cursor = BINARY_DXF_SENTINEL.len();
    let mut groups = Vec::new();
    while cursor < bytes.len() {
        let first = bytes[cursor];
        cursor += 1;
        let code = if first == 0xFF {
            let raw = bytes
                .get(cursor..cursor + 2)
                .ok_or_else(|| "truncated extended code".to_string())?;
            cursor += 2;
            u16::from_le_bytes([raw[0], raw[1]])
        } else {
            first as u16
        };
        let value = match code {
            0..=9 => {
                let tail = bytes
                    .get(cursor..)
                    .ok_or_else(|| "truncated text".to_string())?;
                let length = tail
                    .iter()
                    .position(|byte| *byte == 0)
                    .ok_or_else(|| "unterminated text".to_string())?;
                let text = &tail[..length];
                cursor += length + 1;
                Value::Text(text)
            }
            10..=59 => {
                let raw = bytes
                    .get(cursor..cursor + 8)
                    .ok_or_else(|| "truncated double".to_string())?;
                cursor += 8;
                Value::Double(f64::from_le_bytes(raw.try_into().unwrap()))
            }
            60..=79 => {
                let raw = bytes
                    .get(cursor..cursor + 2)
                    .ok_or_else(|| "truncated i16".to_string())?;
                cursor += 2;
                Value::I16(i16::from_le_bytes(raw.try_into().unwrap()))
            }
            _ => return Err(format!("unsupported code {code}")),
        };
        groups.push((code, value));
    }
    Ok(groups)
}

fn text_records<'bytes>(
    groups: &[(u16, Value<'bytes>)],
    code: u16,
) -> Vec<&'bytes [u8]> {
    groups
        .iter()
        .filter_map(|(candidate, value)| match (candidate, value) {
            (candidate, Value::Text(text)) if *candidate == code => Some(*text),
            _ => None,
        })
        .collect()
}

#[test]
fn exports_strict_binary_ac1009_line_and_finite_extents() {
    let mut document = CadDocument::new();
    document
        .add_entity(EntityType::Line(Line::from_points(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(20.0, 20.0, 0.0),
        )))
        .unwrap();

    let (bytes, report) = export_to_bytes(&document).expect("export");

    assert!(bytes.starts_with(BINARY_DXF_SENTINEL));
    assert_eq!(
        &bytes[BINARY_DXF_SENTINEL.len()..BINARY_DXF_SENTINEL.len() + 9],
        b"\0SECTION\0"
    );
    assert_ne!(
        &bytes[BINARY_DXF_SENTINEL.len()..BINARY_DXF_SENTINEL.len() + 2],
        b"\0\0"
    );
    assert_eq!(report.entity_count, 1);

    let groups = parse_strict_r12(&bytes).expect("strict R12 parse");
    assert!(text_records(&groups, 1).contains(&b"AC1009".as_slice()));
    assert!(text_records(&groups, 2).contains(&b"ENTITIES".as_slice()));
    assert!(text_records(&groups, 0).contains(&b"LINE".as_slice()));
    assert_eq!(text_records(&groups, 0).last(), Some(&b"EOF".as_slice()));

    let extmax = groups
        .windows(4)
        .find(|window| matches!(&window[0], (9, Value::Text(b"$EXTMAX"))))
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
    document
        .add_entity(EntityType::LwPolyline(polyline))
        .unwrap();
    let source = document.clone();

    let (bytes, report) = export_to_bytes(&document).expect("export");

    assert_eq!(document, source);
    assert_eq!(report.converted_lwpolylines, 1);
    let groups = parse_strict_r12(&bytes).expect("strict R12 parse");
    let records = text_records(&groups, 0);
    assert_eq!(
        records
            .iter()
            .filter(|record| **record == b"POLYLINE")
            .count(),
        1
    );
    assert_eq!(
        records
            .iter()
            .filter(|record| **record == b"VERTEX")
            .count(),
        3
    );
    assert_eq!(
        records
            .iter()
            .filter(|record| **record == b"SEQEND")
            .count(),
        1
    );
    assert!(groups.contains(&(42, Value::Double(0.5))));
    assert!(!bytes
        .windows(b"LWPOLYLINE".len())
        .any(|window| window == b"LWPOLYLINE"));
}

#[test]
fn strict_parser_rejects_former_two_byte_group_codes() {
    let mut malformed = BINARY_DXF_SENTINEL.to_vec();
    malformed.extend_from_slice(b"\0\0SECTION\0");

    assert!(parse_strict_r12(&malformed).is_err());
}

#[test]
fn rejects_unsupported_entities() {
    let mut document = CadDocument::new();
    document.add_entity(EntityType::Text(Text::new())).unwrap();

    let error = export_to_bytes(&document).unwrap_err();

    assert!(error.contains("TEXT"));
}
