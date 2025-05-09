#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(clippy::all)]
mod bindings {
    include!("bindings.rs");
}

pub use bindings::*;
