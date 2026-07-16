//! A suffix wrapped in `gts_id!` remains valid inside macro input.

macro_rules! gts_id {
    ($suffix:literal) => {
        $suffix
    };
}

macro_rules! gts_value {
    ($value:expr) => {
        const _: &str = $value;
    };
}

// Should not trigger DE0904 - hard-coded GTS ID prefix
gts_value!(gts_id!("cf.de0904.tests.type.v1~"));

fn main() {}
