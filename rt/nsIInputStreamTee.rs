//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputStreamTee.idl
//


#[repr(C)]
pub struct nsIInputStreamTee {
    vtable: *const nsIInputStreamTeeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInputStreamTee {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x90a9d790, 0x3bca, 0x421e,
            [0xa7, 0x3b, 0x98, 0xf6, 0x8e, 0x13, 0xc9, 0x17])
    }
}

unsafe impl RefCounted for nsIInputStreamTee {
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
pub trait nsIInputStreamTeeCoerce {
    fn coerce_from(v: &nsIInputStreamTee) -> &Self;
}

impl nsIInputStreamTeeCoerce for nsIInputStreamTee {
    #[inline]
    fn coerce_from(v: &nsIInputStreamTee) -> &Self {
        v
    }
}

impl nsIInputStreamTee {
    #[inline]
    pub fn coerce<T: nsIInputStreamTeeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInputStreamTee {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIInputStreamTeeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamTee) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInputStreamTeeVTable {
    pub __base: nsIInputStreamVTable,

    /* attribute nsIInputStream source; */
    pub get_source: unsafe extern "C" fn (this: *const nsIInputStreamTee, aSource: *mut *const nsIInputStream) -> nsresult,
    pub set_source: unsafe extern "C" fn (this: *const nsIInputStreamTee, aSource: *const nsIInputStream) -> nsresult,

    /* attribute nsIOutputStream sink; */
    pub get_sink: unsafe extern "C" fn (this: *const nsIInputStreamTee, aSink: *mut *const nsIOutputStream) -> nsresult,
    pub set_sink: unsafe extern "C" fn (this: *const nsIInputStreamTee, aSink: *const nsIOutputStream) -> nsresult,

    /* attribute nsIEventTarget eventTarget; */
    pub get_eventTarget: unsafe extern "C" fn (this: *const nsIInputStreamTee, aEventTarget: *mut *const nsIEventTarget) -> nsresult,
    pub set_eventTarget: unsafe extern "C" fn (this: *const nsIInputStreamTee, aEventTarget: *const nsIEventTarget) -> nsresult,

}


impl nsIInputStreamTee {
    /* attribute nsIInputStream source; */
    #[inline]
    pub unsafe fn get_source(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_source)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_source(&self, aSource: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).set_source)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIOutputStream sink; */
    #[inline]
    pub unsafe fn get_sink(&self, ) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sink)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_sink(&self, aSink: Option<&nsIOutputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).set_sink)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIEventTarget eventTarget; */
    #[inline]
    pub unsafe fn get_eventTarget(&self, ) -> Result<Option<RefPtr<nsIEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_eventTarget)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_eventTarget(&self, aEventTarget: Option<&nsIEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).set_eventTarget)(self as *const _, aEventTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


