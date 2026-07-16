// Should trigger DE0904 - hard-coded GTS ID prefix
const RAW_ID: &str = "gts.cf.core.users.user.v1~";

// Should trigger DE0904 - hard-coded GTS ID prefix
const BUILT_ID: &str = concat!("gts.", "cf.core.users.user.v1~");

fn main() {
    let _ = (RAW_ID, BUILT_ID);
}
