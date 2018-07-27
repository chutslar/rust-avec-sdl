extern crate image;
extern crate gl;
extern crate nalgebra as na;

use std::{path, ptr};
use graphics::textures::image::GenericImage;
use std::os::raw::c_void;
use graphics::{program, buffer};

pub struct Texture {
    tex_id: u32,
    vbo: buffer::Buffer,
    vao: u32,
    ebo: buffer::Buffer,
    program: program::Program,
}


impl Texture {
    pub fn load<P>(path: P) -> Result<Texture, String>
        where P: AsRef<path::Path>  {
        let img = match image::open(path) {
            Ok(i) => i.flipv(), // Flip for GL
            Err(e) => { return Err(e.to_string()); }
        };
        let mut tex_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexParameteri(
                gl::TEXTURE_2D, 
                gl::TEXTURE_WRAP_S, 
                gl::REPEAT as i32
            );	
            gl::TexParameteri(
                gl::TEXTURE_2D, 
                gl::TEXTURE_WRAP_T, 
                gl::REPEAT as i32
            );
            gl::TexParameteri(
                gl::TEXTURE_2D, 
                gl::TEXTURE_MIN_FILTER, 
                gl::LINEAR as i32
            );
            gl::TexParameteri(
                gl::TEXTURE_2D, 
                gl::TEXTURE_MAG_FILTER, 
                gl::LINEAR as i32
            );
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0, // Mipmap level
                gl::RGBA as i32, // Format
                img.dimensions().0 as i32, // Width
                img.dimensions().1 as i32, // Height
                0, // Legacy
                gl::RGBA, // Format
                gl::UNSIGNED_BYTE, // Datatype
                img.raw_pixels().as_ptr() as *const c_void
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        let vertices: Vec<f32> = vec![
            // positions     // texture coords
            0.5, 0.5, 0.0,   1.0, 1.0,  // top right
            0.5, -0.5, 0.0,  1.0, 0.0,  // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0,  // bottom left
            -0.5, 0.5, 0.0,  0.0, 1.0,  // top left
        ];


        let indices: Vec<u32> = vec![
            0, 1, 3, // first triangle
            1, 2, 3  // second triangle 
        ];

        let vbo = buffer::Buffer::new(
            buffer::BufferTarget::ArrayBuffer,
            buffer::BufferUsage::StaticDraw,
            vertices
        );

        let ebo = buffer::Buffer::new(
            buffer::BufferTarget::ElementArrayBuffer,
            buffer::BufferUsage::StaticDraw,
            indices
        );

        let program = program::Program::standard().unwrap();

        let mut texture = Texture { 
            tex_id,
            vbo,
            vao: 0,
            ebo,
            program
        };
        texture.init();

        Ok(texture)
    }

    fn init(&mut self) {
        self.vbo.bind();

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
                20 as gl::types::GLint, // byte offset
                ptr::null()
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, // index of attrib (layout = 1)
                2, // number of components per attrib
                gl::FLOAT, // data type
                gl::FALSE, // normalized
                20 as gl::types::GLint, // byte offset
                12 as *const c_void
            );
            gl::BindVertexArray(0);
        }

        self.vbo.unbind();
    }

    pub fn draw(&self) {
        self.program.set_used(true);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            
            gl::BindVertexArray(self.vao);
        }
        self.ebo.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES, // Mode
                6, // number of vertices to draw
                gl::UNSIGNED_INT, // type
                ptr::null() // buffer offset
            );
            gl::BindVertexArray(0);
        }
        self.ebo.unbind();
        self.program.set_used(false);
    }
}


/* If you want to send the image again:
    gl::TexSubImage2D(
    gl::TEXTURE_2D, // texture
    0, // level
    0, // xoffset
    0, // yoffset
    self.img.dimensions().0 as i32, // width
    self.img.dimensions().1 as i32, // height
    gl::RGBA, // format
    gl::UNSIGNED_BYTE, // datatype
    self.img.raw_pixels().as_ptr() as *const c_void
);
*/