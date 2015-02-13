#![allow(dead_code)]

use std::default::Default;
use std::mem;

use cql_ffi::bytes::CassBytes;
use cql_ffi::types::cass_int32_t;

#[repr(C)]
#[derive(Copy,Debug)]
pub struct CassDecimal {
    pub scale: cass_int32_t,
    pub varint: CassBytes,
}

impl Default for CassDecimal {
    fn default() -> CassDecimal { unsafe { mem::zeroed() } }
}

extern "C" {
    pub fn cass_decimal_init(scale: cass_int32_t, varint: CassBytes) -> CassDecimal;
}