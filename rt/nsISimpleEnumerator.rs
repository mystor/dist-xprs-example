//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISimpleEnumerator.idl
//


#[repr(C)]
pub struct nsISimpleEnumerator {
    vtable: *const nsISimpleEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISimpleEnumerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd1899240, 0xf9d2, 0x11d2,
            [0xbd, 0xd6, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74])
    }
}

unsafe impl RefCounted for nsISimpleEnumerator {
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
pub trait nsISimpleEnumeratorCoerce {
    fn coerce_from(v: &nsISimpleEnumerator) -> &Self;
}

impl nsISimpleEnumeratorCoerce for nsISimpleEnumerator {
    #[inline]
    fn coerce_from(v: &nsISimpleEnumerator) -> &Self {
        v
    }
}

impl nsISimpleEnumerator {
    #[inline]
    pub fn coerce<T: nsISimpleEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISimpleEnumerator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISimpleEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISimpleEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISimpleEnumeratorVTable {
    pub __base: nsISupportsVTable,

    /* boolean hasMoreElements (); */
    pub hasMoreElements: unsafe extern "C" fn (this: *const nsISimpleEnumerator, _retval: *mut bool) -> nsresult,

    /* nsISupports getNext (); */
    pub getNext: unsafe extern "C" fn (this: *const nsISimpleEnumerator, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsISimpleEnumerator {
    /* boolean hasMoreElements (); */
    #[inline]
    pub unsafe fn hasMoreElements(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasMoreElements)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports getNext (); */
    #[inline]
    pub unsafe fn getNext(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNext)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


