#[derive(Default)]
pub(super) struct Keypad {
    state: u16,
}

impl Keypad {
    pub fn press_key(&mut self, key: u8) {
        let key_mask = (1 << key) as u16;
        self.state |= key_mask;
    }

    pub fn release_key(&mut self, key: u8) {
        let key_mask = (1 << key) as u16;
        self.state &= !key_mask;
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        let key_mask = (1 << key) as u16;
        self.state & key_mask > 0
    }

    pub fn get_any_key_pressed(&self) -> Option<u8> {
        for key in 0..=15 {
            if self.is_key_pressed(key) {
                return Some(key);
            }
        }

        None
    }
}
