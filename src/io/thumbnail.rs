//! CPU wire-raster thumbnail for the DWG preview image.
//!
//! Rasterizes the current layout's wire set into a small bitmap and returns it
//! as an [`acadrust::Preview`] (a Windows DIB, universally accepted as a DWG
//! preview across versions). Embedded on save so OCS drawings show a thumbnail
//! in file browsers and other CAD applications.

use acadrust::{Preview, PreviewFormat};
use image::{ImageFormat, Rgb, RgbImage};
use std::io::Cursor;

use crate::scene::{Scene, WireModel};

/// Longest edge of the generated thumbnail, in pixels.
const MAX_DIM: u32 = 256;
/// Blank border kept around the drawing, in pixels.
const MARGIN: f64 = 6.0;

/// Build a preview from the scene's current-layout wires. `None` when there is
/// nothing finite to draw (empty or degenerate drawing).
pub fn from_scene(scene: &Scene) -> Option<Preview> {
    from_wires(&scene.entity_wires(), scene.bg_color)
}

/// Rasterize `wires` (world XY) onto a `bg`-filled canvas and encode a DIB.
pub fn from_wires(wires: &[WireModel], bg: [f32; 4]) -> Option<Preview> {
    // ── World-XY bounds over every finite vertex ─────────────────────────────
    let (mut min_x, mut min_y) = (f64::INFINITY, f64::INFINITY);
    let (mut max_x, mut max_y) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    for w in wires {
        for (i, p) in w.points.iter().enumerate() {
            if !p[0].is_finite() || !p[1].is_finite() {
                continue;
            }
            let (x, y) = abs_xy(w, i, p);
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }
    let (dw, dh) = (max_x - min_x, max_y - min_y);
    if !dw.is_finite() || !dh.is_finite() || (dw <= 0.0 && dh <= 0.0) {
        return None;
    }

    // ── Canvas sized to the drawing's aspect, longest edge = MAX_DIM ─────────
    let aspect = if dh > 0.0 { dw / dh } else { f64::INFINITY };
    let (cw, ch) = if aspect >= 1.0 {
        (MAX_DIM, ((MAX_DIM as f64 / aspect).round() as u32).clamp(16, MAX_DIM))
    } else {
        (((MAX_DIM as f64 * aspect).round() as u32).clamp(16, MAX_DIM), MAX_DIM)
    };

    // World → pixel: uniform scale to fit inside the margin, drawing centered,
    // Y flipped (world up → image row 0 at top).
    let sx = (cw as f64 - 2.0 * MARGIN) / dw.max(1e-9);
    let sy = (ch as f64 - 2.0 * MARGIN) / dh.max(1e-9);
    let scale = sx.min(sy);
    let off_x = (cw as f64 - dw * scale) * 0.5;
    let off_y = (ch as f64 - dh * scale) * 0.5;
    let to_px = |x: f64, y: f64| -> (i32, i32) {
        let px = off_x + (x - min_x) * scale;
        let py = ch as f64 - (off_y + (y - min_y) * scale); // flip Y
        (px.round() as i32, py.round() as i32)
    };

    // ── Rasterize ────────────────────────────────────────────────────────────
    let bg_rgb = to_rgb(bg);
    let mut img = RgbImage::from_pixel(cw, ch, Rgb(bg_rgb));
    for w in wires {
        let col = Rgb(to_rgb(w.color));
        let mut prev: Option<(i32, i32)> = None;
        for (i, p) in w.points.iter().enumerate() {
            if !p[0].is_finite() || !p[1].is_finite() {
                prev = None; // NaN separator breaks the run
                continue;
            }
            let (x, y) = abs_xy(w, i, p);
            let cur = to_px(x, y);
            if let Some(p0) = prev {
                draw_line(&mut img, p0, cur, col);
            }
            prev = Some(cur);
        }
    }

    // ── Encode BMP, strip the 14-byte BITMAPFILEHEADER → DIB ─────────────────
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Bmp).ok()?;
    let bmp = buf.into_inner();
    if bmp.len() <= 14 {
        return None;
    }
    Some(Preview { format: PreviewFormat::Bmp, data: bmp[14..].to_vec() })
}

/// Absolute world XY of vertex `i`, reconstructing the double-single residual.
#[inline]
fn abs_xy(w: &WireModel, i: usize, p: &[f32; 3]) -> (f64, f64) {
    let (lx, ly) = w
        .points_low
        .get(i)
        .map_or((0.0, 0.0), |l| (l[0] as f64, l[1] as f64));
    (p[0] as f64 + lx, p[1] as f64 + ly)
}

#[inline]
fn to_rgb(c: [f32; 4]) -> [u8; 3] {
    [
        (c[0].clamp(0.0, 1.0) * 255.0).round() as u8,
        (c[1].clamp(0.0, 1.0) * 255.0).round() as u8,
        (c[2].clamp(0.0, 1.0) * 255.0).round() as u8,
    ]
}

/// Cheaply read ONLY the embedded preview from a DWG (file header + preview
/// bytes, no full document parse) and decode it to an iced image handle.
/// `None` for DXF/other files, a missing or empty preview, or a format the
/// `image` crate can't decode (WMF). Used to show recent-file thumbnails.
pub fn read_handle(path: &std::path::Path) -> Option<iced::widget::image::Handle> {
    decode(&read_preview(path)?)
}

