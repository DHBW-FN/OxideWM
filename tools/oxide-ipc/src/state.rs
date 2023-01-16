use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ScreenInfo {
    pub workspaces: HashMap<u16, Workspace>,
    pub active_workspace: usize,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OxideWindow {
    pub window: u32,
    pub title: String,
    pub visible: bool,
    pub urgent: bool,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub titlebar_height: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub focused_window: Option<u32>,
    pub urgent: bool,
    pub windows: HashMap<u32, OxideWindow>,
    pub order: Vec<u32>,
    pub layout: String,
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Keybinding {
    pub keys: Vec<String>,
    pub command: String,
    pub args: Option<String>,
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

    pub fn workspace_tuple(&self, screen: u32) -> Vec<(bool, String)> {
        let workspaces = self.get_workspaces(screen);
        let mut vec = Vec::new();
        for (index, workspace) in workspaces {
            vec.push((workspace.focused, workspace.name));
        }
        vec
    }
}



