// simulated_dir=/cyberfabric/modules/some_module/domain/
use fixture_macros::domain_model;

#[allow(dead_code)]
// Should not trigger DE0101 - Serde in domain
#[domain_model]
#[derive(Debug, Clone, PartialEq)]
pub struct Invoice {
    pub id: String,
    pub amount: i64,
}

#[allow(dead_code)]
// Should not trigger DE0101 - Serde in domain
#[domain_model]
#[derive(Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped,
}

fn main() {}
