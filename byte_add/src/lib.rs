pub fn add_u8_checked(a: u8, b: u8) -> Option<u8> {
    let res: u16 = a as u16 + b as u16;
    if res > u8::MAX as u16 {
        None
    } else {
        Some(res as u8)
    }
}

pub fn add_u8_wrapping(a: u8, b: u8) -> u8 {
    let res: u16 = a as u16 + b as u16;
    res as u8
}

pub fn add_u8_saturating(a: u8, b: u8) -> u8 {
    let res: u16 = a as u16 + b as u16;
    if res > u8::MAX as u16 {
        u8::MAX
    } else {
        res as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_add() {
        assert_eq!(add_u8_checked(10, 20), Some(30));
        assert_eq!(add_u8_checked(255, 1), None);
    }

    #[test]
    fn wrapping_add() {
        assert_eq!(add_u8_wrapping(10, 20), 30);
        assert_eq!(add_u8_wrapping(255, 1), 0);
    }

    #[test]
    fn saturating_add() {
        assert_eq!(add_u8_saturating(10, 20), 30);
        assert_eq!(add_u8_saturating(255, 1), 255);
    }
}
