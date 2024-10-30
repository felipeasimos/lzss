use std::fmt::Display;
use std::iter::zip;

use super::sliding_window::{SlidingWindow,SlidingWindowState};
use super::simd::find_first_match;

#[derive(Debug, PartialEq, Eq)]
pub enum LZSSToken {
    Byte(u8),
    Ref { 
        offset: usize,
        length: usize
    }
}

impl Display for LZSSToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LZSSToken::Byte(b) => write!(f, "{}", b),
            LZSSToken::Ref { offset, length } => write!(f, "<{}, {}>", offset, length),
        }
    }
}

pub fn lzss_decode_ref<'a>(tokens: &[LZSSToken], first_index: usize, length: usize) -> Vec<u8> {
    let end = first_index + length;
    tokens[first_index..end]
        .iter()
        .enumerate()
        .map(|(index, token)| {
            match token {
                LZSSToken::Byte(b) => vec![*b],
                LZSSToken::Ref { offset, length } => {
                    let first_index = index - offset;
                    lzss_decode_ref(tokens, first_index, *length)
                }
            }
        })
        .flatten()
        .collect()
}

pub fn lzss_decode(tokens: &[LZSSToken]) -> Vec<u8> {
    tokens
        .iter()
        .enumerate()
        .map(|(index, token)| {
            match token {
                LZSSToken::Byte(b) => vec![*b],
                LZSSToken::Ref { offset, length } => {
                    let first_index = index - offset;
                    lzss_decode_ref(tokens, first_index, *length)
                }
            }
        })
        .flatten()
        .collect()
}

fn lzss_buffers<'a>(window: &'a [u8], state: &SlidingWindowState, search_buffer: usize, look_ahead_buffer: usize) -> (&'a [u8], &'a [u8]) {
    match &state {
        SlidingWindowState::Expanded => {
            let search_buffer_len = window.len() - look_ahead_buffer;
            (&window[..search_buffer_len], &window[search_buffer_len..])
        },
        SlidingWindowState::Maintained | SlidingWindowState::Shrinked => {
            let look_ahead_buffer_len = std::cmp::max(window.len() - search_buffer, 1);
            let search_buffer_len = window.len() - look_ahead_buffer_len;
            (&window[..search_buffer_len], &window[search_buffer_len..])
        }
    }
}

pub fn lzss_encode(data: &[u8], search_buffer: usize, look_ahead_buffer: usize) -> Vec<LZSSToken> {
    let capacity = search_buffer + look_ahead_buffer;
    let first_skips = core::cmp::min(look_ahead_buffer, data.len()) - 1;
    let mut windows_iter = SlidingWindow::new(data, capacity)
        .skip(first_skips)
        .take(data.len() + capacity - first_skips - search_buffer - 1);
    let mut skip_count = 0;

    std::iter::from_fn(|| {
        (0..skip_count).into_iter().for_each(|_| {
            windows_iter.next();
        });
        skip_count = 0;

        windows_iter.next().map(|(window, state)| {
            let (search_buffer, look_ahead_buffer) = lzss_buffers(window, &state, search_buffer, look_ahead_buffer);
            dbg!(window, &state, &search_buffer, &look_ahead_buffer);
            let target = look_ahead_buffer[0];
            if let Some(first_match_index) = find_first_match(window, target) {
                let length = zip(
                    look_ahead_buffer,
                    window.iter().skip(first_match_index)
                ).take_while(|(a, b)| { a == b }).count();
                if length >= 3 {
                    skip_count = length - 1;
                    let offset = search_buffer.len() - first_match_index + 1;
                    return LZSSToken::Ref {
                        offset,
                        length
                    };
                }
            }
            return LZSSToken::Byte(target);
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_byte() {
        let data: [u8; 1] = [1];
        let result = lzss_encode(&data, 2, 1);
        assert_eq!(result, [LZSSToken::Byte(1)]);
    }
}
