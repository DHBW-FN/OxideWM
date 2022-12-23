use std::process::exit;
use std::collections::HashMap;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::Event;
use x11rb::connection::Connection;
use x11rb::protocol::ErrorKind;
use x11rb::protocol::xproto::{
    ConnectionExt,
    Screen,
    ChangeWindowAttributesAux,
    EventMask,
};
use crate::workspace::Workspace;
use x11rb::rust_connection::ReplyError;
use std::{cell::RefCell, rc::Rc};


#[derive(Debug)]
pub struct ScreenInfo {
    pub connection: Rc<RefCell<RustConnection>>,
    pub id: u32,
    pub workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub width: u16,
    pub height: u16,
}

impl ScreenInfo {
    pub fn new(connection: Rc<RefCell<RustConnection>>, id: u32, height: u16, width: u16) -> ScreenInfo {
        let active_workspace = 0;
        let workspaces = Vec::new();
        ScreenInfo {
            connection,
            id,
            workspaces,
            active_workspace,
            width,
            height,
        }
    }   

    pub fn map_request(&mut self, event: &MapRequestEvent) {
        println!("WINMAN: MapRequestEvent: {:?}", event);
        let workspace = &mut self.workspaces[self.active_workspace.clone()];
        workspace.new_window(event.window);
        workspace.remap_windows();
    }
}
