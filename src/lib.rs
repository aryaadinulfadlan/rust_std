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
}
