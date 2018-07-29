#![allow(dead_code)]

use graphics::{program, textures};

enum TextureContainer<'a> {
    OneTexture(&'a textures::Texture),
    NTextures(Vec<&'a textures::Texture>),
}

pub struct Material<'a> {
    tex_container: TextureContainer<'a>,
    shader_program: &'a program::Program,
}

impl <'a> Material <'a> {
    fn multiple_textures(
        textures: Vec<&'a textures::Texture>,
        shader_program: &'a program::Program
        ) -> Self {
        Material { 
            tex_container: TextureContainer::NTextures(textures), 
            shader_program 
        }
    }

    fn one_texture(
        texture: &'a textures::Texture,
        shader_program: &'a program::Program
        ) -> Self {
        Material { 
            tex_container: TextureContainer::OneTexture(texture), 
            shader_program 
        }
    }

    fn set_used(&self, used: bool) {
        self.shader_program.set_used(used);
        match self.tex_container {
            TextureContainer::NTextures(ref textures) => {
                if used {
                    for i in 0..textures.len() {
                        textures[i].bind(i as u32);
                    }
                } else {
                    for texture in textures.iter() {
                        texture.unbind();
                    } 
                }
            }
            TextureContainer::OneTexture(ref texture) => {
                if used {
                    texture.bind(0);
                } else {
                    texture.unbind();
                }
            }
        }
    }
}