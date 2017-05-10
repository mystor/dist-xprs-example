//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFResource.idl
//


#[repr(C)]
pub struct nsIRDFResource {
    vtable: *const nsIRDFResourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFResource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfb9686a7, 0x719a, 0x49dc,
            [0x91, 0x07, 0x10, 0xde, 0xa5, 0x73, 0x93, 0x41])
    }
}

unsafe impl RefCounted for nsIRDFResource {
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
pub trait nsIRDFResourceCoerce {
    fn coerce_from(v: &nsIRDFResource) -> &Self;
}

impl nsIRDFResourceCoerce for nsIRDFResource {
    #[inline]
    fn coerce_from(v: &nsIRDFResource) -> &Self {
        v
    }
}

impl nsIRDFResource {
    #[inline]
    pub fn coerce<T: nsIRDFResourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFResource {
    type Target = nsIRDFNode;
    #[inline]
    fn deref(&self) -> &nsIRDFNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRDFNodeCoerce> nsIRDFResourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFResource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFResourceVTable {
    pub __base: nsIRDFNodeVTable,

    /* readonly attribute string Value; */
    pub get_Value: unsafe extern "C" fn (this: *const nsIRDFResource, aValue: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute AUTF8String ValueUTF8; */
    pub get_ValueUTF8: unsafe extern "C" fn (this: *const nsIRDFResource, aValueUTF8: *mut nsACString) -> nsresult,

    /* [noscript] void GetValueConst ([shared] out string aConstValue); */
    pub GetValueConst: unsafe extern "C" fn (this: *const nsIRDFResource, aConstValue: *mut *const libc::c_char) -> nsresult,

    /* void Init (in string uri); */
    pub Init: unsafe extern "C" fn (this: *const nsIRDFResource, uri: *const libc::c_char) -> nsresult,

    /* boolean EqualsString (in string aURI); */
    pub EqualsString: unsafe extern "C" fn (this: *const nsIRDFResource, aURI: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* void GetDelegate (in string aKey, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
    pub GetDelegate: unsafe extern "C" fn (this: *const nsIRDFResource, aKey: *const libc::c_char, aIID: *const nsIID, aResult: *mut *const libc::c_void) -> nsresult,

    /* void ReleaseDelegate (in string aKey); */
    pub ReleaseDelegate: unsafe extern "C" fn (this: *const nsIRDFResource, aKey: *const libc::c_char) -> nsresult,

}


impl nsIRDFResource {
    /* readonly attribute string Value; */
    #[inline]
    pub unsafe fn get_Value(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_Value)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String ValueUTF8; */
    #[inline]
    pub unsafe fn get_ValueUTF8(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_ValueUTF8)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void GetValueConst ([shared] out string aConstValue); */
    #[inline]
    pub unsafe fn GetValueConst(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut aConstValue: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).GetValueConst)(self as *const _, &mut aConstValue as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aConstValue)
    }

    /* void Init (in string uri); */
    #[inline]
    pub unsafe fn Init(&self, uri: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).Init)(self as *const _, uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean EqualsString (in string aURI); */
    #[inline]
    pub unsafe fn EqualsString(&self, aURI: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).EqualsString)(self as *const _, aURI, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void GetDelegate (in string aKey, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
    #[inline]
    pub unsafe fn GetDelegate<T: XpCom>(&self, aKey: *const libc::c_char) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut aResult : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).GetDelegate)(self as *const _, aKey, &T::iid(), aResult.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aResult.refptr())
    }

    /* void ReleaseDelegate (in string aKey); */
    #[inline]
    pub unsafe fn ReleaseDelegate(&self, aKey: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).ReleaseDelegate)(self as *const _, aKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


