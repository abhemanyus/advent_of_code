#![feature(iter_next_chunk)]
use advent_of_code::load_file;

const MARKER: usize = 14;

fn main() {
    let data = load_file!("six");
    let mut chars = data.chars();
    let mut buffer: [char; MARKER] = chars.next_chunk().unwrap();
    for (i, ch) in chars.enumerate() {
        buffer.rotate_left(1);
        buffer[MARKER - 1] = ch;
        if check_unique(&buffer) {
            println!(
                "Ding ding, the answer is {} at {}",
                String::from_iter(buffer.iter()),
                i + MARKER + 1
            );
            break;
        }
    }
}

fn check_unique(chars: &[char; MARKER]) -> bool {
    for x in 0..MARKER {
        for y in 0..MARKER {
            if x != y && chars[x] == chars[y] {
                return false;
            }
        }
    }
    true
}
