//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserSetup.idl
//


pub mod nsIWebBrowserSetup_consts {
    pub const SETUP_ALLOW_PLUGINS: i64 = 1;
    pub const SETUP_ALLOW_JAVASCRIPT: i64 = 2;
    pub const SETUP_ALLOW_META_REDIRECTS: i64 = 3;
    pub const SETUP_ALLOW_SUBFRAMES: i64 = 4;
    pub const SETUP_ALLOW_IMAGES: i64 = 5;
    pub const SETUP_FOCUS_DOC_BEFORE_CONTENT: i64 = 6;
    pub const SETUP_USE_GLOBAL_HISTORY: i64 = 256;
    pub const SETUP_IS_CHROME_WRAPPER: i64 = 7;
    pub const SETUP_ALLOW_DNS_PREFETCH: i64 = 8;
}


#[repr(C)]
pub struct nsIWebBrowserSetup {
    vtable: *const nsIWebBrowserSetupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserSetup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf15398a0, 0x8018, 0x11d3,
            [0xaf, 0x70, 0x00, 0xa0, 0x24, 0xff, 0xc0, 0x8c])
    }
}

unsafe impl RefCounted for nsIWebBrowserSetup {
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
pub trait nsIWebBrowserSetupCoerce {
    fn coerce_from(v: &nsIWebBrowserSetup) -> &Self;
}

impl nsIWebBrowserSetupCoerce for nsIWebBrowserSetup {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserSetup) -> &Self {
        v
    }
}

impl nsIWebBrowserSetup {
    #[inline]
    pub fn coerce<T: nsIWebBrowserSetupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserSetup {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserSetupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserSetup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserSetupVTable {
    pub __base: nsISupportsVTable,

    /* void setProperty (in unsigned long aId, in unsigned long aValue); */
    pub setProperty: unsafe extern "C" fn (this: *const nsIWebBrowserSetup, aId: libc::uint32_t, aValue: libc::uint32_t) -> nsresult,

}


impl nsIWebBrowserSetup {
    /* void setProperty (in unsigned long aId, in unsigned long aValue); */
    #[inline]
    pub unsafe fn setProperty(&self, aId: libc::uint32_t, aValue: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setProperty)(self as *const _, aId, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


