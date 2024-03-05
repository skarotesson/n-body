use nalgebra as na;

use crate::shaderutils;
use crate::modelutils;

pub struct RenderObject {
    pub model: modelutils::Model,

    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,

    pub position: na::Vector3<f32>,
    pub velocity: na::Vector3<f32>,
    pub acceleration: na::Vector3<f32>,

    pub scale: f32,

    pub mass: f32,
}

impl RenderObject {
    pub fn render(&self, shader_program: &shaderutils::Program) {

        shader_program.set_used(); 
        let transformation = na::Matrix4::from_euler_angles(self.pitch, self.yaw, self.roll)
            .append_scaling(self.scale)
            .append_translation(&self.position);

        unsafe{
            let uniform_transform = std::ffi::CString::new("model").unwrap();
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(shader_program.get_id(),uniform_transform.as_ptr()),
                1, gl::FALSE, &transformation[(0, 0)] as *const f32);
        }

        self.model.draw_model();
    }

    pub fn new(model: modelutils::Model) -> Self {
        Self {
            model: model, 
            roll: 0.0, pitch: 0.0, yaw: 0.0,
            position: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            velocity: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            acceleration: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            scale: 0.5,
            mass: 1.0,
        }
    }
}


