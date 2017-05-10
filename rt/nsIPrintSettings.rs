//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintSettings.idl
//


pub mod nsIPrintSettings_consts {
    pub const kInitSaveOddEvenPages: i64 = 1;
    pub const kInitSaveHeaderLeft: i64 = 2;
    pub const kInitSaveHeaderCenter: i64 = 4;
    pub const kInitSaveHeaderRight: i64 = 8;
    pub const kInitSaveFooterLeft: i64 = 16;
    pub const kInitSaveFooterCenter: i64 = 32;
    pub const kInitSaveFooterRight: i64 = 64;
    pub const kInitSaveBGColors: i64 = 128;
    pub const kInitSaveBGImages: i64 = 256;
    pub const kInitSavePaperSize: i64 = 512;
    pub const kInitSaveResolution: i64 = 1024;
    pub const kInitSaveDuplex: i64 = 2048;
    pub const kInitSavePaperData: i64 = 8192;
    pub const kInitSaveUnwriteableMargins: i64 = 16384;
    pub const kInitSaveEdges: i64 = 32768;
    pub const kInitSaveReversed: i64 = 65536;
    pub const kInitSaveInColor: i64 = 131072;
    pub const kInitSaveOrientation: i64 = 262144;
    pub const kInitSavePrinterName: i64 = 1048576;
    pub const kInitSavePrintToFile: i64 = 2097152;
    pub const kInitSaveToFileName: i64 = 4194304;
    pub const kInitSavePageDelay: i64 = 8388608;
    pub const kInitSaveMargins: i64 = 16777216;
    pub const kInitSaveNativeData: i64 = 33554432;
    pub const kInitSaveShrinkToFit: i64 = 134217728;
    pub const kInitSaveScaling: i64 = 268435456;
    pub const kInitSaveAll: i64 = 4294967295;
    pub const kPrintOddPages: i64 = 1;
    pub const kPrintEvenPages: i64 = 2;
    pub const kEnableSelectionRB: i64 = 4;
    pub const kRangeAllPages: i64 = 0;
    pub const kRangeSpecifiedPageRange: i64 = 1;
    pub const kRangeSelection: i64 = 2;
    pub const kRangeFocusFrame: i64 = 3;
    pub const kJustLeft: i64 = 0;
    pub const kJustCenter: i64 = 1;
    pub const kJustRight: i64 = 2;
    pub const kUseInternalDefault: i64 = 0;
    pub const kUseSettingWhenPossible: i64 = 1;
    pub const kPaperSizeNativeData: i64 = 0;
    pub const kPaperSizeDefined: i64 = 1;
    pub const kPaperSizeInches: i64 = 0;
    pub const kPaperSizeMillimeters: i64 = 1;
    pub const kPortraitOrientation: i64 = 0;
    pub const kLandscapeOrientation: i64 = 1;
    pub const kNoFrames: i64 = 0;
    pub const kFramesAsIs: i64 = 1;
    pub const kSelectedFrame: i64 = 2;
    pub const kEachFrameSep: i64 = 3;
    pub const kFrameEnableNone: i64 = 0;
    pub const kFrameEnableAll: i64 = 1;
    pub const kFrameEnableAsIsAndEach: i64 = 2;
    pub const kOutputFormatNative: i64 = 0;
    pub const kOutputFormatPS: i64 = 1;
    pub const kOutputFormatPDF: i64 = 2;
}


#[repr(C)]
pub struct nsIPrintSettings {
    vtable: *const nsIPrintSettingsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrintSettings {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xecc5cbad, 0x57fc, 0x4731,
            [0xb0, 0xbd, 0x09, 0xe8, 0x65, 0xbd, 0x62, 0xad])
    }
}

