extern crate sdl2;
extern crate gl;

use std::ptr;

pub mod shaders;
pub mod program;

pub struct Triangles {
    vertices: Vec<f32>,
    vao: gl::types::GLuint,
    program: program::Program,
}

impl Triangles {
    pub fn new(vertices: Vec<f32>) -> Self {
        let mut tri = Triangles { 
            vertices, 
            vao: 0,
            program: program::Program::standard().unwrap(),
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

        //let mut vao: gl::types::GLuint = 0;
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
                12 as gl::types::GLint, // byte offset
                ptr::null()
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        self.program.set_used();
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index
                3 // number of indices to render
            )
        }
    }
}
