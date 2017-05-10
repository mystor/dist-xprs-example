//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentSniffer.idl
//


#[repr(C)]
pub struct nsIContentSniffer {
    vtable: *const nsIContentSnifferVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentSniffer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa5772d1b, 0xfc63, 0x495e,
            [0xa1, 0x69, 0x96, 0xe8, 0xd3, 0x31, 0x1a, 0xf0])
    }
}

unsafe impl RefCounted for nsIContentSniffer {
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
pub trait nsIContentSnifferCoerce {
    fn coerce_from(v: &nsIContentSniffer) -> &Self;
}

impl nsIContentSnifferCoerce for nsIContentSniffer {
    #[inline]
    fn coerce_from(v: &nsIContentSniffer) -> &Self {
        v
    }
}

impl nsIContentSniffer {
    #[inline]
    pub fn coerce<T: nsIContentSnifferCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentSniffer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentSnifferCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSniffer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentSnifferVTable {
    pub __base: nsISupportsVTable,

    /* ACString getMIMETypeFromContent (in nsIRequest aRequest, [array, size_is (aLength), const] in octet aData, in unsigned long aLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMIMETypeFromContent: *const ::libc::c_void,

}


impl nsIContentSniffer {
    /* ACString getMIMETypeFromContent (in nsIRequest aRequest, [array, size_is (aLength), const] in octet aData, in unsigned long aLength); */


}


