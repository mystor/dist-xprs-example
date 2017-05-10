//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserPrint.idl
//


pub mod nsIWebBrowserPrint_consts {
    pub const PRINTPREVIEW_GOTO_PAGENUM: i64 = 0;
    pub const PRINTPREVIEW_PREV_PAGE: i64 = 1;
    pub const PRINTPREVIEW_NEXT_PAGE: i64 = 2;
    pub const PRINTPREVIEW_HOME: i64 = 3;
    pub const PRINTPREVIEW_END: i64 = 4;
}


#[repr(C)]
pub struct nsIWebBrowserPrint {
    vtable: *const nsIWebBrowserPrintVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserPrint {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc9a934ed, 0xfff1, 0x4971,
            [0xbf, 0xba, 0x6c, 0x25, 0xad, 0x70, 0xe1, 0xe6])
    }
}

unsafe impl RefCounted for nsIWebBrowserPrint {
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
pub trait nsIWebBrowserPrintCoerce {
    fn coerce_from(v: &nsIWebBrowserPrint) -> &Self;
}

impl nsIWebBrowserPrintCoerce for nsIWebBrowserPrint {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPrint) -> &Self {
        v
    }
}

impl nsIWebBrowserPrint {
    #[inline]
    pub fn coerce<T: nsIWebBrowserPrintCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserPrint {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserPrintCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPrint) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserPrintVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrintSettings globalPrintSettings; */
    pub get_globalPrintSettings: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aGlobalPrintSettings: *mut *const nsIPrintSettings) -> nsresult,

    /* readonly attribute nsIPrintSettings currentPrintSettings; */
    pub get_currentPrintSettings: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aCurrentPrintSettings: *mut *const nsIPrintSettings) -> nsresult,

    /* readonly attribute mozIDOMWindowProxy currentChildDOMWindow; */
    pub get_currentChildDOMWindow: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aCurrentChildDOMWindow: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute boolean doingPrint; */
    pub get_doingPrint: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aDoingPrint: *mut bool) -> nsresult,

    /* readonly attribute boolean doingPrintPreview; */
    pub get_doingPrintPreview: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aDoingPrintPreview: *mut bool) -> nsresult,

    /* readonly attribute boolean isFramesetDocument; */
    pub get_isFramesetDocument: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aIsFramesetDocument: *mut bool) -> nsresult,

    /* readonly attribute boolean isFramesetFrameSelected; */
    pub get_isFramesetFrameSelected: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aIsFramesetFrameSelected: *mut bool) -> nsresult,

    /* readonly attribute boolean isIFrameSelected; */
    pub get_isIFrameSelected: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aIsIFrameSelected: *mut bool) -> nsresult,

    /* readonly attribute boolean isRangeSelection; */
    pub get_isRangeSelection: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aIsRangeSelection: *mut bool) -> nsresult,

    /* readonly attribute long printPreviewNumPages; */
    pub get_printPreviewNumPages: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aPrintPreviewNumPages: *mut libc::int32_t) -> nsresult,

    /* void print (in nsIPrintSettings aThePrintSettings, in nsIWebProgressListener aWPListener); */
    pub print: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aThePrintSettings: *const nsIPrintSettings, aWPListener: *const nsIWebProgressListener) -> nsresult,

    /* void printPreview (in nsIPrintSettings aThePrintSettings, in mozIDOMWindowProxy aChildDOMWin, in nsIWebProgressListener aWPListener); */
    pub printPreview: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aThePrintSettings: *const nsIPrintSettings, aChildDOMWin: *const mozIDOMWindowProxy, aWPListener: *const nsIWebProgressListener) -> nsresult,

    /* void printPreviewNavigate (in short aNavType, in long aPageNum); */
    pub printPreviewNavigate: unsafe extern "C" fn (this: *const nsIWebBrowserPrint, aNavType: libc::int16_t, aPageNum: libc::int32_t) -> nsresult,

    /* void cancel (); */
    pub cancel: unsafe extern "C" fn (this: *const nsIWebBrowserPrint) -> nsresult,

    /* void enumerateDocumentNames (out uint32_t aCount, [array, size_is (aCount), retval] out wstring aResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub enumerateDocumentNames: *const ::libc::c_void,

    /* void exitPrintPreview (); */
    pub exitPrintPreview: unsafe extern "C" fn (this: *const nsIWebBrowserPrint) -> nsresult,

}


impl nsIWebBrowserPrint {
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

    /* readonly attribute nsIPrintSettings currentPrintSettings; */
    #[inline]
    pub unsafe fn get_currentPrintSettings(&self, ) -> Result<Option<RefPtr<nsIPrintSettings>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentPrintSettings)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIDOMWindowProxy currentChildDOMWindow; */
    #[inline]
    pub unsafe fn get_currentChildDOMWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentChildDOMWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean doingPrint; */
    #[inline]
    pub unsafe fn get_doingPrint(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_doingPrint)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean doingPrintPreview; */
    #[inline]
    pub unsafe fn get_doingPrintPreview(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_doingPrintPreview)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isFramesetDocument; */
    #[inline]
    pub unsafe fn get_isFramesetDocument(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isFramesetDocument)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isFramesetFrameSelected; */
    #[inline]
    pub unsafe fn get_isFramesetFrameSelected(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isFramesetFrameSelected)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isIFrameSelected; */
    #[inline]
    pub unsafe fn get_isIFrameSelected(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isIFrameSelected)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isRangeSelection; */
    #[inline]
    pub unsafe fn get_isRangeSelection(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isRangeSelection)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long printPreviewNumPages; */
    #[inline]
    pub unsafe fn get_printPreviewNumPages(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_printPreviewNumPages)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void print (in nsIPrintSettings aThePrintSettings, in nsIWebProgressListener aWPListener); */
    #[inline]
    pub unsafe fn print(&self, aThePrintSettings: Option<&nsIPrintSettings>, aWPListener: Option<&nsIWebProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).print)(self as *const _, aThePrintSettings.map_or(::std::ptr::null(), |x| x as *const _), aWPListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void printPreview (in nsIPrintSettings aThePrintSettings, in mozIDOMWindowProxy aChildDOMWin, in nsIWebProgressListener aWPListener); */
    #[inline]
    pub unsafe fn printPreview(&self, aThePrintSettings: Option<&nsIPrintSettings>, aChildDOMWin: Option<&mozIDOMWindowProxy>, aWPListener: Option<&nsIWebProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).printPreview)(self as *const _, aThePrintSettings.map_or(::std::ptr::null(), |x| x as *const _), aChildDOMWin.map_or(::std::ptr::null(), |x| x as *const _), aWPListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void printPreviewNavigate (in short aNavType, in long aPageNum); */
    #[inline]
    pub unsafe fn printPreviewNavigate(&self, aNavType: libc::int16_t, aPageNum: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).printPreviewNavigate)(self as *const _, aNavType, aPageNum) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancel (); */
    #[inline]
    pub unsafe fn cancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void enumerateDocumentNames (out uint32_t aCount, [array, size_is (aCount), retval] out wstring aResult); */


    /* void exitPrintPreview (); */
    #[inline]
    pub unsafe fn exitPrintPreview(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).exitPrintPreview)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


