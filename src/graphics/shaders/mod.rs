extern crate gl;

use std::ffi::{CString, CStr};
use std::ptr;

pub static STANDARD_VERTEX_SOURCE_STR: &str = 
    include_str!("standard.vert");

pub static STANDARD_FRAG_SOURCE_STR: &str = 
    include_str!("standard.frag");

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    // Compile shader from string source
    fn from_source(
        source: &CStr,
        kind: gl::types::GLuint
    ) -> Result<Shader, String> {
        
        let id = unsafe { gl::CreateShader(kind) };
        let mut success : gl::types::GLint = 1;
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(id);
            // Check shader compiled successfully
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        // Compilation failed
        if success == 0 {
            return Err(get_shader_error_log(id));
        }

        // Compilation succeeded
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    // Delete shader id to clean up
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn get_shader_error_log(id: gl::types::GLuint) -> String {
    // Length of GL error log
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        
        // Create error buffer
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        // Fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len as usize));
        // Convert buffer to CString
        let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

        unsafe {
            gl::GetShaderInfoLog(
                id, // Shader id
                len, // Length of error log
                ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar // Error buffer
            );
        }
        error.to_string_lossy().into_owned()
}

