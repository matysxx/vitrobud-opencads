use acadrust::entities::{mesh::Mesh, polygon_mesh::PolygonMesh, Face3D, PolyfaceMesh};
use glam::Vec3;

use crate::command::EntityTransform;
use crate::entities::common::{ro_prop as ro, square_grip};
use crate::entities::traits::{Grippable, PropertyEditable, Transformable, TruckConvertible};
use crate::scene::convert::acad_to_truck::{TruckEntity, TruckObject};
use crate::scene::model::object::{GripApply, GripDef, PropSection};
use crate::scene::model::wire_model::SnapHint;

// ── Face3D ────────────────────────────────────────────────────────────────────

fn v3(v: &acadrust::types::Vector3) -> [f64; 3] {
    [v.x, v.y, v.z]
}

fn dvec3(v: &acadrust::types::Vector3) -> glam::DVec3 {
    glam::DVec3::new(v.x, v.y, v.z)
}

fn v3f32(v: &acadrust::types::Vector3) -> [f32; 3] {
    [v.x as f32, v.y as f32, v.z as f32]
}

impl TruckConvertible for Face3D {
    fn to_truck(&self, _document: &acadrust::CadDocument) -> Option<TruckEntity> {
        let p0 = v3(&self.first_corner);
        let p1 = v3(&self.second_corner);
        let p2 = v3(&self.third_corner);
        let p3 = v3(&self.fourth_corner);
        let p0f = v3f32(&self.first_corner);
        let p1f = v3f32(&self.second_corner);
        let p2f = v3f32(&self.third_corner);
        let p3f = v3f32(&self.fourth_corner);
        let inv = self.invisible_edges;

        // Add edge as a line segment (separated by NaN from previous edges).
        let mut pts: Vec<[f64; 3]> = Vec::new();
        let mut add_edge = |a: [f64; 3], b: [f64; 3]| {
            if !pts.is_empty() {
                pts.push([f64::NAN; 3]);
            }
            pts.push(a);
            pts.push(b);
        };

        if !inv.is_first_invisible() {
            add_edge(p0, p1);
        }
        if !inv.is_second_invisible() {
            add_edge(p1, p2);
        }
        if !inv.is_third_invisible() {
            add_edge(p2, p3);
        }
        if !inv.is_fourth_invisible() {
            add_edge(p3, p0);
        }

        if pts.is_empty() {
            // All edges invisible — show a tiny cross at centroid.
            let cx = (p0[0] + p1[0] + p2[0] + p3[0]) / 4.0;
            let cy = (p0[1] + p1[1] + p2[1] + p3[1]) / 4.0;
            let cz = (p0[2] + p1[2] + p2[2] + p3[2]) / 4.0;
            let s = 0.1_f64;
            pts = vec![[cx - s, cy, cz], [cx + s, cy, cz]];
        }

        Some(TruckEntity {
            object: TruckObject::Lines(pts),
            snap_pts: vec![
                (Vec3::from(p0f).as_dvec3(), SnapHint::Node),
                (Vec3::from(p1f).as_dvec3(), SnapHint::Node),
                (Vec3::from(p2f).as_dvec3(), SnapHint::Node),
                (Vec3::from(p3f).as_dvec3(), SnapHint::Node),
            ],
            tangent_geoms: vec![],
            key_vertices: vec![p0, p1, p2, p3],
            fill_tris: vec![],
        })
    }
}

impl Grippable for Face3D {
    fn grips(&self) -> Vec<GripDef> {
        vec![
            square_grip(0, dvec3(&self.first_corner)),
            square_grip(1, dvec3(&self.second_corner)),
            square_grip(2, dvec3(&self.third_corner)),
            square_grip(3, dvec3(&self.fourth_corner)),
        ]
    }

