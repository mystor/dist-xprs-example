//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMTransitionEvent.idl
//


#[repr(C)]
pub struct nsIDOMTransitionEvent {
    vtable: *const nsIDOMTransitionEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMTransitionEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xee3499bf, 0x0f14, 0x4bb6,
            [0x82, 0x9c, 0x19, 0xad, 0x24, 0xfd, 0x4a, 0x85])
    }
}

unsafe impl RefCounted for nsIDOMTransitionEvent {
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
pub trait nsIDOMTransitionEventCoerce {
    fn coerce_from(v: &nsIDOMTransitionEvent) -> &Self;
}

impl nsIDOMTransitionEventCoerce for nsIDOMTransitionEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMTransitionEvent) -> &Self {
        v
    }
}

impl nsIDOMTransitionEvent {
    #[inline]
    pub fn coerce<T: nsIDOMTransitionEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMTransitionEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMTransitionEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMTransitionEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMTransitionEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString propertyName; */
    pub get_propertyName: unsafe extern "C" fn (this: *const nsIDOMTransitionEvent, aPropertyName: *mut nsAString) -> nsresult,

    /* readonly attribute float elapsedTime; */
    pub get_elapsedTime: unsafe extern "C" fn (this: *const nsIDOMTransitionEvent, aElapsedTime: *mut libc::c_float) -> nsresult,

    /* readonly attribute DOMString pseudoElement; */
    pub get_pseudoElement: unsafe extern "C" fn (this: *const nsIDOMTransitionEvent, aPseudoElement: *mut nsAString) -> nsresult,

}


impl nsIDOMTransitionEvent {
    /* readonly attribute DOMString propertyName; */
    #[inline]
    pub unsafe fn get_propertyName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_propertyName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float elapsedTime; */
    #[inline]
    pub unsafe fn get_elapsedTime(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_elapsedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString pseudoElement; */
    #[inline]
    pub unsafe fn get_pseudoElement(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_pseudoElement)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


