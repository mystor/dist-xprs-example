//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXBLAccessible.idl
//


#[repr(C)]
pub struct nsIXBLAccessible {
    vtable: *const nsIXBLAccessibleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXBLAccessible {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3716eb86, 0x166b, 0x445b,
            [0xa9, 0x4a, 0x9b, 0x52, 0x2f, 0xee, 0x96, 0xe6])
    }
}

unsafe impl RefCounted for nsIXBLAccessible {
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
pub trait nsIXBLAccessibleCoerce {
    fn coerce_from(v: &nsIXBLAccessible) -> &Self;
}

impl nsIXBLAccessibleCoerce for nsIXBLAccessible {
    #[inline]
    fn coerce_from(v: &nsIXBLAccessible) -> &Self {
        v
    }
}

impl nsIXBLAccessible {
    #[inline]
    pub fn coerce<T: nsIXBLAccessibleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXBLAccessible {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXBLAccessibleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXBLAccessible) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXBLAccessibleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString accessibleName; */
    pub get_accessibleName: unsafe extern "C" fn (this: *const nsIXBLAccessible, aAccessibleName: *mut nsAString) -> nsresult,

}


impl nsIXBLAccessible {
    /* readonly attribute DOMString accessibleName; */
    #[inline]
    pub unsafe fn get_accessibleName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_accessibleName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


