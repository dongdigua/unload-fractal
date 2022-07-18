pub fn hash(x: i32, y: i32) -> i16 {
    let tmp = x ^ y;
    ((tmp >> 16) as i16) ^ (tmp as i16)
}

#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_equals_y() {
        assert_eq!(0, hash(0, 0));
        assert_eq!(0, hash(5, 5));
        assert_eq!(0, hash(-3, -3));
    }

    #[test]
    fn test_diagonal() {
        assert_eq!(0, hash(-1, 0));
        assert_eq!(0, hash(-2, 1));
        assert_eq!(0, hash(0, -1));
        assert_eq!(0, hash(1, -2));
    }

}
