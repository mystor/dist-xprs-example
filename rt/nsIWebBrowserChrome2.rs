//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserChrome2.idl
//


#[repr(C)]
pub struct nsIWebBrowserChrome2 {
    vtable: *const nsIWebBrowserChrome2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserChrome2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2585a7b1, 0x7b47, 0x43c4,
            [0xbf, 0x17, 0xc6, 0xbf, 0x84, 0xe0, 0x9b, 0x7b])
    }
}

unsafe impl RefCounted for nsIWebBrowserChrome2 {
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
pub trait nsIWebBrowserChrome2Coerce {
    fn coerce_from(v: &nsIWebBrowserChrome2) -> &Self;
}

impl nsIWebBrowserChrome2Coerce for nsIWebBrowserChrome2 {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome2) -> &Self {
        v
    }
}

impl nsIWebBrowserChrome2 {
    #[inline]
    pub fn coerce<T: nsIWebBrowserChrome2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserChrome2 {
    type Target = nsIWebBrowserChrome;
    #[inline]
    fn deref(&self) -> &nsIWebBrowserChrome {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWebBrowserChromeCoerce> nsIWebBrowserChrome2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserChrome2VTable {
    pub __base: nsIWebBrowserChromeVTable,

    /* void setStatusWithContext (in unsigned long statusType, in AString statusText, in nsISupports statusContext); */
    pub setStatusWithContext: unsafe extern "C" fn (this: *const nsIWebBrowserChrome2, statusType: libc::uint32_t, statusText: *const nsAString, statusContext: *const nsISupports) -> nsresult,

}


impl nsIWebBrowserChrome2 {
    /* void setStatusWithContext (in unsigned long statusType, in AString statusText, in nsISupports statusContext); */
    #[inline]
    pub unsafe fn setStatusWithContext(&self, statusType: libc::uint32_t, statusText: &[u16], statusContext: Option<&nsISupports>) -> Result<(), nsresult> {
        let statusText = nsString::from(statusText);
        match ((*self.vtable).setStatusWithContext)(self as *const _, statusType, &*statusText, statusContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


