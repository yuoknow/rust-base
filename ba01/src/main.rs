use std::io::{self, Read};

const MAX_READ_LIMIT_BYTES: u64 = 10 * 1024 * 1024;

fn main() {
    let stdin = io::stdin();
    let bytes_count = process_input(stdin).expect("Failed to read input");

    println!("{}", bytes_count);
}

fn process_input<R: Read>(input: R) -> io::Result<usize> {
    let mut buf = Vec::new();
    input.take(MAX_READ_LIMIT_BYTES).read_to_end(&mut buf)?;

    return Ok(buf.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_has_four_bytes() {
        let crab_emoji = "🦀".as_bytes();

        let len = process_input(crab_emoji).unwrap();

        assert_eq!(len, 4);
    }

    #[test]
    fn binary_input_returns_byte_count() {
        let fake_input: &[u8] = &[0, 255, 10, 0, 42];

        let len = process_input(fake_input).unwrap();

        assert_eq!(len, 5);
    }
}
