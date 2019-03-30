use keybind::{Keybind, Keycode};

fn main() {
    let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);

    keybind.on_trigger(|| {
        webbrowser::open("https://giphy.com/").expect("Unable to open Giphy in browser.");
    });

    keybind.wait();
}