    fn apply_grip(&mut self, grip_id: usize, apply: GripApply) {
        let corner = match grip_id {
            0 => &mut self.first_corner,
            1 => &mut self.second_corner,
            2 => &mut self.third_corner,
            3 => &mut self.fourth_corner,
            _ => return,
        };
        match apply {
            GripApply::Translate(d) => {
                corner.x += d.x as f64;
                corner.y += d.y as f64;
                corner.z += d.z as f64;
            }
            GripApply::Absolute(p) => {
                corner.x = p.x as f64;
                corner.y = p.y as f64;
                corner.z = p.z as f64;
            }
        }
    }
}

impl PropertyEditable for Face3D {
    fn geometry_properties(&self, _text_style_names: &[String]) -> Vec<PropSection> {
        use crate::entities::common::edit_prop as edit;
        let inv = self.invisible_edges;
        let edge = |hidden: bool| if hidden { "Invisible" } else { "Visible" };
        vec![PropSection {
            title: "Geometry".into(),
            props: vec![
                ro("Current vertex", "f3_current", String::new()),
                edit("Vertex 1 X", "f3_p1x", self.first_corner.x),
                edit("Vertex 1 Y", "f3_p1y", self.first_corner.y),
                edit("Vertex 1 Z", "f3_p1z", self.first_corner.z),
                edit("Vertex 2 X", "f3_p2x", self.second_corner.x),
                edit("Vertex 2 Y", "f3_p2y", self.second_corner.y),
                edit("Vertex 2 Z", "f3_p2z", self.second_corner.z),
                edit("Vertex 3 X", "f3_p3x", self.third_corner.x),
                edit("Vertex 3 Y", "f3_p3y", self.third_corner.y),
                edit("Vertex 3 Z", "f3_p3z", self.third_corner.z),
                edit("Vertex 4 X", "f3_p4x", self.fourth_corner.x),
                edit("Vertex 4 Y", "f3_p4y", self.fourth_corner.y),
                edit("Vertex 4 Z", "f3_p4z", self.fourth_corner.z),
                ro("Edge 1", "f3_edge1", edge(inv.is_first_invisible())),
                ro("Edge 2", "f3_edge2", edge(inv.is_second_invisible())),
                ro("Edge 3", "f3_edge3", edge(inv.is_third_invisible())),
                ro("Edge 4", "f3_edge4", edge(inv.is_fourth_invisible())),
            ],
        }]
    }

    fn apply_geom_prop(&mut self, field: &str, value: &str) {
        let Ok(v) = value.trim().parse::<f64>() else {
            return;
        };
        match field {
            "f3_p1x" => self.first_corner.x = v,
            "f3_p1y" => self.first_corner.y = v,
            "f3_p1z" => self.first_corner.z = v,
            "f3_p2x" => self.second_corner.x = v,
            "f3_p2y" => self.second_corner.y = v,
            "f3_p2z" => self.second_corner.z = v,
            "f3_p3x" => self.third_corner.x = v,
            "f3_p3y" => self.third_corner.y = v,
            "f3_p3z" => self.third_corner.z = v,
            "f3_p4x" => self.fourth_corner.x = v,
            "f3_p4y" => self.fourth_corner.y = v,
            "f3_p4z" => self.fourth_corner.z = v,
            _ => {}
        }
    }
}

impl Transformable for Face3D {
    fn apply_transform(&mut self, t: &EntityTransform) {
        crate::scene::view::transform::apply_standard_entity_transform(self, t, |entity, p1, p2| {
            for corner in [
                &mut entity.first_corner,
                &mut entity.second_corner,
                &mut entity.third_corner,
                &mut entity.fourth_corner,
            ] {
                crate::scene::view::transform::reflect_xy_point(&mut corner.x, &mut corner.y, p1, p2);
            }
        });
    }
}

// ── PolygonMesh (N×M grid) ────────────────────────────────────────────────────

