//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIMozIntl.idl
//


#[repr(C)]
pub struct mozIMozIntl {
    vtable: *const mozIMozIntlVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIMozIntl {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7f63279a, 0x1a29, 0x4ae6,
            [0x9e, 0x7a, 0xdc, 0x96, 0x84, 0xa2, 0x35, 0x30])
    }
}

unsafe impl RefCounted for mozIMozIntl {
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
pub trait mozIMozIntlCoerce {
    fn coerce_from(v: &mozIMozIntl) -> &Self;
}

impl mozIMozIntlCoerce for mozIMozIntl {
    #[inline]
    fn coerce_from(v: &mozIMozIntl) -> &Self {
        v
    }
}

impl mozIMozIntl {
    #[inline]
    pub fn coerce<T: mozIMozIntlCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIMozIntl {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIMozIntlCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIMozIntl) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIMozIntlVTable {
    pub __base: nsISupportsVTable,

    /* jsval getCalendarInfo ([optional] in jsval locales); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCalendarInfo: *const ::libc::c_void,

    /* jsval getDisplayNames ([optional] in jsval locales, [optional] in jsval options); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDisplayNames: *const ::libc::c_void,

    /* jsval getLocaleInfo ([optional] in jsval locales); */
    /// Unable to call function as its signature contains a non-rust type
    pub getLocaleInfo: *const ::libc::c_void,

    /* jsval createPluralRules ([optional] in jsval locales, [optional] in jsval options); */
    /// Unable to call function as its signature contains a non-rust type
    pub createPluralRules: *const ::libc::c_void,

    /* jsval createDateTimeFormat ([optional] in jsval locales, [optional] in jsval options); */
    /// Unable to call function as its signature contains a non-rust type
    pub createDateTimeFormat: *const ::libc::c_void,

}


impl mozIMozIntl {
    /* jsval getCalendarInfo ([optional] in jsval locales); */


    /* jsval getDisplayNames ([optional] in jsval locales, [optional] in jsval options); */


    /* jsval getLocaleInfo ([optional] in jsval locales); */


    /* jsval createPluralRules ([optional] in jsval locales, [optional] in jsval options); */


    /* jsval createDateTimeFormat ([optional] in jsval locales, [optional] in jsval options); */


}


