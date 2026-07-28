// simulated_dir=/cyberfabric/modules/some_module/domain/
#![allow(unused)]
#![allow(de0309_must_have_domain_model)]

mod api {
    pub mod rest {
        pub mod dto {
            pub struct UserDto;
        }
    }
}

// Should trigger DE0202 - DTOs not referenced outside api
use crate::api::rest::dto::UserDto;

pub struct UserService;

fn main() {}
