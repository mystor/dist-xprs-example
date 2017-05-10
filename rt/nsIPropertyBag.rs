//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPropertyBag.idl
//


#[repr(C)]
pub struct nsIPropertyBag {
    vtable: *const nsIPropertyBagVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPropertyBag {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbfcd37b0, 0xa49f, 0x11d5,
            [0x91, 0x0d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIPropertyBag {
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
pub trait nsIPropertyBagCoerce {
    fn coerce_from(v: &nsIPropertyBag) -> &Self;
}

impl nsIPropertyBagCoerce for nsIPropertyBag {
    #[inline]
    fn coerce_from(v: &nsIPropertyBag) -> &Self {
        v
    }
}

impl nsIPropertyBag {
    #[inline]
    pub fn coerce<T: nsIPropertyBagCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPropertyBag {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPropertyBagCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPropertyBag) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPropertyBagVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISimpleEnumerator enumerator; */
    pub get_enumerator: unsafe extern "C" fn (this: *const nsIPropertyBag, aEnumerator: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsIVariant getProperty (in AString name); */
    pub getProperty: unsafe extern "C" fn (this: *const nsIPropertyBag, name: *const nsAString, _retval: *mut *const nsIVariant) -> nsresult,

}


impl nsIPropertyBag {
    /* readonly attribute nsISimpleEnumerator enumerator; */
    #[inline]
    pub unsafe fn get_enumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_enumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIVariant getProperty (in AString name); */
    #[inline]
    pub unsafe fn getProperty(&self, name: &[u16]) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getProperty)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


