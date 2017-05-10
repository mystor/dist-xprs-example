//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIByteRangeRequest.idl
//


#[repr(C)]
pub struct nsIByteRangeRequest {
    vtable: *const nsIByteRangeRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIByteRangeRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc1b1f426, 0x7e83, 0x4759,
            [0x9f, 0x88, 0x0e, 0x1b, 0x17, 0xf4, 0x93, 0x66])
    }
}

unsafe impl RefCounted for nsIByteRangeRequest {
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
pub trait nsIByteRangeRequestCoerce {
    fn coerce_from(v: &nsIByteRangeRequest) -> &Self;
}

impl nsIByteRangeRequestCoerce for nsIByteRangeRequest {
    #[inline]
    fn coerce_from(v: &nsIByteRangeRequest) -> &Self {
        v
    }
}

impl nsIByteRangeRequest {
    #[inline]
    pub fn coerce<T: nsIByteRangeRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIByteRangeRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIByteRangeRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIByteRangeRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIByteRangeRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isByteRangeRequest; */
    pub get_isByteRangeRequest: unsafe extern "C" fn (this: *const nsIByteRangeRequest, aIsByteRangeRequest: *mut bool) -> nsresult,

    /* readonly attribute long long startRange; */
    pub get_startRange: unsafe extern "C" fn (this: *const nsIByteRangeRequest, aStartRange: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long endRange; */
    pub get_endRange: unsafe extern "C" fn (this: *const nsIByteRangeRequest, aEndRange: *mut libc::int64_t) -> nsresult,

}


impl nsIByteRangeRequest {
    /* readonly attribute boolean isByteRangeRequest; */
    #[inline]
    pub unsafe fn get_isByteRangeRequest(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isByteRangeRequest)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long startRange; */
    #[inline]
    pub unsafe fn get_startRange(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_startRange)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long endRange; */
    #[inline]
    pub unsafe fn get_endRange(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_endRange)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


