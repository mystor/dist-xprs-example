//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIArrayBufferInputStream.idl
//


#[repr(C)]
pub struct nsIArrayBufferInputStream {
    vtable: *const nsIArrayBufferInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIArrayBufferInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3014dde6, 0xaa1c, 0x41db,
            [0x87, 0xd0, 0x48, 0x76, 0x4a, 0x37, 0x10, 0xf6])
    }
}

unsafe impl RefCounted for nsIArrayBufferInputStream {
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
pub trait nsIArrayBufferInputStreamCoerce {
    fn coerce_from(v: &nsIArrayBufferInputStream) -> &Self;
}

impl nsIArrayBufferInputStreamCoerce for nsIArrayBufferInputStream {
    #[inline]
    fn coerce_from(v: &nsIArrayBufferInputStream) -> &Self {
        v
    }
}

impl nsIArrayBufferInputStream {
    #[inline]
    pub fn coerce<T: nsIArrayBufferInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIArrayBufferInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIArrayBufferInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIArrayBufferInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIArrayBufferInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* [implicit_jscontext] void setData (in jsval buffer, in unsigned long byteOffset, in unsigned long byteLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub setData: *const ::libc::c_void,

}


impl nsIArrayBufferInputStream {
    /* [implicit_jscontext] void setData (in jsval buffer, in unsigned long byteOffset, in unsigned long byteLen); */


}


