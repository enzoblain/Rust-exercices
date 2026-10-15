pub struct WrappingU32 {
    pub value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

pub fn example() {
    let _wrapping: WrappingU32 = 42.into();
    let _wrapping = WrappingU32::from(42);
}
