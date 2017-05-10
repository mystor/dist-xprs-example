//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINSSU2FToken.idl
//


#[repr(C)]
pub struct nsINSSU2FToken {
    vtable: *const nsINSSU2FTokenVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINSSU2FToken {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd9104a00, 0x140b, 0x4f86,
            [0xa4, 0xb0, 0x49, 0x98, 0x87, 0x8e, 0xf4, 0xe6])
    }
}

unsafe impl RefCounted for nsINSSU2FToken {
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
pub trait nsINSSU2FTokenCoerce {
    fn coerce_from(v: &nsINSSU2FToken) -> &Self;
}

impl nsINSSU2FTokenCoerce for nsINSSU2FToken {
    #[inline]
    fn coerce_from(v: &nsINSSU2FToken) -> &Self {
        v
    }
}

impl nsINSSU2FToken {
    #[inline]
    pub fn coerce<T: nsINSSU2FTokenCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINSSU2FToken {
    type Target = nsIU2FToken;
    #[inline]
    fn deref(&self) -> &nsIU2FToken {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIU2FTokenCoerce> nsINSSU2FTokenCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINSSU2FToken) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINSSU2FTokenVTable {
    pub __base: nsIU2FTokenVTable,

}


impl nsINSSU2FToken {
}


