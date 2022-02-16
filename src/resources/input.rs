use std::collections::HashSet;

pub struct Input {
    pressed: HashSet<KeyCode>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            pressed: Default::default(),
        }
    }
}

impl Input {
    pub fn press(&mut self, key_code: KeyCode) {
        self.pressed.insert(key_code);
    }

    pub fn pressed(&self, key_code: KeyCode) -> bool {
        self.pressed.contains(&key_code)
    }

    pub fn release(&mut self, key_code: KeyCode) {
        self.pressed.remove(&key_code);
    }

    pub fn get_pressed(&self) -> impl ExactSizeIterator<Item = &KeyCode> {
        self.pressed.iter()
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum KeyCode {
    W,
    S,
    A,
    D,
    Z,
    X,
}

impl KeyCode {
    pub fn new(value: &str) -> Option<KeyCode> {
        match value {
            "w" => Some(KeyCode::W),
            "s" => Some(KeyCode::S),
            "a" => Some(KeyCode::A),
            "d" => Some(KeyCode::D),
            "z" => Some(KeyCode::Z),
            "x" => Some(KeyCode::X),
            _ => None,
        }
    }
}
