pub struct PNGOptions {
    pub look_ahead_buffer: usize,
    pub search_buffer: usize,
}

impl Default for PNGOptions {
    fn default() -> Self {
        const SLIDING_WINDOW : usize = 32 * 1024;
        const LOOK_AHEAD_BUFFER: usize = 258;
        PNGOptions {
            look_ahead_buffer: LOOK_AHEAD_BUFFER,
            search_buffer: SLIDING_WINDOW - LOOK_AHEAD_BUFFER
        }
    }
}
