use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use std::process::{
    exit
};

use x11rb::{
    connection::Connection,
    protocol::{
        ErrorKind,
        xproto::*, 
        Event
    },
    rust_connection::{
        RustConnection,
        ReplyError, ConnectionError
    },
};

use crate::{
    screeninfo::ScreenInfo,
    workspace::Workspace,
};

#[derive(Debug)]
pub struct WindowManager {
    pub connection: Rc<RefCell<RustConnection>>,
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub focused_screen: u32,
    pub moved_window: Option<u32>,
}

#[derive(Debug)]
pub enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Movement {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "left" => Ok(Movement::Left),
            "right" => Ok(Movement::Right),
            "up" => Ok(Movement::Up),
            "down" => Ok(Movement::Down),
            _ => Err(format!("{} is not a valid movement", value)),
        }
    }
}

impl WindowManager {
    pub fn new() -> WindowManager {
        let connection = Rc::new(RefCell::new(RustConnection::connect(None).unwrap().0));
        let screeninfo = HashMap::new();

        let focused_screen = 0;
        //TODO: Get focused screen from X11
        // Currently the screen setup last is taken as active.
        // We should discuss if this default behaviour is ok or not.

        let mut manager = WindowManager {
            connection,
            screeninfo,
            focused_screen,
            moved_window: None,
        };

        manager.setup_screens();
        manager.update_root_window_event_masks();
        manager.connection.borrow_mut().flush().unwrap();

        manager
    }

    fn get_active_workspace(&self) -> usize {
        return self.screeninfo.get(&self.focused_screen).unwrap().active_workspace;    
    }

    fn get_focused_window(&self) -> (usize, Option<u32>) {
        let active_workspace = self.get_active_workspace();
        let focused_window = self.screeninfo
            .get(&self.focused_screen)
            .unwrap().workspaces[active_workspace]
            .get_focused_window();
        return (active_workspace, focused_window);
    }

    pub fn poll_for_event(&self)->Result<Option<Event>, ConnectionError>{
        self.connection.borrow_mut().poll_for_event()
    }
      
    pub fn handle_keypress_focus(&mut self, args: Option<String>) {
        let args = args.expect("No move arguments for focus");
        let active_workspace = self.get_active_workspace();
        self.screeninfo.get_mut(&self.focused_screen)
            .unwrap()
            .workspaces[active_workspace]
            .move_focus(Movement::try_from(args.as_str()).unwrap());
    }

    pub fn handle_keypress_move(&mut self, args: Option<String>) {
        let args = args.expect("No move arguments for move");
        let active_workspace = self.get_active_workspace();
        self.moved_window = self.screeninfo.get_mut(&self.focused_screen)
            .unwrap()
            .workspaces[active_workspace]
            .move_window(Movement::try_from(args.as_str()).unwrap());
    }
    
    pub fn handle_keypress_kill(&mut self) {
        let (active_workspace, focused_window) = self.get_focused_window();
        println!("Focused window: {:?}", focused_window);
        if let Some(winid) = focused_window {
            self.screeninfo
                .get_mut(&self.focused_screen)
                .unwrap().workspaces[active_workspace]
                .kill_window(&winid);
        } else {
            println!("ERROR: No window to kill \nShould only happen on an empty screen");
        }
    }

    fn setup_screens(&mut self) {
        for screen in self.connection.borrow().setup().roots.iter() {
            let mut screenstruct = ScreenInfo::new(self.connection.clone(),
                                                   screen.root,
                                                   screen.width_in_pixels,
                                                   screen.height_in_pixels,
                                                   );
            screenstruct.workspaces.push(Workspace::new(0,
                                                        self.connection.clone(),
                                                        0,
                                                        0,
                                                        screen.width_in_pixels as u32,
                                                        screen.height_in_pixels as u32,
                                                        ));
            self.screeninfo.insert(screen.root, screenstruct);
            self.focused_screen = screen.root;
        }
    }

    fn update_root_window_event_masks(&self) {
        let mask = ChangeWindowAttributesAux::default()
                   .event_mask(
                        EventMask::SUBSTRUCTURE_REDIRECT |
                        EventMask::SUBSTRUCTURE_NOTIFY |
                        EventMask::BUTTON_MOTION |
                        EventMask::FOCUS_CHANGE |
                        //EventMask::ENTER_WINDOW |
                        //EventMask::LEAVE_WINDOW | //this applies only to the rootwin
                        EventMask::PROPERTY_CHANGE
                    );

        for screen in self.connection.borrow().setup().roots.iter() {
            #[cfg(debug_assertion)]
            println!("Attempting to update event mask of: {} -> ", screen.root);

            self.set_mask(screen, mask).unwrap();

            #[cfg(debug_assertion)]
            println!("Screen: {} -> {}", screen.root, screen.width_in_pixels);
        }
    }

    fn set_mask(
        &self,
        screen: &Screen,
        mask: ChangeWindowAttributesAux
    ) -> Result<(), ReplyError> {
        let update_result = self.connection.borrow().change_window_attributes(
                                screen.root,
                                &mask
                            )?.check();

        if let Err(ReplyError::X11Error(ref error)) = update_result {
            if error.error_kind == ErrorKind::Access {
                eprintln!("\x1b[31m\x1b[1mError:\x1b[0m Access to X11 Client Api denied!");
                exit(1);
            }
        }

        #[cfg(debug_assertion)]
        match update_result {
             Ok(_) => println!("\x1b[32mSuccess\x1b[0m"),
             Err(_) => println!("\x1b[31mFailed\x1b[0m"),
        }

        update_result
    }

    pub fn handle_event_enter_notify(&mut self, event: &EnterNotifyEvent) {
        self.focused_screen = event.root;
        let workspace_id = self.get_active_workspace();
        let mut winid = event.event;
        if self.moved_window.is_some() {
            winid =  self.moved_window.unwrap();
            self.moved_window = None;
        }

        self.screeninfo
            .get_mut(&event.root)
            .unwrap()
            .workspaces[workspace_id]
            .focus_window(winid);
    }

    pub fn handle_event_leave_notify(&mut self, event: &LeaveNotifyEvent) {
        let workspace_id = self.get_active_workspace();
        self.screeninfo
            .get_mut(&event.root)
            .unwrap().workspaces[workspace_id]
            .unfocus_window(event.event);
    }


    pub fn handle_event_unmap_notify(&mut self, event: &UnmapNotifyEvent) {
        let workspace_id = self.screeninfo
            .get(&event.event)
            .unwrap().active_workspace;
        self.screeninfo
            .get_mut(&event.event)
            .unwrap().workspaces[workspace_id]
            .remove_window(&event.window);
    }

    pub fn handle_map_request(&mut self, event: &MapRequestEvent) {
        self.screeninfo.get_mut(&event.parent).unwrap().on_map_request(event);
    }
}


