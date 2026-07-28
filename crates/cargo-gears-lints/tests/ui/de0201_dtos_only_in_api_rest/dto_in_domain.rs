// simulated_dir=/cyberfabric/modules/some_module/domain/
#![allow(dead_code)]
#![allow(de0309_must_have_domain_model)]

// Should trigger DE0201 - DTOs only in api/rest
pub struct UserDto {
    pub id: String,
}

fn main() {}
