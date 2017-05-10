//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageError.idl
//


pub mod mozIStorageError_consts {
    pub const ERROR: i64 = 1;
    pub const INTERNAL: i64 = 2;
    pub const PERM: i64 = 3;
    pub const ABORT: i64 = 4;
    pub const BUSY: i64 = 5;
    pub const LOCKED: i64 = 6;
    pub const NOMEM: i64 = 7;
    pub const READONLY: i64 = 8;
    pub const INTERRUPT: i64 = 9;
    pub const IOERR: i64 = 10;
    pub const CORRUPT: i64 = 11;
    pub const FULL: i64 = 13;
    pub const CANTOPEN: i64 = 14;
    pub const EMPTY: i64 = 16;
    pub const SCHEMA: i64 = 17;
    pub const TOOBIG: i64 = 18;
    pub const CONSTRAINT: i64 = 19;
    pub const MISMATCH: i64 = 20;
    pub const MISUSE: i64 = 21;
    pub const NOLFS: i64 = 22;
    pub const AUTH: i64 = 23;
    pub const FORMAT: i64 = 24;
    pub const RANGE: i64 = 25;
    pub const NOTADB: i64 = 26;
}


#[repr(C)]
pub struct mozIStorageError {
    vtable: *const mozIStorageErrorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageError {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1f350f96, 0x7023, 0x434a,
            [0x88, 0x64, 0x40, 0xa1, 0xc4, 0x93, 0xaa, 0xc1])
    }
}

unsafe impl RefCounted for mozIStorageError {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait mozIStorageErrorCoerce {
    fn coerce_from(v: &mozIStorageError) -> &Self;
}

impl mozIStorageErrorCoerce for mozIStorageError {
    #[inline]
    fn coerce_from(v: &mozIStorageError) -> &Self {
        v
    }
}

impl mozIStorageError {
    #[inline]
    pub fn coerce<T: mozIStorageErrorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageError {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageErrorCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageError) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageErrorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long result; */
    pub get_result: unsafe extern "C" fn (this: *const mozIStorageError, aResult: *mut libc::int32_t) -> nsresult,

    /* readonly attribute AUTF8String message; */
    pub get_message: unsafe extern "C" fn (this: *const mozIStorageError, aMessage: *mut nsACString) -> nsresult,

}


impl mozIStorageError {
    /* readonly attribute long result; */
    #[inline]
    pub unsafe fn get_result(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_result)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String message; */
    #[inline]
    pub unsafe fn get_message(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_message)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


