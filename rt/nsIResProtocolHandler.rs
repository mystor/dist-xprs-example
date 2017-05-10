//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIResProtocolHandler.idl
//


#[repr(C)]
pub struct nsIResProtocolHandler {
    vtable: *const nsIResProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIResProtocolHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x241d34ac, 0x9ed5, 0x46d7,
            [0x91, 0x0c, 0x7a, 0x9d, 0x91, 0x4a, 0xa0, 0xc5])
    }
}

unsafe impl RefCounted for nsIResProtocolHandler {
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
pub trait nsIResProtocolHandlerCoerce {
    fn coerce_from(v: &nsIResProtocolHandler) -> &Self;
}

impl nsIResProtocolHandlerCoerce for nsIResProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIResProtocolHandler) -> &Self {
        v
    }
}

impl nsIResProtocolHandler {
    #[inline]
    pub fn coerce<T: nsIResProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIResProtocolHandler {
    type Target = nsISubstitutingProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsISubstitutingProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISubstitutingProtocolHandlerCoerce> nsIResProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIResProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIResProtocolHandlerVTable {
    pub __base: nsISubstitutingProtocolHandlerVTable,

}


impl nsIResProtocolHandler {
}


