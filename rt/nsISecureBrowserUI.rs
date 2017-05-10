//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecureBrowserUI.idl
//


#[repr(C)]
pub struct nsISecureBrowserUI {
    vtable: *const nsISecureBrowserUIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISecureBrowserUI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x718c662a, 0xf810, 0x4a80,
            [0xa6, 0xc9, 0x0b, 0x18, 0x10, 0xec, 0xad, 0xe2])
    }
}

unsafe impl RefCounted for nsISecureBrowserUI {
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
pub trait nsISecureBrowserUICoerce {
    fn coerce_from(v: &nsISecureBrowserUI) -> &Self;
}

impl nsISecureBrowserUICoerce for nsISecureBrowserUI {
    #[inline]
    fn coerce_from(v: &nsISecureBrowserUI) -> &Self {
        v
    }
}

impl nsISecureBrowserUI {
    #[inline]
    pub fn coerce<T: nsISecureBrowserUICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISecureBrowserUI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISecureBrowserUICoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecureBrowserUI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISecureBrowserUIVTable {
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindowProxy window); */
    pub init: unsafe extern "C" fn (this: *const nsISecureBrowserUI, window: *const mozIDOMWindowProxy) -> nsresult,

    /* void setDocShell (in nsIDocShell docShell); */
    pub setDocShell: unsafe extern "C" fn (this: *const nsISecureBrowserUI, docShell: *const nsIDocShell) -> nsresult,

    /* readonly attribute unsigned long state; */
    pub get_state: unsafe extern "C" fn (this: *const nsISecureBrowserUI, aState: *mut libc::uint32_t) -> nsresult,

}


impl nsISecureBrowserUI {
    /* void init (in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn init(&self, window: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDocShell (in nsIDocShell docShell); */
    #[inline]
    pub unsafe fn setDocShell(&self, docShell: Option<&nsIDocShell>) -> Result<(), nsresult> {

        match ((*self.vtable).setDocShell)(self as *const _, docShell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_state)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


