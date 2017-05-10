//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandHandler.idl
//


#[repr(C)]
pub struct nsICommandHandlerInit {
    vtable: *const nsICommandHandlerInitVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandHandlerInit {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x08aed3cc, 0x69f7, 0x47ba,
            [0xa1, 0x10, 0xf2, 0xef, 0xa8, 0xa6, 0xd7, 0xea])
    }
}

unsafe impl RefCounted for nsICommandHandlerInit {
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
pub trait nsICommandHandlerInitCoerce {
    fn coerce_from(v: &nsICommandHandlerInit) -> &Self;
}

impl nsICommandHandlerInitCoerce for nsICommandHandlerInit {
    #[inline]
    fn coerce_from(v: &nsICommandHandlerInit) -> &Self {
        v
    }
}

impl nsICommandHandlerInit {
    #[inline]
    pub fn coerce<T: nsICommandHandlerInitCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandHandlerInit {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICommandHandlerInitCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandHandlerInit) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandHandlerInitVTable {
    pub __base: nsISupportsVTable,

    /* attribute mozIDOMWindowProxy window; */
    pub get_window: unsafe extern "C" fn (this: *const nsICommandHandlerInit, aWindow: *mut *const mozIDOMWindowProxy) -> nsresult,
    pub set_window: unsafe extern "C" fn (this: *const nsICommandHandlerInit, aWindow: *const mozIDOMWindowProxy) -> nsresult,

}


impl nsICommandHandlerInit {
    /* attribute mozIDOMWindowProxy window; */
    #[inline]
    pub unsafe fn get_window(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_window)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_window(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).set_window)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsICommandHandler {
    vtable: *const nsICommandHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x34a4fcf0, 0x66fc, 0x11d4,
            [0x95, 0x28, 0x00, 0x20, 0x18, 0x3b, 0xf1, 0x81])
    }
}

unsafe impl RefCounted for nsICommandHandler {
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
pub trait nsICommandHandlerCoerce {
    fn coerce_from(v: &nsICommandHandler) -> &Self;
}

impl nsICommandHandlerCoerce for nsICommandHandler {
    #[inline]
    fn coerce_from(v: &nsICommandHandler) -> &Self {
        v
    }
}

impl nsICommandHandler {
    #[inline]
    pub fn coerce<T: nsICommandHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICommandHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandHandlerVTable {
    pub __base: nsISupportsVTable,

    /* string exec (in string aCommand, in string aParameters); */
    pub exec: unsafe extern "C" fn (this: *const nsICommandHandler, aCommand: *const libc::c_char, aParameters: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* string query (in string aCommand, in string aParameters); */
    pub query: unsafe extern "C" fn (this: *const nsICommandHandler, aCommand: *const libc::c_char, aParameters: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsICommandHandler {
    /* string exec (in string aCommand, in string aParameters); */
    #[inline]
    pub unsafe fn exec(&self, aCommand: *const libc::c_char, aParameters: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).exec)(self as *const _, aCommand, aParameters, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string query (in string aCommand, in string aParameters); */
    #[inline]
    pub unsafe fn query(&self, aCommand: *const libc::c_char, aParameters: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).query)(self as *const _, aCommand, aParameters, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


