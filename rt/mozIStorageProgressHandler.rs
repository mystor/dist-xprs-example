//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageProgressHandler.idl
//


#[repr(C)]
pub struct mozIStorageProgressHandler {
    vtable: *const mozIStorageProgressHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageProgressHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa3a6fcd4, 0xbf89, 0x4208,
            [0xa8, 0x37, 0xbf, 0x2a, 0x73, 0xaf, 0xd3, 0x0c])
    }
}

unsafe impl RefCounted for mozIStorageProgressHandler {
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
pub trait mozIStorageProgressHandlerCoerce {
    fn coerce_from(v: &mozIStorageProgressHandler) -> &Self;
}

impl mozIStorageProgressHandlerCoerce for mozIStorageProgressHandler {
    #[inline]
    fn coerce_from(v: &mozIStorageProgressHandler) -> &Self {
        v
    }
}

impl mozIStorageProgressHandler {
    #[inline]
    pub fn coerce<T: mozIStorageProgressHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageProgressHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageProgressHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageProgressHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageProgressHandlerVTable {
    pub __base: nsISupportsVTable,

    /* boolean onProgress (in mozIStorageConnection aConnection); */
    pub onProgress: unsafe extern "C" fn (this: *const mozIStorageProgressHandler, aConnection: *const mozIStorageConnection, _retval: *mut bool) -> nsresult,

}


impl mozIStorageProgressHandler {
    /* boolean onProgress (in mozIStorageConnection aConnection); */
    #[inline]
    pub unsafe fn onProgress(&self, aConnection: Option<&mozIStorageConnection>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onProgress)(self as *const _, aConnection.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


