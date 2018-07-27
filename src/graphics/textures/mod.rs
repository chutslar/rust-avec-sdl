extern crate image;
extern crate gl;
extern crate nalgebra as na;

use std::{path, ptr};
use graphics::textures::image::GenericImage;
use std::os::raw::c_void;
use graphics::{program, buffer, vertex_array, GLDataType};

pub struct Texture {
    tex_id: u32,
    vbo: buffer::Buffer,
    vao: vertex_array::VertexArrayObject,
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

        let vao = vertex_array::VertexArrayObject::new(&vbo);
        vao.set_attribute(
            &vbo,
            0, // index of attrib (layout = 0)
            3, // number of components per attrib
            GLDataType::Float, // data type
            false, // normalized
            20, // byte offset
            0
        );
        vao.set_attribute(
            &vbo,
            1, // index of attrib (layout = 0)
            2, // number of components per attrib
            GLDataType::Float, // data type
            false, // normalized
            20, // byte offset
            12
        );

        let program = program::Program::standard().unwrap();

        Ok(Texture { 
            tex_id,
            vbo,
            vao,
            ebo,
            program
        })
    }

    pub fn draw(&self) {
        self.program.set_used(true);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
        }
        self.vao.bind();
        self.ebo.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES, // Mode
                6, // number of vertices to draw
                gl::UNSIGNED_INT, // type
                ptr::null() // buffer offset
            );
        }
        self.vao.unbind();
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