impl TruckConvertible for PolygonMesh {
    fn to_truck(&self, _document: &acadrust::CadDocument) -> Option<TruckEntity> {
        let m = self.m_vertex_count as usize;
        let n = self.n_vertex_count as usize;
        if m == 0 || n == 0 || self.vertices.len() < m * n {
            return None;
        }

        let closed_m = self
            .flags
            .contains(acadrust::entities::PolygonMeshFlags::CLOSED_M);
        let closed_n = self
            .flags
            .contains(acadrust::entities::PolygonMeshFlags::CLOSED_N);

        let pt = |i: usize, j: usize| -> [f64; 3] {
            let v = &self.vertices[i * n + j];
            [v.location.x, v.location.y, v.location.z]
        };

        let mut pts: Vec<[f64; 3]> = Vec::new();
        let mut fill_tris: Vec<[f64; 3]> = Vec::new();

        // Rows (M direction).
        for i in 0..m {
            pts.push([f64::NAN; 3]);
            for j in 0..n {
                pts.push(pt(i, j));
            }
            if closed_n {
                pts.push(pt(i, 0));
            }
        }

        // Columns (N direction).
        for j in 0..n {
            pts.push([f64::NAN; 3]);
            for i in 0..m {
                pts.push(pt(i, j));
            }
            if closed_m {
                pts.push(pt(0, j));
            }
        }

        // Fill: triangulate each grid quad (two triangles per cell).
        let mi = if closed_m { m } else { m - 1 };
        let ni = if closed_n { n } else { n - 1 };
        for i in 0..mi {
            for j in 0..ni {
                let p00 = pt(i, j);
                let p10 = pt((i + 1) % m, j);
                let p01 = pt(i, (j + 1) % n);
                let p11 = pt((i + 1) % m, (j + 1) % n);
                fill_tris.extend_from_slice(&[p00, p10, p11, p00, p11, p01]);
            }
        }

        Some(TruckEntity {
            object: TruckObject::Lines(pts),
            snap_pts: vec![],
            tangent_geoms: vec![],
            key_vertices: vec![],
            fill_tris,
        })
    }
}

impl Grippable for PolygonMesh {
    fn grips(&self) -> Vec<GripDef> {
        self.vertices
            .iter()
            .enumerate()
            .map(|(i, v)| {
                square_grip(
                    i,
                    glam::DVec3::new(v.location.x, v.location.y, v.location.z),
                )
            })
            .collect()
    }

    fn apply_grip(&mut self, grip_id: usize, apply: GripApply) {
        if let Some(v) = self.vertices.get_mut(grip_id) {
            match apply {
                GripApply::Translate(d) => {
                    v.location.x += d.x as f64;
                    v.location.y += d.y as f64;
                    v.location.z += d.z as f64;
                }
                GripApply::Absolute(p) => {
                    v.location.x = p.x as f64;
                    v.location.y = p.y as f64;
                    v.location.z = p.z as f64;
                }
            }
        }
    }
}

impl PropertyEditable for PolygonMesh {
    fn geometry_properties(&self, _text_style_names: &[String]) -> Vec<PropSection> {
        let smooth = match self.smooth_type {
            acadrust::entities::polygon_mesh::SurfaceSmoothType::NoSmooth => "None",
            acadrust::entities::polygon_mesh::SurfaceSmoothType::Quadratic => "Quadratic",
            acadrust::entities::polygon_mesh::SurfaceSmoothType::Cubic => "Cubic",
            acadrust::entities::polygon_mesh::SurfaceSmoothType::Bezier => "Bezier",
        };
        let yesno = |b: bool| if b { "Yes" } else { "No" };
        let first = self.vertices.first();
        // Grid faces: one quad per cell; closed direction adds a wrap row/column.
        let m = self.m_vertex_count.max(0) as i64;
        let n = self.n_vertex_count.max(0) as i64;
        let cells_m = if self.is_closed_m() { m } else { (m - 1).max(0) };
        let cells_n = if self.is_closed_n() { n } else { (n - 1).max(0) };
        let face_count = cells_m * cells_n;
        vec![
            PropSection {
                title: "Geometry".into(),
                props: vec![
                    ro("Vertex", "pm_vertex", String::new()),
                    ro(
                        "Vertex X",
                        "pm_vx",
                        first.map(|v| format!("{:.4}", v.location.x)).unwrap_or_default(),
                    ),
                    ro(
                        "Vertex Y",
                        "pm_vy",
                        first.map(|v| format!("{:.4}", v.location.y)).unwrap_or_default(),
                    ),
                    ro(
                        "Vertex Z",
                        "pm_vz",
                        first.map(|v| format!("{:.4}", v.location.z)).unwrap_or_default(),
                    ),
                    ro("M vertex count", "pm_m", self.m_vertex_count.to_string()),
                    ro("N vertex count", "pm_n", self.n_vertex_count.to_string()),
                    ro("M closed", "pm_closed_m", yesno(self.is_closed_m())),
                    ro("N closed", "pm_closed_n", yesno(self.is_closed_n())),
                    ro("M density", "pm_smooth_m", self.m_smooth_density.to_string()),
                    ro("N density", "pm_smooth_n", self.n_smooth_density.to_string()),
                    ro("Vertex count", "pm_v", self.vertices.len().to_string()),
                    ro("Face count", "pm_faces", face_count.to_string()),
                ],
            },
            PropSection {
                title: "Misc".into(),
                props: vec![ro("Fit/smooth", "pm_smooth", smooth)],
            },
        ]
    }

