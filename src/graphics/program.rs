extern crate gl;

use std::ffi::{CString, CStr};
use std::ptr;
use graphics::shaders;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[shaders::Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(program_id); }
        
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            return Err(get_program_error_log(program_id));
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program { id: program_id })
    }

    pub fn standard() -> Result<Program, String> {
        let standard_vert = shaders::Shader::from_vert_source(
            &CString::new(shaders::STANDARD_VERTEX_SOURCE_STR).unwrap()
        )?;
        let standard_frag = shaders::Shader::from_frag_source(
            &CString::new(shaders::STANDARD_FRAG_SOURCE_STR).unwrap()
        )?;
        Program::from_shaders(&[standard_vert, standard_frag])
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn get_program_error_log(program_id: gl::types::GLuint) -> String {
    let mut len: gl::types::GLint = 0;
    unsafe {
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
    }

     // Create error buffer
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    // Fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    // Convert buffer to CString
    let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

    unsafe {
        gl::GetProgramInfoLog(
            program_id,
            len,
            ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
        );
    }

    return error.to_string_lossy().into_owned();
}