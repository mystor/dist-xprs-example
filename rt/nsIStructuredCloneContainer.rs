//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStructuredCloneContainer.idl
//


#[repr(C)]
pub struct nsIStructuredCloneContainer {
    vtable: *const nsIStructuredCloneContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStructuredCloneContainer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc664aae7, 0x0d67, 0x4155,
            [0xa2, 0xdd, 0xa3, 0x86, 0x17, 0x78, 0x62, 0x6f])
    }
}

unsafe impl RefCounted for nsIStructuredCloneContainer {
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
pub trait nsIStructuredCloneContainerCoerce {
    fn coerce_from(v: &nsIStructuredCloneContainer) -> &Self;
}

impl nsIStructuredCloneContainerCoerce for nsIStructuredCloneContainer {
    #[inline]
    fn coerce_from(v: &nsIStructuredCloneContainer) -> &Self {
        v
    }
}

impl nsIStructuredCloneContainer {
    #[inline]
    pub fn coerce<T: nsIStructuredCloneContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStructuredCloneContainer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStructuredCloneContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStructuredCloneContainer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStructuredCloneContainerVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext,noscript] void initFromJSVal (in jsval aData); */
    /// Unable to call function as its signature contains a non-rust type
    pub initFromJSVal: *const ::libc::c_void,

    /* void initFromBase64 (in AString aData, in unsigned long aFormatVersion); */
    pub initFromBase64: unsafe extern "C" fn (this: *const nsIStructuredCloneContainer, aData: *const nsAString, aFormatVersion: libc::uint32_t) -> nsresult,

    /* [implicit_jscontext] jsval deserializeToJsval (); */
    /// Unable to call function as its signature contains a non-rust type
    pub deserializeToJsval: *const ::libc::c_void,

    /* [implicit_jscontext] nsIVariant deserializeToVariant (); */
    /// Unable to call function as its signature contains a non-rust type
    pub deserializeToVariant: *const ::libc::c_void,

    /* AString getDataAsBase64 (); */
    pub getDataAsBase64: unsafe extern "C" fn (this: *const nsIStructuredCloneContainer, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long long serializedNBytes; */
    pub get_serializedNBytes: unsafe extern "C" fn (this: *const nsIStructuredCloneContainer, aSerializedNBytes: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long formatVersion; */
    pub get_formatVersion: unsafe extern "C" fn (this: *const nsIStructuredCloneContainer, aFormatVersion: *mut libc::uint32_t) -> nsresult,

}


impl nsIStructuredCloneContainer {
    /* [implicit_jscontext,noscript] void initFromJSVal (in jsval aData); */


    /* void initFromBase64 (in AString aData, in unsigned long aFormatVersion); */
    #[inline]
    pub unsafe fn initFromBase64(&self, aData: &[u16], aFormatVersion: libc::uint32_t) -> Result<(), nsresult> {
        let aData = nsString::from(aData);
        match ((*self.vtable).initFromBase64)(self as *const _, &*aData, aFormatVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval deserializeToJsval (); */


    /* [implicit_jscontext] nsIVariant deserializeToVariant (); */


    /* AString getDataAsBase64 (); */
    #[inline]
    pub unsafe fn getDataAsBase64(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getDataAsBase64)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long serializedNBytes; */
    #[inline]
    pub unsafe fn get_serializedNBytes(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_serializedNBytes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long formatVersion; */
    #[inline]
    pub unsafe fn get_formatVersion(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_formatVersion)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


