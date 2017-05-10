//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFRemoteDataSource.idl
//


#[repr(C)]
pub struct nsIRDFRemoteDataSource {
    vtable: *const nsIRDFRemoteDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFRemoteDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1d297320, 0x27f7, 0x11d3,
            [0xbe, 0x01, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74])
    }
}

unsafe impl RefCounted for nsIRDFRemoteDataSource {
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
pub trait nsIRDFRemoteDataSourceCoerce {
    fn coerce_from(v: &nsIRDFRemoteDataSource) -> &Self;
}

impl nsIRDFRemoteDataSourceCoerce for nsIRDFRemoteDataSource {
    #[inline]
    fn coerce_from(v: &nsIRDFRemoteDataSource) -> &Self {
        v
    }
}

impl nsIRDFRemoteDataSource {
    #[inline]
    pub fn coerce<T: nsIRDFRemoteDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFRemoteDataSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFRemoteDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFRemoteDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFRemoteDataSourceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean loaded; */
    pub get_loaded: unsafe extern "C" fn (this: *const nsIRDFRemoteDataSource, aLoaded: *mut bool) -> nsresult,

    /* void Init (in string aURI); */
    pub Init: unsafe extern "C" fn (this: *const nsIRDFRemoteDataSource, aURI: *const libc::c_char) -> nsresult,

    /* void Refresh (in boolean aBlocking); */
    pub Refresh: unsafe extern "C" fn (this: *const nsIRDFRemoteDataSource, aBlocking: bool) -> nsresult,

    /* void Flush (); */
    pub Flush: unsafe extern "C" fn (this: *const nsIRDFRemoteDataSource) -> nsresult,

    /* void FlushTo (in string aURI); */
    pub FlushTo: unsafe extern "C" fn (this: *const nsIRDFRemoteDataSource, aURI: *const libc::c_char) -> nsresult,

}


impl nsIRDFRemoteDataSource {
    /* readonly attribute boolean loaded; */
    #[inline]
    pub unsafe fn get_loaded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loaded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void Init (in string aURI); */
    #[inline]
    pub unsafe fn Init(&self, aURI: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).Init)(self as *const _, aURI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void Refresh (in boolean aBlocking); */
    #[inline]
    pub unsafe fn Refresh(&self, aBlocking: bool) -> Result<(), nsresult> {

        match ((*self.vtable).Refresh)(self as *const _, aBlocking) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void Flush (); */
    #[inline]
    pub unsafe fn Flush(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).Flush)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void FlushTo (in string aURI); */
    #[inline]
    pub unsafe fn FlushTo(&self, aURI: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).FlushTo)(self as *const _, aURI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


