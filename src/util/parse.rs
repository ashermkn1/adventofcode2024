pub trait ToDigit {
    fn to_digit(&self) -> Option<u8>;
}

impl ToDigit for u8 {
    fn to_digit(&self) -> Option<u8> {
        if self.is_ascii_digit() {
            Some(*self - b'0')
        } else {
            None
        }
    }
}
