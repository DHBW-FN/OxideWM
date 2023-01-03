pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;
pub mod keybindings;

use std::sync::mpsc::{channel, Sender};
use std::error::Error;
use std::thread;

use x11rb::connection::Connection;

use windowmanager::WindowManager;

#[derive(Debug)]
struct IpcEvent {
    test: String,
}


fn dbus_ipc_loop(sender: Sender<IpcEvent>) {
    loop {
        //sender.send(IpcEvent { test: "test".to_string() }).unwrap();
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = WindowManager::new();

    let (sender, receiver) = channel();

    thread::spawn(move || {
        dbus_ipc_loop(sender);
    });

    loop {
        let event = manager.connection.borrow_mut().poll_for_event().unwrap();
        match event {
            Some(event) => manager.handle_event(&event),
            None => (),
        }
        //get_cursor_position(&manager);


        let ipc_event = receiver.try_recv();
        match ipc_event {
            Ok(event) => println!("Received IPC Event: {:?}", event),
            Err(_) => (),
        }
    }
}
