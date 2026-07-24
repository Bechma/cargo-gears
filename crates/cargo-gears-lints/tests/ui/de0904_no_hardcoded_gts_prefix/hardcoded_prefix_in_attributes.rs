//! A hard-coded `gts.` prefix is rejected in every ordinary attribute.

// Should trigger DE0904 - hard-coded GTS ID prefix
#[doc = "gts.cf.de0904.tests.doc.v1~"]
struct Documented;

// Should trigger DE0904 - hard-coded GTS ID prefix
#[allow(dead_code, reason = "gts.cf.de0904.tests.reason.v1~")]
struct Allowed;

fn main() {
    let _ = (Documented, Allowed);
}