    fn apply_geom_prop(&mut self, _field: &str, _value: &str) {}
}

impl Transformable for PolygonMesh {
    fn apply_transform(&mut self, t: &EntityTransform) {
        crate::scene::view::transform::apply_standard_entity_transform(self, t, |entity, p1, p2| {
            for v in &mut entity.vertices {
                crate::scene::view::transform::reflect_xy_point(
                    &mut v.location.x,
                    &mut v.location.y,
                    p1,
                    p2,
                );
            }
        });
    }
}

// ── PolyfaceMesh (arbitrary faces with 1-based vertex indices) ────────────────

impl TruckConvertible for PolyfaceMesh {
    fn to_truck(&self, _document: &acadrust::CadDocument) -> Option<TruckEntity> {
        if self.vertices.is_empty() || self.faces.is_empty() {
            return None;
        }

        let get_v = |idx: i16| -> Option<[f64; 3]> {
            let i = (idx.abs() as usize).checked_sub(1)?;
            let v = self.vertices.get(i)?;
            Some([v.location.x, v.location.y, v.location.z])
        };

        let mut pts: Vec<[f64; 3]> = Vec::new();
        let mut fill_tris: Vec<[f64; 3]> = Vec::new();

        for face in &self.faces {
            // Indices: 0 means unused. Negative = invisible edge (still render for wireframe).
            let indices = [face.index1, face.index2, face.index3, face.index4];
            let verts: Vec<[f64; 3]> = indices
                .iter()
                .filter(|&&i| i != 0)
                .filter_map(|&i| get_v(i))
                .collect();

            if verts.len() < 2 {
                continue;
            }
            pts.push([f64::NAN; 3]);
            for &p in &verts {
                pts.push(p);
            }
            // Close the face polygon.
            pts.push(verts[0]);

            // Fan-triangulate the face for solid fill.
            if verts.len() >= 3 {
                for i in 1..(verts.len() - 1) {
                    fill_tris.push(verts[0]);
                    fill_tris.push(verts[i]);
                    fill_tris.push(verts[i + 1]);
                }
            }
        }

        Some(TruckEntity {
            object: TruckObject::Lines(pts),
            snap_pts: vec![],
            tangent_geoms: vec![],
            key_vertices: vec![],
            fill_tris,
        })
    }
}

impl Grippable for PolyfaceMesh {
    fn grips(&self) -> Vec<GripDef> {
        self.vertices
            .iter()
            .enumerate()
            .map(|(i, v)| {
                square_grip(
                    i,
                    glam::DVec3::new(v.location.x, v.location.y, v.location.z),
                )
            })
            .collect()
    }

    fn apply_grip(&mut self, grip_id: usize, apply: GripApply) {
        if let Some(v) = self.vertices.get_mut(grip_id) {
            match apply {
                GripApply::Translate(d) => {
                    v.location.x += d.x as f64;
                    v.location.y += d.y as f64;
                    v.location.z += d.z as f64;
                }
                GripApply::Absolute(p) => {
                    v.location.x = p.x as f64;
                    v.location.y = p.y as f64;
                    v.location.z = p.z as f64;
                }
            }
        }
    }
}

