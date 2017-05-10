//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILocalFile.idl
//


#[repr(C)]
pub struct nsILocalFile {
    vtable: *const nsILocalFileVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocalFile {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7ba8c6ba, 0x2ce2, 0x48b1,
            [0xbd, 0x60, 0x4c, 0x32, 0xaa, 0xc3, 0x5f, 0x9c])
    }
}

unsafe impl RefCounted for nsILocalFile {
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
pub trait nsILocalFileCoerce {
    fn coerce_from(v: &nsILocalFile) -> &Self;
}

impl nsILocalFileCoerce for nsILocalFile {
    #[inline]
    fn coerce_from(v: &nsILocalFile) -> &Self {
        v
    }
}

impl nsILocalFile {
    #[inline]
    pub fn coerce<T: nsILocalFileCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocalFile {
    type Target = nsIFile;
    #[inline]
    fn deref(&self) -> &nsIFile {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIFileCoerce> nsILocalFileCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalFile) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocalFileVTable {
    pub __base: nsIFileVTable,

}


impl nsILocalFile {
}


