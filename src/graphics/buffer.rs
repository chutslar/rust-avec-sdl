extern crate gl;

use std::mem;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum BufferTarget {
    ArrayBuffer = gl::ARRAY_BUFFER,
    AtomicCounterBuffer = gl::ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = gl::COPY_READ_BUFFER,
    CopyWriteBuffer = gl::COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = gl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = gl::DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = gl::PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,
    QueryBuffer = gl::QUERY_BUFFER,
    ShaderStorageBuffer = gl::SHADER_STORAGE_BUFFER,
    TextureBuffer = gl::TEXTURE_BUFFER,
    TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = gl::UNIFORM_BUFFER,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum BufferUsage {
    StreamDraw = gl::STREAM_DRAW, 
    StreamRead = gl::STREAM_READ, 
    StreamCopy = gl::STREAM_COPY, 
    StaticDraw = gl::STATIC_DRAW, 
    StaticRead = gl::STATIC_READ, 
    StaticCopy = gl::STATIC_COPY, 
    DynamicDraw = gl::DYNAMIC_DRAW, 
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

pub struct Buffer{ 
    id: u32,
    target: BufferTarget,
    usage: BufferUsage 
}

impl Buffer {
    // Generates new buffer for target and buffers data,
    // is not bound after call to new is complete
    pub fn new<T>(
        target: BufferTarget, usage: BufferUsage, data: Vec<T>) 
        -> Self {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        let buf = Buffer { id, target, usage };
        buf.set_data(data);

        buf
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target as u32, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target as u32, 0);
        }
    }

    // Buffers new data to buffer
    pub fn set_data<T>(&self, data: Vec<T>) {
        let target = self.target;
        let usage = self.usage;
        self.bind();
        unsafe {
            gl::BufferData(
                target as u32,
                (data.len() * mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                usage as u32
            );
        }
        self.unbind();
    }
}

impl Drop for Buffer {
    fn drop(&mut self){
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}