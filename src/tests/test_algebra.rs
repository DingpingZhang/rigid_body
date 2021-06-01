mod test_vec2 {
    use crate::algebra::{equals_float, Vec2};

    #[test]
    fn test_dot() {
        let zero = Vec2::new(0.0, 0.0);
        let one = Vec2::new(1.0, 1.0);
        let vector1 = Vec2::new(2.0, 42.0);
        let vector2 = Vec2::new(-42.0, 2.0);

        assert!(equals_float(zero * vector1, 0.0));
        assert!(equals_float(one * vector1, 44.0));
        assert!(equals_float(vector1 * vector2, 0.0));
    }

    #[test]
    fn test_multiply() {
        let zero = Vec2::new(0.0, 0.0);
        let one = Vec2::new(1.0, 1.0);
        let vector1 = Vec2::new(2.0, 42.0);

        assert_eq!(zero * 42.0, zero);
        assert_eq!(one * 42.0, Vec2::new(42.0, 42.0));
        assert_eq!(vector1 * 0.0, zero);
        assert_eq!(vector1 * 10.0, Vec2::new(20.0, 420.0));
    }

    #[test]
    fn test_add() {
        let zero = Vec2::new(0.0, 0.0);
        let one = Vec2::new(1.0, 1.0);
        let vector1 = Vec2::new(2.0, 42.0);

        assert_eq!(zero + one, one);
        assert_eq!(vector1 + zero, vector1);
        assert_eq!(vector1 + vector1, Vec2::new(4.0, 84.0));
    }

    #[test]
    fn test_subtract() {
        let zero = Vec2::new(0.0, 0.0);
        let one = Vec2::new(1.0, 1.0);
        let vector1 = Vec2::new(2.0, 42.0);

        assert_eq!(zero - one, Vec2::new(-1.0, -1.0));
        assert_eq!(one - zero, one);
        assert_eq!(vector1 - zero, vector1);
        assert_eq!(vector1 - vector1, zero);
    }

    #[test]
    fn test_length_squared() {
        let zero = Vec2::new(0.0, 0.0);
        let one = Vec2::new(1.0, 1.0);
        let vector1 = Vec2::new(-2.0, 42.0);

        assert_eq!(zero.length_squared(), 0.0);
        assert_eq!(one.length_squared(), 2.0);
        assert_eq!(vector1.length_squared(), 2.0 * 2.0 + 42.0 * 42.0);
    }
}
