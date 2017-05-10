//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUnicodeNormalizer.idl
//


#[repr(C)]
pub struct nsIUnicodeNormalizer {
    vtable: *const nsIUnicodeNormalizerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUnicodeNormalizer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb43a461f, 0x1bcf, 0x4329,
            [0x82, 0x0b, 0x66, 0xe4, 0x8c, 0x97, 0x9e, 0x14])
    }
}

unsafe impl RefCounted for nsIUnicodeNormalizer {
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
pub trait nsIUnicodeNormalizerCoerce {
    fn coerce_from(v: &nsIUnicodeNormalizer) -> &Self;
}

impl nsIUnicodeNormalizerCoerce for nsIUnicodeNormalizer {
    #[inline]
    fn coerce_from(v: &nsIUnicodeNormalizer) -> &Self {
        v
    }
}

impl nsIUnicodeNormalizer {
    #[inline]
    pub fn coerce<T: nsIUnicodeNormalizerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUnicodeNormalizer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUnicodeNormalizerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnicodeNormalizer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUnicodeNormalizerVTable {
    pub __base: nsISupportsVTable,

    /* void NormalizeUnicodeNFD (in AString aSrc, out AString aDest); */
    pub NormalizeUnicodeNFD: unsafe extern "C" fn (this: *const nsIUnicodeNormalizer, aSrc: *const nsAString, aDest: *mut nsAString) -> nsresult,

    /* void NormalizeUnicodeNFC (in AString aSrc, out AString aDest); */
    pub NormalizeUnicodeNFC: unsafe extern "C" fn (this: *const nsIUnicodeNormalizer, aSrc: *const nsAString, aDest: *mut nsAString) -> nsresult,

    /* void NormalizeUnicodeNFKD (in AString aSrc, out AString aDest); */
    pub NormalizeUnicodeNFKD: unsafe extern "C" fn (this: *const nsIUnicodeNormalizer, aSrc: *const nsAString, aDest: *mut nsAString) -> nsresult,

    /* void NormalizeUnicodeNFKC (in AString aSrc, out AString aDest); */
    pub NormalizeUnicodeNFKC: unsafe extern "C" fn (this: *const nsIUnicodeNormalizer, aSrc: *const nsAString, aDest: *mut nsAString) -> nsresult,

}


impl nsIUnicodeNormalizer {
    /* void NormalizeUnicodeNFD (in AString aSrc, out AString aDest); */
    #[inline]
    pub unsafe fn NormalizeUnicodeNFD(&self, aSrc: &[u16]) -> Result<nsString, nsresult> {
        let aSrc = nsString::from(aSrc);
        let mut aDest = nsString::new();
        match ((*self.vtable).NormalizeUnicodeNFD)(self as *const _, &*aSrc, &mut *aDest) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aDest)
    }

    /* void NormalizeUnicodeNFC (in AString aSrc, out AString aDest); */
    #[inline]
    pub unsafe fn NormalizeUnicodeNFC(&self, aSrc: &[u16]) -> Result<nsString, nsresult> {
        let aSrc = nsString::from(aSrc);
        let mut aDest = nsString::new();
        match ((*self.vtable).NormalizeUnicodeNFC)(self as *const _, &*aSrc, &mut *aDest) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aDest)
    }

    /* void NormalizeUnicodeNFKD (in AString aSrc, out AString aDest); */
    #[inline]
    pub unsafe fn NormalizeUnicodeNFKD(&self, aSrc: &[u16]) -> Result<nsString, nsresult> {
        let aSrc = nsString::from(aSrc);
        let mut aDest = nsString::new();
        match ((*self.vtable).NormalizeUnicodeNFKD)(self as *const _, &*aSrc, &mut *aDest) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aDest)
    }

    /* void NormalizeUnicodeNFKC (in AString aSrc, out AString aDest); */
    #[inline]
    pub unsafe fn NormalizeUnicodeNFKC(&self, aSrc: &[u16]) -> Result<nsString, nsresult> {
        let aSrc = nsString::from(aSrc);
        let mut aDest = nsString::new();
        match ((*self.vtable).NormalizeUnicodeNFKC)(self as *const _, &*aSrc, &mut *aDest) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aDest)
    }

}


