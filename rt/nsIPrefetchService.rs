//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefetchService.idl
//


#[repr(C)]
pub struct nsIPrefetchService {
    vtable: *const nsIPrefetchServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrefetchService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x422a1807, 0x4e7f, 0x463d,
            [0xb8, 0xd7, 0xca, 0x2c, 0xeb, 0x9b, 0x5d, 0x53])
    }
}

unsafe impl RefCounted for nsIPrefetchService {
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
pub trait nsIPrefetchServiceCoerce {
    fn coerce_from(v: &nsIPrefetchService) -> &Self;
}

impl nsIPrefetchServiceCoerce for nsIPrefetchService {
    #[inline]
    fn coerce_from(v: &nsIPrefetchService) -> &Self {
        v
    }
}

impl nsIPrefetchService {
    #[inline]
    pub fn coerce<T: nsIPrefetchServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrefetchService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrefetchServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefetchService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrefetchServiceVTable {
    pub __base: nsISupportsVTable,

    /* void prefetchURI (in nsIURI aURI, in nsIURI aReferrerURI, in nsIDOMNode aSource, in boolean aExplicit); */
    pub prefetchURI: unsafe extern "C" fn (this: *const nsIPrefetchService, aURI: *const nsIURI, aReferrerURI: *const nsIURI, aSource: *const nsIDOMNode, aExplicit: bool) -> nsresult,

    /* boolean hasMoreElements (); */
    pub hasMoreElements: unsafe extern "C" fn (this: *const nsIPrefetchService, _retval: *mut bool) -> nsresult,

    /* void cancelPrefetchURI (in nsIURI aURI, in nsIDOMNode aSource); */
    pub cancelPrefetchURI: unsafe extern "C" fn (this: *const nsIPrefetchService, aURI: *const nsIURI, aSource: *const nsIDOMNode) -> nsresult,

}


impl nsIPrefetchService {
    /* void prefetchURI (in nsIURI aURI, in nsIURI aReferrerURI, in nsIDOMNode aSource, in boolean aExplicit); */
    #[inline]
    pub unsafe fn prefetchURI(&self, aURI: Option<&nsIURI>, aReferrerURI: Option<&nsIURI>, aSource: Option<&nsIDOMNode>, aExplicit: bool) -> Result<(), nsresult> {

        match ((*self.vtable).prefetchURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aReferrerURI.map_or(::std::ptr::null(), |x| x as *const _), aSource.map_or(::std::ptr::null(), |x| x as *const _), aExplicit) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasMoreElements (); */
    #[inline]
    pub unsafe fn hasMoreElements(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasMoreElements)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void cancelPrefetchURI (in nsIURI aURI, in nsIDOMNode aSource); */
    #[inline]
    pub unsafe fn cancelPrefetchURI(&self, aURI: Option<&nsIURI>, aSource: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).cancelPrefetchURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


