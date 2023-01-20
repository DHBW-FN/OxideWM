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

use std::sync::{Arc, Mutex, Condvar};

use std::sync::mpsc::channel;
use std::thread;
use std::{cell::RefCell, rc::Rc};

use config::Config;
use serde_json::Result;
use log::{error, trace, debug};

use crate::{
    logging::init_logger,
    windowmanager::WindowManager,
    eventhandler::EventHandler,
    keybindings::KeyBindings,
    eventhandler::events::IpcEvent,
    ipc::{zbus_serve, zbus_serve_blocking},
};

fn main() -> Result<()> {
    init_logger();

    let mut config = Rc::new(RefCell::new(Config::new()));
    let mut keybindings = KeyBindings::new(&config.borrow());
    
    let wm_state_change_ipc = Arc::new((Mutex::new(false), Condvar::new()));
    let wm_state_change = Rc::new(RefCell::new(wm_state_change_ipc.clone()));

    let mut manager = WindowManager::new(&keybindings, config.clone(), wm_state_change);
    let mut eventhandler = EventHandler::new(&mut manager, &keybindings);

    let (ipc_sender, wm_receiver) = channel::<IpcEvent>();
    let (wm_sender, ipc_receiver) = channel::<String>();

    let ipc_sender_mutex = Arc::new(Mutex::new(ipc_sender));
    let ipc_receiver_mutex = Arc::new(Mutex::new(ipc_receiver));

    thread::spawn(move || {
        async_std::task::block_on(zbus_serve(ipc_sender_mutex, ipc_receiver_mutex)).unwrap();
    });


    let (ipc_sender_blocking, wm_receiver_blocking) = channel::<IpcEvent>();
    let (wm_sender_blocking, ipc_receiver_blocking) = channel::<String>();

    let ipc_sender_mutex_blocking = Arc::new(Mutex::new(ipc_sender_blocking));
    let ipc_receiver_mutex_blocking = Arc::new(Mutex::new(ipc_receiver_blocking));

    thread::spawn(move || {
        async_std::task::block_on(zbus_serve_blocking(ipc_sender_mutex_blocking, ipc_receiver_mutex_blocking, wm_state_change_ipc)).unwrap();
    });


    loop {
        let result = eventhandler.window_manager.poll_for_event();
        if let Ok(Some(event)) = result {
            eventhandler.handle_event(&event);
        } else {
            if let Some(error) = result.err(){
                error!("Error retreiving Event from Window manager {:?}", error);
            }
        }

        if let Ok(event) = wm_receiver.try_recv() {
            if event.status {
                let wm_state = eventhandler.window_manager.get_state();
                let j = serde_json::to_string(&wm_state)?;
                debug!("IPC status request");
                wm_sender.send(j).unwrap();
            } else {
                eventhandler.handle_ipc_event(event);
            }
        }

        if let Ok(event) = wm_receiver_blocking.try_recv() {
            if event.status {
                let wm_state = eventhandler.window_manager.get_state();
                let j = serde_json::to_string(&wm_state)?;
                debug!("IPC status request blocking");
                wm_sender_blocking.send(j).unwrap();
            } else {
                eventhandler.handle_ipc_event(event);
            }
        }


        if eventhandler.window_manager.restart {
            config = Rc::new(RefCell::new(Config::new()));
            keybindings = KeyBindings::new(&config.borrow());

            eventhandler = EventHandler::new(&mut manager, &keybindings);
            eventhandler.window_manager.restart_wm(&keybindings, config.clone());
        }
    }
}
