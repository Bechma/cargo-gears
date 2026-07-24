//! A hard-coded `gts.` prefix is rejected inside macro input.

macro_rules! gts_value {
    ($value:expr) => {
        const _: &str = $value;
    };
}

// Should trigger DE0904 - hard-coded GTS ID prefix
gts_value!("gts.cf.de0904.tests.type.v1~");

fn main() {}
