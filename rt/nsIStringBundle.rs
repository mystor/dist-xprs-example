//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStringBundle.idl
//


#[repr(C)]
pub struct nsIStringBundle {
    vtable: *const nsIStringBundleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStringBundle {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd85a17c2, 0xaa7c, 0x11d2,
            [0x9b, 0x8c, 0x00, 0x80, 0x5f, 0x8a, 0x16, 0xd9])
    }
}

unsafe impl RefCounted for nsIStringBundle {
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
pub trait nsIStringBundleCoerce {
    fn coerce_from(v: &nsIStringBundle) -> &Self;
}

impl nsIStringBundleCoerce for nsIStringBundle {
    #[inline]
    fn coerce_from(v: &nsIStringBundle) -> &Self {
        v
    }
}

impl nsIStringBundle {
    #[inline]
    pub fn coerce<T: nsIStringBundleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStringBundle {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStringBundleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringBundle) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStringBundleVTable {
    pub __base: nsISupportsVTable,

    /* wstring GetStringFromID (in long aID); */
    pub GetStringFromID: unsafe extern "C" fn (this: *const nsIStringBundle, aID: libc::int32_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* wstring GetStringFromName (in wstring aName); */
    pub GetStringFromName: unsafe extern "C" fn (this: *const nsIStringBundle, aName: *const libc::int16_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* wstring formatStringFromID (in long aID, [array, size_is (length)] in wstring params, in unsigned long length); */
    /// Unable to call function as its signature contains a non-rust type
    pub formatStringFromID: *const ::libc::c_void,

    /* wstring formatStringFromName (in wstring aName, [array, size_is (length)] in wstring params, in unsigned long length); */
    /// Unable to call function as its signature contains a non-rust type
    pub formatStringFromName: *const ::libc::c_void,

    /* nsISimpleEnumerator getSimpleEnumeration (); */
    pub getSimpleEnumeration: unsafe extern "C" fn (this: *const nsIStringBundle, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIStringBundle {
    /* wstring GetStringFromID (in long aID); */
    #[inline]
    pub unsafe fn GetStringFromID(&self, aID: libc::int32_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).GetStringFromID)(self as *const _, aID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring GetStringFromName (in wstring aName); */
    #[inline]
    pub unsafe fn GetStringFromName(&self, aName: *const libc::int16_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).GetStringFromName)(self as *const _, aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring formatStringFromID (in long aID, [array, size_is (length)] in wstring params, in unsigned long length); */


    /* wstring formatStringFromName (in wstring aName, [array, size_is (length)] in wstring params, in unsigned long length); */


    /* nsISimpleEnumerator getSimpleEnumeration (); */
    #[inline]
    pub unsafe fn getSimpleEnumeration(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSimpleEnumeration)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIStringBundleService {
    vtable: *const nsIStringBundleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStringBundleService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd85a17c0, 0xaa7c, 0x11d2,
            [0x9b, 0x8c, 0x00, 0x80, 0x5f, 0x8a, 0x16, 0xd9])
    }
}

unsafe impl RefCounted for nsIStringBundleService {
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
pub trait nsIStringBundleServiceCoerce {
    fn coerce_from(v: &nsIStringBundleService) -> &Self;
}

impl nsIStringBundleServiceCoerce for nsIStringBundleService {
    #[inline]
    fn coerce_from(v: &nsIStringBundleService) -> &Self {
        v
    }
}

impl nsIStringBundleService {
    #[inline]
    pub fn coerce<T: nsIStringBundleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStringBundleService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStringBundleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringBundleService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStringBundleServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIStringBundle createBundle (in string aURLSpec); */
    pub createBundle: unsafe extern "C" fn (this: *const nsIStringBundleService, aURLSpec: *const libc::c_char, _retval: *mut *const nsIStringBundle) -> nsresult,

    /* nsIStringBundle createExtensibleBundle (in string aRegistryKey); */
    pub createExtensibleBundle: unsafe extern "C" fn (this: *const nsIStringBundleService, aRegistryKey: *const libc::c_char, _retval: *mut *const nsIStringBundle) -> nsresult,

    /* wstring formatStatusMessage (in nsresult aStatus, in wstring aStatusArg); */
    pub formatStatusMessage: unsafe extern "C" fn (this: *const nsIStringBundleService, aStatus: nsresult, aStatusArg: *const libc::int16_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* void flushBundles (); */
    pub flushBundles: unsafe extern "C" fn (this: *const nsIStringBundleService) -> nsresult,

}


impl nsIStringBundleService {
    /* nsIStringBundle createBundle (in string aURLSpec); */
    #[inline]
    pub unsafe fn createBundle(&self, aURLSpec: *const libc::c_char) -> Result<Option<RefPtr<nsIStringBundle>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createBundle)(self as *const _, aURLSpec, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIStringBundle createExtensibleBundle (in string aRegistryKey); */
    #[inline]
    pub unsafe fn createExtensibleBundle(&self, aRegistryKey: *const libc::c_char) -> Result<Option<RefPtr<nsIStringBundle>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createExtensibleBundle)(self as *const _, aRegistryKey, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* wstring formatStatusMessage (in nsresult aStatus, in wstring aStatusArg); */
    #[inline]
    pub unsafe fn formatStatusMessage(&self, aStatus: nsresult, aStatusArg: *const libc::int16_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).formatStatusMessage)(self as *const _, aStatus, aStatusArg, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void flushBundles (); */
    #[inline]
    pub unsafe fn flushBundles(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).flushBundles)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


