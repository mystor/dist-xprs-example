//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProfiler.idl
//


#[repr(C)]
pub struct nsIProfilerStartParams {
    vtable: *const nsIProfilerStartParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProfilerStartParams {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0a175ba7, 0x8fcf, 0x4ce9,
            [0x9c, 0x4b, 0xcc, 0xc6, 0x27, 0x2f, 0x44, 0x25])
    }
}

unsafe impl RefCounted for nsIProfilerStartParams {
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
pub trait nsIProfilerStartParamsCoerce {
    fn coerce_from(v: &nsIProfilerStartParams) -> &Self;
}

impl nsIProfilerStartParamsCoerce for nsIProfilerStartParams {
    #[inline]
    fn coerce_from(v: &nsIProfilerStartParams) -> &Self {
        v
    }
}

impl nsIProfilerStartParams {
    #[inline]
    pub fn coerce<T: nsIProfilerStartParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProfilerStartParams {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProfilerStartParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfilerStartParams) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProfilerStartParamsVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t entries; */
    pub get_entries: unsafe extern "C" fn (this: *const nsIProfilerStartParams, aEntries: *mut uint32_t) -> nsresult,

    /* readonly attribute double interval; */
    pub get_interval: unsafe extern "C" fn (this: *const nsIProfilerStartParams, aInterval: *mut libc::c_double) -> nsresult,

    /* readonly attribute uint32_t features; */
    pub get_features: unsafe extern "C" fn (this: *const nsIProfilerStartParams, aFeatures: *mut uint32_t) -> nsresult,

    /* [noscript,nostdcall,notxpcom] StringArrayRef getFilters (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFilters: *const ::libc::c_void,

}


impl nsIProfilerStartParams {
    /* readonly attribute uint32_t entries; */
    #[inline]
    pub unsafe fn get_entries(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_entries)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double interval; */
    #[inline]
    pub unsafe fn get_interval(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_interval)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t features; */
    #[inline]
    pub unsafe fn get_features(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_features)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,nostdcall,notxpcom] StringArrayRef getFilters (); */


}


#[repr(C)]
pub struct nsIProfiler {
    vtable: *const nsIProfilerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProfiler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xead3f75c, 0x0e0e, 0x4fbb,
            [0x90, 0x1c, 0x1e, 0x53, 0x92, 0xef, 0x5b, 0x2a])
    }
}

unsafe impl RefCounted for nsIProfiler {
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
pub trait nsIProfilerCoerce {
    fn coerce_from(v: &nsIProfiler) -> &Self;
}

impl nsIProfilerCoerce for nsIProfiler {
    #[inline]
    fn coerce_from(v: &nsIProfiler) -> &Self {
        v
    }
}

impl nsIProfiler {
    #[inline]
    pub fn coerce<T: nsIProfilerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProfiler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProfilerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfiler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProfilerVTable {
    pub __base: nsISupportsVTable,

    /* boolean CanProfile (); */
    pub CanProfile: unsafe extern "C" fn (this: *const nsIProfiler, _retval: *mut bool) -> nsresult,

    /* void StartProfiler (in uint32_t aEntries, in double aInterval, [array, size_is (aFeatureCount)] in string aFeatures, in uint32_t aFeatureCount, [array, size_is (aFilterCount), optional] in string aFilters, [optional] in uint32_t aFilterCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub StartProfiler: *const ::libc::c_void,

    /* void StopProfiler (); */
    pub StopProfiler: unsafe extern "C" fn (this: *const nsIProfiler) -> nsresult,

    /* boolean IsPaused (); */
    pub IsPaused: unsafe extern "C" fn (this: *const nsIProfiler, _retval: *mut bool) -> nsresult,

    /* void PauseSampling (); */
    pub PauseSampling: unsafe extern "C" fn (this: *const nsIProfiler) -> nsresult,

    /* void ResumeSampling (); */
    pub ResumeSampling: unsafe extern "C" fn (this: *const nsIProfiler) -> nsresult,

    /* void AddMarker (in string aMarker); */
    pub AddMarker: unsafe extern "C" fn (this: *const nsIProfiler, aMarker: *const libc::c_char) -> nsresult,

    /* string GetProfile ([optional] in double aSinceTime); */
    pub GetProfile: unsafe extern "C" fn (this: *const nsIProfiler, aSinceTime: libc::c_double, _retval: *mut *const libc::c_char) -> nsresult,

    /* [implicit_jscontext] jsval getProfileData ([optional] in double aSinceTime); */
    /// Unable to call function as its signature contains a non-rust type
    pub getProfileData: *const ::libc::c_void,

    /* [implicit_jscontext] nsISupports getProfileDataAsync ([optional] in double aSinceTime); */
    /// Unable to call function as its signature contains a non-rust type
    pub getProfileDataAsync: *const ::libc::c_void,

    /* void dumpProfileToFileAsync (in ACString aFilename, [optional] in double aSinceTime); */
    pub dumpProfileToFileAsync: unsafe extern "C" fn (this: *const nsIProfiler, aFilename: *const nsACString, aSinceTime: libc::c_double) -> nsresult,

    /* boolean IsActive (); */
    pub IsActive: unsafe extern "C" fn (this: *const nsIProfiler, _retval: *mut bool) -> nsresult,

    /* void GetFeatures (out uint32_t aCount, [array, size_is (aCount), retval] out string aFeatures); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetFeatures: *const ::libc::c_void,

    /* [noscript] readonly attribute nsIProfilerStartParams startParams; */
    pub get_startParams: unsafe extern "C" fn (this: *const nsIProfiler, aStartParams: *mut *const nsIProfilerStartParams) -> nsresult,

    /* void GetBufferInfo (out uint32_t aCurrentPosition, out uint32_t aTotalSize, out uint32_t aGeneration); */
    pub GetBufferInfo: unsafe extern "C" fn (this: *const nsIProfiler, aCurrentPosition: *mut uint32_t, aTotalSize: *mut uint32_t, aGeneration: *mut uint32_t) -> nsresult,

    /* double getElapsedTime (); */
    pub getElapsedTime: unsafe extern "C" fn (this: *const nsIProfiler, _retval: *mut libc::c_double) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval sharedLibraries; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_sharedLibraries: *const ::libc::c_void,

    /* void dumpProfileToFile (in string aFilename); */
    pub dumpProfileToFile: unsafe extern "C" fn (this: *const nsIProfiler, aFilename: *const libc::c_char) -> nsresult,

}


impl nsIProfiler {
    /* boolean CanProfile (); */
    #[inline]
    pub unsafe fn CanProfile(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).CanProfile)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void StartProfiler (in uint32_t aEntries, in double aInterval, [array, size_is (aFeatureCount)] in string aFeatures, in uint32_t aFeatureCount, [array, size_is (aFilterCount), optional] in string aFilters, [optional] in uint32_t aFilterCount); */


