//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCommandEvent.idl
//


#[repr(C)]
pub struct nsIDOMCommandEvent {
    vtable: *const nsIDOMCommandEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCommandEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x73a50e55, 0x3eaa, 0x4a38,
            [0xa5, 0x88, 0x9b, 0x68, 0xa6, 0xd6, 0x50, 0x32])
    }
}

unsafe impl RefCounted for nsIDOMCommandEvent {
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
pub trait nsIDOMCommandEventCoerce {
    fn coerce_from(v: &nsIDOMCommandEvent) -> &Self;
}

impl nsIDOMCommandEventCoerce for nsIDOMCommandEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMCommandEvent) -> &Self {
        v
    }
}

impl nsIDOMCommandEvent {
    #[inline]
    pub fn coerce<T: nsIDOMCommandEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCommandEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCommandEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCommandEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCommandEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString command; */
    pub get_command: unsafe extern "C" fn (this: *const nsIDOMCommandEvent, aCommand: *mut nsAString) -> nsresult,

    /* void initCommandEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean canCancelArg, in DOMString command); */
    pub initCommandEvent: unsafe extern "C" fn (this: *const nsIDOMCommandEvent, typeArg: *const nsAString, canBubbleArg: bool, canCancelArg: bool, command: *const nsAString) -> nsresult,

}


impl nsIDOMCommandEvent {
    /* readonly attribute DOMString command; */
    #[inline]
    pub unsafe fn get_command(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_command)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void initCommandEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean canCancelArg, in DOMString command); */
    #[inline]
    pub unsafe fn initCommandEvent(&self, typeArg: &[u16], canBubbleArg: bool, canCancelArg: bool, command: &[u16]) -> Result<(), nsresult> {
        let typeArg = nsString::from(typeArg);
        let command = nsString::from(command);
        match ((*self.vtable).initCommandEvent)(self as *const _, &*typeArg, canBubbleArg, canCancelArg, &*command) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


