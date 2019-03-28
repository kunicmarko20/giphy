use device_query::{DeviceQuery, DeviceState};

pub use device_query::Keycode;

pub struct Keybind {
    device_state: DeviceState,
    previous_pressed_keys: Vec<Keycode>,
    key_binds: Vec<Keycode>,
    on_trigger: Box<Fn()>,
}

impl Keybind {
    pub fn default() -> Keybind {
        Keybind {
            device_state: DeviceState::new(),
            previous_pressed_keys: Vec::new(),
            key_binds: Vec::new(),
            on_trigger: Box::new(||{})
        }
    }

    pub fn bind(&mut self, key: Keycode) {
        self.key_binds.push(key);
    }

    pub fn triggered(&mut self) -> bool {
        let pressed_keys = self.device_state.get_keys();
        let previous_pressed_keys = self.previous_pressed_keys.clone();
        self.previous_pressed_keys = pressed_keys.clone();

        if pressed_keys.len() != self.key_binds.len() {
            return false;
        }

        previous_pressed_keys != pressed_keys
            && pressed_keys == self.key_binds
    }

    pub fn on_trigger<C: Fn() + 'static>(&mut self, callback: C) {
        self.on_trigger = Box::new(callback);
    }

    pub fn wait(&mut self) {
        loop {
            if self.triggered() {
                (self.on_trigger)();
            }
        }
    }
}