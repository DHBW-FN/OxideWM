use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct ScreenInfo {
    pub workspaces: HashMap<u16, Workspace>,
    pub active_workspace: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OxideWindow {
    pub frame: u32,
    pub window: u32,
    pub title: String,
    pub visible: bool,
    pub urgent: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub gap_size: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Workspace {
    pub name: u16,
    pub focused_window: Option<u32>,
    pub fullscreen: Option<u32>,
    pub urgent: bool,
    pub windows: HashMap<u32, OxideWindow>,
    pub order: Vec<u32>,
    pub layout: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    pub command: String,
    pub args: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Keybinding {
    pub keys: Vec<String>,
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub cmds: Vec<Keybinding>,
    pub exec: Vec<String>,
    pub exec_always: Vec<String>,
    pub border_width: u8,
    pub border_color: String,
    pub border_focus_color: String,
    pub gap: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OxideState {
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Config,
    pub focused_screen: u32,
}

impl OxideState {
    pub fn get_workspaces(&self, screen: u32) -> HashMap<u16, Workspace> {
        println!("screen: {}", screen);
        println!("screeninfo: {:?}", self.screeninfo);
        self.screeninfo.get(&screen).unwrap().workspaces.clone()
    }

    pub fn get_workspace_list(&self, screen: u32) -> Vec<u16> {
        let mut vec = self
            .get_workspaces(screen)
            .iter()
            .map(|(ws, _)| *ws)
            .collect_vec();
        vec.sort();
        vec
    }

    pub fn get_active_workspace(&self, screen: u32) -> u16 {
        self.screeninfo.get(&screen).unwrap().active_workspace as u16
    }
}
