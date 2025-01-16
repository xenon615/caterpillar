use bevy::prelude::*;
use avian3d::prelude::*;

// ---

pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        ;
    }
}

// ---
#[derive(Component)]
pub struct Floor;

// ---

const FLOOR_SIZE: f32 = 100.;

fn startup(
    mut cmd: Commands,
    // mut al: ResMut<AmbientLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    // al.brightness = 0.;
    cmd.spawn((
        DirectionalLight {
            illuminance: 500.,
            shadows_enabled: false,
            ..default()
        },
        Transform::IDENTITY.looking_at(Vec3::ZERO, Vec3::Y)
    ));

    cmd.spawn((
        Floor,
        MeshMaterial3d(materials.add(Color::BLACK)),
        Mesh3d(meshes.add(
            Plane3d::new(Vec3::Y, Vec2::splat(FLOOR_SIZE))   
        )),
        Collider::cuboid(FLOOR_SIZE * 2., 0.1, FLOOR_SIZE * 2.),
        RigidBody::Static,
    ));
}

// ---

