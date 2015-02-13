#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use cql_ffi::error::CassError;
use cql_ffi::uuid::CassUuid;
use cql_ffi::bytes::CassBytes;
use cql_ffi::string::CassString;
use cql_ffi::decimal::CassDecimal;
use cql_ffi::inet::CassInet;
use cql_ffi::types::cass_size_t;
use cql_ffi::types::cass_bool_t;
use cql_ffi::types::cass_double_t;
use cql_ffi::types::cass_int32_t;
use cql_ffi::types::cass_int64_t;
use cql_ffi::types::cass_float_t;

#[derive(Copy,Debug)]
pub enum CassCollection{ }

#[repr(C)]
#[derive(Debug,Copy)]
pub enum CassCollectionType {
    LIST = 32is,
    MAP = 33,
    SET = 34
}

extern "C" {
    pub fn cass_collection_new(_type: CassCollectionType, item_count: cass_size_t) -> *mut CassCollection;
    pub fn cass_collection_free(collection: *mut CassCollection);
    pub fn cass_collection_append_int32(collection: *mut CassCollection, value: cass_int32_t) -> CassError;
    pub fn cass_collection_append_int64(collection: *mut CassCollection, value: cass_int64_t) -> CassError;
    pub fn cass_collection_append_float(collection: *mut CassCollection, value: cass_float_t) -> CassError;
    pub fn cass_collection_append_double(collection: *mut CassCollection, value: cass_double_t) -> CassError;
    pub fn cass_collection_append_bool(collection: *mut CassCollection, value: cass_bool_t) -> CassError;
    pub fn cass_collection_append_string(collection: *mut CassCollection, value: CassString) -> CassError;
    pub fn cass_collection_append_bytes(collection: *mut CassCollection, value: CassBytes) -> CassError;
    pub fn cass_collection_append_uuid(collection: *mut CassCollection, value: CassUuid) -> CassError;
    pub fn cass_collection_append_inet(collection: *mut CassCollection, value: CassInet) -> CassError;
    pub fn cass_collection_append_decimal(collection: *mut CassCollection, value: CassDecimal) -> CassError;
}