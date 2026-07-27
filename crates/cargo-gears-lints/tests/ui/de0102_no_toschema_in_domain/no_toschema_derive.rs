// simulated_dir=/cyberfabric/modules/some_module/domain/
use fixture_macros::domain_model;

#[allow(dead_code)]
// Should not trigger DE0102 - ToSchema in domain
#[domain_model]
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
}

#[allow(dead_code)]
// Should not trigger DE0102 - ToSchema in domain
#[domain_model]
#[derive(Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

fn main() {}
