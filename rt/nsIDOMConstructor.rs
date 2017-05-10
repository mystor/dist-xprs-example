//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMConstructor.idl
//


#[repr(C)]
pub struct nsIDOMDOMConstructor {
    vtable: *const nsIDOMDOMConstructorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDOMConstructor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0ccbcf19, 0xd1b4, 0x489e,
            [0x98, 0x4c, 0xcd, 0x8c, 0x43, 0x67, 0x2b, 0xb9])
    }
}

unsafe impl RefCounted for nsIDOMDOMConstructor {
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
pub trait nsIDOMDOMConstructorCoerce {
    fn coerce_from(v: &nsIDOMDOMConstructor) -> &Self;
}

impl nsIDOMDOMConstructorCoerce for nsIDOMDOMConstructor {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMConstructor) -> &Self {
        v
    }
}

impl nsIDOMDOMConstructor {
    #[inline]
    pub fn coerce<T: nsIDOMDOMConstructorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDOMConstructor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMDOMConstructorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMConstructor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDOMConstructorVTable {
    pub __base: nsISupportsVTable,

    /* AString toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIDOMDOMConstructor, _retval: *mut nsAString) -> nsresult,

}


impl nsIDOMDOMConstructor {
    /* AString toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).toString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


