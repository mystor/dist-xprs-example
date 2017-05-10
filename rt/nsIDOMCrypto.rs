//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCrypto.idl
//


#[repr(C)]
pub struct nsIDOMCrypto {
    vtable: *const nsIDOMCryptoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCrypto {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x48d7f7fd, 0xbb85, 0x4c04,
            [0x9b, 0x8b, 0x5c, 0xd9, 0x13, 0x1a, 0xcd, 0xef])
    }
}

unsafe impl RefCounted for nsIDOMCrypto {
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
pub trait nsIDOMCryptoCoerce {
    fn coerce_from(v: &nsIDOMCrypto) -> &Self;
}

impl nsIDOMCryptoCoerce for nsIDOMCrypto {
    #[inline]
    fn coerce_from(v: &nsIDOMCrypto) -> &Self {
        v
    }
}

impl nsIDOMCrypto {
    #[inline]
    pub fn coerce<T: nsIDOMCryptoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCrypto {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCryptoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCrypto) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCryptoVTable {
    pub __base: nsISupportsVTable,

    /* [notxpcom] void init (in nsIGlobalObject parent); */
    pub init: unsafe extern "C" fn (this: *const nsIDOMCrypto, parent: *const nsIGlobalObject) -> libc::c_void,

}


impl nsIDOMCrypto {
    /* [notxpcom] void init (in nsIGlobalObject parent); */
    #[inline]
    pub unsafe fn init(&self, parent: Option<&nsIGlobalObject>) -> () {

        let _retval = ((*self.vtable).init)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

}


