# DE0904: No Hard-Coded GTS Prefix

Rejects Rust source that writes a GTS identifier with the `gts.` prefix,
including in every ordinary Rust attribute and macro invocation. Use
`gts_id!("<suffix>")` so the final prefix comes from the consuming crate's
`GTS_ID_PREFIX` configuration.

The lint runs before macro expansion. This deliberately limits it to
user-authored source and makes it compatible with wrapper macros that generate
additional GTS values internally.

```rust,ignore
// Wrong
const TYPE_ID: &str = "gts.cf.core.users.user.v1~";

// Correct
const TYPE_ID: &str = gts_id!("cf.core.users.user.v1~");
```

Both ordinary literals and macro or attribute arguments with a user-written
`gts.` prefix are rejected.
