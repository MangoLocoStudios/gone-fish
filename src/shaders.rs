use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
pub struct GradientMaterial {}

impl Material2d for GradientMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/gradient_material.wgsl".into()
    }
}
