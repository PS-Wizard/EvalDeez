pub struct Board(pub(crate) u64);

impl Board {
    pub fn has_bit(&self, idx: u8) -> bool {
        if idx >= 64 {
            return false;
        }
        (self.0 & (1 << idx)) != 0
    }

    pub fn set_bit(&mut self, idx: usize) {
        self.0 |= 1 << idx;
    }
}
