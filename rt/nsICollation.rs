//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICollation.idl
//


#[repr(C)]
pub struct nsICollationFactory {
    vtable: *const nsICollationFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICollationFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x04971e14, 0xd6b3, 0x4ada,
            [0x8c, 0xbb, 0xc3, 0xa1, 0x38, 0x42, 0xb3, 0x49])
    }
}

unsafe impl RefCounted for nsICollationFactory {
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
pub trait nsICollationFactoryCoerce {
    fn coerce_from(v: &nsICollationFactory) -> &Self;
}

impl nsICollationFactoryCoerce for nsICollationFactory {
    #[inline]
    fn coerce_from(v: &nsICollationFactory) -> &Self {
        v
    }
}

impl nsICollationFactory {
    #[inline]
    pub fn coerce<T: nsICollationFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICollationFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICollationFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICollationFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICollationFactoryVTable {
    pub __base: nsISupportsVTable,

    /* nsICollation CreateCollation (); */
    pub CreateCollation: unsafe extern "C" fn (this: *const nsICollationFactory, _retval: *mut *const nsICollation) -> nsresult,

    /* nsICollation CreateCollationForLocale (in ACString locale); */
    pub CreateCollationForLocale: unsafe extern "C" fn (this: *const nsICollationFactory, locale: *const nsACString, _retval: *mut *const nsICollation) -> nsresult,

}


impl nsICollationFactory {
    /* nsICollation CreateCollation (); */
    #[inline]
    pub unsafe fn CreateCollation(&self, ) -> Result<Option<RefPtr<nsICollation>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).CreateCollation)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsICollation CreateCollationForLocale (in ACString locale); */
    #[inline]
    pub unsafe fn CreateCollationForLocale(&self, locale: &[u8]) -> Result<Option<RefPtr<nsICollation>>, nsresult> {
        let locale = nsCString::from(locale);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).CreateCollationForLocale)(self as *const _, &*locale, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod nsICollation_consts {
    pub const kCollationStrengthDefault: i64 = 0;
    pub const kCollationCaseInsensitiveAscii: i64 = 1;
    pub const kCollationAccentInsenstive: i64 = 2;
    pub const kCollationCaseSensitive: i64 = 0;
    pub const kCollationCaseInSensitive: i64 = 3;
}


#[repr(C)]
pub struct nsICollation {
    vtable: *const nsICollationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICollation {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb0132cc0, 0x3786, 0x4557,
            [0x98, 0x74, 0x91, 0x0d, 0x7d, 0xef, 0x5f, 0x93])
    }
}

unsafe impl RefCounted for nsICollation {
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
pub trait nsICollationCoerce {
    fn coerce_from(v: &nsICollation) -> &Self;
}

impl nsICollationCoerce for nsICollation {
    #[inline]
    fn coerce_from(v: &nsICollation) -> &Self {
        v
    }
}

impl nsICollation {
    #[inline]
    pub fn coerce<T: nsICollationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICollation {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICollationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICollation) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICollationVTable {
    pub __base: nsISupportsVTable,

    /* void initialize (in ACString locale); */
    pub initialize: unsafe extern "C" fn (this: *const nsICollation, locale: *const nsACString) -> nsresult,

    /* long compareString (in long strength, in AString string1, in AString string2); */
    pub compareString: unsafe extern "C" fn (this: *const nsICollation, strength: libc::int32_t, string1: *const nsAString, string2: *const nsAString, _retval: *mut libc::int32_t) -> nsresult,

    /* [noscript] void allocateRawSortKey (in long strength, in AString stringIn, [array, size_is (outLen)] out octet key, out unsigned long outLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub allocateRawSortKey: *const ::libc::c_void,

    /* [noscript] long compareRawSortKey ([array, size_is (len1), const] in octet key1, in unsigned long len1, [array, size_is (len2), const] in octet key2, in unsigned long len2); */
    /// Unable to call function as its signature contains a non-rust type
    pub compareRawSortKey: *const ::libc::c_void,

}


impl nsICollation {
    /* void initialize (in ACString locale); */
    #[inline]
    pub unsafe fn initialize(&self, locale: &[u8]) -> Result<(), nsresult> {
        let locale = nsCString::from(locale);
        match ((*self.vtable).initialize)(self as *const _, &*locale) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long compareString (in long strength, in AString string1, in AString string2); */
    #[inline]
    pub unsafe fn compareString(&self, strength: libc::int32_t, string1: &[u16], string2: &[u16]) -> Result<libc::int32_t, nsresult> {
        let string1 = nsString::from(string1);
        let string2 = nsString::from(string2);
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).compareString)(self as *const _, strength, &*string1, &*string2, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void allocateRawSortKey (in long strength, in AString stringIn, [array, size_is (outLen)] out octet key, out unsigned long outLen); */


    /* [noscript] long compareRawSortKey ([array, size_is (len1), const] in octet key1, in unsigned long len1, [array, size_is (len2), const] in octet key2, in unsigned long len2); */


}


