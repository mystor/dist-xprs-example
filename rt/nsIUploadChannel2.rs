//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUploadChannel2.idl
//


#[repr(C)]
pub struct nsIUploadChannel2 {
    vtable: *const nsIUploadChannel2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUploadChannel2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2f712b52, 0x19c5, 0x4e0c,
            [0x9e, 0x8f, 0xb5, 0xc7, 0xc3, 0xb6, 0x70, 0x49])
    }
}

unsafe impl RefCounted for nsIUploadChannel2 {
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
pub trait nsIUploadChannel2Coerce {
    fn coerce_from(v: &nsIUploadChannel2) -> &Self;
}

impl nsIUploadChannel2Coerce for nsIUploadChannel2 {
    #[inline]
    fn coerce_from(v: &nsIUploadChannel2) -> &Self {
        v
    }
}

impl nsIUploadChannel2 {
    #[inline]
    pub fn coerce<T: nsIUploadChannel2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUploadChannel2 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUploadChannel2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIUploadChannel2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUploadChannel2VTable {
    pub __base: nsISupportsVTable,

    /* void explicitSetUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength, in ACString aMethod, in boolean aStreamHasHeaders); */
    pub explicitSetUploadStream: unsafe extern "C" fn (this: *const nsIUploadChannel2, aStream: *const nsIInputStream, aContentType: *const nsACString, aContentLength: libc::int64_t, aMethod: *const nsACString, aStreamHasHeaders: bool) -> nsresult,

    /* readonly attribute boolean uploadStreamHasHeaders; */
    pub get_uploadStreamHasHeaders: unsafe extern "C" fn (this: *const nsIUploadChannel2, aUploadStreamHasHeaders: *mut bool) -> nsresult,

    /* [noscript] void ensureUploadStreamIsCloneable (in nsIRunnable aCallback); */
    pub ensureUploadStreamIsCloneable: unsafe extern "C" fn (this: *const nsIUploadChannel2, aCallback: *const nsIRunnable) -> nsresult,

    /* [noscript] nsIInputStream cloneUploadStream (); */
    pub cloneUploadStream: unsafe extern "C" fn (this: *const nsIUploadChannel2, _retval: *mut *const nsIInputStream) -> nsresult,

}


impl nsIUploadChannel2 {
    /* void explicitSetUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength, in ACString aMethod, in boolean aStreamHasHeaders); */
    #[inline]
    pub unsafe fn explicitSetUploadStream(&self, aStream: Option<&nsIInputStream>, aContentType: &[u8], aContentLength: libc::int64_t, aMethod: &[u8], aStreamHasHeaders: bool) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        let aMethod = nsCString::from(aMethod);
        match ((*self.vtable).explicitSetUploadStream)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), &*aContentType, aContentLength, &*aMethod, aStreamHasHeaders) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean uploadStreamHasHeaders; */
    #[inline]
    pub unsafe fn get_uploadStreamHasHeaders(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_uploadStreamHasHeaders)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void ensureUploadStreamIsCloneable (in nsIRunnable aCallback); */
    #[inline]
    pub unsafe fn ensureUploadStreamIsCloneable(&self, aCallback: Option<&nsIRunnable>) -> Result<(), nsresult> {

        match ((*self.vtable).ensureUploadStreamIsCloneable)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] nsIInputStream cloneUploadStream (); */
    #[inline]
    pub unsafe fn cloneUploadStream(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).cloneUploadStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


