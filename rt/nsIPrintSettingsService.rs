//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintSettingsService.idl
//


#[repr(C)]
pub struct nsIPrintSettingsService {
    vtable: *const nsIPrintSettingsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrintSettingsService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x841387c8, 0x72e6, 0x484b,
            [0x92, 0x96, 0xbf, 0x6e, 0xea, 0x80, 0xd5, 0x8a])
    }
}

unsafe impl RefCounted for nsIPrintSettingsService {
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
pub trait nsIPrintSettingsServiceCoerce {
    fn coerce_from(v: &nsIPrintSettingsService) -> &Self;
}

impl nsIPrintSettingsServiceCoerce for nsIPrintSettingsService {
    #[inline]
    fn coerce_from(v: &nsIPrintSettingsService) -> &Self {
        v
    }
}

impl nsIPrintSettingsService {
    #[inline]
    pub fn coerce<T: nsIPrintSettingsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrintSettingsService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrintSettingsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintSettingsService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrintSettingsServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrintSettings globalPrintSettings; */
    pub get_globalPrintSettings: unsafe extern "C" fn (this: *const nsIPrintSettingsService, aGlobalPrintSettings: *mut *const nsIPrintSettings) -> nsresult,

    /* readonly attribute nsIPrintSettings newPrintSettings; */
    pub get_newPrintSettings: unsafe extern "C" fn (this: *const nsIPrintSettingsService, aNewPrintSettings: *mut *const nsIPrintSettings) -> nsresult,

    /* readonly attribute wstring defaultPrinterName; */
    pub get_defaultPrinterName: unsafe extern "C" fn (this: *const nsIPrintSettingsService, aDefaultPrinterName: *mut *const libc::int16_t) -> nsresult,

    /* void initPrintSettingsFromPrinter (in wstring aPrinterName, in nsIPrintSettings aPrintSettings); */
    pub initPrintSettingsFromPrinter: unsafe extern "C" fn (this: *const nsIPrintSettingsService, aPrinterName: *const libc::int16_t, aPrintSettings: *const nsIPrintSettings) -> nsresult,

    /* void initPrintSettingsFromPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags); */
    pub initPrintSettingsFromPrefs: unsafe extern "C" fn (this: *const nsIPrintSettingsService, aPrintSettings: *const nsIPrintSettings, aUsePrinterNamePrefix: bool, aFlags: libc::uint32_t) -> nsresult,

    /* void savePrintSettingsToPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags); */
    pub savePrintSettingsToPrefs: unsafe extern "C" fn (this: *const nsIPrintSettingsService, aPrintSettings: *const nsIPrintSettings, aUsePrinterNamePrefix: bool, aFlags: libc::uint32_t) -> nsresult,

    /* [noscript] void SerializeToPrintData (in nsIPrintSettings aPrintSettings, in nsIWebBrowserPrint aWebBrowserPrint, in PrintDataPtr data); */
    /// Unable to call function as its signature contains a non-rust type
    pub SerializeToPrintData: *const ::libc::c_void,

    /* [noscript] void DeserializeToPrintSettings (in PrintDataRef data, in nsIPrintSettings aPrintSettings); */
    /// Unable to call function as its signature contains a non-rust type
    pub DeserializeToPrintSettings: *const ::libc::c_void,

}


impl nsIPrintSettingsService {
    /* readonly attribute nsIPrintSettings globalPrintSettings; */
    #[inline]
    pub unsafe fn get_globalPrintSettings(&self, ) -> Result<Option<RefPtr<nsIPrintSettings>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_globalPrintSettings)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIPrintSettings newPrintSettings; */
    #[inline]
    pub unsafe fn get_newPrintSettings(&self, ) -> Result<Option<RefPtr<nsIPrintSettings>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_newPrintSettings)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute wstring defaultPrinterName; */
    #[inline]
    pub unsafe fn get_defaultPrinterName(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultPrinterName)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void initPrintSettingsFromPrinter (in wstring aPrinterName, in nsIPrintSettings aPrintSettings); */
    #[inline]
    pub unsafe fn initPrintSettingsFromPrinter(&self, aPrinterName: *const libc::int16_t, aPrintSettings: Option<&nsIPrintSettings>) -> Result<(), nsresult> {

        match ((*self.vtable).initPrintSettingsFromPrinter)(self as *const _, aPrinterName, aPrintSettings.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initPrintSettingsFromPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn initPrintSettingsFromPrefs(&self, aPrintSettings: Option<&nsIPrintSettings>, aUsePrinterNamePrefix: bool, aFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).initPrintSettingsFromPrefs)(self as *const _, aPrintSettings.map_or(::std::ptr::null(), |x| x as *const _), aUsePrinterNamePrefix, aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void savePrintSettingsToPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn savePrintSettingsToPrefs(&self, aPrintSettings: Option<&nsIPrintSettings>, aUsePrinterNamePrefix: bool, aFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).savePrintSettingsToPrefs)(self as *const _, aPrintSettings.map_or(::std::ptr::null(), |x| x as *const _), aUsePrinterNamePrefix, aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void SerializeToPrintData (in nsIPrintSettings aPrintSettings, in nsIWebBrowserPrint aWebBrowserPrint, in PrintDataPtr data); */


    /* [noscript] void DeserializeToPrintSettings (in PrintDataRef data, in nsIPrintSettings aPrintSettings); */


}


