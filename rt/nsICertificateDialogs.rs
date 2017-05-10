//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICertificateDialogs.idl
//


#[repr(C)]
pub struct nsICertificateDialogs {
    vtable: *const nsICertificateDialogsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICertificateDialogs {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xda871dab, 0xf69e, 0x4173,
            [0xab, 0x26, 0x99, 0xfc, 0xd4, 0x7b, 0x0e, 0x85])
    }
}

unsafe impl RefCounted for nsICertificateDialogs {
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
pub trait nsICertificateDialogsCoerce {
    fn coerce_from(v: &nsICertificateDialogs) -> &Self;
}

impl nsICertificateDialogsCoerce for nsICertificateDialogs {
    #[inline]
    fn coerce_from(v: &nsICertificateDialogs) -> &Self {
        v
    }
}

impl nsICertificateDialogs {
    #[inline]
    pub fn coerce<T: nsICertificateDialogsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICertificateDialogs {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICertificateDialogsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertificateDialogs) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICertificateDialogsVTable {
    pub __base: nsISupportsVTable,

    /* boolean confirmDownloadCACert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert, out unsigned long trust); */
    pub confirmDownloadCACert: unsafe extern "C" fn (this: *const nsICertificateDialogs, ctx: *const nsIInterfaceRequestor, cert: *const nsIX509Cert, trust: *mut libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* boolean setPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
    pub setPKCS12FilePassword: unsafe extern "C" fn (this: *const nsICertificateDialogs, ctx: *const nsIInterfaceRequestor, password: *mut nsAString, _retval: *mut bool) -> nsresult,

    /* boolean getPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
    pub getPKCS12FilePassword: unsafe extern "C" fn (this: *const nsICertificateDialogs, ctx: *const nsIInterfaceRequestor, password: *mut nsAString, _retval: *mut bool) -> nsresult,

    /* void viewCert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert); */
    pub viewCert: unsafe extern "C" fn (this: *const nsICertificateDialogs, ctx: *const nsIInterfaceRequestor, cert: *const nsIX509Cert) -> nsresult,

}


impl nsICertificateDialogs {
    /* boolean confirmDownloadCACert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert, out unsigned long trust); */
    #[inline]
    pub unsafe fn confirmDownloadCACert(&self, ctx: Option<&nsIInterfaceRequestor>, cert: Option<&nsIX509Cert>) -> Result<(libc::uint32_t, bool), nsresult> {
        let mut trust: libc::uint32_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).confirmDownloadCACert)(self as *const _, ctx.map_or(::std::ptr::null(), |x| x as *const _), cert.map_or(::std::ptr::null(), |x| x as *const _), &mut trust as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((trust, _retval))
    }

    /* boolean setPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
    #[inline]
    pub unsafe fn setPKCS12FilePassword(&self, ctx: Option<&nsIInterfaceRequestor>) -> Result<(nsString, bool), nsresult> {
        let mut password = nsString::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).setPKCS12FilePassword)(self as *const _, ctx.map_or(::std::ptr::null(), |x| x as *const _), &mut *password, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((password, _retval))
    }

    /* boolean getPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
    #[inline]
    pub unsafe fn getPKCS12FilePassword(&self, ctx: Option<&nsIInterfaceRequestor>) -> Result<(nsString, bool), nsresult> {
        let mut password = nsString::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getPKCS12FilePassword)(self as *const _, ctx.map_or(::std::ptr::null(), |x| x as *const _), &mut *password, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((password, _retval))
    }

    /* void viewCert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert); */
    #[inline]
    pub unsafe fn viewCert(&self, ctx: Option<&nsIInterfaceRequestor>, cert: Option<&nsIX509Cert>) -> Result<(), nsresult> {

        match ((*self.vtable).viewCert)(self as *const _, ctx.map_or(::std::ptr::null(), |x| x as *const _), cert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


