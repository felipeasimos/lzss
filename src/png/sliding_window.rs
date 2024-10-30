pub struct SlidingWindow<'a, T: 'a> {
    v: &'a [T],
    size: usize,
    capacity: usize,
}

#[derive(Debug)]
pub enum SlidingWindowState {
    Expanded,
    Maintained,
    Shrinked,
}

impl <'a, T: 'a> SlidingWindow<'a, T> {
    pub fn new(slice: &'a [T], capacity: usize) -> Self {
        Self {
            v: slice,
            size: 0,
            capacity
        }
    }
}

impl <'a, T> Iterator for SlidingWindow<'a, T> {
    type Item = (&'a [T], SlidingWindowState);

    fn next(&mut self) -> Option<(&'a [T], SlidingWindowState)> {
        if (self.v.len() == 0 && self.size == 0) || self.capacity == 0 {
            return None
        }
        let state;
        // expanding
        if self.size < self.v.len() && self.size < self.capacity {
            state = SlidingWindowState::Expanded;
            self.size += 1;
        // maintaining
        } else if self.capacity < self.v.len() && self.size == self.capacity {
            state = SlidingWindowState::Maintained;
            self.v = &self.v[1..];
        // shrinking
        } else if self.size == self.v.len() && self.v.len() > 1 {
            state = SlidingWindowState::Shrinked;
            self.size -= 1;
            self.v = &self.v[1..];
        } else {
            return None
        }
        Some((&self.v[..self.size], state))
    }
}
