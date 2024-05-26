use std::collections::BTreeMap;
use std::num::ParseIntError;

use config::Config;
use hyprland::dispatch::*;
use hyprland::shared::HyprData;
use hyprland::data::{Workspace, Workspaces};
use hyprland::event_listener::EventListener;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RenameConfig {
    workspace_map: BTreeMap<i32, String>
}

fn main() {
    let config_path = dirs_next::config_dir().unwrap();
    let config = config_path.join("hypr/rename_workspaces.toml").canonicalize().unwrap();
    let rename_config: RenameConfig = Config::builder()
                                            .add_source(config::File::with_name(config.to_str().unwrap()))
                                            .build()
                                            .expect("Unable to read config file!")
                                            .try_deserialize()
                                            .expect("Unable to parse config");
    println!("{rename_config:#?}");
    // rename all existing workspace
    Workspaces::get().and_then(|workspaces| {
        for ws in workspaces {
            match rename_config.workspace_map.get(&ws.id) {
                Some(name) => {
                    hyprland::dispatch!(RenameWorkspace, ws.id, Some(name));
                },
                None => {  } //do nothing if there is no name configured for the workspace,
            }
        }
        Ok(())
    });
    // listen for new workspaces to rename
    let mut ws_events_listener = EventListener::new();
    ws_events_listener.add_workspace_added_handler(move |ws_type| {
        match ws_type {
            hyprland::shared::WorkspaceType::Regular(id) => {
                let id_result: Result<i32, ParseIntError> = id.parse();
                match id_result {
                    Ok(id) => {
                        match rename_config.workspace_map.get(&id) {
                            Some(name) => {
                                hyprland::dispatch!(RenameWorkspace, id, Some(name));
                            },
                            None => {  } //do nothing if there is no name configured for the workspace,
                        }
                    },
                    Err(_) => {}, // do nothing here
                }
            },
            hyprland::shared::WorkspaceType::Special(_) => {}, // do nothing here
        }
    });
    ws_events_listener.start_listener();
}
