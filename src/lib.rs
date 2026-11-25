#[cfg(test)]
mod tests {
    #[test]
    fn associated_constants() {
        assert_eq!(i8::BITS, 8);
        assert_eq!(i8::MIN, -128);
        assert_eq!(u8::MIN, 0);
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);
    }
    // BIT & BYTE OPERATIONS
    #[test]
    fn count_ones() {
        let x = 100_u8; //01100100 
        assert_eq!(x.count_ones(), 3);
    }
    #[test]
    fn count_zeros() {
        let x = 100_u8; //01100100 
        assert_eq!(x.count_zeros(), 5);
    }
    #[test]
    fn leading_ones() {
        assert_eq!(200_u8.leading_ones(), 2); //11001000
        assert_eq!(200_u16.leading_ones(), 0); //00000000 11001000
    }
    #[test]
    fn leading_zeros() {
        assert_eq!(4_u8.leading_zeros(), 5); //00000100
        assert_eq!(4_u16.leading_zeros(), 13); //00000000 00000100
    }
    #[test]
    fn trailing_ones() {
        assert_eq!(19_u8.trailing_ones(), 2); //00010011
        assert_eq!(19_u16.trailing_ones(), 2); //00000000 00010011
    }
    #[test]
    fn trailing_zeros() {
        assert_eq!(4_u8.trailing_zeros(), 2); //00000100
        assert_eq!(4_u16.trailing_zeros(), 2); //00000000 00000100
    }
    #[test]
    fn isolate_highest_one() {
        assert_eq!(19_u8.isolate_highest_one(), 16); //00010011 | 00010000
        assert_eq!(0_u8.isolate_highest_one(), 0);
    }
    #[test]
    fn isolate_lowest_one() {
        assert_eq!(19_u8.isolate_lowest_one(), 1); //00010011 | 00000001
        assert_eq!(0_u8.isolate_lowest_one(), 0);
    }
    #[test]
    fn highest_one() {
        assert_eq!(19_u8.highest_one(), Some(4)); //00010011
        assert_eq!(0_u8.highest_one(), None);
    }
    #[test]
    fn lowest_one() {
        assert_eq!(19_u8.lowest_one(), Some(0)); //00010011
        assert_eq!(0_u8.lowest_one(), None);
    }
    #[test]
    fn rotate_left() {
        assert_eq!(100_u8.rotate_left(3), 35); //01100100 | 00100011
    }
    #[test]
    fn rotate_right() {
        assert_eq!(100_u8.rotate_right(3), 140); //01100100 | 10001100
    }
    #[test]
    fn checked_shl() {
        let x = 5_u8; //00000101
        assert_eq!(x.checked_shl(2), Some(20)); //00010100
        assert_eq!(x.checked_shl(6), Some(64)); //01000000
        assert_eq!(x.checked_shl(8), None);
    }
    #[test]
    fn overflowing_shl() {
        let x = 5_u8; //00000101
        assert_eq!(x.overflowing_shl(2), (20, false)); //00010100
        assert_eq!(x.overflowing_shl(6), (64, false)); //01000000
        assert_eq!(x.overflowing_shl(25), (10, true)); //25 % 8 = 1 | 00001010
    }
    #[test]
    fn unbounded_shl() {
        let x = 5_u8; //00000101
        assert_eq!(x.unbounded_shl(2), 20); //00010100
        assert_eq!(x.unbounded_shl(6), 64); //01000000
        assert_eq!(x.unbounded_shl(8), 0);
    }
    #[test]
    fn strict_shl() {
        let x = 5_u8; //00000101
        assert_eq!(x.strict_shl(2), 20); //00010100
        assert_eq!(x.strict_shl(6), 64); //01000000
        // let _ = x.strict_shl(8); //panic
    }
    #[test]
    fn wrapping_shl() {
        let x = 5_u8; //00000101
        assert_eq!(x.wrapping_shl(2), 20); //00010100
        assert_eq!(x.wrapping_shl(6), 64); //01000000
        assert_eq!(x.wrapping_shl(25), 10); //25 % 8 = 1 | 00001010
    }
    #[test]
    fn checked_shr() {
        let x = 20_i8; //00010100
        assert_eq!(x.checked_shr(2), Some(5)); //00000101
        assert_eq!(x.checked_shr(4), Some(1)); //00000001
        assert_eq!(x.checked_shr(8), None);
    }
    #[test]
    fn overflowing_shr() {
        let x = 20_i8; //00010100
        assert_eq!(x.overflowing_shr(2), (5, false)); //00000101
        assert_eq!(x.overflowing_shr(4), (1, false)); //00000001
        assert_eq!(x.overflowing_shr(25), (10, true)); //25 % 8 = 1 | 00001010
    }
    #[test]
    fn unbounded_shr() {
        let x = 20_i8; //00010100
        assert_eq!(x.unbounded_shr(2), 5); //00000101
        assert_eq!(x.unbounded_shr(4), 1); //00000001
        assert_eq!(x.unbounded_shr(8), 0);
        assert_eq!((-x).unbounded_shr(8), -1);
    }
    #[test]
    fn strict_shr() {
        let x = 20_i8; //00010100
        assert_eq!(x.strict_shr(2), 5); //00000101
        assert_eq!(x.strict_shr(4), 1); //00000001
        // let _ = x.strict_shr(8); //panic
    }
    #[test]
    fn wrapping_shr() {
        let x = 20_i8; //00010100
        assert_eq!(x.wrapping_shr(2), 5); //00000101
        assert_eq!(x.wrapping_shr(4), 1); //00000001
        assert_eq!(x.wrapping_shr(25), 10); //25 % 8 = 1 | 00001010
    }
    #[test]
    fn swap_bytes() {
        assert_eq!(3_u8.swap_bytes(), 3);
        let x = 0b_00000001_00000010_00000100_00001000_u32;
        assert_eq!(x.swap_bytes(), 0b_00001000_00000100_00000010_00000001_u32);
    }
    #[test]
    fn reverse_bits() {
        let x = 5_u8; //00000101
        assert_eq!(x.reverse_bits(), 160); //10100000
    }
    #[test]
    fn bit_width() {
        assert_eq!(5_u32.bit_width(), 3); //00000000 00000000 00000000 00000101
        assert_eq!(0_u32.bit_width(), 0);
    }
    #[test]
    fn to_be() {
        let x = 0x01020304_u32;
        if cfg!(target_endian = "big") {
            assert_eq!(x.to_be(), x);
        } else {
            assert_eq!(x.to_be(), x.swap_bytes());
        }
    }
    #[test]
    fn from_be() {
        let x = 0x01020304_u32;
        if cfg!(target_endian = "big") {
            assert_eq!(u32::from_be(x), x);
        } else {
            assert_eq!(u32::from_be(x), x.swap_bytes());
        }
        let x_be = x.to_be();
        let x_value = u32::from_be(x_be);
        assert_eq!(x_value, x);
    }
    #[test]
    fn to_le() {
        let x = 0x01020304_u32;
        if cfg!(target_endian = "little") {
            assert_eq!(x.to_le(), x);
        } else {
            assert_eq!(x.to_le(), x.swap_bytes());
        }
    }
    #[test]
    fn from_le() {
        let x = 0x01020304_u32;
        if cfg!(target_endian = "little") {
            assert_eq!(u32::from_le(x), x);
        } else {
            assert_eq!(u32::from_le(x), x.swap_bytes());
        }
        let x_le = x.to_le();
        let x_value = u32::from_le(x_le);
        assert_eq!(x_value, x);
    }
    #[test]
    fn to_be_bytes() {
        assert_eq!(0x01020304_u32.to_be_bytes(), [0x01, 0x02, 0x03, 0x04]);
    }
    #[test]
    fn from_be_bytes() {
        assert_eq!(u32::from_be_bytes([0x01, 0x02, 0x03, 0x04]), 0x01020304_u32);
    }
    #[test]
    fn to_le_bytes() {
        assert_eq!(0x01020304_u32.to_le_bytes(), [0x04, 0x03, 0x02, 0x01]);
    }
    #[test]
    fn from_le_bytes() {
        assert_eq!(u32::from_le_bytes([0x04, 0x03, 0x02, 0x01]), 0x01020304_u32);
    }
    #[test]
    fn to_ne_bytes() {
        let x = 0x01020304_u32.to_ne_bytes();
        if cfg!(target_endian = "big") {
            assert_eq!(x, [0x01, 0x02, 0x03, 0x04]);
        } else {
            assert_eq!(x, [0x04, 0x03, 0x02, 0x01]);
        }
    }
    #[test]
    fn from_ne_bytes() {
        let x = u32::from_ne_bytes(if cfg!(target_endian = "big") {
            [0x01, 0x02, 0x03, 0x04]
        } else {
            [0x04, 0x03, 0x02, 0x01]
        });
        assert_eq!(x, 0x01020304_u32);
    }
    // ADDITION OPERATIONS
    #[test]
    fn checked_add() {
        assert_eq!(120_i8.checked_add(7), Some(i8::MAX));
        assert_eq!(120_i8.checked_add(8), None);
        assert_eq!(i8::MIN.checked_add(-1), None);
    }
    #[test]
    fn checked_add_unsigned() {
        assert_eq!(120_i8.checked_add_unsigned(7_u8), Some(i8::MAX));
        assert_eq!(120_i8.checked_add_unsigned(8_u8), None);
    }
    #[test]
    fn checked_add_signed() {
        assert_eq!(120_u8.checked_add_signed(-7_i8), Some(113));
        assert_eq!(120_u8.checked_add_signed(8_i8), Some(128));
        assert_eq!(u8::MAX.checked_add_signed(1_i8), None);
        assert_eq!(u8::MIN.checked_add_signed(-1_i8), None);
    }
    #[test]
    fn overflowing_add() {
        assert_eq!(120_i8.overflowing_add(7), (i8::MAX, false));
        assert_eq!(i8::MAX.overflowing_add(3), (-126, true)); //3 | -128, -127, -126
        assert_eq!(i8::MIN.overflowing_add(-3), (125, true)); //3 | 127, 126, 125
    }
    #[test]
    fn overflowing_add_unsigned() {
        assert_eq!(120_i8.overflowing_add_unsigned(7_u8), (i8::MAX, false));
        assert_eq!(i8::MAX.overflowing_add_unsigned(3_u8), (-126, true)); //3 | -128, -127, -126
    }
    #[test]
    fn overflowing_add_signed() {
        assert_eq!(120_u8.overflowing_add_signed(-7_i8), (113, false));
        assert_eq!(120_u8.overflowing_add_signed(8_i8), (128, false));
        assert_eq!(u8::MAX.overflowing_add_signed(3_i8), (2, true)); //3 | 0, 1, 2
        assert_eq!(u8::MIN.overflowing_add_signed(-3_i8), (253, true)); //3 | 255, 254, 253
    }
    #[test]
    fn saturating_add() {
        assert_eq!(120_i8.saturating_add(7), i8::MAX);
        assert_eq!(120_i8.saturating_add(8), i8::MAX);
        assert_eq!(i8::MIN.saturating_add(-1), i8::MIN);
    }
    #[test]
    fn saturating_add_unsigned() {
        assert_eq!(120_i8.saturating_add_unsigned(7_u8), i8::MAX);
        assert_eq!(120_i8.saturating_add_unsigned(8_u8), i8::MAX);
    }
    #[test]
    fn saturating_add_signed() {
        assert_eq!(120_u8.saturating_add_signed(-7_i8), 113);
        assert_eq!(120_u8.saturating_add_signed(8_i8), 128);
        assert_eq!(u8::MAX.saturating_add_signed(1_i8), u8::MAX);
        assert_eq!(u8::MIN.saturating_add_signed(-1_i8), u8::MIN);
    }
    #[test]
    fn strict_add() {
        assert_eq!(120_i8.strict_add(7), i8::MAX);
        // let _ = 120_i8.strict_add(8); //panic
        // let _ = i8::MIN.strict_add(-1); //panic
    }
    #[test]
    fn strict_add_unsigned() {
        assert_eq!(120_i8.strict_add_unsigned(7_u8), i8::MAX);
        // let _ = 120_i8.strict_add_unsigned(8_u8); //panic
    }
    #[test]
    fn strict_add_signed() {
        assert_eq!(120_u8.strict_add_signed(-7_i8), 113);
        assert_eq!(120_u8.strict_add_signed(8_i8), 128);
        // let _ = u8::MAX.strict_add_signed(1_i8); //panic
        // let _ = u8::MIN.strict_add_signed(-1_i8); //panic
    }
    #[test]
    fn wrapping_add() {
        assert_eq!(120_i8.wrapping_add(7), i8::MAX);
        assert_eq!(i8::MAX.wrapping_add(3), -126); //3 | -128, -127, -126
        assert_eq!(i8::MIN.wrapping_add(-3), 125); //3 | 127, 126, 125
    }
    #[test]
    fn wrapping_add_unsigned() {
        assert_eq!(120_i8.wrapping_add_unsigned(7_u8), i8::MAX);
        assert_eq!(i8::MAX.wrapping_add_unsigned(3_u8), -126); //3 | -128, -127, -126
    }
    #[test]
    fn wrapping_add_signed() {
        assert_eq!(120_u8.wrapping_add_signed(-7_i8), 113);
        assert_eq!(120_u8.wrapping_add_signed(8_i8), 128);
        assert_eq!(u8::MAX.wrapping_add_signed(3_i8), 2); //3 | 0, 1, 2
        assert_eq!(u8::MIN.wrapping_add_signed(-3_i8), 253); //3 | 255, 254, 253
    }
}
