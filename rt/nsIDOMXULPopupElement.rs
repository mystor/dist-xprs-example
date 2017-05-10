//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULPopupElement.idl
//


pub mod nsIDOMXULPopupElement_consts {
    pub const BEFORE_START: i64 = 1;
    pub const BEFORE_END: i64 = 2;
    pub const AFTER_START: i64 = 3;
    pub const AFTER_END: i64 = 4;
    pub const START_BEFORE: i64 = 5;
    pub const START_AFTER: i64 = 6;
    pub const END_BEFORE: i64 = 7;
    pub const END_AFTER: i64 = 8;
    pub const OVERLAP: i64 = 9;
    pub const AT_POINTER: i64 = 10;
    pub const AFTER_POINTER: i64 = 11;
}


#[repr(C)]
pub struct nsIDOMXULPopupElement {
    vtable: *const nsIDOMXULPopupElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULPopupElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcb7eaa79, 0x45d5, 0x4ea3,
            [0xae, 0x17, 0xb6, 0x5f, 0xdc, 0xfe, 0x5e, 0x30])
    }
}

unsafe impl RefCounted for nsIDOMXULPopupElement {
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
pub trait nsIDOMXULPopupElementCoerce {
    fn coerce_from(v: &nsIDOMXULPopupElement) -> &Self;
}

impl nsIDOMXULPopupElementCoerce for nsIDOMXULPopupElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULPopupElement) -> &Self {
        v
    }
}

impl nsIDOMXULPopupElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULPopupElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULPopupElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULPopupElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULPopupElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULPopupElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString position; */
    pub get_position: unsafe extern "C" fn (this: *const nsIDOMXULPopupElement, aPosition: *mut nsAString) -> nsresult,
    pub set_position: unsafe extern "C" fn (this: *const nsIDOMXULPopupElement, aPosition: *const nsAString) -> nsresult,

    /* void showPopup (in unsigned short alignment, in nsIDOMElement target, in nsIDOMElement anchor); */
    pub showPopup: unsafe extern "C" fn (this: *const nsIDOMXULPopupElement, alignment: libc::uint16_t, target: *const nsIDOMElement, anchor: *const nsIDOMElement) -> nsresult,

    /* void hidePopup (); */
    pub hidePopup: unsafe extern "C" fn (this: *const nsIDOMXULPopupElement) -> nsresult,

}


impl nsIDOMXULPopupElement {
    /* attribute DOMString position; */
    #[inline]
    pub unsafe fn get_position(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_position)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_position(&self, aPosition: &[u16]) -> Result<(), nsresult> {
        let aPosition = nsString::from(aPosition);
        match ((*self.vtable).set_position)(self as *const _, &*aPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showPopup (in unsigned short alignment, in nsIDOMElement target, in nsIDOMElement anchor); */
    #[inline]
    pub unsafe fn showPopup(&self, alignment: libc::uint16_t, target: Option<&nsIDOMElement>, anchor: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).showPopup)(self as *const _, alignment, target.map_or(::std::ptr::null(), |x| x as *const _), anchor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void hidePopup (); */
    #[inline]
    pub unsafe fn hidePopup(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).hidePopup)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


