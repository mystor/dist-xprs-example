//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDNSListener.idl
//


#[repr(C)]
pub struct nsIDNSListener {
    vtable: *const nsIDNSListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x27d49bfe, 0x280c, 0x49e0,
            [0xbb, 0xaa, 0xf6, 0x20, 0x0c, 0x23, 0x2c, 0x3d])
    }
}

unsafe impl RefCounted for nsIDNSListener {
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
pub trait nsIDNSListenerCoerce {
    fn coerce_from(v: &nsIDNSListener) -> &Self;
}

impl nsIDNSListenerCoerce for nsIDNSListener {
    #[inline]
    fn coerce_from(v: &nsIDNSListener) -> &Self {
        v
    }
}

impl nsIDNSListener {
    #[inline]
    pub fn coerce<T: nsIDNSListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onLookupComplete (in nsICancelable aRequest, in nsIDNSRecord aRecord, in nsresult aStatus); */
    pub onLookupComplete: unsafe extern "C" fn (this: *const nsIDNSListener, aRequest: *const nsICancelable, aRecord: *const nsIDNSRecord, aStatus: nsresult) -> nsresult,

}


impl nsIDNSListener {
    /* void onLookupComplete (in nsICancelable aRequest, in nsIDNSRecord aRecord, in nsresult aStatus); */
    #[inline]
    pub unsafe fn onLookupComplete(&self, aRequest: Option<&nsICancelable>, aRecord: Option<&nsIDNSRecord>, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onLookupComplete)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aRecord.map_or(::std::ptr::null(), |x| x as *const _), aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDNSListenerProxy {
    vtable: *const nsIDNSListenerProxyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSListenerProxy {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x60eff0e4, 0x6f7c, 0x493c,
            [0xad, 0xd9, 0x1c, 0xbe, 0xa5, 0x90, 0x63, 0xad])
    }
}

unsafe impl RefCounted for nsIDNSListenerProxy {
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
pub trait nsIDNSListenerProxyCoerce {
    fn coerce_from(v: &nsIDNSListenerProxy) -> &Self;
}

impl nsIDNSListenerProxyCoerce for nsIDNSListenerProxy {
    #[inline]
    fn coerce_from(v: &nsIDNSListenerProxy) -> &Self {
        v
    }
}

impl nsIDNSListenerProxy {
    #[inline]
    pub fn coerce<T: nsIDNSListenerProxyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSListenerProxy {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSListenerProxyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSListenerProxy) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSListenerProxyVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDNSListener originalListener; */
    pub get_originalListener: unsafe extern "C" fn (this: *const nsIDNSListenerProxy, aOriginalListener: *mut *const nsIDNSListener) -> nsresult,

}


impl nsIDNSListenerProxy {
    /* readonly attribute nsIDNSListener originalListener; */
    #[inline]
    pub unsafe fn get_originalListener(&self, ) -> Result<Option<RefPtr<nsIDNSListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_originalListener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


