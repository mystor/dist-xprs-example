//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFTPChannelParentInternal.idl
//


#[repr(C)]
pub struct nsIFTPChannelParentInternal {
    vtable: *const nsIFTPChannelParentInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFTPChannelParentInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x87b58410, 0x83cb, 0x42a7,
            [0xb5, 0x7b, 0x27, 0xc0, 0x7e, 0xf8, 0x28, 0xd7])
    }
}

unsafe impl RefCounted for nsIFTPChannelParentInternal {
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
pub trait nsIFTPChannelParentInternalCoerce {
    fn coerce_from(v: &nsIFTPChannelParentInternal) -> &Self;
}

impl nsIFTPChannelParentInternalCoerce for nsIFTPChannelParentInternal {
    #[inline]
    fn coerce_from(v: &nsIFTPChannelParentInternal) -> &Self {
        v
    }
}

impl nsIFTPChannelParentInternal {
    #[inline]
    pub fn coerce<T: nsIFTPChannelParentInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFTPChannelParentInternal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFTPChannelParentInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFTPChannelParentInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFTPChannelParentInternalVTable {
    pub __base: nsISupportsVTable,

    /* void setErrorMsg (in string msg, in boolean useUTF8); */
    pub setErrorMsg: unsafe extern "C" fn (this: *const nsIFTPChannelParentInternal, msg: *const libc::c_char, useUTF8: bool) -> nsresult,

}


impl nsIFTPChannelParentInternal {
    /* void setErrorMsg (in string msg, in boolean useUTF8); */
    #[inline]
    pub unsafe fn setErrorMsg(&self, msg: *const libc::c_char, useUTF8: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setErrorMsg)(self as *const _, msg, useUTF8) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


