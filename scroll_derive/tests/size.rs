use scroll::{BE, Endian, ctx::ActualSizeWith};
use scroll_derive::ActualSizeWith;

#[derive(Debug)]
pub struct VariableZeroes {
    pub zeroes: u8,
}

impl ActualSizeWith<Endian> for VariableZeroes {
    fn actual_size_with(&self, _ctx: &Endian) -> usize {
        self.zeroes as usize
    }
}

#[derive(Debug, ActualSizeWith)]
pub struct VariableSized {
    pub foo: u32,
    pub zeroes: VariableZeroes,
}

#[test]
fn test_variable_sized() {
    let vs = VariableSized {
        foo: 0x01020304,
        zeroes: VariableZeroes { zeroes: 3 },
    };
    assert_eq!(vs.actual_size_with(&BE), 4 + 3);
}
