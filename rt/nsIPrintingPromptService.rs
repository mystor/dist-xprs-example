//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintingPromptService.idl
//


#[repr(C)]
pub struct nsIPrintingPromptService {
    vtable: *const nsIPrintingPromptServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrintingPromptService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x328daa3e, 0x09e4, 0x455f,
            [0xbb, 0x6f, 0x0a, 0x92, 0x17, 0x66, 0x04, 0x2f])
    }
}

unsafe impl RefCounted for nsIPrintingPromptService {
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
pub trait nsIPrintingPromptServiceCoerce {
    fn coerce_from(v: &nsIPrintingPromptService) -> &Self;
}

impl nsIPrintingPromptServiceCoerce for nsIPrintingPromptService {
    #[inline]
    fn coerce_from(v: &nsIPrintingPromptService) -> &Self {
        v
    }
}

impl nsIPrintingPromptService {
    #[inline]
    pub fn coerce<T: nsIPrintingPromptServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrintingPromptService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrintingPromptServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintingPromptService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrintingPromptServiceVTable {
    pub __base: nsISupportsVTable,

    /* void showPrintDialog (in mozIDOMWindowProxy parent, in nsIWebBrowserPrint webBrowserPrint, in nsIPrintSettings printSettings); */
    pub showPrintDialog: unsafe extern "C" fn (this: *const nsIPrintingPromptService, parent: *const mozIDOMWindowProxy, webBrowserPrint: *const nsIWebBrowserPrint, printSettings: *const nsIPrintSettings) -> nsresult,

    /* void showProgress (in mozIDOMWindowProxy parent, in nsIWebBrowserPrint webBrowserPrint, in nsIPrintSettings printSettings, in nsIObserver openDialogObserver, in boolean isForPrinting, out nsIWebProgressListener webProgressListener, out nsIPrintProgressParams printProgressParams, out boolean notifyOnOpen); */
    pub showProgress: unsafe extern "C" fn (this: *const nsIPrintingPromptService, parent: *const mozIDOMWindowProxy, webBrowserPrint: *const nsIWebBrowserPrint, printSettings: *const nsIPrintSettings, openDialogObserver: *const nsIObserver, isForPrinting: bool, webProgressListener: *mut *const nsIWebProgressListener, printProgressParams: *mut *const nsIPrintProgressParams, notifyOnOpen: *mut bool) -> nsresult,

    /* void showPageSetup (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings, in nsIObserver aObs); */
    pub showPageSetup: unsafe extern "C" fn (this: *const nsIPrintingPromptService, parent: *const mozIDOMWindowProxy, printSettings: *const nsIPrintSettings, aObs: *const nsIObserver) -> nsresult,

    /* void showPrinterProperties (in mozIDOMWindowProxy parent, in wstring printerName, in nsIPrintSettings printSettings); */
    pub showPrinterProperties: unsafe extern "C" fn (this: *const nsIPrintingPromptService, parent: *const mozIDOMWindowProxy, printerName: *const libc::int16_t, printSettings: *const nsIPrintSettings) -> nsresult,

}


impl nsIPrintingPromptService {
    /* void showPrintDialog (in mozIDOMWindowProxy parent, in nsIWebBrowserPrint webBrowserPrint, in nsIPrintSettings printSettings); */
    #[inline]
    pub unsafe fn showPrintDialog(&self, parent: Option<&mozIDOMWindowProxy>, webBrowserPrint: Option<&nsIWebBrowserPrint>, printSettings: Option<&nsIPrintSettings>) -> Result<(), nsresult> {

        match ((*self.vtable).showPrintDialog)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), webBrowserPrint.map_or(::std::ptr::null(), |x| x as *const _), printSettings.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showProgress (in mozIDOMWindowProxy parent, in nsIWebBrowserPrint webBrowserPrint, in nsIPrintSettings printSettings, in nsIObserver openDialogObserver, in boolean isForPrinting, out nsIWebProgressListener webProgressListener, out nsIPrintProgressParams printProgressParams, out boolean notifyOnOpen); */
    #[inline]
    pub unsafe fn showProgress(&self, parent: Option<&mozIDOMWindowProxy>, webBrowserPrint: Option<&nsIWebBrowserPrint>, printSettings: Option<&nsIPrintSettings>, openDialogObserver: Option<&nsIObserver>, isForPrinting: bool) -> Result<(Option<RefPtr<nsIWebProgressListener>>, Option<RefPtr<nsIPrintProgressParams>>, bool), nsresult> {
        let mut webProgressListener = GetterAddrefs::new();
        let mut printProgressParams = GetterAddrefs::new();
        let mut notifyOnOpen: bool = ::std::mem::zeroed();
        match ((*self.vtable).showProgress)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), webBrowserPrint.map_or(::std::ptr::null(), |x| x as *const _), printSettings.map_or(::std::ptr::null(), |x| x as *const _), openDialogObserver.map_or(::std::ptr::null(), |x| x as *const _), isForPrinting, webProgressListener.ptr(), printProgressParams.ptr(), &mut notifyOnOpen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((webProgressListener.refptr(), printProgressParams.refptr(), notifyOnOpen))
    }

    /* void showPageSetup (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings, in nsIObserver aObs); */
    #[inline]
    pub unsafe fn showPageSetup(&self, parent: Option<&mozIDOMWindowProxy>, printSettings: Option<&nsIPrintSettings>, aObs: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).showPageSetup)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), printSettings.map_or(::std::ptr::null(), |x| x as *const _), aObs.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showPrinterProperties (in mozIDOMWindowProxy parent, in wstring printerName, in nsIPrintSettings printSettings); */
    #[inline]
    pub unsafe fn showPrinterProperties(&self, parent: Option<&mozIDOMWindowProxy>, printerName: *const libc::int16_t, printSettings: Option<&nsIPrintSettings>) -> Result<(), nsresult> {

        match ((*self.vtable).showPrinterProperties)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), printerName, printSettings.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


