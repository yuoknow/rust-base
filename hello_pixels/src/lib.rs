pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] {
    let mut res: [u8; 8] = [0; 8];

    lines.iter().enumerate().for_each(|(idx, line)| {
        line.chars().enumerate().for_each(|(ch_idx, c)| match c {
            '#' => res[idx] = res[idx] | 1 << 7 - ch_idx,
            _ => (),
        });
    });

    res
}

pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] {
    bytes.map(|b| {
        (0..8)
            .rev()
            .map(|num| if b & (1 << num) != 0 { '#' } else { '.' })
            .collect()
    })
}

pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
    bytes.map(|b| b ^ 0xFF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_pixel_top_left() {
        let image = [
            "#.......", ".#......", "..#.....", "...#....", "....#...", ".....#..", "......#.",
            ".......#",
        ];
        let bytes = parse_bitmap_8x8(image);

        assert_eq!(bytes[0], 0b10000000);
        assert_eq!(bytes[7], 0b00000001);
    }

    #[test]
    fn parse_all_dots_returns_zeros() {
        let image = [
            "........", "........", "........", "........", "........", "........", "........",
            "........",
        ];
        let bytes = parse_bitmap_8x8(image);

        assert_eq!(bytes, [0u8; 8]);
    }

    #[test]
    fn parse_all_hashes_returns_ones() {
        let image = [
            "########", "########", "########", "########", "########", "########", "########",
            "########",
        ];
        let bytes = parse_bitmap_8x8(image);

        assert_eq!(bytes, [0xFFu8; 8]);
    }

    #[test]
    fn parse_and_render_are_inverse() {
        let image = [
            "..####..", ".#....#.", "#.#..#.#", "#..##..#", "#......#", "#.#..#.#", ".#....#.",
            "..####..",
        ];
        let bytes = parse_bitmap_8x8(image);
        let rendered = render_bitmap_8x8(bytes);

        assert_eq!(rendered, image);
    }

    #[test]
    fn invert_flips_all_bits() {
        let bytes: [u8; 8] = [
            0b10101010, 0b01010101, 0b11110000, 0b00001111, 0xFF, 0x00, 0b11001100, 0b00110011,
        ];
        let inverted = invert_bitmap_8x8(bytes);

        assert_eq!(
            inverted,
            [
                0b01010101, 0b10101010, 0b00001111, 0b11110000, 0x00, 0xFF, 0b00110011, 0b11001100
            ]
        );
    }

    #[test]
    fn invert_twice_returns_original() {
        let bytes: [u8; 8] = [0x0F, 0xF0, 0xAA, 0x55, 0x12, 0x34, 0x56, 0x78];
        let double_inverted = invert_bitmap_8x8(invert_bitmap_8x8(bytes));

        assert_eq!(double_inverted, bytes);
    }
}
