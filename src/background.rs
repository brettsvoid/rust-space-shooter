use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

const FRAGMENT_SHADER_PATH: &str = "../assets/background_shader.frag";

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .add_systems(Startup, spawn_background)
            .add_systems(Update, update_shader);
    }
}

/// Spawn a stretched rectangle material to be the base of the background shader
fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    window: Single<&Window>,
) {
    let resolution = Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(BackgroundMaterial {
            resolution,
            time: 0.0,
            speed: 1.0,
        })),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)).with_scale(resolution.extend(0.0)),
    ));
}

fn update_shader(
    time: Res<Time>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    window: Single<&Window>,
) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_secs();
        material.1.resolution = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );
    }
}

// This is the struct that will be passed to the shader
#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct BackgroundMaterial {
    #[uniform(0)]
    resolution: Vec2,
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    speed: f32,
}

// TODO: consider wgsl instead of glsl (supposedly a better experience)

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        FRAGMENT_SHADER_PATH.into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