/// Read the preview image bytes straight from the file's raw preview offset.
fn read_preview(path: &std::path::Path) -> Option<Preview> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = std::fs::File::open(path).ok()?;
    let mut ver = [0u8; 6];
    f.read_exact(&mut ver).ok()?;
    if &ver[..2] != b"AC" {
        return None; // not a DWG (DXF has no thumbnail)
    }
    // Preview seeker: absolute file offset at header byte 0x0D.
    f.seek(SeekFrom::Start(0x0D)).ok()?;
    let mut a = [0u8; 4];
    f.read_exact(&mut a).ok()?;
    let base = i32::from_le_bytes(a);
    if base <= 0 {
        return None;
    }
    let base = base as u64;
    f.seek(SeekFrom::Start(base)).ok()?;
    let mut head = [0u8; 20];
    f.read_exact(&mut head).ok()?;
    let overall = acadrust::io::dwg::preview::overall_size(&head)?;
    if overall == 0 || overall > 64 * 1024 * 1024 {
        return None;
    }
    let total = acadrust::io::dwg::preview::container_len(overall);
    f.seek(SeekFrom::Start(base)).ok()?;
    let mut buf = vec![0u8; total];
    f.read_exact(&mut buf).ok()?;
    acadrust::io::dwg::preview::parse_preview(&buf, base)
}

/// Decode a stored preview to an iced RGBA handle.
fn decode(p: &Preview) -> Option<iced::widget::image::Handle> {
    let img = match p.format {
        PreviewFormat::Png => image::load_from_memory_with_format(&p.data, ImageFormat::Png).ok()?,
        PreviewFormat::Bmp => {
            image::load_from_memory_with_format(&dib_to_bmp(&p.data), ImageFormat::Bmp).ok()?
        }
        PreviewFormat::Wmf => return None,
    };
    let rgba = img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    Some(iced::widget::image::Handle::from_rgba(w, h, rgba.into_raw()))
}

/// Reconstruct a full in-memory BMP from a stored DIB (prepend the 14-byte
/// `BITMAPFILEHEADER`) so a BMP decoder can read it.
fn dib_to_bmp(dib: &[u8]) -> Vec<u8> {
    if dib.len() < 16 {
        return Vec::new();
    }
    let bi_size = u32::from_le_bytes([dib[0], dib[1], dib[2], dib[3]]) as usize;
    let bpp = u16::from_le_bytes([dib[14], dib[15]]) as usize;
    let palette = if (1..=8).contains(&bpp) { (1usize << bpp) * 4 } else { 0 };
    let mut v = Vec::with_capacity(14 + dib.len());
    v.extend_from_slice(b"BM");
    v.extend_from_slice(&((14 + dib.len()) as u32).to_le_bytes());
    v.extend_from_slice(&0u32.to_le_bytes());
    v.extend_from_slice(&((14 + bi_size + palette) as u32).to_le_bytes());
    v.extend_from_slice(dib);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wire(pts: &[[f32; 2]], color: [f32; 4]) -> WireModel {
        WireModel {
            points: pts.iter().map(|&[x, y]| [x, y, 0.0]).collect(),
            color,
            ..Default::default()
        }
    }

    #[test]
    fn empty_input_yields_none() {
        assert!(from_wires(&[], [0.0, 0.0, 0.0, 1.0]).is_none());
    }

    #[test]
    fn draws_a_valid_non_blank_dib() {
        let bg = [0.1, 0.1, 0.1, 1.0];
        // A closed square (connected polyline) in a distinct colour.
        let sq = wire(&[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0], [0.0, 0.0]],
            [1.0, 0.0, 0.0, 1.0]);
        let p = from_wires(&[sq], bg).expect("some preview");
        assert_eq!(p.format, PreviewFormat::Bmp);
        // DIB starts with a 40-byte BITMAPINFOHEADER.
        assert_eq!(&p.data[0..4], &40u32.to_le_bytes());
        // Decodes to a square canvas (drawing aspect 1:1 → MAX_DIM²).
        let img = image::load_from_memory(&dib_to_bmp(&p.data)).expect("decodes").to_rgb8();
        assert_eq!((img.width(), img.height()), (MAX_DIM, MAX_DIM));
        // At least one red pixel was drawn (not a blank fill).
        let bg_px = to_rgb(bg);
        assert!(img.pixels().any(|px| px.0 != bg_px), "nothing drawn");
        assert!(img.pixels().any(|px| px.0[0] > 128 && px.0[1] < 64), "square not red");
    }
}

/// Bresenham line, clipped to the image bounds.
fn draw_line(img: &mut RgbImage, (x0, y0): (i32, i32), (x1, y1): (i32, i32), col: Rgb<u8>) {
    let (w, h) = (img.width() as i32, img.height() as i32);
    let (mut x0, mut y0) = (x0, y0);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        if x0 >= 0 && x0 < w && y0 >= 0 && y0 < h {
            img.put_pixel(x0 as u32, y0 as u32, col);
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
