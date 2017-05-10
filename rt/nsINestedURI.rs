//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINestedURI.idl
//


#[repr(C)]
pub struct nsINestedURI {
    vtable: *const nsINestedURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINestedURI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6de2c874, 0x796c, 0x46bf,
            [0xb5, 0x7f, 0x0d, 0x7b, 0xd7, 0xd6, 0xca, 0xb0])
    }
}

unsafe impl RefCounted for nsINestedURI {
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
pub trait nsINestedURICoerce {
    fn coerce_from(v: &nsINestedURI) -> &Self;
}

impl nsINestedURICoerce for nsINestedURI {
    #[inline]
    fn coerce_from(v: &nsINestedURI) -> &Self {
        v
    }
}

impl nsINestedURI {
    #[inline]
    pub fn coerce<T: nsINestedURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINestedURI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINestedURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsINestedURI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINestedURIVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIURI innerURI; */
    pub get_innerURI: unsafe extern "C" fn (this: *const nsINestedURI, aInnerURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI innermostURI; */
    pub get_innermostURI: unsafe extern "C" fn (this: *const nsINestedURI, aInnermostURI: *mut *const nsIURI) -> nsresult,

}


impl nsINestedURI {
    /* readonly attribute nsIURI innerURI; */
    #[inline]
    pub unsafe fn get_innerURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_innerURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI innermostURI; */
    #[inline]
    pub unsafe fn get_innermostURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_innermostURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


