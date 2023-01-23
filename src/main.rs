#![deny(clippy::pedantic)]

pub mod eventhandler;
pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;
pub mod keybindings;
pub mod auxiliary;
pub mod ipc;
pub mod atom;
pub mod constants;
pub mod common;
pub mod logging;

use std::sync::{Arc, Mutex};

use std::sync::mpsc::channel;
use std::thread;
use std::{cell::RefCell, rc::Rc};

use config::Config;
use log::info;
use serde_json::Result;

use crate::{
    eventhandler::events::EnumEventType,
    logging::init_logger,
    windowmanager::WindowManager,
    eventhandler::EventHandler,
    keybindings::KeyBindings,
    ipc::zbus_serve,
};

fn main() -> Result<()> {
    init_logger();

    let mut config = Rc::new(RefCell::new(Config::new()));
    let mut keybindings = KeyBindings::new(&config.borrow());
    
    let keybinding_clone = keybindings.clone();
    let mut manager = WindowManager::new(config.clone());
    let mut eventhandler = EventHandler::new(&mut manager, &keybinding_clone);

    let (event_sender, event_receiver) = channel::<EnumEventType>();
    let (status_sender, status_receiver) = channel::<String>();


    let event_sender_mutex = Arc::new(Mutex::new(event_sender));
    let event_receiver_mutex = Arc::new(Mutex::new(event_receiver));

    let status_sender_mutex = Arc::new(Mutex::new(status_sender));
    let status_receiver_mutex = Arc::new(Mutex::new(status_receiver));

    loop {

        info!("starting zbus serve");
        let event_mutex_zbus = event_sender_mutex.clone();
        let status_mutex_zbus = status_receiver_mutex.clone();
        thread::spawn(move || {
            async_std::task::block_on(
                zbus_serve(event_mutex_zbus, status_mutex_zbus)
            ).unwrap();
        });
            
    
        info!("starting x event proxy");
        let event_mutex_x = event_sender_mutex.clone();
        let keybinding_x = keybindings.clone();
        thread::spawn(move || {
            WindowManager::run_event_proxy( event_mutex_x, &keybinding_x);
        });

        info!("starting event loop");
        eventhandler.run_event_loop(event_receiver_mutex.clone(), status_sender_mutex.clone());

        // Todo js handle restart threads currently do not finish

        if eventhandler.window_manager.restart {
            config = Rc::new(RefCell::new(Config::new()));
            keybindings = KeyBindings::new(&config.borrow());

            eventhandler = EventHandler::new(&mut manager, &keybindings);
            eventhandler.window_manager.restart_wm(&keybindings, config.clone());
        }else{
            break;
        }
    }
    Ok(())
}
