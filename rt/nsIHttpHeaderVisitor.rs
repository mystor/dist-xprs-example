//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpHeaderVisitor.idl
//


#[repr(C)]
pub struct nsIHttpHeaderVisitor {
    vtable: *const nsIHttpHeaderVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpHeaderVisitor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x35412859, 0xb9d9, 0x423c,
            [0x88, 0x66, 0x2d, 0x45, 0x59, 0xfd, 0xd2, 0xbe])
    }
}

unsafe impl RefCounted for nsIHttpHeaderVisitor {
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
pub trait nsIHttpHeaderVisitorCoerce {
    fn coerce_from(v: &nsIHttpHeaderVisitor) -> &Self;
}

impl nsIHttpHeaderVisitorCoerce for nsIHttpHeaderVisitor {
    #[inline]
    fn coerce_from(v: &nsIHttpHeaderVisitor) -> &Self {
        v
    }
}

impl nsIHttpHeaderVisitor {
    #[inline]
    pub fn coerce<T: nsIHttpHeaderVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpHeaderVisitor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpHeaderVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpHeaderVisitor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpHeaderVisitorVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void visitHeader (in ACString aHeader, in ACString aValue); */
    pub visitHeader: unsafe extern "C" fn (this: *const nsIHttpHeaderVisitor, aHeader: *const nsACString, aValue: *const nsACString) -> nsresult,

}


impl nsIHttpHeaderVisitor {
    /* [must_use] void visitHeader (in ACString aHeader, in ACString aValue); */
    #[inline]
    pub unsafe fn visitHeader(&self, aHeader: &[u8], aValue: &[u8]) -> Result<(), nsresult> {
        let aHeader = nsCString::from(aHeader);
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).visitHeader)(self as *const _, &*aHeader, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


