//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPhonetic.idl
//


#[repr(C)]
pub struct nsIPhonetic {
    vtable: *const nsIPhoneticVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPhonetic {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbc6ea726, 0xab56, 0x46b6,
            [0xa2, 0x1a, 0xaa, 0x7b, 0x76, 0xd6, 0x81, 0x8f])
    }
}

unsafe impl RefCounted for nsIPhonetic {
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
pub trait nsIPhoneticCoerce {
    fn coerce_from(v: &nsIPhonetic) -> &Self;
}

impl nsIPhoneticCoerce for nsIPhonetic {
    #[inline]
    fn coerce_from(v: &nsIPhonetic) -> &Self {
        v
    }
}

impl nsIPhonetic {
    #[inline]
    pub fn coerce<T: nsIPhoneticCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPhonetic {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPhoneticCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPhonetic) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPhoneticVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString phonetic; */
    pub get_phonetic: unsafe extern "C" fn (this: *const nsIPhonetic, aPhonetic: *mut nsAString) -> nsresult,

}


impl nsIPhonetic {
    /* readonly attribute DOMString phonetic; */
    #[inline]
    pub unsafe fn get_phonetic(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_phonetic)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


