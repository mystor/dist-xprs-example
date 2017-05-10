//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRemoteWindowContext.idl
//


#[repr(C)]
pub struct nsIRemoteWindowContext {
    vtable: *const nsIRemoteWindowContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRemoteWindowContext {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x94f4a92b, 0x752e, 0x4fd9,
            [0x83, 0x45, 0x11, 0xb0, 0x69, 0xca, 0x19, 0xf3])
    }
}

unsafe impl RefCounted for nsIRemoteWindowContext {
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
pub trait nsIRemoteWindowContextCoerce {
    fn coerce_from(v: &nsIRemoteWindowContext) -> &Self;
}

impl nsIRemoteWindowContextCoerce for nsIRemoteWindowContext {
    #[inline]
    fn coerce_from(v: &nsIRemoteWindowContext) -> &Self {
        v
    }
}

impl nsIRemoteWindowContext {
    #[inline]
    pub fn coerce<T: nsIRemoteWindowContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRemoteWindowContext {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRemoteWindowContextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRemoteWindowContext) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRemoteWindowContextVTable {
    pub __base: nsISupportsVTable,

    /* void openURI (in nsIURI aURI); */
    pub openURI: unsafe extern "C" fn (this: *const nsIRemoteWindowContext, aURI: *const nsIURI) -> nsresult,

}


impl nsIRemoteWindowContext {
    /* void openURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn openURI(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).openURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