impl PropertyEditable for PolyfaceMesh {
    fn geometry_properties(&self, _text_style_names: &[String]) -> Vec<PropSection> {
        let smooth = match self.smooth_surface {
            acadrust::entities::PolyfaceSmoothType::None => "None",
            acadrust::entities::PolyfaceSmoothType::Quadratic => "Quadratic",
            acadrust::entities::PolyfaceSmoothType::Cubic => "Cubic",
            acadrust::entities::PolyfaceSmoothType::Bezier => "Bezier",
        };
        let first = self.vertices.first();
        vec![
            PropSection {
                title: "Geometry".into(),
                props: vec![
                    ro("Vertex", "pfm_vertex", String::new()),
                    ro(
                        "Vertex X",
                        "pfm_vx",
                        first.map(|v| format!("{:.4}", v.location.x)).unwrap_or_default(),
                    ),
                    ro(
                        "Vertex Y",
                        "pfm_vy",
                        first.map(|v| format!("{:.4}", v.location.y)).unwrap_or_default(),
                    ),
                    ro(
                        "Vertex Z",
                        "pfm_vz",
                        first.map(|v| format!("{:.4}", v.location.z)).unwrap_or_default(),
                    ),
                    // Polyface meshes store an explicit vertex/face list rather
                    // than an M×N grid, so the grid-only rows are not applicable.
                    ro("M vertex count", "pfm_m", String::new()),
                    ro("N vertex count", "pfm_n", String::new()),
                    ro("M closed", "pfm_closed_m", String::new()),
                    ro("N closed", "pfm_closed_n", String::new()),
                    ro("M density", "pfm_density_m", String::new()),
                    ro("N density", "pfm_density_n", String::new()),
                    ro("Vertex count", "pfm_v", self.vertices.len().to_string()),
                    ro("Face count", "pfm_f", self.faces.len().to_string()),
                ],
            },
            PropSection {
                title: "Misc".into(),
                props: vec![ro("Fit/smooth", "pfm_smooth", smooth)],
            },
        ]
    }

    fn apply_geom_prop(&mut self, _field: &str, _value: &str) {}
}

impl Transformable for PolyfaceMesh {
    fn apply_transform(&mut self, t: &EntityTransform) {
        crate::scene::view::transform::apply_standard_entity_transform(self, t, |entity, p1, p2| {
            for v in &mut entity.vertices {
                crate::scene::view::transform::reflect_xy_point(
                    &mut v.location.x,
                    &mut v.location.y,
                    p1,
                    p2,
                );
            }
        });
    }
}

// ── Mesh (SubD mesh) ──────────────────────────────────────────────────────────
//
// Modern subdivision mesh — distinct from PolygonMesh. The render path emits
// the file's per-edge wireframe and triangulates each face into fill_tris so
// solid views still draw a shaded surface. Subdivision-level smoothing is
// honoured only as metadata; we don't run a Catmull-Clark refinement pass
// here yet.

