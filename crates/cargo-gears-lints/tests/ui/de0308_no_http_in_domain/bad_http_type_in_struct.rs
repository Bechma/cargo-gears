// simulated_dir=/cyberfabric/modules/some_module/domain/service.rs
#![feature(register_tool)]
#![register_tool(dylint)]
#![allow(dead_code)]
#![allow(de0309_must_have_domain_model)]

pub struct Hello {
    // Should trigger DE0308 - HTTP in domain
    param1: http::StatusCode,
}

fn main() {}
