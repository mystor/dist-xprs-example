//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWritablePropertyBag.idl
//


#[repr(C)]
pub struct nsIWritablePropertyBag {
    vtable: *const nsIWritablePropertyBagVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWritablePropertyBag {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x96fc4671, 0xeeb4, 0x4823,
            [0x94, 0x21, 0xe5, 0x0f, 0xb7, 0x0a, 0xd3, 0x53])
    }
}

unsafe impl RefCounted for nsIWritablePropertyBag {
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
pub trait nsIWritablePropertyBagCoerce {
    fn coerce_from(v: &nsIWritablePropertyBag) -> &Self;
}

impl nsIWritablePropertyBagCoerce for nsIWritablePropertyBag {
    #[inline]
    fn coerce_from(v: &nsIWritablePropertyBag) -> &Self {
        v
    }
}

impl nsIWritablePropertyBag {
    #[inline]
    pub fn coerce<T: nsIWritablePropertyBagCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWritablePropertyBag {
    type Target = nsIPropertyBag;
    #[inline]
    fn deref(&self) -> &nsIPropertyBag {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPropertyBagCoerce> nsIWritablePropertyBagCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWritablePropertyBag) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWritablePropertyBagVTable {
    pub __base: nsIPropertyBagVTable,

    /* void setProperty (in AString name, in nsIVariant value); */
    pub setProperty: unsafe extern "C" fn (this: *const nsIWritablePropertyBag, name: *const nsAString, value: *const nsIVariant) -> nsresult,

    /* void deleteProperty (in AString name); */
    pub deleteProperty: unsafe extern "C" fn (this: *const nsIWritablePropertyBag, name: *const nsAString) -> nsresult,

}


impl nsIWritablePropertyBag {
    /* void setProperty (in AString name, in nsIVariant value); */
    #[inline]
    pub unsafe fn setProperty(&self, name: &[u16], value: Option<&nsIVariant>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).setProperty)(self as *const _, &*name, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteProperty (in AString name); */
    #[inline]
    pub unsafe fn deleteProperty(&self, name: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).deleteProperty)(self as *const _, &*name) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


