//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginManagerCrypto.idl
//


pub mod nsILoginManagerCrypto_consts {
    pub const ENCTYPE_BASE64: i64 = 0;
    pub const ENCTYPE_SDR: i64 = 1;
}


#[repr(C)]
pub struct nsILoginManagerCrypto {
    vtable: *const nsILoginManagerCryptoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoginManagerCrypto {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2030770e, 0x542e, 0x40cd,
            [0x80, 0x61, 0xcd, 0x9d, 0x4a, 0xd4, 0x22, 0x7f])
    }
}

unsafe impl RefCounted for nsILoginManagerCrypto {
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
pub trait nsILoginManagerCryptoCoerce {
    fn coerce_from(v: &nsILoginManagerCrypto) -> &Self;
}

impl nsILoginManagerCryptoCoerce for nsILoginManagerCrypto {
    #[inline]
    fn coerce_from(v: &nsILoginManagerCrypto) -> &Self {
        v
    }
}

impl nsILoginManagerCrypto {
    #[inline]
    pub fn coerce<T: nsILoginManagerCryptoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoginManagerCrypto {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoginManagerCryptoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginManagerCrypto) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoginManagerCryptoVTable {
    pub __base: nsISupportsVTable,

    /* AString encrypt (in AString plainText); */
    pub encrypt: unsafe extern "C" fn (this: *const nsILoginManagerCrypto, plainText: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString decrypt (in AString cipherText); */
    pub decrypt: unsafe extern "C" fn (this: *const nsILoginManagerCrypto, cipherText: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute boolean uiBusy; */
    pub get_uiBusy: unsafe extern "C" fn (this: *const nsILoginManagerCrypto, aUiBusy: *mut bool) -> nsresult,

    /* readonly attribute boolean isLoggedIn; */
    pub get_isLoggedIn: unsafe extern "C" fn (this: *const nsILoginManagerCrypto, aIsLoggedIn: *mut bool) -> nsresult,

    /* readonly attribute unsigned long defaultEncType; */
    pub get_defaultEncType: unsafe extern "C" fn (this: *const nsILoginManagerCrypto, aDefaultEncType: *mut libc::uint32_t) -> nsresult,

}


impl nsILoginManagerCrypto {
    /* AString encrypt (in AString plainText); */
    #[inline]
    pub unsafe fn encrypt(&self, plainText: &[u16]) -> Result<nsString, nsresult> {
        let plainText = nsString::from(plainText);
        let mut _retval = nsString::new();
        match ((*self.vtable).encrypt)(self as *const _, &*plainText, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString decrypt (in AString cipherText); */
    #[inline]
    pub unsafe fn decrypt(&self, cipherText: &[u16]) -> Result<nsString, nsresult> {
        let cipherText = nsString::from(cipherText);
        let mut _retval = nsString::new();
        match ((*self.vtable).decrypt)(self as *const _, &*cipherText, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean uiBusy; */
    #[inline]
    pub unsafe fn get_uiBusy(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_uiBusy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isLoggedIn; */
    #[inline]
    pub unsafe fn get_isLoggedIn(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isLoggedIn)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long defaultEncType; */
    #[inline]
    pub unsafe fn get_defaultEncType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultEncType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