    /* void StopProfiler (); */
    #[inline]
    pub unsafe fn StopProfiler(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).StopProfiler)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean IsPaused (); */
    #[inline]
    pub unsafe fn IsPaused(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsPaused)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void PauseSampling (); */
    #[inline]
    pub unsafe fn PauseSampling(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).PauseSampling)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ResumeSampling (); */
    #[inline]
    pub unsafe fn ResumeSampling(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).ResumeSampling)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void AddMarker (in string aMarker); */
    #[inline]
    pub unsafe fn AddMarker(&self, aMarker: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).AddMarker)(self as *const _, aMarker) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string GetProfile ([optional] in double aSinceTime); */
    #[inline]
    pub unsafe fn GetProfile(&self, aSinceTime: libc::c_double) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).GetProfile)(self as *const _, aSinceTime, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval getProfileData ([optional] in double aSinceTime); */


    /* [implicit_jscontext] nsISupports getProfileDataAsync ([optional] in double aSinceTime); */


    /* void dumpProfileToFileAsync (in ACString aFilename, [optional] in double aSinceTime); */
    #[inline]
    pub unsafe fn dumpProfileToFileAsync(&self, aFilename: &[u8], aSinceTime: libc::c_double) -> Result<(), nsresult> {
        let aFilename = nsCString::from(aFilename);
        match ((*self.vtable).dumpProfileToFileAsync)(self as *const _, &*aFilename, aSinceTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean IsActive (); */
    #[inline]
    pub unsafe fn IsActive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsActive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void GetFeatures (out uint32_t aCount, [array, size_is (aCount), retval] out string aFeatures); */


    /* [noscript] readonly attribute nsIProfilerStartParams startParams; */
    #[inline]
    pub unsafe fn get_startParams(&self, ) -> Result<Option<RefPtr<nsIProfilerStartParams>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_startParams)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void GetBufferInfo (out uint32_t aCurrentPosition, out uint32_t aTotalSize, out uint32_t aGeneration); */
    #[inline]
    pub unsafe fn GetBufferInfo(&self, ) -> Result<(uint32_t, uint32_t, uint32_t), nsresult> {
        let mut aCurrentPosition: uint32_t = ::std::mem::zeroed();
        let mut aTotalSize: uint32_t = ::std::mem::zeroed();
        let mut aGeneration: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetBufferInfo)(self as *const _, &mut aCurrentPosition as *mut _, &mut aTotalSize as *mut _, &mut aGeneration as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCurrentPosition, aTotalSize, aGeneration))
    }

    /* double getElapsedTime (); */
    #[inline]
    pub unsafe fn getElapsedTime(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).getElapsedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval sharedLibraries; */


    /* void dumpProfileToFile (in string aFilename); */
    #[inline]
    pub unsafe fn dumpProfileToFile(&self, aFilename: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).dumpProfileToFile)(self as *const _, aFilename) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


