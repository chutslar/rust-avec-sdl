#![allow(dead_code)]

extern crate gl;

use graphics::Color;

pub struct Renderer {
    
}

impl Renderer {
    fn new() -> Self {
        Renderer {}
    }

    fn set_clear_color(&self, color: &Color) {
        unsafe {
            gl::ClearColor(color.a, color.b, color.g, color.r);
        }
    }

    fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn enable_blending() {
        unsafe {
            // Set a blend function so we can have transparency
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // Enable blending
            gl::Enable(gl::BLEND);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {

    }
}