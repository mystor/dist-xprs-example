//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTTPIndex.idl
//


#[repr(C)]
pub struct nsIHTTPIndex {
    vtable: *const nsIHTTPIndexVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHTTPIndex {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6f2bdbd0, 0x58c3, 0x11d3,
            [0xbe, 0x36, 0x00, 0x10, 0x4b, 0xde, 0x60, 0x48])
    }
}

unsafe impl RefCounted for nsIHTTPIndex {
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
pub trait nsIHTTPIndexCoerce {
    fn coerce_from(v: &nsIHTTPIndex) -> &Self;
}

impl nsIHTTPIndexCoerce for nsIHTTPIndex {
    #[inline]
    fn coerce_from(v: &nsIHTTPIndex) -> &Self {
        v
    }
}

impl nsIHTTPIndex {
    #[inline]
    pub fn coerce<T: nsIHTTPIndexCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHTTPIndex {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHTTPIndexCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTTPIndex) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHTTPIndexVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string BaseURL; */
    pub get_BaseURL: unsafe extern "C" fn (this: *const nsIHTTPIndex, aBaseURL: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute nsIRDFDataSource DataSource; */
    pub get_DataSource: unsafe extern "C" fn (this: *const nsIHTTPIndex, aDataSource: *mut *const nsIRDFDataSource) -> nsresult,

    /* attribute string encoding; */
    pub get_encoding: unsafe extern "C" fn (this: *const nsIHTTPIndex, aEncoding: *mut *const libc::c_char) -> nsresult,
    pub set_encoding: unsafe extern "C" fn (this: *const nsIHTTPIndex, aEncoding: *const libc::c_char) -> nsresult,

}


impl nsIHTTPIndex {
    /* readonly attribute string BaseURL; */
    #[inline]
    pub unsafe fn get_BaseURL(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_BaseURL)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIRDFDataSource DataSource; */
    #[inline]
    pub unsafe fn get_DataSource(&self, ) -> Result<Option<RefPtr<nsIRDFDataSource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DataSource)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute string encoding; */
    #[inline]
    pub unsafe fn get_encoding(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_encoding)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_encoding(&self, aEncoding: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_encoding)(self as *const _, aEncoding) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


