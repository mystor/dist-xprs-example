//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProgrammingLanguage.idl
//


pub mod nsIProgrammingLanguage_consts {
    pub const UNKNOWN: i64 = 0;
    pub const JAVASCRIPT: i64 = 2;
}


#[repr(C)]
pub struct nsIProgrammingLanguage {
    vtable: *const nsIProgrammingLanguageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProgrammingLanguage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x02ad9f22, 0x3c98, 0x46f3,
            [0xbe, 0x4e, 0x2f, 0x5c, 0x92, 0x99, 0xe2, 0x9a])
    }
}

unsafe impl RefCounted for nsIProgrammingLanguage {
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
pub trait nsIProgrammingLanguageCoerce {
    fn coerce_from(v: &nsIProgrammingLanguage) -> &Self;
}

impl nsIProgrammingLanguageCoerce for nsIProgrammingLanguage {
    #[inline]
    fn coerce_from(v: &nsIProgrammingLanguage) -> &Self {
        v
    }
}

impl nsIProgrammingLanguage {
    #[inline]
    pub fn coerce<T: nsIProgrammingLanguageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProgrammingLanguage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProgrammingLanguageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProgrammingLanguage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProgrammingLanguageVTable {
    pub __base: nsISupportsVTable,

}


impl nsIProgrammingLanguage {
}


