#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    prelude::*, 
    window::{
        // WindowMode, 
        WindowPosition, 
        WindowResolution
    }
};
use avian3d::{
    prelude::{
        PhysicsDebugPlugin, 
        RigidBody
    }, 
    PhysicsPlugins
};

// use bevy_inspector_egui::quick::WorldInspectorPlugin;


mod camera;
mod env;
mod caterpillar;
// ---
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins
        .set(
            WindowPlugin {
                primary_window : Some(Window {
                    // canvas: Some("#siege-canvas".into()),
                    resolution : WindowResolution::new(1400., 900.),
                    // mode: WindowMode::BorderlessFullscreen,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            },
        ),

        PhysicsPlugins::default(),
        // PhysicsDebugPlugin::default(),
        // WorldInspectorPlugin::new(),
        camera::CameraPlugin,
        env::EnvPlugin,
        caterpillar::CaterpillarPlugin

    ))
    .run();
}


