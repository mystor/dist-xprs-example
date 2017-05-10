//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISafeOutputStream.idl
//


#[repr(C)]
pub struct nsISafeOutputStream {
    vtable: *const nsISafeOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISafeOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5f914307, 0x5c34, 0x4e1f,
            [0x8e, 0x32, 0xec, 0x74, 0x9d, 0x25, 0xb2, 0x7a])
    }
}

unsafe impl RefCounted for nsISafeOutputStream {
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
pub trait nsISafeOutputStreamCoerce {
    fn coerce_from(v: &nsISafeOutputStream) -> &Self;
}

impl nsISafeOutputStreamCoerce for nsISafeOutputStream {
    #[inline]
    fn coerce_from(v: &nsISafeOutputStream) -> &Self {
        v
    }
}

impl nsISafeOutputStream {
    #[inline]
    pub fn coerce<T: nsISafeOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISafeOutputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISafeOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISafeOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISafeOutputStreamVTable {
    pub __base: nsISupportsVTable,

    /* void finish (); */
    pub finish: unsafe extern "C" fn (this: *const nsISafeOutputStream) -> nsresult,

}


impl nsISafeOutputStream {
    /* void finish (); */
    #[inline]
    pub unsafe fn finish(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finish)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


