//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAboutModule.idl
//


pub mod nsIAboutModule_consts {
    pub const URI_SAFE_FOR_UNTRUSTED_CONTENT: i64 = 1;
    pub const ALLOW_SCRIPT: i64 = 2;
    pub const HIDE_FROM_ABOUTABOUT: i64 = 4;
    pub const ENABLE_INDEXED_DB: i64 = 8;
    pub const URI_CAN_LOAD_IN_CHILD: i64 = 16;
    pub const URI_MUST_LOAD_IN_CHILD: i64 = 32;
    pub const MAKE_UNLINKABLE: i64 = 64;
    pub const MAKE_LINKABLE: i64 = 128;
}


#[repr(C)]
pub struct nsIAboutModule {
    vtable: *const nsIAboutModuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAboutModule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc0c19db9, 0x1b5a, 0x4ac5,
            [0xb6, 0x56, 0xed, 0x6f, 0x81, 0x49, 0xfa, 0x48])
    }
}

unsafe impl RefCounted for nsIAboutModule {
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
pub trait nsIAboutModuleCoerce {
    fn coerce_from(v: &nsIAboutModule) -> &Self;
}

impl nsIAboutModuleCoerce for nsIAboutModule {
    #[inline]
    fn coerce_from(v: &nsIAboutModule) -> &Self {
        v
    }
}

impl nsIAboutModule {
    #[inline]
    pub fn coerce<T: nsIAboutModuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAboutModule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAboutModuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAboutModule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAboutModuleVTable {
    pub __base: nsISupportsVTable,

    /* nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
    pub newChannel: unsafe extern "C" fn (this: *const nsIAboutModule, aURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut *const nsIChannel) -> nsresult,

    /* unsigned long getURIFlags (in nsIURI aURI); */
    pub getURIFlags: unsafe extern "C" fn (this: *const nsIAboutModule, aURI: *const nsIURI, _retval: *mut libc::uint32_t) -> nsresult,

}


impl nsIAboutModule {
    /* nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
    #[inline]
    pub unsafe fn newChannel(&self, aURI: Option<&nsIURI>, aLoadInfo: Option<&nsILoadInfo>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannel)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long getURIFlags (in nsIURI aURI); */
    #[inline]
    pub unsafe fn getURIFlags(&self, aURI: Option<&nsIURI>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getURIFlags)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


