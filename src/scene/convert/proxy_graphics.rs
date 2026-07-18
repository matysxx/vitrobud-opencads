//! Proxy entity graphics — the cached vector preview an application stores
//! alongside a custom entity so viewers without its object enabler can still
//! draw something. AutoCAD falls back to exactly this; so do we (e.g. an
//! AutoCAD Architecture door/wall arrives as an `Unknown` entity we cannot
//! interpret, but it ships this preview).
//!
//! LibreDWG and ACadSharp both keep the blob raw and never parse it, so there
//! is no reference decoder to copy. The layout below was reverse-engineered
//! from real previews (a Raster Design image frame, an ACA door and wall) and
//! is deliberately conservative: it reads only the primitive records it has
//! verified and treats every other record as an opaque trait to skip. Phase 1
//! covers geometry (poly-lines, poly-gons and circular arcs); colour/lineweight
//! traits are ignored, so everything renders in the entity's own colour.
//!
//! Blob grammar (all little-endian):
//! ```text
//! u32 total_size        (== blob length)
//! u32 record_count
//! record_count × {
//!     u32 record_size   (bytes, including this 8-byte header)
//!     u32 record_type
//!     u8[record_size-8] data
//! }
//! ```
//! Record types decoded here:
//! * 6 poly-line / 7 poly-gon: `u32 point_count`, then `[f64;3] × point_count`
//!   (type 7 closes back to the first point).
//! * 4 / 5 circular arc: `[f64;3] centre`, `f64 radius`, `[f64;3] normal`,
//!   `[f64;3] start_dir`, `f64 sweep` (radians), then a trailing flag.

/// A poly-line lifted from a proxy-graphics blob, in world coordinates. Arcs
/// and closed poly-gons are pre-flattened into points so callers only draw
/// line strips.
pub struct ProxyPolyline {
    pub points: Vec<[f64; 3]>,
    /// The colour in force when this primitive was emitted, as an AutoCAD
    /// Color Index. 256 = ByLayer / 0 = ByBlock (inherit the entity's colour);
    /// 1..=255 override it. Set from the preview's colour traits.
    pub color: i32,
}

/// AutoCAD Color Index meaning "inherit from the layer" — the default until a
/// colour trait says otherwise.
const COLOR_BYLAYER: i32 = 256;

const REC_ARC: u32 = 4;
const REC_ARC5: u32 = 5;
const REC_POLYLINE: u32 = 6;
const REC_POLYGON: u32 = 7;
/// Trait record that sets the current colour (ACI) for following primitives.
const REC_COLOR: u32 = 14;

