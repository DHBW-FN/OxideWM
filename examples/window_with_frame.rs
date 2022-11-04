use std::error::Error;
use std::process::exit;

use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::errors::ReplyError;
use x11rb::protocol::Event;
use x11rb::protocol::ErrorKind;
use x11rb::protocol::xproto::{
    ChangeWindowAttributesAux,
    CreateWindowAux,
    MapRequestEvent,
    ConnectionExt,
    WindowClass,
    EventMask,
    Window,
    Screen,
};

fn on_map_request<C: Connection>(
    manager: &C,
    screen_index: usize,
    event: &MapRequestEvent
) -> Result<(), Box<dyn Error>> {
    draw_window(
        manager,
        &manager.setup().roots[screen_index],
        event.window,
        20,
        20,
        300,
        400,
        10,
        5,
    )
}

fn draw_window<C: Connection>(
    manager: &C,
    screen: &Screen,
    window: Window,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    titlebar_height: u16,
    ) -> Result<(), Box<dyn Error>> {

    let frame_id = manager.generate_id()?;
    let titlebar_id = manager.generate_id()?;
    let window_id = manager.generate_id()?;

    let window_aux = CreateWindowAux::new()
                     .event_mask(
                         EventMask::EXPOSURE            |
                         EventMask::SUBSTRUCTURE_NOTIFY |
                         EventMask::BUTTON_PRESS        |
                         EventMask::BUTTON_RELEASE      |
                         EventMask::POINTER_MOTION      |
                         EventMask::ENTER_WINDOW)
                     .background_pixel(screen.black_pixel);

    manager.create_window(
        COPY_DEPTH_FROM_PARENT,
        frame_id,
        screen.root,
        x,
        y,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;

    manager.create_window(
        COPY_DEPTH_FROM_PARENT,
        titlebar_id,
        frame_id,
        x + border_width as i16,
        y + border_width as i16,
        width - 2*border_width,
        titlebar_height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;

    manager.create_window(
        COPY_DEPTH_FROM_PARENT,
        window,
        frame_id,
        x + border_width as i16,
        y + (border_width + titlebar_height) as i16,
        width - 2*border_width,
        height - 2*border_width - titlebar_height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &window_aux,
    )?;

    manager.map_window(frame_id)?;
    manager.map_window(titlebar_id)?;
    manager.map_window(window_id)?;
    manager.flush()?;

    Ok(())
}

fn handle_event<C: Connection>(
    manager: &C,
    screen_index: usize,
    event: &Event) {
    println!("Event: {:?}", event);

    match event {
        Event::Expose(_event) => println!("Ignored event!"),
        Event::UnmapNotify(_event) => println!("Ignored event!"),
        Event::EnterNotify(_event) => println!("Ignored event!"),
        Event::ButtonPress(_event) => println!("Ignored event!"),
        Event::MotionNotify(_event) => println!("Ignored event!"),
        Event::ButtonRelease(_event) => println!("Ignored event!"),
        Event::ConfigureRequest(_event) => println!("Ignored event!"),
        Event::MapRequest(_event) => {
            on_map_request(manager, screen_index, _event).unwrap();
        },
        _ => {}
    };
}

fn become_wm<C: Connection>(manager: &C, screen: &Screen) -> Result<(), ReplyError> {
    let mask = ChangeWindowAttributesAux::default()
               .event_mask(
                    EventMask::SUBSTRUCTURE_REDIRECT |
                    EventMask::SUBSTRUCTURE_NOTIFY
                );

    let become_wm_result = manager.change_window_attributes(
                                      screen.root,
                                      &mask
                                  )?.check();

    if let Err(ReplyError::X11Error(ref error)) = become_wm_result {
        if error.error_kind == ErrorKind::Access {
            eprintln!("Error: Access to x11 client api denied.");
            exit(1);
        }
    }

    become_wm_result
}

fn main() -> Result<(), Box<dyn Error>> {
    let (manager, screen_index) = RustConnection::connect(None)?;
    let screen = &manager.setup().roots[screen_index];

    become_wm(&manager, screen)?;

    let mut event;
    loop {
        manager.flush()?;

        event = manager.wait_for_event();
        match event {
            Ok(event) => handle_event(&manager, screen_index, &event),
            Err(error) => {
                eprintln!("Error: {}", error);
                break;
            }
        }
    }

    Ok(())
}
