use std::collections::HashMap;
use std::error::Error;
use std::{cell::RefCell, rc::Rc};
use std::process::{
    exit
};

use log::{warn, error, info};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::{
    protocol::{
        ErrorKind,
        Event
    },
    protocol::xproto::{
        ChangeWindowAttributesAux,
        Screen,
        MapRequestEvent, 
        UnmapNotifyEvent, 
        LeaveNotifyEvent, 
        EnterNotifyEvent, 
        EventMask, 
        GrabMode, 
        ModMask
    },
    rust_connection::{
        ConnectionError,
        RustConnection,
        ReplyError
    }
};
use serde::Serialize;

use crate::workspace::{GoToWorkspace, Layout, Workspace};
use crate::{
    keybindings::KeyBindings,
    screeninfo::ScreenInfo,
    config::Config,
    eventhandler::commands::WmCommands,
};

use zbus::zvariant::{DeserializeDict, SerializeDict, Type};


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

#[derive(Type, DeserializeDict, SerializeDict, Debug)]
#[zvariant(signature = "dict")]
pub struct WmActionEvent {
    pub command: WmCommands,
    pub args: Option<String>,
}

impl WmActionEvent {
    pub fn new(command: &str, args: Option<String>) -> Self {
        WmActionEvent {
            command: WmCommands::try_from(command).unwrap(),
            args,
        }
    }
}

#[derive(DeserializeDict, SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct IpcEvent {
    pub status: bool,
    pub event: Option<WmActionEvent>,
}

