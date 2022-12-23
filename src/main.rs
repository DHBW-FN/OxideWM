pub mod windowmanager;
pub mod workspace;
pub mod windowstate;

use windowmanager::WindowManager;
use std::error::Error;
use x11rb::connection::Connection;
use x11rb::protocol::Event;

fn handle_event(manager: &WindowManager, event: &Event) {

    print!("Received Event: ");
    match event {
        Event::Expose(event_data) => println!("Expose"),
        Event::UnmapNotify(event_data) => println!("UnmapNotify"),
        Event::EnterNotify(event_data) => println!("EnterNotify"),
        Event::ButtonPress(event_data) => println!("ButtonPress"),
        Event::MotionNotify(event_data) => println!("MotionNotify"),
        Event::ButtonRelease(event_data) => println!("ButtonRelease"),
        Event::ConfigureRequest(event_data) => println!("ConfigureRequest"),
        Event::MapRequest(event_data) => {
            println!("Received map request:\n{:?}", event_data);

        },
        _ => println!("\x1b[33mUnknown\x1b[0m"),
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let manager = WindowManager::new();

    let mut event;
    loop {
        manager.connection.flush()?;

        event = manager.connection.wait_for_event();
        match event {
            Ok(event) => handle_event(&manager, &event),
            Err(error) => {
                eprintln!("\x1b[31m\x1b[1mError:\x1b[0m {}", error);
                break;
            }
        }
    }

    Ok(())
}
