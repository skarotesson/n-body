use glfw::Context;

use nalgebra as na;

use crate::modelutils;
use crate::shaderutils;
use crate::object;
use crate::camera;

pub struct Natu {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    pub shader_program: shaderutils::Program,
    pub camera: camera::Camera,
    // Hashmap was chosen over vector to allow human readable object access
    // e.g. let object = objects.get_mut("monkey");
    pub objects: std::collections::HashMap<String, object::RenderObject>,

    pub fps: f64,
    // Time between frames. Useful in physics calculations.
    pub delta: f64,
    // Internal variables used to calculate time of adjacent frames and calculate delta
    target_time: f64,
    before_time: f64,
}

impl Natu {
    fn window_event_handle(window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(glfw::Key::Q, _, glfw::Action::Release, _) => {
                window.set_should_close(true)
            } 
            _ => {}
        }
    }

    // Miscellaneous state updates:
    // - Update delta
    // - Handle events
    // - Update camera based on input
    // - Render objects in scene
    pub fn update(&mut self) {
        // Compute delta for use in controls and physics|
        self.delta = self.glfw.get_time() - self.before_time;
        self.before_time = self.glfw.get_time();

        // Poll events (keyboard etc.)
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            Self::window_event_handle(&mut self.window, event);
        }
        
        self.update_camera();

        // Renders objects in scene
        self.render();
    }

    // Pause until target time is reached. Makeshift framelimiter to enforce framerate
    // Frames will render continuously if condition is not met
    pub fn pause_until_frame(&mut self) {
        while self.glfw.get_time() < self.target_time + 1.0/self.fps {}
        self.target_time += 1.0/self.fps;
    }

    pub fn update_camera(&mut self) { 
        // Store x and y coordinates of mouse 
        let (mx, my): (f64, f64) = self.window.get_cursor_pos();
        // Convert to Vec2 to simplify subsequent operations
        let updated_mouse = na::Vector2::new(mx, my);

        // Find diff in mouse to find net movement
        self.camera.mouse_diff = updated_mouse - self.camera.mouse_pos;
        self.camera.mouse_pos = updated_mouse;
        // Adjust movement with time 
        self.camera.mouse_diff *= self.delta;

        // x movement corresponds to the camera looking laterally 
        self.camera.yaw += self.camera.mouse_diff.x as f32;

        // Comparisons to clamp camera pitch. >= is used because pitch change is jittery when using
        // just > (position is clamped to value, but condition is not met on next iteration, so clamp must occur again).
        // mouse_diff is compared because pitch gets locked at max or min otherwise.
        const MAX_PITCH: f32 = 3.05/2.0;
        if self.camera.pitch >= MAX_PITCH && self.camera.mouse_diff.y > 0.0 {
            self.camera.pitch = MAX_PITCH
        } else if self.camera.pitch <= -MAX_PITCH && self.camera.mouse_diff.y < 0.0 {
            self.camera.pitch = -MAX_PITCH;
        } else {
            self.camera.pitch += self.camera.mouse_diff.y as f32;
        }


        if self.window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Press {
            self.camera.position += 3.0 * self.camera.direction * self.delta as f32;
        }
        if self.window.get_mouse_button(glfw::MouseButtonRight) == glfw::Action::Press {
            self.camera.position -= 3.0 * self.camera.direction * self.delta as f32;
        }


        // Pass view matrix and projection matrix uniforms to shader
        // We define the names using a variable to avoid the rust compiler shouting at us...
        let uniform_view = std::ffi::CString::new("view").unwrap();
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_program.get_id(), uniform_view.as_ptr()),
                1, gl::FALSE,
                &self.camera.get_view_matrix()[(0,0)] as *const f32
            );
        }

        let uniform_proj = std::ffi::CString::new("proj").unwrap();
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_program.get_id(), uniform_proj.as_ptr()),
                1, gl::FALSE,
                &na::Matrix4::<f32>::new_perspective(self.camera.aspect_ratio, self.camera.fov, 0.01, 10000.0)[(0,0)] as *const f32
            );
        }
    }

    // Loads model and texture from fs into objects hashmap
    pub fn load_object(
            &mut self,
            path: &str,
            name: &str,
            texture_path: &str,
        )
    {

        // Create a new object. We don't pass it directly because we have to load the texture
        // first.
        let mut obj = object::RenderObject::new(modelutils::file_parser::file2obj(&path));
        if texture_path != "" {
            modelutils::Model::load_texture(&mut obj.model, texture_path, &self.shader_program);
        }
        self.objects.insert(
            name.to_string(),
            obj
        );
    }

    pub fn get(&mut self, object_name: &str) -> &mut object::RenderObject {
        self.objects.get_mut(object_name).unwrap()
    }

    pub fn render(&mut self) {
        unsafe {
            // Clear bits to set background colour and depth buffer check
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.objects.iter()
            .for_each(|(_, object)| object.render(&self.shader_program));
        self.window.swap_buffers();
    }


    pub fn init() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::Decorated(false));
        glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

        let width: u32 = 1200;
        let height: u32 = 600;

        let (mut window, events) = glfw.create_window(width, height, "Window", glfw::WindowMode::Windowed)
            .expect("GLFW window creation failed.");

        // Ignores acceleration effects. Good for FPS style camera
        window.set_raw_mouse_motion(true);
        // Prevents cursor from escaping window
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        window.make_current();
        window.set_key_polling(true);

        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        // Create and compile shaders
        // TODO: Shaders are hardcoded.
        let vert_shader = shaderutils::Shader::from_vert_source("./resources/shaders/model_shader/shader.vert").unwrap();
        let frag_shader = shaderutils::Shader::from_frag_source("./resources/shaders/model_shader/shader.frag").unwrap();

        let shader_program = shaderutils::Program::from_shaders(
            &[vert_shader, frag_shader]
        ).unwrap();

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(0.0,0.0,0.0,1.0);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::Enable(gl::DEPTH_TEST)
        }

        Self {
            glfw: glfw,
            window: window,
            events: events,

            camera: camera::Camera::new((width/height) as f32),
            shader_program: shader_program,
            objects: std::collections::HashMap::new(),

            fps: 60.0,
            delta: 0.0,
            target_time: 0.0,
            before_time: 0.0,
        }
    }
}
