use std::io::{self, Read};

const MAX_READ_LIMIT_BYTES: u64 = 10 * 1024 * 1024;

struct InputStats {
    pub line_count: usize,
    pub word_count: usize,
    pub bytes_count: usize,
}

fn main() {
    let stdin = io::stdin();
    let intput_stats = process_input(stdin).expect("Failed to read input");

    println!(
        "{} {} {}",
        intput_stats.line_count, intput_stats.word_count, intput_stats.bytes_count,
    );
}

fn process_input<R: Read>(input: R) -> io::Result<InputStats> {
    let mut buf = Vec::new();
    input.take(MAX_READ_LIMIT_BYTES).read_to_end(&mut buf)?;

    return Ok(InputStats {
        line_count: buf.iter().filter(|&b| *b == b'\n').count(),
        word_count: buf
            .split(|&b| b.is_ascii_whitespace())
            .filter(|c| !c.is_empty())
            .count(),
        bytes_count: buf.len(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_returns_zero_counts() {
        let empty_input = "".as_bytes();

        let result = process_input(empty_input).unwrap();

        assert_eq!(result.line_count, 0);
        assert_eq!(result.word_count, 0);
        assert_eq!(result.bytes_count, 0);
    }

    #[test]
    fn single_word_has_no_newlines() {
        let empty_input = "hello".as_bytes();

        let result = process_input(empty_input).unwrap();

        assert_eq!(result.line_count, 0);
        assert_eq!(result.word_count, 1);
        assert_eq!(result.bytes_count, 5);
    }

    #[test]
    fn single_word_with_newline_counts_line() {
        let empty_input = "hello\n".as_bytes();

        let result = process_input(empty_input).unwrap();

        assert_eq!(result.line_count, 1);
        assert_eq!(result.word_count, 1);
        assert_eq!(result.bytes_count, 6);
    }

    #[test]
    fn two_words_with_newline_counts_both() {
        let empty_input = "hello rust\n".as_bytes();

        let result = process_input(empty_input).unwrap();

        assert_eq!(result.line_count, 1);
        assert_eq!(result.word_count, 2);
        assert_eq!(result.bytes_count, 11);
    }

    #[test]
    fn leading_trailing_whitespace_is_ignored() {
        let empty_input = " hello rust \n".as_bytes();

        let result = process_input(empty_input).unwrap();

        assert_eq!(result.line_count, 1);
        assert_eq!(result.word_count, 2);
        assert_eq!(result.bytes_count, 13);
    }

    #[test]
    fn tabs_and_newlines_split_words() {
        let empty_input = "a\tb\nc".as_bytes();

        let result = process_input(empty_input).unwrap();

        assert_eq!(result.line_count, 1);
        assert_eq!(result.word_count, 3);
        assert_eq!(result.bytes_count, 5);
    }
}
