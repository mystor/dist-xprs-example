//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDirectoryService.idl
//


#[repr(C)]
pub struct nsIDirectoryServiceProvider {
    vtable: *const nsIDirectoryServiceProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDirectoryServiceProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbbf8cab0, 0xd43a, 0x11d3,
            [0x8c, 0xc2, 0x00, 0x60, 0x97, 0x92, 0x27, 0x8c])
    }
}

unsafe impl RefCounted for nsIDirectoryServiceProvider {
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
pub trait nsIDirectoryServiceProviderCoerce {
    fn coerce_from(v: &nsIDirectoryServiceProvider) -> &Self;
}

impl nsIDirectoryServiceProviderCoerce for nsIDirectoryServiceProvider {
    #[inline]
    fn coerce_from(v: &nsIDirectoryServiceProvider) -> &Self {
        v
    }
}

impl nsIDirectoryServiceProvider {
    #[inline]
    pub fn coerce<T: nsIDirectoryServiceProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDirectoryServiceProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDirectoryServiceProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectoryServiceProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDirectoryServiceProviderVTable {
    pub __base: nsISupportsVTable,

    /* nsIFile getFile (in string prop, out boolean persistent); */
    pub getFile: unsafe extern "C" fn (this: *const nsIDirectoryServiceProvider, prop: *const libc::c_char, persistent: *mut bool, _retval: *mut *const nsIFile) -> nsresult,

}


impl nsIDirectoryServiceProvider {
    /* nsIFile getFile (in string prop, out boolean persistent); */
    #[inline]
    pub unsafe fn getFile(&self, prop: *const libc::c_char) -> Result<(bool, Option<RefPtr<nsIFile>>), nsresult> {
        let mut persistent: bool = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFile)(self as *const _, prop, &mut persistent as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((persistent, _retval.refptr()))
    }

}


#[repr(C)]
pub struct nsIDirectoryServiceProvider2 {
    vtable: *const nsIDirectoryServiceProvider2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDirectoryServiceProvider2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2f977d4b, 0x5485, 0x11d4,
            [0x87, 0xe2, 0x00, 0x10, 0xa4, 0xe7, 0x5e, 0xf2])
    }
}

unsafe impl RefCounted for nsIDirectoryServiceProvider2 {
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
pub trait nsIDirectoryServiceProvider2Coerce {
    fn coerce_from(v: &nsIDirectoryServiceProvider2) -> &Self;
}

impl nsIDirectoryServiceProvider2Coerce for nsIDirectoryServiceProvider2 {
    #[inline]
    fn coerce_from(v: &nsIDirectoryServiceProvider2) -> &Self {
        v
    }
}

impl nsIDirectoryServiceProvider2 {
    #[inline]
    pub fn coerce<T: nsIDirectoryServiceProvider2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDirectoryServiceProvider2 {
    type Target = nsIDirectoryServiceProvider;
    #[inline]
    fn deref(&self) -> &nsIDirectoryServiceProvider {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDirectoryServiceProviderCoerce> nsIDirectoryServiceProvider2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectoryServiceProvider2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDirectoryServiceProvider2VTable {
    pub __base: nsIDirectoryServiceProviderVTable,

    /* nsISimpleEnumerator getFiles (in string prop); */
    pub getFiles: unsafe extern "C" fn (this: *const nsIDirectoryServiceProvider2, prop: *const libc::c_char, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIDirectoryServiceProvider2 {
    /* nsISimpleEnumerator getFiles (in string prop); */
    #[inline]
    pub unsafe fn getFiles(&self, prop: *const libc::c_char) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFiles)(self as *const _, prop, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIDirectoryService {
    vtable: *const nsIDirectoryServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDirectoryService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x57a66a60, 0xd43a, 0x11d3,
            [0x8c, 0xc2, 0x00, 0x60, 0x97, 0x92, 0x27, 0x8c])
    }
}

unsafe impl RefCounted for nsIDirectoryService {
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
pub trait nsIDirectoryServiceCoerce {
    fn coerce_from(v: &nsIDirectoryService) -> &Self;
}

impl nsIDirectoryServiceCoerce for nsIDirectoryService {
    #[inline]
    fn coerce_from(v: &nsIDirectoryService) -> &Self {
        v
    }
}

impl nsIDirectoryService {
    #[inline]
    pub fn coerce<T: nsIDirectoryServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDirectoryService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDirectoryServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectoryService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDirectoryServiceVTable {
    pub __base: nsISupportsVTable,

    /* void init (); */
    pub init: unsafe extern "C" fn (this: *const nsIDirectoryService) -> nsresult,

    /* void registerProvider (in nsIDirectoryServiceProvider prov); */
    pub registerProvider: unsafe extern "C" fn (this: *const nsIDirectoryService, prov: *const nsIDirectoryServiceProvider) -> nsresult,

    /* void unregisterProvider (in nsIDirectoryServiceProvider prov); */
    pub unregisterProvider: unsafe extern "C" fn (this: *const nsIDirectoryService, prov: *const nsIDirectoryServiceProvider) -> nsresult,

}


impl nsIDirectoryService {
    /* void init (); */
    #[inline]
    pub unsafe fn init(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerProvider (in nsIDirectoryServiceProvider prov); */
    #[inline]
    pub unsafe fn registerProvider(&self, prov: Option<&nsIDirectoryServiceProvider>) -> Result<(), nsresult> {

        match ((*self.vtable).registerProvider)(self as *const _, prov.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterProvider (in nsIDirectoryServiceProvider prov); */
    #[inline]
    pub unsafe fn unregisterProvider(&self, prov: Option<&nsIDirectoryServiceProvider>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterProvider)(self as *const _, prov.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


