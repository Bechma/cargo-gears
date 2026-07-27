// simulated_dir=/cf-gears/gears/example/src/domain/

// Test: Domain structs WITH #[domain_model] should NOT trigger lint

// In real code, domain_model comes from toolkit_macros;
// fixture_macros provides a no-op stand-in for testing.
use fixture_macros::domain_model;

// Should not trigger DE0309 - domain_model attribute
#[domain_model]
pub struct User {
    pub id: i64,
    pub email: String,
}

// Should not trigger DE0309 - domain_model attribute
#[domain_model]
pub enum UserStatus {
    Active,
    Inactive,
}

// Should not trigger DE0309 - domain_model attribute
#[domain_model]
pub struct ServiceConfig {
    pub timeout_ms: u64,
}

fn main() {}
