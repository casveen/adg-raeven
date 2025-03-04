/**
 * lets make some water!
 */

 use bevy::{
    asset, prelude::*, reflect::TypePath, render::render_resource::{AsBindGroup, ShaderRef}
};


const SHADER_ASSET_PATH: &str = "shaders/water_material.wgsl";
const WATER_COLOR_ASSET_PATH: &str = "shaders/water_color.png";
const WATER_NOISE_ASSET_PATH: &str = "shaders/water_noise.png";

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Water;


 
fn update_materials(
    mut commands: Commands,
    q : Query<Entity, (With<MeshMaterial3d<StandardMaterial>>, Added<Water>)>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    asset_server: Res<AssetServer>,
) {

    let water_material_handle = materials.add(WaterMaterial {
        t: 0.0,
        noise_texture: Some(asset_server.load(WATER_NOISE_ASSET_PATH)),
        color_texture: Some(asset_server.load(WATER_COLOR_ASSET_PATH)),
        alpha_mode: AlphaMode::Blend,
    });

    for entity in q.iter() {
        commands.entity(entity)
        .remove::<MeshMaterial3d<StandardMaterial>>()
        .insert(MeshMaterial3d(water_material_handle.clone()));
    }
}



// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterMaterial {
    #[uniform(0)]
    t: f32,
    #[texture(1)]
    noise_texture: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}



pub struct WaterPlugin;
impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Water>()
        .add_systems(Update, update_materials)
        .add_plugins(MaterialPlugin::<WaterMaterial>::default());
    }
}