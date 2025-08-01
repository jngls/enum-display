#![no_std]

extern crate alloc;

use enum_display::*;
use alloc::string::ToString;

use alloc::format;

#[derive(EnumDisplay)]
enum NoStd {
    #[display("format unavailable")]
    VariantFallback
}

pub fn test_no_std() {
    assert_eq!(NoStd::VariantFallback.to_string(), "VariantFallback");
}
