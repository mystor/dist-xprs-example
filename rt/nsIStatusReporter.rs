//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStatusReporter.idl
//


#[repr(C)]
pub struct nsIStatusReporter {
    vtable: *const nsIStatusReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStatusReporter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xffcb716c, 0xdeeb, 0x44ea,
            [0x9d, 0x9d, 0xab, 0x25, 0xdc, 0x69, 0x80, 0xa8])
    }
}

unsafe impl RefCounted for nsIStatusReporter {
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
pub trait nsIStatusReporterCoerce {
    fn coerce_from(v: &nsIStatusReporter) -> &Self;
}

impl nsIStatusReporterCoerce for nsIStatusReporter {
    #[inline]
    fn coerce_from(v: &nsIStatusReporter) -> &Self {
        v
    }
}

impl nsIStatusReporter {
    #[inline]
    pub fn coerce<T: nsIStatusReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStatusReporter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStatusReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStatusReporter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStatusReporterVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIStatusReporter, aName: *mut nsACString) -> nsresult,

    /* readonly attribute ACString process; */
    pub get_process: unsafe extern "C" fn (this: *const nsIStatusReporter, aProcess: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String description; */
    pub get_description: unsafe extern "C" fn (this: *const nsIStatusReporter, aDescription: *mut nsACString) -> nsresult,

}


impl nsIStatusReporter {
    /* readonly attribute ACString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString process; */
    #[inline]
    pub unsafe fn get_process(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_process)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String description; */
    #[inline]
    pub unsafe fn get_description(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_description)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIStatusReporterManager {
    vtable: *const nsIStatusReporterManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStatusReporterManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfd531273, 0x3319, 0x4fcd,
            [0x90, 0xf2, 0x9f, 0x53, 0x87, 0x6c, 0x38, 0x28])
    }
}

unsafe impl RefCounted for nsIStatusReporterManager {
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
pub trait nsIStatusReporterManagerCoerce {
    fn coerce_from(v: &nsIStatusReporterManager) -> &Self;
}

impl nsIStatusReporterManagerCoerce for nsIStatusReporterManager {
    #[inline]
    fn coerce_from(v: &nsIStatusReporterManager) -> &Self {
        v
    }
}

impl nsIStatusReporterManager {
    #[inline]
    pub fn coerce<T: nsIStatusReporterManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStatusReporterManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStatusReporterManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStatusReporterManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStatusReporterManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsISimpleEnumerator enumerateReporters (); */
    pub enumerateReporters: unsafe extern "C" fn (this: *const nsIStatusReporterManager, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void registerReporter (in nsIStatusReporter reporter); */
    pub registerReporter: unsafe extern "C" fn (this: *const nsIStatusReporterManager, reporter: *const nsIStatusReporter) -> nsresult,

    /* void unregisterReporter (in nsIStatusReporter reporter); */
    pub unregisterReporter: unsafe extern "C" fn (this: *const nsIStatusReporterManager, reporter: *const nsIStatusReporter) -> nsresult,

    /* void init (); */
    pub init: unsafe extern "C" fn (this: *const nsIStatusReporterManager) -> nsresult,

    /* void dumpReports (); */
    pub dumpReports: unsafe extern "C" fn (this: *const nsIStatusReporterManager) -> nsresult,

}


impl nsIStatusReporterManager {
    /* nsISimpleEnumerator enumerateReporters (); */
    #[inline]
    pub unsafe fn enumerateReporters(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerateReporters)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void registerReporter (in nsIStatusReporter reporter); */
    #[inline]
    pub unsafe fn registerReporter(&self, reporter: Option<&nsIStatusReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).registerReporter)(self as *const _, reporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterReporter (in nsIStatusReporter reporter); */
    #[inline]
    pub unsafe fn unregisterReporter(&self, reporter: Option<&nsIStatusReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterReporter)(self as *const _, reporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void init (); */
    #[inline]
    pub unsafe fn init(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpReports (); */
    #[inline]
    pub unsafe fn dumpReports(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpReports)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


