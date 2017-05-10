//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowCreator.idl
//


#[repr(C)]
pub struct nsIWindowCreator {
    vtable: *const nsIWindowCreatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowCreator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x30465632, 0xa777, 0x44cc,
            [0x90, 0xf9, 0x81, 0x45, 0x47, 0x5e, 0xf9, 0x99])
    }
}

unsafe impl RefCounted for nsIWindowCreator {
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
pub trait nsIWindowCreatorCoerce {
    fn coerce_from(v: &nsIWindowCreator) -> &Self;
}

impl nsIWindowCreatorCoerce for nsIWindowCreator {
    #[inline]
    fn coerce_from(v: &nsIWindowCreator) -> &Self {
        v
    }
}

impl nsIWindowCreator {
    #[inline]
    pub fn coerce<T: nsIWindowCreatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowCreator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWindowCreatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowCreator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowCreatorVTable {
    pub __base: nsISupportsVTable,

    /* nsIWebBrowserChrome createChromeWindow (in nsIWebBrowserChrome parent, in uint32_t chromeFlags); */
    pub createChromeWindow: unsafe extern "C" fn (this: *const nsIWindowCreator, parent: *const nsIWebBrowserChrome, chromeFlags: uint32_t, _retval: *mut *const nsIWebBrowserChrome) -> nsresult,

}


impl nsIWindowCreator {
    /* nsIWebBrowserChrome createChromeWindow (in nsIWebBrowserChrome parent, in uint32_t chromeFlags); */
    #[inline]
    pub unsafe fn createChromeWindow(&self, parent: Option<&nsIWebBrowserChrome>, chromeFlags: uint32_t) -> Result<Option<RefPtr<nsIWebBrowserChrome>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createChromeWindow)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), chromeFlags, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


