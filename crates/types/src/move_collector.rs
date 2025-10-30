use crate::move_type::Move;
use std::mem::MaybeUninit;

/// A struct holding a fixed array of side 256 (max legal moves in any position) to avoid
/// allocating memory when generating moves and a count usize that represents the number of moves
/// in the array. Elements of the array are `MaybeUninit`  for better performance
pub struct MoveCollector {
    pub captures: [MaybeUninit<Move>; 128], // Reduced size
    pub quiets: [MaybeUninit<Move>; 128],   // Reduced size
    pub capture_count: usize,
    pub quiet_count: usize,
}

impl MoveCollector {
    /// Returns a New Movecollector
    #[inline(always)]
    pub fn new() -> Self {
        MoveCollector {
            captures: unsafe { MaybeUninit::uninit().assume_init() },
            quiets: unsafe { MaybeUninit::uninit().assume_init() },
            capture_count: 0,
            quiet_count: 0,
        }
    }

    #[inline(always)]
    pub fn push_capture(&mut self, m: Move) {
        unsafe {
            *self.captures.get_unchecked_mut(self.capture_count) = MaybeUninit::new(m);
        }
        self.capture_count += 1;
    }

    #[inline(always)]
    pub fn push_quiet(&mut self, m: Move) {
        unsafe {
            *self.quiets.get_unchecked_mut(self.quiet_count) = MaybeUninit::new(m);
        }
        self.quiet_count += 1;
    }

    /// Clears the array, by setting the count to 0 so the next pass of generating moves overwrites
    /// the previous values
    #[inline(always)]
    pub fn clear_quiet(&mut self) {
        self.quiet_count = 0;
    }

    /// Clears the array, by setting the count to 0 so the next pass of generating moves overwrites
    /// the previous values
    #[inline(always)]
    pub fn clear_captures(&mut self) {
        self.capture_count = 0;
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.capture_count = 0;
        self.quiet_count = 0;
    }

    #[inline(always)]
    pub fn captures_iter(&self) -> MoveCollectorIter<'_> {
        MoveCollectorIter {
            moves: &self.captures,
            index: 0,
            len: self.capture_count,
        }
    }

    #[inline(always)]
    pub fn quiets_iter(&self) -> MoveCollectorIter<'_> {
        MoveCollectorIter {
            moves: &self.quiets,
            index: 0,
            len: self.quiet_count,
        }
    }
}

/// Iterator for MoveCollector - avoids bounds checking
pub struct MoveCollectorIter<'a> {
    moves: &'a [MaybeUninit<Move>; 128], // Changed from 256 to 128
    index: usize,
    len: usize,
}

impl<'a> Iterator for MoveCollectorIter<'a> {
    type Item = Move;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let m = unsafe { self.moves.get_unchecked(self.index).assume_init() };
            self.index += 1;
            Some(m)
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.index;
        (remaining, Some(remaining))
    }
}
