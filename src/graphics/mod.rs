extern crate sdl2;
extern crate gl;

use std::{ptr, mem};

pub mod renderer;
pub mod shaders;
pub mod material;
pub mod program;
pub mod textures;
pub mod buffer;
pub mod vertex_array;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLDataType {
    Byte = gl::BYTE, 
    UnsignedByte = gl::UNSIGNED_BYTE, 
    Short = gl::SHORT, 
    UnsignedShort = gl::UNSIGNED_SHORT, 
    Int = gl::INT, 
    UnsignedInt = gl::UNSIGNED_INT, 
    HalfFloat = gl::HALF_FLOAT, 
    Float = gl::FLOAT, 
    Double = gl::DOUBLE, 
    Fixed = gl::FIXED, 
    // Don't see need for the following datatypes, and so have excluded them:
    //  -GL_INT_2_10_10_10_REV, 
    //  -GL_UNSIGNED_INT_2_10_10_10_REV,
    //  -GL_UNSIGNED_INT_10F_11F_11F_REV
}

pub struct Color {
    r: f32, g: f32, b: f32, a: f32,
}

pub struct Triangles {
    vertices: Vec<f32>,
    vao: u32,
    program: program::Program,
}

impl Triangles {
    pub fn new(vertices: Vec<f32>) -> Self {
        let mut tri = Triangles { 
            vertices, 
            vao: 0,
            program: program::Program::triangle().unwrap(),
        };
        tri.init();
        tri
    }

    fn init(&mut self) {
        let mut vbo: gl::types::GLuint = 0;
        // Send buffer data
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (self.vertices.len() * 4)  as gl::types::GLsizeiptr, // size in bytes
                self.vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
        }

        // Bind VAO information, then unbind VAO and VBO
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, // index of attrib (layout = 0)
                3, // number of components per attrib
                gl::FLOAT, // data type
                gl::FALSE, // normalized
                3 * mem::size_of::<f32>() as gl::types::GLint, // byte offset
                ptr::null()
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        self.program.set_used(true);
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index
                3 // number of indices to render
            )
        }
        self.program.set_used(false);
    }
}

pub struct Rectangle {
    
}