use bevy::prelude::*;
use bevy::render::camera::{Exposure, PhysicalCameraParameters};

// ---

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn) 
         ;
    }
} 

// ---

#[derive(Component)]
pub struct Cam;

// ---

fn spawn (
    mut commands : Commands,
 ) {
    commands.spawn((
        Camera3d::default(),
        Camera{
            hdr: true,
            ..default()
        },
        
        Transform::from_xyz(33.488613, 16.260841, 34.402775).looking_at(Vec3::ZERO, Vec3::Y),
        Exposure::from_physical_camera(PhysicalCameraParameters{
            sensitivity_iso: 80.,
            ..default()
        }),
        Cam,
    ));
}
