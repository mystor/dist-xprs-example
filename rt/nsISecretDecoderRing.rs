//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecretDecoderRing.idl
//


#[repr(C)]
pub struct nsISecretDecoderRing {
    vtable: *const nsISecretDecoderRingVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISecretDecoderRing {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0ec80360, 0x075c, 0x11d4,
            [0x9f, 0xd4, 0x00, 0xc0, 0x4f, 0x1b, 0x83, 0xd8])
    }
}

unsafe impl RefCounted for nsISecretDecoderRing {
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
pub trait nsISecretDecoderRingCoerce {
    fn coerce_from(v: &nsISecretDecoderRing) -> &Self;
}

impl nsISecretDecoderRingCoerce for nsISecretDecoderRing {
    #[inline]
    fn coerce_from(v: &nsISecretDecoderRing) -> &Self {
        v
    }
}

impl nsISecretDecoderRing {
    #[inline]
    pub fn coerce<T: nsISecretDecoderRingCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISecretDecoderRing {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISecretDecoderRingCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecretDecoderRing) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISecretDecoderRingVTable {
    pub __base: nsISupportsVTable,

    /* ACString encryptString (in ACString text); */
    pub encryptString: unsafe extern "C" fn (this: *const nsISecretDecoderRing, text: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* ACString decryptString (in ACString encryptedBase64Text); */
    pub decryptString: unsafe extern "C" fn (this: *const nsISecretDecoderRing, encryptedBase64Text: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* void changePassword (); */
    pub changePassword: unsafe extern "C" fn (this: *const nsISecretDecoderRing) -> nsresult,

    /* void logout (); */
    pub logout: unsafe extern "C" fn (this: *const nsISecretDecoderRing) -> nsresult,

    /* void logoutAndTeardown (); */
    pub logoutAndTeardown: unsafe extern "C" fn (this: *const nsISecretDecoderRing) -> nsresult,

}


impl nsISecretDecoderRing {
    /* ACString encryptString (in ACString text); */
    #[inline]
    pub unsafe fn encryptString(&self, text: &[u8]) -> Result<nsCString, nsresult> {
        let text = nsCString::from(text);
        let mut _retval = nsCString::new();
        match ((*self.vtable).encryptString)(self as *const _, &*text, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString decryptString (in ACString encryptedBase64Text); */
    #[inline]
    pub unsafe fn decryptString(&self, encryptedBase64Text: &[u8]) -> Result<nsCString, nsresult> {
        let encryptedBase64Text = nsCString::from(encryptedBase64Text);
        let mut _retval = nsCString::new();
        match ((*self.vtable).decryptString)(self as *const _, &*encryptedBase64Text, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void changePassword (); */
    #[inline]
    pub unsafe fn changePassword(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).changePassword)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void logout (); */
    #[inline]
    pub unsafe fn logout(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).logout)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void logoutAndTeardown (); */
    #[inline]
    pub unsafe fn logoutAndTeardown(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).logoutAndTeardown)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


