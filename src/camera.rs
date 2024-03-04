use nalgebra as na;

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    pub position: na::Vector3::<f32>,
    pub direction: na::Vector3::<f32>,

    pub transformation: na::Matrix4::<f32>,

    pub fov: f32,

    pub mouse_pos: na::Vector2::<f64>,
    pub mouse_diff: na::Vector2::<f64>,

    pub aspect_ratio: f32,
}

impl Camera {
    pub fn get_view_matrix(&mut self) -> na::Matrix4<f32> {
        // Translate all objects so that camera is at centre (i.e. take away cam pos from all
        // objects)
        self.transformation = na::Matrix4::<f32>::from_axis_angle(&na::Vector3::<f32>::x_axis(), self.pitch)
            * na::Matrix4::<f32>::from_axis_angle(&na::Vector3::<f32>::y_axis(), self.yaw)
            .prepend_translation(&(-self.position));

        self.direction =  ((na::Matrix4::<f32>::from_axis_angle(&na::Vector3::<f32>::x_axis(), self.pitch)
            * na::Matrix4::<f32>::from_axis_angle(&na::Vector3::<f32>::y_axis(), self.yaw))
            .try_inverse().unwrap()
            * na::Vector4::<f32>::new(0.0,0.0,-1.0,1.0)).xyz();

        self.transformation
    }

    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            yaw: 0.0, pitch: 0.0, roll: 0.0,
            position: na::Vector3::<f32>::zeros(),
            direction: na::Vector3::<f32>::zeros(),
            transformation: na::Matrix4::<f32>::identity(),
            mouse_pos: na::Vector2::<f64>::zeros(),
            mouse_diff: na::Vector2::<f64>::zeros(),
            aspect_ratio: aspect_ratio,
            fov: 3.14/3.0,
                //7.0*3.14/12.0 // 120 degrees
        }
    }

}

