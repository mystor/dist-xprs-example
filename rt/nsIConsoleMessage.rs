//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConsoleMessage.idl
//


pub mod nsIConsoleMessage_consts {
    pub const debug: i64 = 0;
    pub const info: i64 = 1;
    pub const warn: i64 = 2;
    pub const error: i64 = 3;
}


#[repr(C)]
pub struct nsIConsoleMessage {
    vtable: *const nsIConsoleMessageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIConsoleMessage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3aba9617, 0x10e2, 0x4839,
            [0x83, 0xae, 0x2e, 0x6f, 0xc4, 0xdf, 0x42, 0x8b])
    }
}

unsafe impl RefCounted for nsIConsoleMessage {
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
pub trait nsIConsoleMessageCoerce {
    fn coerce_from(v: &nsIConsoleMessage) -> &Self;
}

impl nsIConsoleMessageCoerce for nsIConsoleMessage {
    #[inline]
    fn coerce_from(v: &nsIConsoleMessage) -> &Self {
        v
    }
}

impl nsIConsoleMessage {
    #[inline]
    pub fn coerce<T: nsIConsoleMessageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIConsoleMessage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIConsoleMessageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConsoleMessage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIConsoleMessageVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t logLevel; */
    pub get_logLevel: unsafe extern "C" fn (this: *const nsIConsoleMessage, aLogLevel: *mut uint32_t) -> nsresult,

    /* readonly attribute long long timeStamp; */
    pub get_timeStamp: unsafe extern "C" fn (this: *const nsIConsoleMessage, aTimeStamp: *mut libc::int64_t) -> nsresult,

    /* [binaryname(MessageMoz)] readonly attribute wstring message; */
    pub get_MessageMoz: unsafe extern "C" fn (this: *const nsIConsoleMessage, aMessage: *mut *const libc::int16_t) -> nsresult,

    /* AUTF8String toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIConsoleMessage, _retval: *mut nsACString) -> nsresult,

}


impl nsIConsoleMessage {
    /* readonly attribute uint32_t logLevel; */
    #[inline]
    pub unsafe fn get_logLevel(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_logLevel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long timeStamp; */
    #[inline]
    pub unsafe fn get_timeStamp(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_timeStamp)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(MessageMoz)] readonly attribute wstring message; */
    #[inline]
    pub unsafe fn get_MessageMoz(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_MessageMoz)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).toString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


