//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSValue.idl
//


pub mod nsIDOMCSSValue_consts {
    pub const CSS_INHERIT: i64 = 0;
    pub const CSS_PRIMITIVE_VALUE: i64 = 1;
    pub const CSS_VALUE_LIST: i64 = 2;
    pub const CSS_CUSTOM: i64 = 3;
}


#[repr(C)]
pub struct nsIDOMCSSValue {
    vtable: *const nsIDOMCSSValueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSValue {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x009f7ea5, 0x9e80, 0x41be,
            [0xb0, 0x08, 0xdb, 0x62, 0xf1, 0x08, 0x23, 0xf2])
    }
}

unsafe impl RefCounted for nsIDOMCSSValue {
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
pub trait nsIDOMCSSValueCoerce {
    fn coerce_from(v: &nsIDOMCSSValue) -> &Self;
}

impl nsIDOMCSSValueCoerce for nsIDOMCSSValue {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSValue) -> &Self {
        v
    }
}

impl nsIDOMCSSValue {
    #[inline]
    pub fn coerce<T: nsIDOMCSSValueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSValue {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSValueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSValue) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSValueVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString cssText; */
    pub get_cssText: unsafe extern "C" fn (this: *const nsIDOMCSSValue, aCssText: *mut nsAString) -> nsresult,
    pub set_cssText: unsafe extern "C" fn (this: *const nsIDOMCSSValue, aCssText: *const nsAString) -> nsresult,

    /* readonly attribute unsigned short cssValueType; */
    pub get_cssValueType: unsafe extern "C" fn (this: *const nsIDOMCSSValue, aCssValueType: *mut libc::uint16_t) -> nsresult,

}


impl nsIDOMCSSValue {
    /* attribute DOMString cssText; */
    #[inline]
    pub unsafe fn get_cssText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cssText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cssText(&self, aCssText: &[u16]) -> Result<(), nsresult> {
        let aCssText = nsString::from(aCssText);
        match ((*self.vtable).set_cssText)(self as *const _, &*aCssText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned short cssValueType; */
    #[inline]
    pub unsafe fn get_cssValueType(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_cssValueType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


