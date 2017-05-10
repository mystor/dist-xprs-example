//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICache.idl
//


pub type nsCacheStoragePolicy = libc::int32_t;


pub type nsCacheAccessMode = libc::int32_t;


pub mod nsICache_consts {
    pub const ACCESS_NONE: i64 = 0;
    pub const ACCESS_READ: i64 = 1;
    pub const ACCESS_WRITE: i64 = 2;
    pub const ACCESS_READ_WRITE: i64 = 3;
    pub const STORE_ANYWHERE: i64 = 0;
    pub const STORE_IN_MEMORY: i64 = 1;
    pub const STORE_ON_DISK: i64 = 2;
    pub const STORE_OFFLINE: i64 = 4;
    pub const NOT_STREAM_BASED: i64 = 0;
    pub const STREAM_BASED: i64 = 1;
    pub const NON_BLOCKING: i64 = 0;
    pub const BLOCKING: i64 = 1;
    pub const NO_EXPIRATION_TIME: i64 = 4294967295;
}


