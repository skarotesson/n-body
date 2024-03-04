pub mod file_parser;
use image::EncodableLayout;

use crate::shaderutils;

// Model holds raw vertex data
pub struct Model {
    // Assigned automatically by GenBuffers
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
    ebo: gl::types::GLuint,
    tex: gl::types::GLuint,

    // Amount of indices. Used in glDrawElements command
    pub ebo_length: u32,

    // Raw texture data
    //texture: Vec<u8>,
    
    // e.g. gl::TRIANGLES, gl::
    primitive: gl::types::GLenum,
}

impl Drop for Model  {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteTextures(1, &self.tex);
        }
    }
}

impl Model {

    fn create_vbo(mut self) -> Self {
        unsafe{ gl::GenBuffers(1, &mut self.vbo); }
        self
    }
    
    fn create_vao(mut self) -> Self {
        unsafe{ gl::GenVertexArrays(1, &mut self.vao); }
        self
    }

    fn create_ebo(mut self) -> Self {
        unsafe{ gl::GenBuffers(1, &mut self.ebo); }
        self
    }
    
    pub fn load_texture(obj: &mut Self, filename: &str, shader_program: &shaderutils::Program){
        let texture = image::open(filename).unwrap()
            .flipv()
            .to_rgba8();

        unsafe{
            gl::GenTextures(1, &mut obj.tex);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, obj.tex);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGBA as i32, 
                texture.width() as i32, texture.height() as i32,
                0, gl::RGBA as u32,
                gl::UNSIGNED_BYTE, texture.as_bytes().as_ptr() as *const core::ffi::c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        
        let uniform_tex = std::ffi::CString::new("tex1").unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(shader_program.get_id(), uniform_tex.as_ptr()),
                0
            );
        }
    }

    fn setup_model(self, vertices: &Vec<f32>, indices: &Vec<u32>) -> Self {
        unsafe{
            gl::BindVertexArray(self.vao);
           
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );


            gl::VertexAttribPointer(
                0, 3, gl::FLOAT, gl::FALSE,
                (5* std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),              

            );
            gl::EnableVertexAttribArray(0);


            gl::VertexAttribPointer(
                1, 2, gl::FLOAT, gl::FALSE,
                (5* std::mem::size_of::<f32>()) as gl::types::GLint,
                (3* std::mem::size_of::<f32>()) as *const gl::types::GLvoid
            );
            gl::EnableVertexAttribArray(1);
            


            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        self
    }

    pub fn draw_model(&self) {
        unsafe {
            // Binding VAO implicitly binds EBO
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            
            gl::BindVertexArray(self.vao);
            gl::DrawElements(self.primitive, self.ebo_length as i32, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    pub fn new(vertices: &Vec<f32>, indices: &Vec<u32>) -> Self {
        Self { vbo: 0, ebo: 0, vao: 0, tex: 0, ebo_length: indices.len() as u32, primitive: gl::TRIANGLES,}
            .create_vbo()
            .create_ebo()
            .create_vao()
            .setup_model(&vertices, &indices)
    }  
}



