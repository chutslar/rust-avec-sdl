extern crate gl;

use std::os::raw::c_void;
use graphics::{buffer, GLDataType};


pub struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    pub fn new(vbo: &buffer::Buffer) -> Self {
        match vbo.target() {
            buffer::BufferTarget::ArrayBuffer => (),
            _ => {
                println!(
                    "Warning: Trying to attach VAO to non-VBO buffer ({}:{}:{})", 
                    file!(), line!(), column!());
            }
        }
        vbo.bind();
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        vbo.unbind();
        VertexArrayObject { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn set_attribute(&self, vbo: &buffer::Buffer,
        layout_index: u32, num_components: i32, data_type: GLDataType,
        normalized: bool, stride: isize, byte_offset: usize) {
        match vbo.target() {
            buffer::BufferTarget::ArrayBuffer => (),
            _ => {
                println!(
                    "Warning: Trying to attach VAO to non-VBO buffer ({}:{}:{})", 
                    file!(), line!(), column!());
            }
        }
        vbo.bind();
        self.bind();
        unsafe {
            gl::EnableVertexAttribArray(layout_index);
            gl::VertexAttribPointer(
                layout_index,
                num_components,
                data_type as u32,
                if normalized { gl::TRUE } else { gl::FALSE },
                stride as gl::types::GLint,
                byte_offset as *const c_void
            );
        }
        self.unbind();
        vbo.unbind();
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
