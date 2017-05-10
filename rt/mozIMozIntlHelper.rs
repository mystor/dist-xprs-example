//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIMozIntlHelper.idl
//


#[repr(C)]
pub struct mozIMozIntlHelper {
    vtable: *const mozIMozIntlHelperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIMozIntlHelper {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x189eaa7d, 0xb29a, 0x43a9,
            [0xb1, 0xfb, 0x76, 0x58, 0x99, 0x0d, 0xf9, 0x40])
    }
}

unsafe impl RefCounted for mozIMozIntlHelper {
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
pub trait mozIMozIntlHelperCoerce {
    fn coerce_from(v: &mozIMozIntlHelper) -> &Self;
}

impl mozIMozIntlHelperCoerce for mozIMozIntlHelper {
    #[inline]
    fn coerce_from(v: &mozIMozIntlHelper) -> &Self {
        v
    }
}

impl mozIMozIntlHelper {
    #[inline]
    pub fn coerce<T: mozIMozIntlHelperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIMozIntlHelper {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIMozIntlHelperCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIMozIntlHelper) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIMozIntlHelperVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void addGetCalendarInfo (in jsval intlObject); */
    /// Unable to call function as its signature contains a non-rust type
    pub addGetCalendarInfo: *const ::libc::c_void,

    /* [implicit_jscontext] void addGetDisplayNames (in jsval intlObject); */
    /// Unable to call function as its signature contains a non-rust type
    pub addGetDisplayNames: *const ::libc::c_void,

    /* [implicit_jscontext] void addGetLocaleInfo (in jsval intlObject); */
    /// Unable to call function as its signature contains a non-rust type
    pub addGetLocaleInfo: *const ::libc::c_void,

    /* [implicit_jscontext] void addPluralRulesConstructor (in jsval intlObject); */
    /// Unable to call function as its signature contains a non-rust type
    pub addPluralRulesConstructor: *const ::libc::c_void,

    /* [implicit_jscontext] void addDateTimeFormatConstructor (in jsval intlObject); */
    /// Unable to call function as its signature contains a non-rust type
    pub addDateTimeFormatConstructor: *const ::libc::c_void,

}


impl mozIMozIntlHelper {
    /* [implicit_jscontext] void addGetCalendarInfo (in jsval intlObject); */


    /* [implicit_jscontext] void addGetDisplayNames (in jsval intlObject); */


    /* [implicit_jscontext] void addGetLocaleInfo (in jsval intlObject); */


    /* [implicit_jscontext] void addPluralRulesConstructor (in jsval intlObject); */


    /* [implicit_jscontext] void addDateTimeFormatConstructor (in jsval intlObject); */


}


