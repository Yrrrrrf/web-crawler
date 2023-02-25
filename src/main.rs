#![allow(unused_imports)]

// extern
use bevy::prelude::*;  // Import the Bevy Engine

// mod
mod config;
mod read_file;
mod crawler;


/**
 * This is a Web Crawling program
 * - get the tree structure of a website
 * 
 * author: yrrrrrf
 * date: monday, february 13, 2023
 */


fn main() {
    let app_name: &str = "Web Crawler";
    println!("\x1b[32m{}\x1b[0m\n", &app_name);
    
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        // .add_startup_system(spawn_basic_scene)
        // .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins.set(WindowPlugin {  
            window: WindowDescriptor {
                title: app_name.to_string(),
                width: 720.0, height: 480.0,
                resizable: true,
                ..Default::default()  // Set the window descriptor
            },
            ..default()  // Set the window plugin
        }))
        .run();



}
