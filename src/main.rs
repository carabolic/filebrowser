use fltk::{
    app,
    button::Button,
    enums,
    input::Input,
    prelude::*,
    text::{TextBuffer, TextDisplay},
    window::Window,
};

use std::{
    fs::{self}, iter::zip, path::{Path, PathBuf}
};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    PathBtnClicked,
    PathDoAutocomplete,
}

fn list_dir(path: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    match fs::read_dir(path) {
        Ok(child_paths) => {
            for child_path in child_paths {
                match child_path {
                    Ok(p) => {
                        result.push(p.path());
                    }
                    Err(e) => println!("error: child path {}", e),
                }
            }
        }
        Err(e) => println!("{}", e),
    };
    result
}

trait PathExt {
    fn starts_with_incomplete<P: AsRef<Path>>(&self, base: P) -> bool;
}

impl PathExt for Path {
    /// Checks if given path is a prefix of this path
    /// _uses the underlying bytes_
    fn starts_with_incomplete<P: AsRef<Path>>(&self, base: P) -> bool {
        let self_bytes = self.as_os_str().as_encoded_bytes();
        let base_bytes = base.as_ref().as_os_str().as_encoded_bytes();
        for (b, s) in zip(base_bytes, self_bytes) {
            if b != s {
                return false;
            }
        }
        return true;
    }
}

fn autocomplete_dir(incomplete_path: &String) -> Vec<PathBuf> {
    let mut possible_paths = Vec::new();

    let p = Path::new(incomplete_path);
    let parent_dir = if p.is_dir() && p.to_str().expect("should not happen").ends_with("/") {
        p
    } else {
        p.parent().unwrap_or(Path::new("/"))
    };

    match parent_dir.read_dir() {
        Ok(listing) => {
            possible_paths = listing
                .flatten()
                .map(|e| e.path())
                .filter(|c| c.starts_with_incomplete(p))
                .collect()
        }
        Err(e) => {
            println!("error: no listing for {}, {}", p.display(), e);
        }
    }

    possible_paths
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