unsafe impl RefCounted for nsIPrintSettings {
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
pub trait nsIPrintSettingsCoerce {
    fn coerce_from(v: &nsIPrintSettings) -> &Self;
}

impl nsIPrintSettingsCoerce for nsIPrintSettings {
    #[inline]
    fn coerce_from(v: &nsIPrintSettings) -> &Self {
        v
    }
}

impl nsIPrintSettings {
    #[inline]
    pub fn coerce<T: nsIPrintSettingsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrintSettings {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrintSettingsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintSettings) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrintSettingsVTable {
    pub __base: nsISupportsVTable,

    /* void SetPrintOptions (in int32_t aType, in boolean aTurnOnOff); */
    pub SetPrintOptions: unsafe extern "C" fn (this: *const nsIPrintSettings, aType: int32_t, aTurnOnOff: bool) -> nsresult,

    /* boolean GetPrintOptions (in int32_t aType); */
    pub GetPrintOptions: unsafe extern "C" fn (this: *const nsIPrintSettings, aType: int32_t, _retval: *mut bool) -> nsresult,

    /* int32_t GetPrintOptionsBits (); */
    pub GetPrintOptionsBits: unsafe extern "C" fn (this: *const nsIPrintSettings, _retval: *mut int32_t) -> nsresult,

    /* void SetPrintOptionsBits (in int32_t bits); */
    pub SetPrintOptionsBits: unsafe extern "C" fn (this: *const nsIPrintSettings, bits: int32_t) -> nsresult,

    /* void GetEffectivePageSize (out double aWidth, out double aHeight); */
    pub GetEffectivePageSize: unsafe extern "C" fn (this: *const nsIPrintSettings, aWidth: *mut libc::c_double, aHeight: *mut libc::c_double) -> nsresult,

    /* nsIPrintSettings clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsIPrintSettings, _retval: *mut *const nsIPrintSettings) -> nsresult,

    /* void assign (in nsIPrintSettings aPS); */
    pub assign: unsafe extern "C" fn (this: *const nsIPrintSettings, aPS: *const nsIPrintSettings) -> nsresult,

    /* [noscript] attribute nsIPrintSession printSession; */
    pub get_printSession: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintSession: *mut *const nsIPrintSession) -> nsresult,
    pub set_printSession: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintSession: *const nsIPrintSession) -> nsresult,

    /* attribute long startPageRange; */
    pub get_startPageRange: unsafe extern "C" fn (this: *const nsIPrintSettings, aStartPageRange: *mut libc::int32_t) -> nsresult,
    pub set_startPageRange: unsafe extern "C" fn (this: *const nsIPrintSettings, aStartPageRange: libc::int32_t) -> nsresult,

    /* attribute long endPageRange; */
    pub get_endPageRange: unsafe extern "C" fn (this: *const nsIPrintSettings, aEndPageRange: *mut libc::int32_t) -> nsresult,
    pub set_endPageRange: unsafe extern "C" fn (this: *const nsIPrintSettings, aEndPageRange: libc::int32_t) -> nsresult,

    /* attribute double edgeTop; */
    pub get_edgeTop: unsafe extern "C" fn (this: *const nsIPrintSettings, aEdgeTop: *mut libc::c_double) -> nsresult,
    pub set_edgeTop: unsafe extern "C" fn (this: *const nsIPrintSettings, aEdgeTop: libc::c_double) -> nsresult,

