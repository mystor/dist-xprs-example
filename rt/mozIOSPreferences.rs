//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIOSPreferences.idl
//


pub mod mozIOSPreferences_consts {
    pub const dateTimeFormatStyleNone: i64 = 0;
    pub const dateTimeFormatStyleShort: i64 = 1;
    pub const dateTimeFormatStyleMedium: i64 = 2;
    pub const dateTimeFormatStyleLong: i64 = 3;
    pub const dateTimeFormatStyleFull: i64 = 4;
}


#[repr(C)]
pub struct mozIOSPreferences {
    vtable: *const mozIOSPreferencesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIOSPreferences {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x65944815, 0xe9ae, 0x48bd,
            [0xa2, 0xbf, 0xf1, 0x10, 0x87, 0x20, 0x95, 0x0c])
    }
}

unsafe impl RefCounted for mozIOSPreferences {
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
pub trait mozIOSPreferencesCoerce {
    fn coerce_from(v: &mozIOSPreferences) -> &Self;
}

impl mozIOSPreferencesCoerce for mozIOSPreferences {
    #[inline]
    fn coerce_from(v: &mozIOSPreferences) -> &Self {
        v
    }
}

impl mozIOSPreferences {
    #[inline]
    pub fn coerce<T: mozIOSPreferencesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIOSPreferences {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIOSPreferencesCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIOSPreferences) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIOSPreferencesVTable {
    pub __base: nsISupportsVTable,

    /* void getSystemLocales ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aOutArray); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSystemLocales: *const ::libc::c_void,

    /* readonly attribute ACString systemLocale; */
    pub get_systemLocale: unsafe extern "C" fn (this: *const mozIOSPreferences, aSystemLocale: *mut nsACString) -> nsresult,

    /* AString getDateTimePattern (in long timeFormatStyle, in long dateFormatStyle, [optional] in ACString locale); */
    pub getDateTimePattern: unsafe extern "C" fn (this: *const mozIOSPreferences, timeFormatStyle: libc::int32_t, dateFormatStyle: libc::int32_t, locale: *const nsACString, _retval: *mut nsAString) -> nsresult,

}


impl mozIOSPreferences {
    /* void getSystemLocales ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aOutArray); */


    /* readonly attribute ACString systemLocale; */
    #[inline]
    pub unsafe fn get_systemLocale(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_systemLocale)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getDateTimePattern (in long timeFormatStyle, in long dateFormatStyle, [optional] in ACString locale); */
    #[inline]
    pub unsafe fn getDateTimePattern(&self, timeFormatStyle: libc::int32_t, dateFormatStyle: libc::int32_t, locale: &[u8]) -> Result<nsString, nsresult> {
        let locale = nsCString::from(locale);
        let mut _retval = nsString::new();
        match ((*self.vtable).getDateTimePattern)(self as *const _, timeFormatStyle, dateFormatStyle, &*locale, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


