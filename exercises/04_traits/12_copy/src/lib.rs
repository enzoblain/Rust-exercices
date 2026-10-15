use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct WrappingU32 {
    pub value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    type Output = WrappingU32;

    fn add(self, other: WrappingU32) -> WrappingU32 {
        WrappingU32::new(self.value.wrapping_add(other.value))
    }
}

impl PartialEq for WrappingU32 {
    fn eq(&self, other: &WrappingU32) -> bool {
        self.value == other.value
    }
}

impl Eq for WrappingU32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);

        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