impl From<WmActionEvent> for IpcEvent {
    fn from(command: WmActionEvent) -> Self {
        IpcEvent {
            status: false,
            event: Some(command),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WindowManagerState {
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Config,
    pub focused_screen: u32,
}

#[derive(Debug, Clone)]
pub struct WindowManager {
    pub connection: Rc<RefCell<RustConnection>>,
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Rc<RefCell<Config>>,
    pub focused_screen: u32,
    pub moved_window: Option<u32>,
}


impl WindowManager {
    pub fn new(keybindings: &KeyBindings, config: Rc<RefCell<Config>>) -> WindowManager {
        let connection = Rc::new(RefCell::new(RustConnection::connect(None).unwrap().0));
        let screeninfo = HashMap::new();

        let focused_screen = 0;
        //TODO: Get focused screen from X11
        // Currently the screen setup last is taken as active.
        // We should discuss if this default behaviour is ok or not.

        let mut manager = WindowManager {
            connection,
            screeninfo,
            config,
            focused_screen,
            moved_window: None,
        };

        manager.setup_screens();
        manager.update_root_window_event_masks();
        manager.grab_keys(keybindings).expect("Failed to grab Keys");
        let result = manager.connection.borrow_mut().flush();
        if result.is_err() {
            info!("Failed to flush rust connection");
        }

        manager
    }

    pub fn get_state(&self) -> WindowManagerState {
        WindowManagerState {
            screeninfo: self.screeninfo.clone(),
            config: self.config.borrow().clone(),
            focused_screen: self.focused_screen.clone(),
        }
    }

    fn grab_keys(&self, keybindings: &KeyBindings) -> Result<(), Box<dyn Error>> {
        println!("grabbing keys");
        //TODO check if the the screen iterations should be merged
        for screen in self.connection.borrow().setup().roots.iter() {
            for modifier in [0, u16::from(ModMask::M2)] {
                for keyevent in keybindings.events_vec.iter() {
                    self.connection.borrow().grab_key(
                        false,
                        screen.root,
                        (keyevent.keycode.mask | modifier).into(),
                        keyevent.keycode.code,
                        GrabMode::ASYNC,
                        GrabMode::ASYNC,
                    )?;
                }
            }
        }
        Ok(())
    }

    fn get_active_workspace_id(&self) -> usize {
        return self.screeninfo.get(&self.focused_screen).unwrap().active_workspace;    
    }

    fn get_active_workspace(&mut self) -> Option<&mut Workspace> {
        let active_workspace_id = self.get_active_workspace_id();
        return self.screeninfo.get_mut(&self.focused_screen)
            .unwrap()
            .get_workspace(active_workspace_id);
    }

    fn get_focused_window(&mut self) -> (Option<usize>, Option<u32>) {
        let workspace_option = self.get_active_workspace();
        return match workspace_option {
            Some(workspace) => {
                let focused_window = workspace.get_focused_window();
                (Some(workspace.index), focused_window)
            },
            None => (None, None),
        }
    }

    pub fn poll_for_event(&self)->Result<Option<Event>, ConnectionError>{
        self.connection.borrow().poll_for_event()
    }
      
    pub fn handle_keypress_focus(&mut self, args_option: Option<String>) {
        if args_option.is_none(){
            warn!("Argument must be provided");
            return;
        }
        let args = args_option.unwrap();
        
        let movement = Movement::try_from(args.as_str());
        if movement.is_err() {
            warn!("Could not parse movement from argument {}", args);
            return;
        }

        let active_workspace = self.get_active_workspace();
        match active_workspace {
            Some(workspace) => workspace.move_focus(movement.unwrap()),
            None => warn!("No workspace is currently active"),
        }
    }

    pub fn handle_keypress_move(&mut self, args_option: Option<String>) {
        if args_option.is_none(){
            warn!("Argument must be provided");
            return;
        }
        let args = args_option.unwrap();

        let movement = Movement::try_from(args.as_str());
        if movement.is_err() {
            warn!("Could not parse movement from argument {}",args);
            return;
        }

        let active_workspace = self.get_active_workspace();
        match active_workspace {
            Some(workspace) => {
                workspace.move_window(movement.unwrap());
            },
            None => warn!("No workspace is currently active"),
        }
    }
    
    pub fn handle_keypress_kill(&mut self) {
        let (_, focused_window) = self.get_focused_window();
        println!("Focused window: {:?}", focused_window);
        if let Some(winid) = focused_window {
            if let Some(workspace) = self.get_active_workspace(){
                workspace.kill_window(&winid);
            }else{
                warn!("No workspace currently selected")
            }
        } else {
            error!("ERROR: No window to kill \nShould only happen on an empty screen");
        }
    }

    pub fn handle_keypress_layout(&mut self, args: Option<String>) {    
        if args.is_none() {
            warn!("No argument provided");
            return;
        }

        let active_workspace_option = self.get_active_workspace();
        if active_workspace_option.is_none(){
            warn!("No workspace currently selected");
            return;
        }
        let active_workspace = active_workspace_option.unwrap();

        match args {
            Some(args) => {
                let layout = Layout::try_from(args.as_str());
                if layout.is_err(){
                    warn!("Layout could not be parsed from argument {}", args);
                }
                active_workspace.set_layout(layout.unwrap());
            },
            None => active_workspace.next_layout()
        }
    }

    pub fn handle_keypress_go_to_workspace(&mut self, args: Option<String>){
        let screen_option = self.screeninfo
            .get_mut(&self.focused_screen);
        if screen_option.is_none() {
            warn!("Could not switch workspace, no screen was focused");
            return;
        }

        let arg = GoToWorkspace::try_from(args);
        if arg.is_none() {
            warn!("No argument for key binding go to workspace");
            return;
        }
        let screen= screen_option.unwrap();
        
        let max_workspace = screen.get_workspace_count() - 1;
        let active_workspace = screen.active_workspace;
        let new_workspace = arg.unwrap().calculate_new_workspace(active_workspace, max_workspace);
        screen.set_workspace(new_workspace);
    }

    fn setup_screens(&mut self) {
        for screen in self.connection.borrow().setup().roots.iter() {
            let mut screenstruct = ScreenInfo::new(self.connection.clone(),
                                                   screen.root,
                                                   screen.width_in_pixels as u32,
                                                   screen.height_in_pixels as u32,
                                                   );
            screenstruct.create_new_workspace();
            screenstruct.create_new_workspace();
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
        let mut winid = event.event;
        if self.moved_window.is_some() {
            winid =  self.moved_window.unwrap();
            self.moved_window = None;
        }

        let active_workspace = self.get_active_workspace();
        match active_workspace{
            Some(workspace) => workspace.focus_window(winid),
            None=>warn!("No workspace currently selected")
        }
    }

    pub fn handle_event_leave_notify(&mut self, _event: &LeaveNotifyEvent) {
        let active_workspace = self.get_active_workspace();
        match active_workspace{
            Some(workspace) => workspace.unfocus_window(),
            None=>warn!("No workspace currently selected")
        }
    }


    pub fn handle_event_unmap_notify(&mut self, event: &UnmapNotifyEvent) {
        let active_workspace = self.get_active_workspace();
        match active_workspace{
            Some(workspace) => workspace.remove_window(&event.window),
            None=>warn!("No workspace currently selected")
        }
    }

    pub fn handle_map_request(&mut self, event: &MapRequestEvent) {
        self.screeninfo.get_mut(&event.parent).unwrap().on_map_request(event);
    }
}


