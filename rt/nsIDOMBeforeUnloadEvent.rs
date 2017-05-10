//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMBeforeUnloadEvent.idl
//


#[repr(C)]
pub struct nsIDOMBeforeUnloadEvent {
    vtable: *const nsIDOMBeforeUnloadEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMBeforeUnloadEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x26c83933, 0xa5a4, 0x455e,
            [0x8c, 0x46, 0x69, 0xfa, 0x24, 0xdf, 0xa9, 0x91])
    }
}

unsafe impl RefCounted for nsIDOMBeforeUnloadEvent {
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
pub trait nsIDOMBeforeUnloadEventCoerce {
    fn coerce_from(v: &nsIDOMBeforeUnloadEvent) -> &Self;
}

impl nsIDOMBeforeUnloadEventCoerce for nsIDOMBeforeUnloadEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMBeforeUnloadEvent) -> &Self {
        v
    }
}

impl nsIDOMBeforeUnloadEvent {
    #[inline]
    pub fn coerce<T: nsIDOMBeforeUnloadEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMBeforeUnloadEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMBeforeUnloadEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMBeforeUnloadEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMBeforeUnloadEventVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString returnValue; */
    pub get_returnValue: unsafe extern "C" fn (this: *const nsIDOMBeforeUnloadEvent, aReturnValue: *mut nsAString) -> nsresult,
    pub set_returnValue: unsafe extern "C" fn (this: *const nsIDOMBeforeUnloadEvent, aReturnValue: *const nsAString) -> nsresult,

}


impl nsIDOMBeforeUnloadEvent {
    /* attribute DOMString returnValue; */
    #[inline]
    pub unsafe fn get_returnValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_returnValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_returnValue(&self, aReturnValue: &[u16]) -> Result<(), nsresult> {
        let aReturnValue = nsString::from(aReturnValue);
        match ((*self.vtable).set_returnValue)(self as *const _, &*aReturnValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


