use std::io::Read;

#[derive(Clone)] 
pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn set_used(&self) -> &Self {
        unsafe{ gl::UseProgram(self.id); }
        self
    }

    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()) }
        }

        unsafe { gl::LinkProgram(program_id); }  

        let mut status_link = 1;
        
        unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut status_link); }
        
        if status_link == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            } 
            
            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                 );
            }

            return Err(error.to_string_lossy().into_owned())
        }


        for shader in shaders {
            unsafe{ gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program {id: program_id})
    }

    pub fn new(id: u32) -> Self {
        Self {
            id: id,
        }
    }
}


// Program segfaults when implementing drop. Perhaps rust takes care of it automatically?

/*
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
*/

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(
        source: &std::ffi::CStr,
        kind: gl::types::GLenum
    ) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_frag_source (source_path: &str) -> Result<Shader, String> {
        let source = std::ffi::CString::new(Shader::read_file(source_path)).unwrap();
        Shader::from_source(source.as_c_str(), gl::FRAGMENT_SHADER)
    }

    pub fn from_vert_source (source_path: &str) -> Result<Shader, String> {
        let source = std::ffi::CString::new(Shader::read_file(source_path)).unwrap();
        Shader::from_source(source.as_c_str(), gl::VERTEX_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn read_file(filename: &str) -> String {
        let file_path = std::path::Path::new(filename);
        let display_name = file_path.display();
    
        let mut file_handle = match std::fs::File::open(&file_path) {
            Err(err_msg) => panic!("Couldn't open file \"{}\". Reason: {}", display_name, err_msg),
            Ok(file) => file
        };
    
        let mut file_contents = String::new();
        match file_handle.read_to_string(&mut file_contents) {
            Err(err_msg) => panic!("Couldn't read file \"{}\". Reason: {}", display_name, err_msg),
            Ok(_) => ()
        }

        return file_contents
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id)
        }
    }
}

fn shader_from_source(source: &std::ffi::CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe{ gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(),std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH , &mut len);
        }
        
        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }



    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> std::ffi::CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
} 
