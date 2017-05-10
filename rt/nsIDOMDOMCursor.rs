//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDOMCursor.idl
//


#[repr(C)]
pub struct nsICursorContinueCallback {
    vtable: *const nsICursorContinueCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICursorContinueCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3a75d80a, 0x9258, 0x4ab8,
            [0x95, 0xfd, 0xec, 0x0b, 0x5f, 0x63, 0x4d, 0xf1])
    }
}

unsafe impl RefCounted for nsICursorContinueCallback {
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
pub trait nsICursorContinueCallbackCoerce {
    fn coerce_from(v: &nsICursorContinueCallback) -> &Self;
}

impl nsICursorContinueCallbackCoerce for nsICursorContinueCallback {
    #[inline]
    fn coerce_from(v: &nsICursorContinueCallback) -> &Self {
        v
    }
}

impl nsICursorContinueCallback {
    #[inline]
    pub fn coerce<T: nsICursorContinueCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICursorContinueCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICursorContinueCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICursorContinueCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICursorContinueCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleContinue (); */
    pub handleContinue: unsafe extern "C" fn (this: *const nsICursorContinueCallback) -> nsresult,

}


impl nsICursorContinueCallback {
    /* void handleContinue (); */
    #[inline]
    pub unsafe fn handleContinue(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).handleContinue)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDOMDOMCursor {
    vtable: *const nsIDOMDOMCursorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDOMCursor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x062ea35a, 0x5158, 0x425a,
            [0xb7, 0xbc, 0x3a, 0xe9, 0xda, 0xa8, 0x43, 0x98])
    }
}

unsafe impl RefCounted for nsIDOMDOMCursor {
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
pub trait nsIDOMDOMCursorCoerce {
    fn coerce_from(v: &nsIDOMDOMCursor) -> &Self;
}

impl nsIDOMDOMCursorCoerce for nsIDOMDOMCursor {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMCursor) -> &Self {
        v
    }
}

impl nsIDOMDOMCursor {
    #[inline]
    pub fn coerce<T: nsIDOMDOMCursorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDOMCursor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMDOMCursorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMCursor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDOMCursorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean done; */
    pub get_done: unsafe extern "C" fn (this: *const nsIDOMDOMCursor, aDone: *mut bool) -> nsresult,

    /* void continue (); */
    pub continue_: unsafe extern "C" fn (this: *const nsIDOMDOMCursor) -> nsresult,

}


impl nsIDOMDOMCursor {
    /* readonly attribute boolean done; */
    #[inline]
    pub unsafe fn get_done(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_done)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void continue (); */
    #[inline]
    pub unsafe fn continue_(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).continue_)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