impl TruckConvertible for Mesh {
    fn to_truck(&self, _document: &acadrust::CadDocument) -> Option<TruckEntity> {
        if self.vertices.is_empty() {
            return None;
        }
        let get = |i: usize| -> Option<[f64; 3]> { self.vertices.get(i).map(|v| [v.x, v.y, v.z]) };

        let mut pts: Vec<[f64; 3]> = Vec::new();
        if !self.edges.is_empty() {
            for edge in &self.edges {
                if let (Some(a), Some(b)) = (get(edge.start), get(edge.end)) {
                    pts.push([f64::NAN; 3]);
                    pts.push(a);
                    pts.push(b);
                }
            }
        } else {
            for face in &self.faces {
                if face.vertices.len() < 2 {
                    continue;
                }
                pts.push([f64::NAN; 3]);
                for &vi in &face.vertices {
                    if let Some(p) = get(vi) {
                        pts.push(p);
                    }
                }
                if let Some(first) = face.vertices.first().and_then(|&i| get(i)) {
                    pts.push(first);
                }
            }
        }

        // Fan-triangulate each face into fill_tris so shaded views render
        // the mesh as a solid surface.
        let mut fill_tris: Vec<[f64; 3]> = Vec::new();
        for face in &self.faces {
            if face.vertices.len() < 3 {
                continue;
            }
            let v0 = match get(face.vertices[0]) {
                Some(p) => p,
                None => continue,
            };
            for tri in 1..(face.vertices.len() - 1) {
                let v1 = match get(face.vertices[tri]) {
                    Some(p) => p,
                    None => continue,
                };
                let v2 = match get(face.vertices[tri + 1]) {
                    Some(p) => p,
                    None => continue,
                };
                fill_tris.push(v0);
                fill_tris.push(v1);
                fill_tris.push(v2);
            }
        }

        let snap_pts: Vec<(glam::DVec3, SnapHint)> = self
            .vertices
            .iter()
            .map(|v| (glam::DVec3::new(v.x, v.y, v.z), SnapHint::Node))
            .collect();
        let key_vertices: Vec<[f64; 3]> = self.vertices.iter().map(|v| [v.x, v.y, v.z]).collect();

        Some(TruckEntity {
            object: TruckObject::Lines(pts),
            snap_pts,
            tangent_geoms: vec![],
            key_vertices,
            fill_tris,
        })
    }
}

impl Grippable for Mesh {
    fn grips(&self) -> Vec<GripDef> {
        self.vertices
            .iter()
            .enumerate()
            .map(|(i, v)| square_grip(i, glam::DVec3::new(v.x, v.y, v.z)))
            .collect()
    }

    fn apply_grip(&mut self, grip_id: usize, apply: GripApply) {
        if let Some(v) = self.vertices.get_mut(grip_id) {
            match apply {
                GripApply::Translate(d) => {
                    v.x += d.x as f64;
                    v.y += d.y as f64;
                    v.z += d.z as f64;
                }
                GripApply::Absolute(p) => {
                    v.x = p.x as f64;
                    v.y = p.y as f64;
                    v.z = p.z as f64;
                }
            }
        }
    }
}

impl PropertyEditable for Mesh {
    fn geometry_properties(&self, _text_style_names: &[String]) -> Vec<PropSection> {
        // Watertight when every face edge is shared by exactly two faces
        // (closed manifold). Empty meshes are not watertight.
        let mut edge_use: std::collections::HashMap<(usize, usize), u32> =
            std::collections::HashMap::new();
        for face in &self.faces {
            let vs = &face.vertices;
            for i in 0..vs.len() {
                let a = vs[i];
                let b = vs[(i + 1) % vs.len()];
                let key = if a < b { (a, b) } else { (b, a) };
                *edge_use.entry(key).or_insert(0) += 1;
            }
        }
        let watertight =
            !self.faces.is_empty() && edge_use.values().all(|&c| c == 2);
        vec![PropSection {
            title: "Geometry".into(),
            props: vec![
                ro(
                    "Level of Smoothness",
                    "msh_subdiv",
                    self.subdivision_level.to_string(),
                ),
                ro("Number of Faces", "msh_f", self.faces.len().to_string()),
                ro("Number of Grips", "msh_grips", self.vertices.len().to_string()),
                ro(
                    "Watertight",
                    "msh_watertight",
                    if watertight { "Yes" } else { "No" },
                ),
            ],
        }]
    }

    fn apply_geom_prop(&mut self, _field: &str, _value: &str) {}
}

impl Transformable for Mesh {
    fn apply_transform(&mut self, t: &EntityTransform) {
        crate::scene::view::transform::apply_standard_entity_transform(self, t, |entity, p1, p2| {
            for v in &mut entity.vertices {
                crate::scene::view::transform::reflect_xy_point(&mut v.x, &mut v.y, p1, p2);
            }
        });
    }
}
