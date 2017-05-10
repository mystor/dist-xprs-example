//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIU2FToken.idl
//


#[repr(C)]
pub struct nsIU2FToken {
    vtable: *const nsIU2FTokenVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIU2FToken {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5778242f, 0x1f42, 0x47a2,
            [0xb5, 0x14, 0xfa, 0x1a, 0xdd, 0xe2, 0xd9, 0x04])
    }
}

unsafe impl RefCounted for nsIU2FToken {
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
pub trait nsIU2FTokenCoerce {
    fn coerce_from(v: &nsIU2FToken) -> &Self;
}

impl nsIU2FTokenCoerce for nsIU2FToken {
    #[inline]
    fn coerce_from(v: &nsIU2FToken) -> &Self {
        v
    }
}

impl nsIU2FToken {
    #[inline]
    pub fn coerce<T: nsIU2FTokenCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIU2FToken {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIU2FTokenCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIU2FToken) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIU2FTokenVTable {
    pub __base: nsISupportsVTable,

    /* void isCompatibleVersion (in AString version, [retval] out boolean result); */
    pub isCompatibleVersion: unsafe extern "C" fn (this: *const nsIU2FToken, version: *const nsAString, result: *mut bool) -> nsresult,

    /* void isRegistered ([array, size_is (keyHandleLen)] in octet keyHandle, in uint32_t keyHandleLen, [array, size_is (applicationLen)] in octet application, in uint32_t applicationLen, [retval] out boolean result); */
    /// Unable to call function as its signature contains a non-rust type
    pub isRegistered: *const ::libc::c_void,

    /* void register ([array, size_is (applicationLen)] in octet application, in uint32_t applicationLen, [array, size_is (challengeLen)] in octet challenge, in uint32_t challengeLen, [array, size_is (registrationLen)] out octet registration, out uint32_t registrationLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub register: *const ::libc::c_void,

    /* void sign ([array, size_is (applicationLen)] in octet application, in uint32_t applicationLen, [array, size_is (challengeLen)] in octet challenge, in uint32_t challengeLen, [array, size_is (keyHandleLen)] in octet keyHandle, in uint32_t keyHandleLen, [array, size_is (signatureLen)] out octet signature, out uint32_t signatureLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub sign: *const ::libc::c_void,

}


impl nsIU2FToken {
    /* void isCompatibleVersion (in AString version, [retval] out boolean result); */
    #[inline]
    pub unsafe fn isCompatibleVersion(&self, version: &[u16]) -> Result<bool, nsresult> {
        let version = nsString::from(version);
        let mut result: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCompatibleVersion)(self as *const _, &*version, &mut result as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result)
    }

    /* void isRegistered ([array, size_is (keyHandleLen)] in octet keyHandle, in uint32_t keyHandleLen, [array, size_is (applicationLen)] in octet application, in uint32_t applicationLen, [retval] out boolean result); */


    /* void register ([array, size_is (applicationLen)] in octet application, in uint32_t applicationLen, [array, size_is (challengeLen)] in octet challenge, in uint32_t challengeLen, [array, size_is (registrationLen)] out octet registration, out uint32_t registrationLen); */


    /* void sign ([array, size_is (applicationLen)] in octet application, in uint32_t applicationLen, [array, size_is (challengeLen)] in octet challenge, in uint32_t challengeLen, [array, size_is (keyHandleLen)] in octet keyHandle, in uint32_t keyHandleLen, [array, size_is (signatureLen)] out octet signature, out uint32_t signatureLen); */


}


