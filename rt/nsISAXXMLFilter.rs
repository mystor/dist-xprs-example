//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXXMLFilter.idl
//


#[repr(C)]
pub struct nsISAXXMLFilter {
    vtable: *const nsISAXXMLFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXXMLFilter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x77a22cf0, 0x6cdf, 0x11da,
            [0xbe, 0x43, 0x00, 0x14, 0x22, 0x10, 0x69, 0x90])
    }
}

unsafe impl RefCounted for nsISAXXMLFilter {
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
pub trait nsISAXXMLFilterCoerce {
    fn coerce_from(v: &nsISAXXMLFilter) -> &Self;
}

impl nsISAXXMLFilterCoerce for nsISAXXMLFilter {
    #[inline]
    fn coerce_from(v: &nsISAXXMLFilter) -> &Self {
        v
    }
}

impl nsISAXXMLFilter {
    #[inline]
    pub fn coerce<T: nsISAXXMLFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXXMLFilter {
    type Target = nsISAXXMLReader;
    #[inline]
    fn deref(&self) -> &nsISAXXMLReader {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISAXXMLReaderCoerce> nsISAXXMLFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXXMLFilter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXXMLFilterVTable {
    pub __base: nsISAXXMLReaderVTable,

    /* attribute nsISAXXMLReader parent; */
    pub get_parent: unsafe extern "C" fn (this: *const nsISAXXMLFilter, aParent: *mut *const nsISAXXMLReader) -> nsresult,
    pub set_parent: unsafe extern "C" fn (this: *const nsISAXXMLFilter, aParent: *const nsISAXXMLReader) -> nsresult,

}


impl nsISAXXMLFilter {
    /* attribute nsISAXXMLReader parent; */
    #[inline]
    pub unsafe fn get_parent(&self, ) -> Result<Option<RefPtr<nsISAXXMLReader>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_parent(&self, aParent: Option<&nsISAXXMLReader>) -> Result<(), nsresult> {

        match ((*self.vtable).set_parent)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