    /* attribute double edgeLeft; */
    pub get_edgeLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aEdgeLeft: *mut libc::c_double) -> nsresult,
    pub set_edgeLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aEdgeLeft: libc::c_double) -> nsresult,

    /* attribute double edgeBottom; */
    pub get_edgeBottom: unsafe extern "C" fn (this: *const nsIPrintSettings, aEdgeBottom: *mut libc::c_double) -> nsresult,
    pub set_edgeBottom: unsafe extern "C" fn (this: *const nsIPrintSettings, aEdgeBottom: libc::c_double) -> nsresult,

    /* attribute double edgeRight; */
    pub get_edgeRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aEdgeRight: *mut libc::c_double) -> nsresult,
    pub set_edgeRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aEdgeRight: libc::c_double) -> nsresult,

    /* attribute double marginTop; */
    pub get_marginTop: unsafe extern "C" fn (this: *const nsIPrintSettings, aMarginTop: *mut libc::c_double) -> nsresult,
    pub set_marginTop: unsafe extern "C" fn (this: *const nsIPrintSettings, aMarginTop: libc::c_double) -> nsresult,

    /* attribute double marginLeft; */
    pub get_marginLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aMarginLeft: *mut libc::c_double) -> nsresult,
    pub set_marginLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aMarginLeft: libc::c_double) -> nsresult,

    /* attribute double marginBottom; */
    pub get_marginBottom: unsafe extern "C" fn (this: *const nsIPrintSettings, aMarginBottom: *mut libc::c_double) -> nsresult,
    pub set_marginBottom: unsafe extern "C" fn (this: *const nsIPrintSettings, aMarginBottom: libc::c_double) -> nsresult,

    /* attribute double marginRight; */
    pub get_marginRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aMarginRight: *mut libc::c_double) -> nsresult,
    pub set_marginRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aMarginRight: libc::c_double) -> nsresult,

    /* attribute double unwriteableMarginTop; */
    pub get_unwriteableMarginTop: unsafe extern "C" fn (this: *const nsIPrintSettings, aUnwriteableMarginTop: *mut libc::c_double) -> nsresult,
    pub set_unwriteableMarginTop: unsafe extern "C" fn (this: *const nsIPrintSettings, aUnwriteableMarginTop: libc::c_double) -> nsresult,

    /* attribute double unwriteableMarginLeft; */
    pub get_unwriteableMarginLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aUnwriteableMarginLeft: *mut libc::c_double) -> nsresult,
    pub set_unwriteableMarginLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aUnwriteableMarginLeft: libc::c_double) -> nsresult,

    /* attribute double unwriteableMarginBottom; */
    pub get_unwriteableMarginBottom: unsafe extern "C" fn (this: *const nsIPrintSettings, aUnwriteableMarginBottom: *mut libc::c_double) -> nsresult,
    pub set_unwriteableMarginBottom: unsafe extern "C" fn (this: *const nsIPrintSettings, aUnwriteableMarginBottom: libc::c_double) -> nsresult,

    /* attribute double unwriteableMarginRight; */
    pub get_unwriteableMarginRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aUnwriteableMarginRight: *mut libc::c_double) -> nsresult,
    pub set_unwriteableMarginRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aUnwriteableMarginRight: libc::c_double) -> nsresult,

    /* attribute double scaling; */
    pub get_scaling: unsafe extern "C" fn (this: *const nsIPrintSettings, aScaling: *mut libc::c_double) -> nsresult,
    pub set_scaling: unsafe extern "C" fn (this: *const nsIPrintSettings, aScaling: libc::c_double) -> nsresult,

    /* attribute boolean printBGColors; */
    pub get_printBGColors: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintBGColors: *mut bool) -> nsresult,
    pub set_printBGColors: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintBGColors: bool) -> nsresult,

    /* attribute boolean printBGImages; */
    pub get_printBGImages: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintBGImages: *mut bool) -> nsresult,
    pub set_printBGImages: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintBGImages: bool) -> nsresult,

    /* attribute short printRange; */
    pub get_printRange: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintRange: *mut libc::int16_t) -> nsresult,
    pub set_printRange: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintRange: libc::int16_t) -> nsresult,

    /* attribute wstring title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIPrintSettings, aTitle: *mut *const libc::int16_t) -> nsresult,
    pub set_title: unsafe extern "C" fn (this: *const nsIPrintSettings, aTitle: *const libc::int16_t) -> nsresult,

    /* attribute wstring docURL; */
    pub get_docURL: unsafe extern "C" fn (this: *const nsIPrintSettings, aDocURL: *mut *const libc::int16_t) -> nsresult,
    pub set_docURL: unsafe extern "C" fn (this: *const nsIPrintSettings, aDocURL: *const libc::int16_t) -> nsresult,

    /* attribute wstring headerStrLeft; */
    pub get_headerStrLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aHeaderStrLeft: *mut *const libc::int16_t) -> nsresult,
    pub set_headerStrLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aHeaderStrLeft: *const libc::int16_t) -> nsresult,

    /* attribute wstring headerStrCenter; */
    pub get_headerStrCenter: unsafe extern "C" fn (this: *const nsIPrintSettings, aHeaderStrCenter: *mut *const libc::int16_t) -> nsresult,
    pub set_headerStrCenter: unsafe extern "C" fn (this: *const nsIPrintSettings, aHeaderStrCenter: *const libc::int16_t) -> nsresult,

    /* attribute wstring headerStrRight; */
    pub get_headerStrRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aHeaderStrRight: *mut *const libc::int16_t) -> nsresult,
    pub set_headerStrRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aHeaderStrRight: *const libc::int16_t) -> nsresult,

    /* attribute wstring footerStrLeft; */
    pub get_footerStrLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aFooterStrLeft: *mut *const libc::int16_t) -> nsresult,
    pub set_footerStrLeft: unsafe extern "C" fn (this: *const nsIPrintSettings, aFooterStrLeft: *const libc::int16_t) -> nsresult,

    /* attribute wstring footerStrCenter; */
    pub get_footerStrCenter: unsafe extern "C" fn (this: *const nsIPrintSettings, aFooterStrCenter: *mut *const libc::int16_t) -> nsresult,
    pub set_footerStrCenter: unsafe extern "C" fn (this: *const nsIPrintSettings, aFooterStrCenter: *const libc::int16_t) -> nsresult,

    /* attribute wstring footerStrRight; */
    pub get_footerStrRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aFooterStrRight: *mut *const libc::int16_t) -> nsresult,
    pub set_footerStrRight: unsafe extern "C" fn (this: *const nsIPrintSettings, aFooterStrRight: *const libc::int16_t) -> nsresult,

    /* attribute short howToEnableFrameUI; */
    pub get_howToEnableFrameUI: unsafe extern "C" fn (this: *const nsIPrintSettings, aHowToEnableFrameUI: *mut libc::int16_t) -> nsresult,
    pub set_howToEnableFrameUI: unsafe extern "C" fn (this: *const nsIPrintSettings, aHowToEnableFrameUI: libc::int16_t) -> nsresult,

    /* attribute boolean isCancelled; */
    pub get_isCancelled: unsafe extern "C" fn (this: *const nsIPrintSettings, aIsCancelled: *mut bool) -> nsresult,
    pub set_isCancelled: unsafe extern "C" fn (this: *const nsIPrintSettings, aIsCancelled: bool) -> nsresult,

    /* attribute short printFrameTypeUsage; */
    pub get_printFrameTypeUsage: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintFrameTypeUsage: *mut libc::int16_t) -> nsresult,
    pub set_printFrameTypeUsage: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintFrameTypeUsage: libc::int16_t) -> nsresult,

    /* attribute short printFrameType; */
    pub get_printFrameType: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintFrameType: *mut libc::int16_t) -> nsresult,
    pub set_printFrameType: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintFrameType: libc::int16_t) -> nsresult,

    /* attribute boolean printSilent; */
    pub get_printSilent: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintSilent: *mut bool) -> nsresult,
    pub set_printSilent: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintSilent: bool) -> nsresult,

    /* attribute boolean shrinkToFit; */
    pub get_shrinkToFit: unsafe extern "C" fn (this: *const nsIPrintSettings, aShrinkToFit: *mut bool) -> nsresult,
    pub set_shrinkToFit: unsafe extern "C" fn (this: *const nsIPrintSettings, aShrinkToFit: bool) -> nsresult,

    /* attribute boolean showPrintProgress; */
    pub get_showPrintProgress: unsafe extern "C" fn (this: *const nsIPrintSettings, aShowPrintProgress: *mut bool) -> nsresult,
    pub set_showPrintProgress: unsafe extern "C" fn (this: *const nsIPrintSettings, aShowPrintProgress: bool) -> nsresult,

    /* attribute wstring paperName; */
    pub get_paperName: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperName: *mut *const libc::int16_t) -> nsresult,
    pub set_paperName: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperName: *const libc::int16_t) -> nsresult,

    /* attribute short paperData; */
    pub get_paperData: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperData: *mut libc::int16_t) -> nsresult,
    pub set_paperData: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperData: libc::int16_t) -> nsresult,

    /* attribute double paperWidth; */
    pub get_paperWidth: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperWidth: *mut libc::c_double) -> nsresult,
    pub set_paperWidth: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperWidth: libc::c_double) -> nsresult,

    /* attribute double paperHeight; */
    pub get_paperHeight: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperHeight: *mut libc::c_double) -> nsresult,
    pub set_paperHeight: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperHeight: libc::c_double) -> nsresult,

    /* attribute short paperSizeUnit; */
    pub get_paperSizeUnit: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperSizeUnit: *mut libc::int16_t) -> nsresult,
    pub set_paperSizeUnit: unsafe extern "C" fn (this: *const nsIPrintSettings, aPaperSizeUnit: libc::int16_t) -> nsresult,

    /* attribute boolean printReversed; */
    pub get_printReversed: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintReversed: *mut bool) -> nsresult,
    pub set_printReversed: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintReversed: bool) -> nsresult,

    /* attribute boolean printInColor; */
    pub get_printInColor: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintInColor: *mut bool) -> nsresult,
    pub set_printInColor: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintInColor: bool) -> nsresult,

    /* attribute long orientation; */
    pub get_orientation: unsafe extern "C" fn (this: *const nsIPrintSettings, aOrientation: *mut libc::int32_t) -> nsresult,
    pub set_orientation: unsafe extern "C" fn (this: *const nsIPrintSettings, aOrientation: libc::int32_t) -> nsresult,

    /* attribute long numCopies; */
    pub get_numCopies: unsafe extern "C" fn (this: *const nsIPrintSettings, aNumCopies: *mut libc::int32_t) -> nsresult,
    pub set_numCopies: unsafe extern "C" fn (this: *const nsIPrintSettings, aNumCopies: libc::int32_t) -> nsresult,

    /* attribute wstring printerName; */
    pub get_printerName: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrinterName: *mut *const libc::int16_t) -> nsresult,
    pub set_printerName: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrinterName: *const libc::int16_t) -> nsresult,

    /* attribute boolean printToFile; */
    pub get_printToFile: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintToFile: *mut bool) -> nsresult,
    pub set_printToFile: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintToFile: bool) -> nsresult,

    /* attribute wstring toFileName; */
    pub get_toFileName: unsafe extern "C" fn (this: *const nsIPrintSettings, aToFileName: *mut *const libc::int16_t) -> nsresult,
    pub set_toFileName: unsafe extern "C" fn (this: *const nsIPrintSettings, aToFileName: *const libc::int16_t) -> nsresult,

    /* attribute short outputFormat; */
    pub get_outputFormat: unsafe extern "C" fn (this: *const nsIPrintSettings, aOutputFormat: *mut libc::int16_t) -> nsresult,
    pub set_outputFormat: unsafe extern "C" fn (this: *const nsIPrintSettings, aOutputFormat: libc::int16_t) -> nsresult,

    /* attribute long printPageDelay; */
    pub get_printPageDelay: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintPageDelay: *mut libc::int32_t) -> nsresult,
    pub set_printPageDelay: unsafe extern "C" fn (this: *const nsIPrintSettings, aPrintPageDelay: libc::int32_t) -> nsresult,

    /* attribute long resolution; */
    pub get_resolution: unsafe extern "C" fn (this: *const nsIPrintSettings, aResolution: *mut libc::int32_t) -> nsresult,
    pub set_resolution: unsafe extern "C" fn (this: *const nsIPrintSettings, aResolution: libc::int32_t) -> nsresult,

    /* attribute long duplex; */
    pub get_duplex: unsafe extern "C" fn (this: *const nsIPrintSettings, aDuplex: *mut libc::int32_t) -> nsresult,
    pub set_duplex: unsafe extern "C" fn (this: *const nsIPrintSettings, aDuplex: libc::int32_t) -> nsresult,

    /* attribute boolean isInitializedFromPrinter; */
    pub get_isInitializedFromPrinter: unsafe extern "C" fn (this: *const nsIPrintSettings, aIsInitializedFromPrinter: *mut bool) -> nsresult,
    pub set_isInitializedFromPrinter: unsafe extern "C" fn (this: *const nsIPrintSettings, aIsInitializedFromPrinter: bool) -> nsresult,

    /* attribute boolean isInitializedFromPrefs; */
    pub get_isInitializedFromPrefs: unsafe extern "C" fn (this: *const nsIPrintSettings, aIsInitializedFromPrefs: *mut bool) -> nsresult,
    pub set_isInitializedFromPrefs: unsafe extern "C" fn (this: *const nsIPrintSettings, aIsInitializedFromPrefs: bool) -> nsresult,

    /* [noscript] void SetMarginInTwips (in nsNativeIntMarginRef aMargin); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetMarginInTwips: *const ::libc::c_void,

    /* [noscript] void SetEdgeInTwips (in nsNativeIntMarginRef aEdge); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetEdgeInTwips: *const ::libc::c_void,

    /* [noscript] void GetMarginInTwips (in nsNativeIntMarginRef aMargin); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetMarginInTwips: *const ::libc::c_void,

    /* [noscript] void GetEdgeInTwips (in nsNativeIntMarginRef aEdge); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetEdgeInTwips: *const ::libc::c_void,

    /* [noscript] void SetupSilentPrinting (); */
    pub SetupSilentPrinting: unsafe extern "C" fn (this: *const nsIPrintSettings) -> nsresult,

    /* [noscript] void SetUnwriteableMarginInTwips (in nsNativeIntMarginRef aEdge); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetUnwriteableMarginInTwips: *const ::libc::c_void,

    /* [noscript] void GetUnwriteableMarginInTwips (in nsNativeIntMarginRef aEdge); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetUnwriteableMarginInTwips: *const ::libc::c_void,

    /* [noscript] void GetPageRanges (in IntegerArray aPages); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetPageRanges: *const ::libc::c_void,

}


impl nsIPrintSettings {
    /* void SetPrintOptions (in int32_t aType, in boolean aTurnOnOff); */
    #[inline]
    pub unsafe fn SetPrintOptions(&self, aType: int32_t, aTurnOnOff: bool) -> Result<(), nsresult> {

        match ((*self.vtable).SetPrintOptions)(self as *const _, aType, aTurnOnOff) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean GetPrintOptions (in int32_t aType); */
    #[inline]
    pub unsafe fn GetPrintOptions(&self, aType: int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).GetPrintOptions)(self as *const _, aType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* int32_t GetPrintOptionsBits (); */
    #[inline]
    pub unsafe fn GetPrintOptionsBits(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetPrintOptionsBits)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void SetPrintOptionsBits (in int32_t bits); */
    #[inline]
    pub unsafe fn SetPrintOptionsBits(&self, bits: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).SetPrintOptionsBits)(self as *const _, bits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void GetEffectivePageSize (out double aWidth, out double aHeight); */
    #[inline]
    pub unsafe fn GetEffectivePageSize(&self, ) -> Result<(libc::c_double, libc::c_double), nsresult> {
        let mut aWidth: libc::c_double = ::std::mem::zeroed();
        let mut aHeight: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).GetEffectivePageSize)(self as *const _, &mut aWidth as *mut _, &mut aHeight as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aWidth, aHeight))
    }

    /* nsIPrintSettings clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsIPrintSettings>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void assign (in nsIPrintSettings aPS); */
    #[inline]
    pub unsafe fn assign(&self, aPS: Option<&nsIPrintSettings>) -> Result<(), nsresult> {

        match ((*self.vtable).assign)(self as *const _, aPS.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute nsIPrintSession printSession; */
    #[inline]
    pub unsafe fn get_printSession(&self, ) -> Result<Option<RefPtr<nsIPrintSession>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_printSession)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_printSession(&self, aPrintSession: Option<&nsIPrintSession>) -> Result<(), nsresult> {

        match ((*self.vtable).set_printSession)(self as *const _, aPrintSession.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long startPageRange; */
    #[inline]
    pub unsafe fn get_startPageRange(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_startPageRange)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_startPageRange(&self, aStartPageRange: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_startPageRange)(self as *const _, aStartPageRange) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long endPageRange; */
    #[inline]
    pub unsafe fn get_endPageRange(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_endPageRange)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_endPageRange(&self, aEndPageRange: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_endPageRange)(self as *const _, aEndPageRange) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double edgeTop; */
    #[inline]
    pub unsafe fn get_edgeTop(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_edgeTop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_edgeTop(&self, aEdgeTop: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_edgeTop)(self as *const _, aEdgeTop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double edgeLeft; */
    #[inline]
    pub unsafe fn get_edgeLeft(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_edgeLeft)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_edgeLeft(&self, aEdgeLeft: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_edgeLeft)(self as *const _, aEdgeLeft) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double edgeBottom; */
    #[inline]
    pub unsafe fn get_edgeBottom(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_edgeBottom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_edgeBottom(&self, aEdgeBottom: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_edgeBottom)(self as *const _, aEdgeBottom) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double edgeRight; */
    #[inline]
    pub unsafe fn get_edgeRight(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_edgeRight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_edgeRight(&self, aEdgeRight: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_edgeRight)(self as *const _, aEdgeRight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double marginTop; */
    #[inline]
    pub unsafe fn get_marginTop(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_marginTop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_marginTop(&self, aMarginTop: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_marginTop)(self as *const _, aMarginTop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double marginLeft; */
    #[inline]
    pub unsafe fn get_marginLeft(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_marginLeft)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_marginLeft(&self, aMarginLeft: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_marginLeft)(self as *const _, aMarginLeft) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double marginBottom; */
    #[inline]
    pub unsafe fn get_marginBottom(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_marginBottom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_marginBottom(&self, aMarginBottom: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_marginBottom)(self as *const _, aMarginBottom) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double marginRight; */
    #[inline]
    pub unsafe fn get_marginRight(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_marginRight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_marginRight(&self, aMarginRight: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_marginRight)(self as *const _, aMarginRight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double unwriteableMarginTop; */
    #[inline]
    pub unsafe fn get_unwriteableMarginTop(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_unwriteableMarginTop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_unwriteableMarginTop(&self, aUnwriteableMarginTop: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_unwriteableMarginTop)(self as *const _, aUnwriteableMarginTop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double unwriteableMarginLeft; */
    #[inline]
    pub unsafe fn get_unwriteableMarginLeft(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_unwriteableMarginLeft)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_unwriteableMarginLeft(&self, aUnwriteableMarginLeft: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_unwriteableMarginLeft)(self as *const _, aUnwriteableMarginLeft) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double unwriteableMarginBottom; */
    #[inline]
    pub unsafe fn get_unwriteableMarginBottom(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_unwriteableMarginBottom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_unwriteableMarginBottom(&self, aUnwriteableMarginBottom: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_unwriteableMarginBottom)(self as *const _, aUnwriteableMarginBottom) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double unwriteableMarginRight; */
    #[inline]
    pub unsafe fn get_unwriteableMarginRight(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_unwriteableMarginRight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_unwriteableMarginRight(&self, aUnwriteableMarginRight: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_unwriteableMarginRight)(self as *const _, aUnwriteableMarginRight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double scaling; */
    #[inline]
    pub unsafe fn get_scaling(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_scaling)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_scaling(&self, aScaling: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_scaling)(self as *const _, aScaling) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean printBGColors; */
    #[inline]
    pub unsafe fn get_printBGColors(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_printBGColors)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printBGColors(&self, aPrintBGColors: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_printBGColors)(self as *const _, aPrintBGColors) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean printBGImages; */
    #[inline]
    pub unsafe fn get_printBGImages(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_printBGImages)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printBGImages(&self, aPrintBGImages: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_printBGImages)(self as *const _, aPrintBGImages) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short printRange; */
    #[inline]
    pub unsafe fn get_printRange(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_printRange)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printRange(&self, aPrintRange: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_printRange)(self as *const _, aPrintRange) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_title)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_title(&self, aTitle: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_title)(self as *const _, aTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring docURL; */
    #[inline]
    pub unsafe fn get_docURL(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_docURL)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_docURL(&self, aDocURL: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_docURL)(self as *const _, aDocURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring headerStrLeft; */
    #[inline]
    pub unsafe fn get_headerStrLeft(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_headerStrLeft)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_headerStrLeft(&self, aHeaderStrLeft: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_headerStrLeft)(self as *const _, aHeaderStrLeft) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring headerStrCenter; */
    #[inline]
    pub unsafe fn get_headerStrCenter(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_headerStrCenter)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_headerStrCenter(&self, aHeaderStrCenter: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_headerStrCenter)(self as *const _, aHeaderStrCenter) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring headerStrRight; */
    #[inline]
    pub unsafe fn get_headerStrRight(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_headerStrRight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_headerStrRight(&self, aHeaderStrRight: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_headerStrRight)(self as *const _, aHeaderStrRight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring footerStrLeft; */
    #[inline]
    pub unsafe fn get_footerStrLeft(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_footerStrLeft)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_footerStrLeft(&self, aFooterStrLeft: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_footerStrLeft)(self as *const _, aFooterStrLeft) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring footerStrCenter; */
    #[inline]
    pub unsafe fn get_footerStrCenter(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_footerStrCenter)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_footerStrCenter(&self, aFooterStrCenter: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_footerStrCenter)(self as *const _, aFooterStrCenter) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring footerStrRight; */
    #[inline]
    pub unsafe fn get_footerStrRight(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_footerStrRight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_footerStrRight(&self, aFooterStrRight: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_footerStrRight)(self as *const _, aFooterStrRight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short howToEnableFrameUI; */
    #[inline]
    pub unsafe fn get_howToEnableFrameUI(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_howToEnableFrameUI)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_howToEnableFrameUI(&self, aHowToEnableFrameUI: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_howToEnableFrameUI)(self as *const _, aHowToEnableFrameUI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isCancelled; */
    #[inline]
    pub unsafe fn get_isCancelled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isCancelled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isCancelled(&self, aIsCancelled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isCancelled)(self as *const _, aIsCancelled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short printFrameTypeUsage; */
    #[inline]
    pub unsafe fn get_printFrameTypeUsage(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_printFrameTypeUsage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printFrameTypeUsage(&self, aPrintFrameTypeUsage: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_printFrameTypeUsage)(self as *const _, aPrintFrameTypeUsage) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short printFrameType; */
    #[inline]
    pub unsafe fn get_printFrameType(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_printFrameType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printFrameType(&self, aPrintFrameType: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_printFrameType)(self as *const _, aPrintFrameType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean printSilent; */
    #[inline]
    pub unsafe fn get_printSilent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_printSilent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printSilent(&self, aPrintSilent: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_printSilent)(self as *const _, aPrintSilent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean shrinkToFit; */
    #[inline]
    pub unsafe fn get_shrinkToFit(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_shrinkToFit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_shrinkToFit(&self, aShrinkToFit: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_shrinkToFit)(self as *const _, aShrinkToFit) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean showPrintProgress; */
    #[inline]
    pub unsafe fn get_showPrintProgress(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showPrintProgress)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showPrintProgress(&self, aShowPrintProgress: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showPrintProgress)(self as *const _, aShowPrintProgress) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring paperName; */
    #[inline]
    pub unsafe fn get_paperName(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_paperName)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_paperName(&self, aPaperName: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_paperName)(self as *const _, aPaperName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short paperData; */
    #[inline]
    pub unsafe fn get_paperData(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_paperData)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_paperData(&self, aPaperData: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_paperData)(self as *const _, aPaperData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double paperWidth; */
    #[inline]
    pub unsafe fn get_paperWidth(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_paperWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_paperWidth(&self, aPaperWidth: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_paperWidth)(self as *const _, aPaperWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double paperHeight; */
    #[inline]
    pub unsafe fn get_paperHeight(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_paperHeight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_paperHeight(&self, aPaperHeight: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_paperHeight)(self as *const _, aPaperHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short paperSizeUnit; */
    #[inline]
    pub unsafe fn get_paperSizeUnit(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_paperSizeUnit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_paperSizeUnit(&self, aPaperSizeUnit: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_paperSizeUnit)(self as *const _, aPaperSizeUnit) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean printReversed; */
    #[inline]
    pub unsafe fn get_printReversed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_printReversed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printReversed(&self, aPrintReversed: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_printReversed)(self as *const _, aPrintReversed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean printInColor; */
    #[inline]
    pub unsafe fn get_printInColor(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_printInColor)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printInColor(&self, aPrintInColor: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_printInColor)(self as *const _, aPrintInColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long orientation; */
    #[inline]
    pub unsafe fn get_orientation(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_orientation)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_orientation(&self, aOrientation: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_orientation)(self as *const _, aOrientation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long numCopies; */
    #[inline]
    pub unsafe fn get_numCopies(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numCopies)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_numCopies(&self, aNumCopies: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_numCopies)(self as *const _, aNumCopies) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring printerName; */
    #[inline]
    pub unsafe fn get_printerName(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_printerName)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printerName(&self, aPrinterName: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_printerName)(self as *const _, aPrinterName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean printToFile; */
    #[inline]
    pub unsafe fn get_printToFile(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_printToFile)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printToFile(&self, aPrintToFile: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_printToFile)(self as *const _, aPrintToFile) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring toFileName; */
    #[inline]
    pub unsafe fn get_toFileName(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_toFileName)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_toFileName(&self, aToFileName: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_toFileName)(self as *const _, aToFileName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short outputFormat; */
    #[inline]
    pub unsafe fn get_outputFormat(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_outputFormat)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_outputFormat(&self, aOutputFormat: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_outputFormat)(self as *const _, aOutputFormat) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long printPageDelay; */
    #[inline]
    pub unsafe fn get_printPageDelay(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_printPageDelay)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_printPageDelay(&self, aPrintPageDelay: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_printPageDelay)(self as *const _, aPrintPageDelay) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long resolution; */
    #[inline]
    pub unsafe fn get_resolution(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_resolution)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_resolution(&self, aResolution: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_resolution)(self as *const _, aResolution) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long duplex; */
    #[inline]
    pub unsafe fn get_duplex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_duplex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_duplex(&self, aDuplex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_duplex)(self as *const _, aDuplex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isInitializedFromPrinter; */
    #[inline]
    pub unsafe fn get_isInitializedFromPrinter(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInitializedFromPrinter)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isInitializedFromPrinter(&self, aIsInitializedFromPrinter: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isInitializedFromPrinter)(self as *const _, aIsInitializedFromPrinter) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isInitializedFromPrefs; */
    #[inline]
    pub unsafe fn get_isInitializedFromPrefs(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInitializedFromPrefs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isInitializedFromPrefs(&self, aIsInitializedFromPrefs: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isInitializedFromPrefs)(self as *const _, aIsInitializedFromPrefs) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void SetMarginInTwips (in nsNativeIntMarginRef aMargin); */


    /* [noscript] void SetEdgeInTwips (in nsNativeIntMarginRef aEdge); */


    /* [noscript] void GetMarginInTwips (in nsNativeIntMarginRef aMargin); */


    /* [noscript] void GetEdgeInTwips (in nsNativeIntMarginRef aEdge); */


    /* [noscript] void SetupSilentPrinting (); */
    #[inline]
    pub unsafe fn SetupSilentPrinting(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).SetupSilentPrinting)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void SetUnwriteableMarginInTwips (in nsNativeIntMarginRef aEdge); */


    /* [noscript] void GetUnwriteableMarginInTwips (in nsNativeIntMarginRef aEdge); */


    /* [noscript] void GetPageRanges (in IntegerArray aPages); */


}