fn u32_at(b: &[u8], o: usize) -> Option<u32> {
    b.get(o..o + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

fn f64_at(b: &[u8], o: usize) -> Option<f64> {
    b.get(o..o + 8)
        .map(|s| f64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
}

fn pt3_at(b: &[u8], o: usize) -> Option<[f64; 3]> {
    let p = [f64_at(b, o)?, f64_at(b, o + 8)?, f64_at(b, o + 16)?];
    p.iter().all(|v| v.is_finite() && v.abs() < 1e12).then_some(p)
}

/// Decode every geometry record in a proxy-graphics `blob` into world-space
/// poly-lines. Returns an empty vec when the blob is absent, malformed, or
/// carries no geometry this decoder models — never any invented shape.
pub fn decode(blob: &[u8]) -> Vec<ProxyPolyline> {
    let mut out = Vec::new();
    let total = match u32_at(blob, 0) {
        Some(t) => t as usize,
        None => return out,
    };
    // Trust the length field only when it fits; a mismatch means a layout this
    // decoder does not model, so bail rather than walk off the end.
    if total > blob.len() || total < 8 {
        return out;
    }
    let count = u32_at(blob, 4).unwrap_or(0);
    if count > 1_000_000 {
        return out;
    }

    let mut pos = 8usize;
    let mut color = COLOR_BYLAYER;
    for _ in 0..count {
        let Some(rsize) = u32_at(blob, pos) else { break };
        let rsize = rsize as usize;
        let Some(rtype) = u32_at(blob, pos + 4) else { break };
        if rsize < 8 || pos + rsize > total {
            break;
        }
        match rtype {
            REC_COLOR => {
                if let Some(c) = u32_at(blob, pos + 8) {
                    color = c as i32;
                }
            }
            REC_POLYLINE | REC_POLYGON => {
                if let Some(n) = u32_at(blob, pos + 8) {
                    let n = n as usize;
                    if n >= 2 && 12 + n * 24 <= rsize {
                        let mut pts = Vec::with_capacity(n + 1);
                        let ok = (0..n).all(|i| match pt3_at(blob, pos + 12 + i * 24) {
                            Some(p) => {
                                pts.push(p);
                                true
                            }
                            None => false,
                        });
                        if ok {
                            // A poly-gon is a closed loop.
                            if rtype == REC_POLYGON {
                                if let Some(&first) = pts.first() {
                                    pts.push(first);
                                }
                            }
                            out.push(ProxyPolyline { points: pts, color });
                        }
                    }
                }
            }
            REC_ARC | REC_ARC5 => {
                if let Some(points) = decode_arc(blob, pos) {
                    out.push(ProxyPolyline { points, color });
                }
            }
            // Every other record is a trait (layer, lineweight, …) — skipped
            // for now; `rsize` still advances us past it.
            _ => {}
        }
        pos += rsize;
    }
    out
}

/// Flatten a circular-arc record (centre, radius, normal, start direction,
/// sweep) into a strip of points. The normal's Z sign gives the sweep
/// direction.
fn decode_arc(blob: &[u8], pos: usize) -> Option<Vec<[f64; 3]>> {
    let center = pt3_at(blob, pos + 8)?;
    let radius = f64_at(blob, pos + 32)?;
    let normal = pt3_at(blob, pos + 40)?;
    let start_dir = pt3_at(blob, pos + 64)?;
    let sweep = f64_at(blob, pos + 88)?;
    if !radius.is_finite() || radius <= 0.0 || radius > 1e9 || !sweep.is_finite() {
        return None;
    }
    let start = start_dir[1].atan2(start_dir[0]);
    let dir = if normal[2] < 0.0 { -1.0 } else { 1.0 };
    // Segment the sweep — ~64 per full turn, at least 2.
    let segs = ((sweep.abs() / std::f64::consts::TAU * 64.0).ceil() as usize).clamp(2, 512);
    let mut points = Vec::with_capacity(segs + 1);
    for i in 0..=segs {
        let a = start + dir * sweep * (i as f64 / segs as f64);
        points.push([
            center[0] + radius * a.cos(),
            center[1] + radius * a.sin(),
            center[2],
        ]);
    }
    Some(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real 140-byte preview of an Autodesk Raster Design embedded raster
    /// image: a single type-6 poly-line, its frame closed back to the origin.
    #[test]
    fn decodes_the_raster_design_image_frame() {
        let hex = "8C00000001000000840000000600000005000000\
                   000000000000000000000000000000000000000000000000\
                   DEF97E6A1C422E3D3DDF4F8D976E8B400000000000000000\
                   6BBC749398A092403DDF4F8D976E8B400000000000000000\
                   6BBC7493 98A09240 0000000000000000 0000000000000000\
                   000000000000000000000000000000000000000000000000";
        let blob: Vec<u8> = hex
            .split_whitespace()
            .collect::<String>()
            .as_bytes()
            .chunks(2)
            .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
            .collect();
        assert_eq!(blob.len(), 140);
        let polys = decode(&blob);
        assert_eq!(polys.len(), 1);
        assert_eq!(polys[0].points.len(), 5);
        assert!((polys[0].points[2][0] - 1192.1490).abs() < 1e-3);
        assert!((polys[0].points[2][1] - 877.8240).abs() < 1e-3);
    }

    #[test]
    fn rejects_anything_it_does_not_model() {
        assert!(decode(&[]).is_empty());
        assert!(decode(&[0u8; 140]).is_empty()); // size field wrong
        let mut b = vec![0u8; 140];
        b[0] = 140; // right size, zero records
        assert!(decode(&b).is_empty());
    }
}
