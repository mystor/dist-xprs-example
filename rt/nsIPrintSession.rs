//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintSession.idl
//


#[repr(C)]
pub struct nsIPrintSession {
    vtable: *const nsIPrintSessionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrintSession {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x424ae4bb, 0x10ca, 0x4f35,
            [0xb8, 0x4e, 0xea, 0xb8, 0x93, 0x32, 0x2d, 0xf4])
    }
}

unsafe impl RefCounted for nsIPrintSession {
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
pub trait nsIPrintSessionCoerce {
    fn coerce_from(v: &nsIPrintSession) -> &Self;
}

impl nsIPrintSessionCoerce for nsIPrintSession {
    #[inline]
    fn coerce_from(v: &nsIPrintSession) -> &Self {
        v
    }
}

impl nsIPrintSession {
    #[inline]
    pub fn coerce<T: nsIPrintSessionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrintSession {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrintSessionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintSession) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrintSessionVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] attribute RemotePrintJobChildPtr remotePrintJob; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_remotePrintJob: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_remotePrintJob: *const ::libc::c_void,

}


impl nsIPrintSession {
    /* [noscript] attribute RemotePrintJobChildPtr remotePrintJob; */



}


