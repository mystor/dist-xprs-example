//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConsoleListener.idl
//


#[repr(C)]
pub struct nsIConsoleListener {
    vtable: *const nsIConsoleListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIConsoleListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x35c400a4, 0x5792, 0x438c,
            [0xb9, 0x15, 0x65, 0xe3, 0x0d, 0x58, 0xd5, 0x57])
    }
}

unsafe impl RefCounted for nsIConsoleListener {
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
pub trait nsIConsoleListenerCoerce {
    fn coerce_from(v: &nsIConsoleListener) -> &Self;
}

impl nsIConsoleListenerCoerce for nsIConsoleListener {
    #[inline]
    fn coerce_from(v: &nsIConsoleListener) -> &Self {
        v
    }
}

impl nsIConsoleListener {
    #[inline]
    pub fn coerce<T: nsIConsoleListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIConsoleListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIConsoleListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConsoleListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIConsoleListenerVTable {
    pub __base: nsISupportsVTable,

    /* void observe (in nsIConsoleMessage aMessage); */
    pub observe: unsafe extern "C" fn (this: *const nsIConsoleListener, aMessage: *const nsIConsoleMessage) -> nsresult,

}


impl nsIConsoleListener {
    /* void observe (in nsIConsoleMessage aMessage); */
    #[inline]
    pub unsafe fn observe(&self, aMessage: Option<&nsIConsoleMessage>) -> Result<(), nsresult> {

        match ((*self.vtable).observe)(self as *const _, aMessage.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


