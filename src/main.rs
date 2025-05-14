use fltk::{
    app,
    button::Button,
    enums,
    input::Input,
    prelude::*,
    text::{TextBuffer, TextDisplay},
    window::Window,
};

use std::
    path::Path
;

use filebrowser::*;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    PathBtnClicked,
    PathDoAutocomplete,
}

fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 800, 600, "Hello");
    let mut path_input = Input::new(10, 10, 750, 20, "");
    let mut btn_path = Button::new(770, 10, 20, 20, "@>");
    let mut output = TextDisplay::new(10, 40, 780, 550, "");
    let mut autocomplete = TextDisplay::new(10, 30, 200, 100, "");
    autocomplete.hide();

    // we don't need the button
    btn_path.deactivate();
    btn_path.hide();

    window.resizable(&output);
    window.end();
    window.show();

    let (sender, receiver) = app::channel::<Event>();

    btn_path.emit(sender, Event::PathBtnClicked);

    // keyboard shortcuts
    path_input.set_trigger(enums::CallbackTrigger::EnterKeyChanged);
    path_input.set_callback(move |_| match app::event_key() {
        enums::Key::Enter => {
            sender.send(Event::PathBtnClicked);
        }
        _ => {
            sender.send(Event::PathDoAutocomplete);
        }
    });

    while app.wait() {
        let current_path = path_input.value();
        if let Some(msg) = receiver.recv() {
            match msg {
                Event::PathBtnClicked => {
                    let mut listing_buf = TextBuffer::default();
                    let p = Path::new(&current_path);
                    let listing = list_dir(p);
                    for path in listing {
                        if let Some(p_str) = path.to_str() {
                            listing_buf.append(format!("{}\n", p_str).as_str());
                        }
                    }
                    output.set_buffer(listing_buf.clone());
                }
                Event::PathDoAutocomplete => {
                    let candidates = autocomplete_dir(&current_path);
                    if candidates.len() > 0 {
                        let mut autocomplete_buf = TextBuffer::default();
                        for c in candidates {
                            if let Some(p_str) = c.to_str() {
                                autocomplete_buf.append(format!("{}\n", p_str).as_str());
                            }
                        }
                        autocomplete.set_buffer(autocomplete_buf);
                        autocomplete.show();
                    } else {
                        autocomplete.hide();
                    }
                }
            }
        }
    }

    println!("End");
}
