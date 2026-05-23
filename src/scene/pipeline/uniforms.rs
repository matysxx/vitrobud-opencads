use crate::scene::camera::Camera;
use iced::Rectangle;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub view_proj: glam::Mat4,
    pub camera_pos: glam::Vec4,
    pub viewport_size: [f32; 2],
    /// World units per screen pixel at the current zoom. Used by the
    /// hatch shader to substitute solid fill when pattern line spacing
    /// falls below ~2 px (Phase 3.3 LOD).
    pub world_per_pixel: f32,
    /// LWDISPLAY toggle (1.0 = show lineweights, 0.0 = force 1 px).
    /// Read by the wire shader so the toggle does not require a retessellate.
    pub lwdisplay_enable: f32,
}

impl Uniforms {
    pub fn new(camera: &Camera, bounds: Rectangle, lwdisplay_enable: bool) -> Self {
        let half_h = camera.ortho_size();
        let world_per_pixel = if bounds.height > 0.0 {
            (2.0 * half_h) / bounds.height
        } else {
            0.0
        };
        Self {
            view_proj: camera.view_proj(bounds),
            camera_pos: camera.position_vec4(),
            viewport_size: [bounds.width, bounds.height],
            world_per_pixel,
            lwdisplay_enable: if lwdisplay_enable { 1.0 } else { 0.0 },
        }
    }
}
