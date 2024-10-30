use png::prelude::{lzss_encode, PNGOptions};

fn main() {
    let data: [u8; 5] = [1, 1, 1, 1, 1];
    let options = PNGOptions {
        search_buffer: 2,
        look_ahead_buffer: 1
    };
    dbg!(lzss_encode(&data, options.search_buffer, options.look_ahead_buffer));
}
