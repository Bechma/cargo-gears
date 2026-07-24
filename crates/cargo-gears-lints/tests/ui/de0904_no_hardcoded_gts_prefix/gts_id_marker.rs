macro_rules! gts_id {
    ($suffix:literal) => {
        $suffix
    };
}

// Should not trigger DE0904 - hard-coded GTS ID prefix
const TYPE_ID: &str = gts_id!("cf.core.users.user.v1~");

fn main() {
    let _ = TYPE_ID;
}
