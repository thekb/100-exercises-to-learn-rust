// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl core::convert::Into<u32> for WrappingU32 {
    fn into(self) -> u32 {
        return self.value
    }
}

impl core::convert::From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        return WrappingU32 { value: value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
