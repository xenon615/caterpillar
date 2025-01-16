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

const FLOOR_SIZE: f32 = 50.;

fn startup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let black = materials.add(Color::BLACK);
    let white = materials.add(Color::WHITE);

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
        MeshMaterial3d(black.clone()),
        Mesh3d(meshes.add(
            Plane3d::new(Vec3::Y, Vec2::splat(FLOOR_SIZE))   
        )),
        Collider::cuboid(FLOOR_SIZE * 2., 0.1, FLOOR_SIZE * 2.),
        RigidBody::Static,
    ));

    let mesh = meshes.add(Cuboid::from_size(Vec3::new(0.1, 2., 1.)));

    for z in 0 .. FLOOR_SIZE as usize {
        cmd.spawn((
            Transform::from_xyz(-4., 1., z as f32 - FLOOR_SIZE / 2. ),
            MeshMaterial3d(if z % 2 == 0 {white.clone()} else {black.clone()}),
            Mesh3d(mesh.clone()),
        ));
    }
}

// ---

