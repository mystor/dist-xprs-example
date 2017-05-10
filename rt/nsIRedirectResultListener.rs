//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRedirectResultListener.idl
//


#[repr(C)]
pub struct nsIRedirectResultListener {
    vtable: *const nsIRedirectResultListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRedirectResultListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x85cd2640, 0xe91e, 0x41ac,
            [0xbd, 0xca, 0x1d, 0xbf, 0x10, 0xdc, 0x13, 0x1e])
    }
}

unsafe impl RefCounted for nsIRedirectResultListener {
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
pub trait nsIRedirectResultListenerCoerce {
    fn coerce_from(v: &nsIRedirectResultListener) -> &Self;
}

impl nsIRedirectResultListenerCoerce for nsIRedirectResultListener {
    #[inline]
    fn coerce_from(v: &nsIRedirectResultListener) -> &Self {
        v
    }
}

impl nsIRedirectResultListener {
    #[inline]
    pub fn coerce<T: nsIRedirectResultListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRedirectResultListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRedirectResultListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRedirectResultListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRedirectResultListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onRedirectResult (in boolean proceeding); */
    pub onRedirectResult: unsafe extern "C" fn (this: *const nsIRedirectResultListener, proceeding: bool) -> nsresult,

}


impl nsIRedirectResultListener {
    /* void onRedirectResult (in boolean proceeding); */
    #[inline]
    pub unsafe fn onRedirectResult(&self, proceeding: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onRedirectResult)(self as *const _, proceeding) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


