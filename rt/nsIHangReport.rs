//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHangReport.idl
//


pub mod nsIHangReport_consts {
    pub const SLOW_SCRIPT: i64 = 1;
    pub const PLUGIN_HANG: i64 = 2;
}


#[repr(C)]
pub struct nsIHangReport {
    vtable: *const nsIHangReportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHangReport {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5fcffbb9, 0xbe62, 0x49b1,
            [0xb8, 0xa1, 0x36, 0xe8, 0x20, 0x78, 0x7a, 0x74])
    }
}

unsafe impl RefCounted for nsIHangReport {
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
pub trait nsIHangReportCoerce {
    fn coerce_from(v: &nsIHangReport) -> &Self;
}

impl nsIHangReportCoerce for nsIHangReport {
    #[inline]
    fn coerce_from(v: &nsIHangReport) -> &Self {
        v
    }
}

impl nsIHangReport {
    #[inline]
    pub fn coerce<T: nsIHangReportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHangReport {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHangReportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHangReport) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHangReportVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long hangType; */
    pub get_hangType: unsafe extern "C" fn (this: *const nsIHangReport, aHangType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMElement scriptBrowser; */
    pub get_scriptBrowser: unsafe extern "C" fn (this: *const nsIHangReport, aScriptBrowser: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute ACString scriptFileName; */
    pub get_scriptFileName: unsafe extern "C" fn (this: *const nsIHangReport, aScriptFileName: *mut nsACString) -> nsresult,

    /* readonly attribute ACString pluginName; */
    pub get_pluginName: unsafe extern "C" fn (this: *const nsIHangReport, aPluginName: *mut nsACString) -> nsresult,

    /* void userCanceled (); */
    pub userCanceled: unsafe extern "C" fn (this: *const nsIHangReport) -> nsresult,

    /* void terminateScript (); */
    pub terminateScript: unsafe extern "C" fn (this: *const nsIHangReport) -> nsresult,

    /* void terminatePlugin (); */
    pub terminatePlugin: unsafe extern "C" fn (this: *const nsIHangReport) -> nsresult,

    /* void beginStartingDebugger (); */
    pub beginStartingDebugger: unsafe extern "C" fn (this: *const nsIHangReport) -> nsresult,

    /* void endStartingDebugger (); */
    pub endStartingDebugger: unsafe extern "C" fn (this: *const nsIHangReport) -> nsresult,

    /* bool isReportForBrowser (in nsIFrameLoader aFrameLoader); */
    pub isReportForBrowser: unsafe extern "C" fn (this: *const nsIHangReport, aFrameLoader: *const nsIFrameLoader, _retval: *mut bool) -> nsresult,

}


impl nsIHangReport {
    /* readonly attribute unsigned long hangType; */
    #[inline]
    pub unsafe fn get_hangType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_hangType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMElement scriptBrowser; */
    #[inline]
    pub unsafe fn get_scriptBrowser(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_scriptBrowser)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute ACString scriptFileName; */
    #[inline]
    pub unsafe fn get_scriptFileName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_scriptFileName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString pluginName; */
    #[inline]
    pub unsafe fn get_pluginName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_pluginName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void userCanceled (); */
    #[inline]
    pub unsafe fn userCanceled(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).userCanceled)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void terminateScript (); */
    #[inline]
    pub unsafe fn terminateScript(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).terminateScript)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void terminatePlugin (); */
    #[inline]
    pub unsafe fn terminatePlugin(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).terminatePlugin)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginStartingDebugger (); */
    #[inline]
    pub unsafe fn beginStartingDebugger(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).beginStartingDebugger)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endStartingDebugger (); */
    #[inline]
    pub unsafe fn endStartingDebugger(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endStartingDebugger)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool isReportForBrowser (in nsIFrameLoader aFrameLoader); */
    #[inline]
    pub unsafe fn isReportForBrowser(&self, aFrameLoader: Option<&nsIFrameLoader>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isReportForBrowser)(self as *const _, aFrameLoader.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


