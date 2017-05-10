//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDOMException.idl
//


pub mod nsIDOMDOMException_consts {
    pub const INDEX_SIZE_ERR: i64 = 1;
    pub const DOMSTRING_SIZE_ERR: i64 = 2;
    pub const HIERARCHY_REQUEST_ERR: i64 = 3;
    pub const WRONG_DOCUMENT_ERR: i64 = 4;
    pub const INVALID_CHARACTER_ERR: i64 = 5;
    pub const NO_DATA_ALLOWED_ERR: i64 = 6;
    pub const NO_MODIFICATION_ALLOWED_ERR: i64 = 7;
    pub const NOT_FOUND_ERR: i64 = 8;
    pub const NOT_SUPPORTED_ERR: i64 = 9;
    pub const INUSE_ATTRIBUTE_ERR: i64 = 10;
    pub const INVALID_STATE_ERR: i64 = 11;
    pub const SYNTAX_ERR: i64 = 12;
    pub const INVALID_MODIFICATION_ERR: i64 = 13;
    pub const NAMESPACE_ERR: i64 = 14;
    pub const INVALID_ACCESS_ERR: i64 = 15;
    pub const VALIDATION_ERR: i64 = 16;
    pub const TYPE_MISMATCH_ERR: i64 = 17;
    pub const SECURITY_ERR: i64 = 18;
    pub const NETWORK_ERR: i64 = 19;
    pub const ABORT_ERR: i64 = 20;
    pub const URL_MISMATCH_ERR: i64 = 21;
    pub const QUOTA_EXCEEDED_ERR: i64 = 22;
    pub const TIMEOUT_ERR: i64 = 23;
    pub const INVALID_NODE_TYPE_ERR: i64 = 24;
    pub const DATA_CLONE_ERR: i64 = 25;
    pub const INVALID_POINTER_ERR: i64 = 26;
}


#[repr(C)]
pub struct nsIDOMDOMException {
    vtable: *const nsIDOMDOMExceptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDOMException {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5bd766d3, 0x57a9, 0x4833,
            [0x99, 0x5d, 0xdb, 0xe2, 0x1d, 0xa2, 0x95, 0x95])
    }
}

unsafe impl RefCounted for nsIDOMDOMException {
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
pub trait nsIDOMDOMExceptionCoerce {
    fn coerce_from(v: &nsIDOMDOMException) -> &Self;
}

impl nsIDOMDOMExceptionCoerce for nsIDOMDOMException {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMException) -> &Self {
        v
    }
}

impl nsIDOMDOMException {
    #[inline]
    pub fn coerce<T: nsIDOMDOMExceptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDOMException {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMDOMExceptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMException) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDOMExceptionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short code; */
    pub get_code: unsafe extern "C" fn (this: *const nsIDOMDOMException, aCode: *mut libc::uint16_t) -> nsresult,

}


impl nsIDOMDOMException {
    /* readonly attribute unsigned short code; */
    #[inline]
    pub unsafe fn get_code(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_code)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


