#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![windows_subsystem = "windows"]
mod components;
mod plugins;
mod systems;
mod stares;
pub mod ui;


pub use plugins::game_plugin::GamePlugin;

use bevy::{log::LogPlugin, prelude::*, remote::{RemotePlugin, http::RemoteHttpPlugin}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::WARN,
            filter: "ppGame=debug,bevy=info".to_string(),
            ..Default::default()
        }))
        .add_plugins(GamePlugin)
        // .add_plugins(DefaultPlugins)
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .run();
}

