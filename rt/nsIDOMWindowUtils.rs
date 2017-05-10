//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWindowUtils.idl
//


pub type nsViewID = libc::uint64_t;


pub mod nsIDOMWindowUtils_consts {
    pub const MODIFIER_ALT: i64 = 1;
    pub const MODIFIER_CONTROL: i64 = 2;
    pub const MODIFIER_SHIFT: i64 = 4;
    pub const MODIFIER_META: i64 = 8;
    pub const MODIFIER_ALTGRAPH: i64 = 16;
    pub const MODIFIER_CAPSLOCK: i64 = 32;
    pub const MODIFIER_FN: i64 = 64;
    pub const MODIFIER_FNLOCK: i64 = 128;
    pub const MODIFIER_NUMLOCK: i64 = 256;
    pub const MODIFIER_SCROLLLOCK: i64 = 512;
    pub const MODIFIER_SYMBOL: i64 = 1024;
    pub const MODIFIER_SYMBOLLOCK: i64 = 2048;
    pub const MODIFIER_OS: i64 = 4096;
    pub const WHEEL_EVENT_CAUSED_BY_NO_LINE_OR_PAGE_DELTA_DEVICE: i64 = 1;
    pub const WHEEL_EVENT_CAUSED_BY_PIXEL_ONLY_DEVICE: i64 = 1;
    pub const WHEEL_EVENT_CAUSED_BY_MOMENTUM: i64 = 2;
    pub const WHEEL_EVENT_CUSTOMIZED_BY_USER_PREFS: i64 = 4;
    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_ZERO: i64 = 16;
    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_POSITIVE: i64 = 32;
    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_NEGATIVE: i64 = 64;
    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_ZERO: i64 = 256;
    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_POSITIVE: i64 = 512;
    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_NEGATIVE: i64 = 1024;
    pub const KEY_FLAG_PREVENT_DEFAULT: i64 = 1;
    pub const KEY_FLAG_NOT_SYNTHESIZED_FOR_TESTS: i64 = 2;
    pub const KEY_FLAG_LOCATION_STANDARD: i64 = 16;
    pub const KEY_FLAG_LOCATION_LEFT: i64 = 32;
    pub const KEY_FLAG_LOCATION_RIGHT: i64 = 64;
    pub const KEY_FLAG_LOCATION_NUMPAD: i64 = 128;
    pub const MOUSESCROLL_PREFER_WIDGET_AT_POINT: i64 = 1;
    pub const MOUSESCROLL_SCROLL_LINES: i64 = 2;
    pub const MOUSESCROLL_WIN_SCROLL_LPARAM_NOT_NULL: i64 = 65536;
    pub const TOUCH_HOVER: i64 = 1;
    pub const TOUCH_CONTACT: i64 = 2;
    pub const TOUCH_REMOVE: i64 = 4;
    pub const TOUCH_CANCEL: i64 = 8;
    pub const IME_STATUS_DISABLED: i64 = 0;
    pub const IME_STATUS_ENABLED: i64 = 1;
    pub const IME_STATUS_PASSWORD: i64 = 2;
    pub const IME_STATUS_PLUGIN: i64 = 3;
    pub const QUERY_CONTENT_FLAG_USE_NATIVE_LINE_BREAK: i64 = 0;
    pub const QUERY_CONTENT_FLAG_USE_XP_LINE_BREAK: i64 = 1;
    pub const QUERY_CONTENT_FLAG_SELECTION_SPELLCHECK: i64 = 2;
    pub const QUERY_CONTENT_FLAG_SELECTION_IME_RAWINPUT: i64 = 4;
    pub const QUERY_CONTENT_FLAG_SELECTION_IME_SELECTEDRAWTEXT: i64 = 8;
    pub const QUERY_CONTENT_FLAG_SELECTION_IME_CONVERTEDTEXT: i64 = 16;
    pub const QUERY_CONTENT_FLAG_SELECTION_IME_SELECTEDCONVERTEDTEXT: i64 = 32;
    pub const QUERY_CONTENT_FLAG_SELECTION_ACCESSIBILITY: i64 = 64;
    pub const QUERY_CONTENT_FLAG_SELECTION_FIND: i64 = 128;
    pub const QUERY_CONTENT_FLAG_SELECTION_URLSECONDARY: i64 = 256;
    pub const QUERY_CONTENT_FLAG_SELECTION_URLSTRIKEOUT: i64 = 512;
    pub const QUERY_CONTENT_FLAG_OFFSET_RELATIVE_TO_INSERTION_POINT: i64 = 1024;
    pub const QUERY_SELECTED_TEXT: i64 = 3200;
    pub const QUERY_TEXT_CONTENT: i64 = 3201;
    pub const QUERY_CARET_RECT: i64 = 3203;
    pub const QUERY_TEXT_RECT: i64 = 3204;
    pub const QUERY_EDITOR_RECT: i64 = 3205;
    pub const QUERY_CHARACTER_AT_POINT: i64 = 3208;
    pub const QUERY_TEXT_RECT_ARRAY: i64 = 3209;
    pub const SELECTION_SET_FLAG_USE_NATIVE_LINE_BREAK: i64 = 0;
    pub const SELECTION_SET_FLAG_USE_XP_LINE_BREAK: i64 = 1;
    pub const SELECTION_SET_FLAG_REVERSE: i64 = 2;
    pub const SELECT_CHARACTER: i64 = 0;
    pub const SELECT_CLUSTER: i64 = 1;
    pub const SELECT_WORD: i64 = 2;
    pub const SELECT_LINE: i64 = 3;
    pub const SELECT_BEGINLINE: i64 = 4;
    pub const SELECT_ENDLINE: i64 = 5;
    pub const SELECT_PARAGRAPH: i64 = 6;
    pub const SELECT_WORDNOSPACE: i64 = 7;
    pub const AGENT_SHEET: i64 = 0;
    pub const USER_SHEET: i64 = 1;
    pub const AUTHOR_SHEET: i64 = 2;
    pub const DEFAULT_MOUSE_POINTER_ID: i64 = 0;
    pub const DEFAULT_PEN_POINTER_ID: i64 = 1;
    pub const DEFAULT_TOUCH_POINTER_ID: i64 = 2;
    pub const MOUSE_BUTTON_LEFT_BUTTON: i64 = 0;
    pub const MOUSE_BUTTON_MIDDLE_BUTTON: i64 = 1;
    pub const MOUSE_BUTTON_RIGHT_BUTTON: i64 = 2;
    pub const MOUSE_BUTTONS_NO_BUTTON: i64 = 0;
    pub const MOUSE_BUTTONS_LEFT_BUTTON: i64 = 1;
    pub const MOUSE_BUTTONS_RIGHT_BUTTON: i64 = 2;
    pub const MOUSE_BUTTONS_MIDDLE_BUTTON: i64 = 4;
    pub const MOUSE_BUTTONS_4TH_BUTTON: i64 = 8;
    pub const MOUSE_BUTTONS_5TH_BUTTON: i64 = 16;
    pub const MOUSE_BUTTONS_NOT_SPECIFIED: i64 = -1;
}


#[repr(C)]
pub struct nsIDOMWindowUtils {
    vtable: *const nsIDOMWindowUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMWindowUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc471d440, 0x004b, 0x4c50,
            [0xa6, 0xf2, 0x74, 0x7d, 0xb5, 0xf4, 0x43, 0xb6])
    }
}

unsafe impl RefCounted for nsIDOMWindowUtils {
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
pub trait nsIDOMWindowUtilsCoerce {
    fn coerce_from(v: &nsIDOMWindowUtils) -> &Self;
}

impl nsIDOMWindowUtilsCoerce for nsIDOMWindowUtils {
    #[inline]
    fn coerce_from(v: &nsIDOMWindowUtils) -> &Self {
        v
    }
}

impl nsIDOMWindowUtils {
    #[inline]
    pub fn coerce<T: nsIDOMWindowUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMWindowUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMWindowUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMWindowUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMWindowUtilsVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned short imageAnimationMode; */
    pub get_imageAnimationMode: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aImageAnimationMode: *mut libc::uint16_t) -> nsresult,
    pub set_imageAnimationMode: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aImageAnimationMode: libc::uint16_t) -> nsresult,

    /* readonly attribute boolean docCharsetIsForced; */
    pub get_docCharsetIsForced: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aDocCharsetIsForced: *mut bool) -> nsresult,

    /* short getCursorType (); */
    pub getCursorType: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, _retval: *mut libc::int16_t) -> nsresult,

    /* AString getDocumentMetadata (in AString aName); */
    pub getDocumentMetadata: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* unsigned long redraw ([optional] in unsigned long aCount); */
    pub redraw: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aCount: libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* void updateLayerTree (); */
    pub updateLayerTree: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* readonly attribute unsigned long long lastTransactionId; */
    pub get_lastTransactionId: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aLastTransactionId: *mut libc::uint64_t) -> nsresult,

    /* void getViewportInfo (in uint32_t aDisplayWidth, in uint32_t aDisplayHeight, out double aDefaultZoom, out boolean aAllowZoom, out double aMinZoom, out double aMaxZoom, out uint32_t aWidth, out uint32_t aHeight, out boolean aAutoSize); */
    pub getViewportInfo: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aDisplayWidth: uint32_t, aDisplayHeight: uint32_t, aDefaultZoom: *mut libc::c_double, aAllowZoom: *mut bool, aMinZoom: *mut libc::c_double, aMaxZoom: *mut libc::c_double, aWidth: *mut uint32_t, aHeight: *mut uint32_t, aAutoSize: *mut bool) -> nsresult,

    /* void getContentViewerSize (out uint32_t aDisplayWidth, out uint32_t aDisplayHeight); */
    pub getContentViewerSize: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aDisplayWidth: *mut uint32_t, aDisplayHeight: *mut uint32_t) -> nsresult,

    /* void setDisplayPortForElement (in float aXPx, in float aYPx, in float aWidthPx, in float aHeightPx, in nsIDOMElement aElement, in uint32_t aPriority); */
    pub setDisplayPortForElement: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aXPx: libc::c_float, aYPx: libc::c_float, aWidthPx: libc::c_float, aHeightPx: libc::c_float, aElement: *const nsIDOMElement, aPriority: uint32_t) -> nsresult,

    /* void setDisplayPortMarginsForElement (in float aLeftMargin, in float aTopMargin, in float aRightMargin, in float aBottomMargin, in nsIDOMElement aElement, in uint32_t aPriority); */
    pub setDisplayPortMarginsForElement: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aLeftMargin: libc::c_float, aTopMargin: libc::c_float, aRightMargin: libc::c_float, aBottomMargin: libc::c_float, aElement: *const nsIDOMElement, aPriority: uint32_t) -> nsresult,

    /* void setDisplayPortBaseForElement (in int32_t aX, in int32_t aY, in int32_t aWidth, in int32_t aHeight, in nsIDOMElement aElement); */
    pub setDisplayPortBaseForElement: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aX: int32_t, aY: int32_t, aWidth: int32_t, aHeight: int32_t, aElement: *const nsIDOMElement) -> nsresult,

    /* void setResolution (in float aResolution); */
    pub setResolution: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aResolution: libc::c_float) -> nsresult,

    /* void getResolution (out float aResolution); */
    pub getResolution: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aResolution: *mut libc::c_float) -> nsresult,

    /* void setResolutionAndScaleTo (in float aResolution); */
    pub setResolutionAndScaleTo: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aResolution: libc::c_float) -> nsresult,

    /* void setRestoreResolution (in float aResolution, in uint32_t aDisplayWidth, in uint32_t aDisplayHeight); */
    pub setRestoreResolution: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aResolution: libc::c_float, aDisplayWidth: uint32_t, aDisplayHeight: uint32_t) -> nsresult,

    /* readonly attribute boolean isResolutionSet; */
    pub get_isResolutionSet: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIsResolutionSet: *mut bool) -> nsresult,

    /* attribute boolean isFirstPaint; */
    pub get_isFirstPaint: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIsFirstPaint: *mut bool) -> nsresult,
    pub set_isFirstPaint: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIsFirstPaint: bool) -> nsresult,

    /* void getPresShellId (out uint32_t aPresShellId); */
    pub getPresShellId: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aPresShellId: *mut uint32_t) -> nsresult,

    /* [optional_argc] boolean sendMouseEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendMouseEvent: *const ::libc::c_void,

    /* [optional_argc] boolean sendPointerEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in long aPointerId, [optional] in long aWidth, [optional] in long aHeight, [optional] in long aTiltX, [optional] in long aTiltY, [optional] in boolean aIsPrimary, [optional] in boolean aIsSynthesized); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendPointerEvent: *const ::libc::c_void,

    /* boolean sendTouchEvent (in AString aType, [array, size_is (count)] in uint32_t aIdentifiers, [array, size_is (count)] in int32_t aXs, [array, size_is (count)] in int32_t aYs, [array, size_is (count)] in uint32_t aRxs, [array, size_is (count)] in uint32_t aRys, [array, size_is (count)] in float aRotationAngles, [array, size_is (count)] in float aForces, in uint32_t count, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendTouchEvent: *const ::libc::c_void,

    /* [optional_argc] void sendMouseEventToWindow (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendMouseEventToWindow: *const ::libc::c_void,

    /* [optional_argc] void sendPointerEventToWindow (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in long aPointerId, [optional] in long aWidth, [optional] in long aHeight, [optional] in long aTiltX, [optional] in long aTiltY, [optional] in boolean aIsPrimary, [optional] in boolean aIsSynthesized); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendPointerEventToWindow: *const ::libc::c_void,

    /* boolean sendTouchEventToWindow (in AString aType, [array, size_is (count)] in uint32_t aIdentifiers, [array, size_is (count)] in int32_t aXs, [array, size_is (count)] in int32_t aYs, [array, size_is (count)] in uint32_t aRxs, [array, size_is (count)] in uint32_t aRys, [array, size_is (count)] in float aRotationAngles, [array, size_is (count)] in float aForces, in uint32_t count, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendTouchEventToWindow: *const ::libc::c_void,

    /* void sendWheelEvent (in float aX, in float aY, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aDeltaMode, in long aModifiers, in long aLineOrPageDeltaX, in long aLineOrPageDeltaY, in unsigned long aOptions); */
    pub sendWheelEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aDeltaX: libc::c_double, aDeltaY: libc::c_double, aDeltaZ: libc::c_double, aDeltaMode: libc::uint32_t, aModifiers: libc::int32_t, aLineOrPageDeltaX: libc::int32_t, aLineOrPageDeltaY: libc::int32_t, aOptions: libc::uint32_t) -> nsresult,

    /* boolean sendKeyEvent (in AString aType, in long aKeyCode, in long aCharCode, in long aModifiers, [optional] in unsigned long aAdditionalFlags); */
    pub sendKeyEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aType: *const nsAString, aKeyCode: libc::int32_t, aCharCode: libc::int32_t, aModifiers: libc::int32_t, aAdditionalFlags: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* void sendNativeKeyEvent (in long aNativeKeyboardLayout, in long aNativeKeyCode, in long aModifierFlags, in AString aCharacters, in AString aUnmodifiedCharacters, [optional] in nsIObserver aObserver); */
    pub sendNativeKeyEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aNativeKeyboardLayout: libc::int32_t, aNativeKeyCode: libc::int32_t, aModifierFlags: libc::int32_t, aCharacters: *const nsAString, aUnmodifiedCharacters: *const nsAString, aObserver: *const nsIObserver) -> nsresult,

    /* void sendNativeMouseEvent (in long aScreenX, in long aScreenY, in long aNativeMessage, in long aModifierFlags, in nsIDOMElement aElement, [optional] in nsIObserver aObserver); */
    pub sendNativeMouseEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aNativeMessage: libc::int32_t, aModifierFlags: libc::int32_t, aElement: *const nsIDOMElement, aObserver: *const nsIObserver) -> nsresult,

    /* void sendNativeMouseMove (in long aScreenX, in long aScreenY, in nsIDOMElement aElement, [optional] in nsIObserver aObserver); */
    pub sendNativeMouseMove: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aElement: *const nsIDOMElement, aObserver: *const nsIObserver) -> nsresult,

    /* void sendNativeMouseScrollEvent (in long aScreenX, in long aScreenY, in unsigned long aNativeMessage, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aModifierFlags, in unsigned long aAdditionalFlags, in nsIDOMElement aElement, [optional] in nsIObserver aObserver); */
    pub sendNativeMouseScrollEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aNativeMessage: libc::uint32_t, aDeltaX: libc::c_double, aDeltaY: libc::c_double, aDeltaZ: libc::c_double, aModifierFlags: libc::uint32_t, aAdditionalFlags: libc::uint32_t, aElement: *const nsIDOMElement, aObserver: *const nsIObserver) -> nsresult,

    /* void sendNativeTouchPoint (in unsigned long aPointerId, in unsigned long aTouchState, in long aScreenX, in long aScreenY, in double aPressure, in unsigned long aOrientation, [optional] in nsIObserver aObserver); */
    pub sendNativeTouchPoint: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aPointerId: libc::uint32_t, aTouchState: libc::uint32_t, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aPressure: libc::c_double, aOrientation: libc::uint32_t, aObserver: *const nsIObserver) -> nsresult,

    /* void sendNativeTouchTap (in long aScreenX, in long aScreenY, in boolean aLongTap, [optional] in nsIObserver aObserver); */
    pub sendNativeTouchTap: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aLongTap: bool, aObserver: *const nsIObserver) -> nsresult,

    /* void clearNativeTouchSequence ([optional] in nsIObserver aObserver); */
    pub clearNativeTouchSequence: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aObserver: *const nsIObserver) -> nsresult,

    /* void activateNativeMenuItemAt (in AString indexString); */
    pub activateNativeMenuItemAt: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, indexString: *const nsAString) -> nsresult,

    /* void forceUpdateNativeMenuAt (in AString indexString); */
    pub forceUpdateNativeMenuAt: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, indexString: *const nsAString) -> nsresult,

    /* AString GetSelectionAsPlaintext (); */
    pub GetSelectionAsPlaintext: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, _retval: *mut nsAString) -> nsresult,

    /* void focus (in nsIDOMElement aElement); */
    pub focus: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement) -> nsresult,

    /* void garbageCollect ([optional] in nsICycleCollectorListener aListener, [optional] in long aExtraForgetSkippableCalls); */
    pub garbageCollect: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aListener: *const nsICycleCollectorListener, aExtraForgetSkippableCalls: libc::int32_t) -> nsresult,

    /* void cycleCollect ([optional] in nsICycleCollectorListener aListener, [optional] in long aExtraForgetSkippableCalls); */
    pub cycleCollect: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aListener: *const nsICycleCollectorListener, aExtraForgetSkippableCalls: libc::int32_t) -> nsresult,

    /* void runNextCollectorTimer (); */
    pub runNextCollectorTimer: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* void sendSimpleGestureEvent (in AString aType, in float aX, in float aY, in unsigned long aDirection, in double aDelta, in long aModifiers, [optional] in unsigned long aClickCount); */
    pub sendSimpleGestureEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aType: *const nsAString, aX: libc::c_float, aY: libc::c_float, aDirection: libc::uint32_t, aDelta: libc::c_double, aModifiers: libc::int32_t, aClickCount: libc::uint32_t) -> nsresult,

    /* nsIDOMElement elementFromPoint (in float aX, in float aY, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout); */
    pub elementFromPoint: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aIgnoreRootScrollFrame: bool, aFlushLayout: bool, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMNodeList nodesFromRect (in float aX, in float aY, in float aTopSize, in float aRightSize, in float aBottomSize, in float aLeftSize, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout); */
    pub nodesFromRect: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aTopSize: libc::c_float, aRightSize: libc::c_float, aBottomSize: libc::c_float, aLeftSize: libc::c_float, aIgnoreRootScrollFrame: bool, aFlushLayout: bool, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* nsITranslationNodeList getTranslationNodes (in nsIDOMNode aRoot); */
    pub getTranslationNodes: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aRoot: *const nsIDOMNode, _retval: *mut *const nsITranslationNodeList) -> nsresult,

    /* uint32_t compareCanvases (in nsIDOMHTMLCanvasElement aCanvas1, in nsIDOMHTMLCanvasElement aCanvas2, out unsigned long aMaxDifference); */
    pub compareCanvases: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aCanvas1: *const nsIDOMHTMLCanvasElement, aCanvas2: *const nsIDOMHTMLCanvasElement, aMaxDifference: *mut libc::uint32_t, _retval: *mut uint32_t) -> nsresult,

    /* readonly attribute boolean isMozAfterPaintPending; */
    pub get_isMozAfterPaintPending: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIsMozAfterPaintPending: *mut bool) -> nsresult,

    /* void suppressEventHandling (in boolean aSuppress); */
    pub suppressEventHandling: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aSuppress: bool) -> nsresult,

    /* void disableNonTestMouseEvents (in boolean aDisable); */
    pub disableNonTestMouseEvents: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aDisable: bool) -> nsresult,

    /* void getScrollXY (in boolean aFlushLayout, out long aScrollX, out long aScrollY); */
    pub getScrollXY: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aFlushLayout: bool, aScrollX: *mut libc::int32_t, aScrollY: *mut libc::int32_t) -> nsresult,

    /* void getScrollXYFloat (in boolean aFlushLayout, out float aScrollX, out float aScrollY); */
    pub getScrollXYFloat: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aFlushLayout: bool, aScrollX: *mut libc::c_float, aScrollY: *mut libc::c_float) -> nsresult,

    /* void getScrollbarSize (in boolean aFlushLayout, out long aWidth, out long aHeight); */
    pub getScrollbarSize: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aFlushLayout: bool, aWidth: *mut libc::int32_t, aHeight: *mut libc::int32_t) -> nsresult,

    /* nsIDOMClientRect getBoundsWithoutFlushing (in nsIDOMElement aElement); */
    pub getBoundsWithoutFlushing: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement, _retval: *mut *const nsIDOMClientRect) -> nsresult,

    /* nsIDOMClientRect getRootBounds (); */
    pub getRootBounds: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, _retval: *mut *const nsIDOMClientRect) -> nsresult,

    /* readonly attribute boolean IMEIsOpen; */
    pub get_IMEIsOpen: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIMEIsOpen: *mut bool) -> nsresult,

    /* readonly attribute unsigned long IMEStatus; */
    pub get_IMEStatus: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIMEStatus: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute float screenPixelsPerCSSPixel; */
    pub get_screenPixelsPerCSSPixel: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aScreenPixelsPerCSSPixel: *mut libc::c_float) -> nsresult,

    /* readonly attribute float fullZoom; */
    pub get_fullZoom: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aFullZoom: *mut libc::c_float) -> nsresult,

    /* boolean dispatchDOMEventViaPresShell (in nsIDOMNode aTarget, in nsIDOMEvent aEvent, in boolean aTrusted); */
    pub dispatchDOMEventViaPresShell: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aTarget: *const nsIDOMNode, aEvent: *const nsIDOMEvent, aTrusted: bool, _retval: *mut bool) -> nsresult,

    /* boolean dispatchEventToChromeOnly (in nsIDOMEventTarget aTarget, in nsIDOMEvent aEvent); */
    pub dispatchEventToChromeOnly: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aTarget: *const nsIDOMEventTarget, aEvent: *const nsIDOMEvent, _retval: *mut bool) -> nsresult,

    /* [implicit_jscontext] string getClassName (in jsval aObject); */
    /// Unable to call function as its signature contains a non-rust type
    pub getClassName: *const ::libc::c_void,

    /* void sendContentCommandEvent (in AString aType, [optional] in nsITransferable aTransferable); */
    pub sendContentCommandEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aType: *const nsAString, aTransferable: *const nsITransferable) -> nsresult,

    /* nsIQueryContentEventResult sendQueryContentEvent (in unsigned long aType, in long long aOffset, in unsigned long aLength, in long aX, in long aY, [optional] in unsigned long aAdditionalFlags); */
    pub sendQueryContentEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aType: libc::uint32_t, aOffset: libc::int64_t, aLength: libc::uint32_t, aX: libc::int32_t, aY: libc::int32_t, aAdditionalFlags: libc::uint32_t, _retval: *mut *const nsIQueryContentEventResult) -> nsresult,

    /* void remoteFrameFullscreenChanged (in nsIDOMElement aFrameElement); */
    pub remoteFrameFullscreenChanged: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aFrameElement: *const nsIDOMElement) -> nsresult,

    /* void remoteFrameFullscreenReverted (); */
    pub remoteFrameFullscreenReverted: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* boolean handleFullscreenRequests (); */
    pub handleFullscreenRequests: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> nsresult,

    /* void exitFullscreen (); */
    pub exitFullscreen: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* boolean sendSelectionSetEvent (in unsigned long aOffset, in unsigned long aLength, [optional] in unsigned long aAdditionalFlags); */
    pub sendSelectionSetEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aOffset: libc::uint32_t, aLength: libc::uint32_t, aAdditionalFlags: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* boolean selectAtPoint (in float aX, in float aY, in unsigned long aSelectBehavior); */
    pub selectAtPoint: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aSelectBehavior: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* AString getVisitedDependentComputedStyle (in nsIDOMElement aElement, in AString aPseudoElement, in AString aPropertyName); */
    pub getVisitedDependentComputedStyle: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement, aPseudoElement: *const nsAString, aPropertyName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long long outerWindowID; */
    pub get_outerWindowID: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aOuterWindowID: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long currentInnerWindowID; */
    pub get_currentInnerWindowID: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aCurrentInnerWindowID: *mut libc::uint64_t) -> nsresult,

    /* void enterModalState (); */
    pub enterModalState: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* void leaveModalState (); */
    pub leaveModalState: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* [noscript] boolean isInModalState (); */
    pub isInModalState: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> nsresult,

    /* void setDesktopModeViewport (in boolean aDesktopModeViewport); */
    pub setDesktopModeViewport: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aDesktopModeViewport: bool) -> nsresult,

    /* void suspendTimeouts (); */
    pub suspendTimeouts: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* void resumeTimeouts (); */
    pub resumeTimeouts: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* readonly attribute AString layerManagerType; */
    pub get_layerManagerType: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aLayerManagerType: *mut nsAString) -> nsresult,

    /* readonly attribute boolean layerManagerRemote; */
    pub get_layerManagerRemote: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aLayerManagerRemote: *mut bool) -> nsresult,

    /* readonly attribute jsval supportsHardwareH264Decoding; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_supportsHardwareH264Decoding: *const ::libc::c_void,

    /* readonly attribute AString currentAudioBackend; */
    pub get_currentAudioBackend: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aCurrentAudioBackend: *mut nsAString) -> nsresult,

    /* void startFrameTimeRecording ([retval] out unsigned long startIndex); */
    pub startFrameTimeRecording: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, startIndex: *mut libc::uint32_t) -> nsresult,

    /* void stopFrameTimeRecording (in unsigned long startIndex, [optional] out unsigned long frameCount, [array, size_is (frameCount), retval] out float frameIntervals); */
    /// Unable to call function as its signature contains a non-rust type
    pub stopFrameTimeRecording: *const ::libc::c_void,

    /* readonly attribute float displayDPI; */
    pub get_displayDPI: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aDisplayDPI: *mut libc::c_float) -> nsresult,

    /* nsIDOMWindow getOuterWindowWithId (in unsigned long long aOuterWindowID); */
    pub getOuterWindowWithId: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aOuterWindowID: libc::uint64_t, _retval: *mut *const nsIDOMWindow) -> nsresult,

    /* readonly attribute nsIDOMElement containerElement; */
    pub get_containerElement: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aContainerElement: *mut *const nsIDOMElement) -> nsresult,

    /* [noscript] void RenderDocument (in nsConstRect aRect, in uint32_t aFlags, in nscolor aBackgroundColor, in gfxContext aThebesContext); */
    /// Unable to call function as its signature contains a non-rust type
    pub RenderDocument: *const ::libc::c_void,

    /* void advanceTimeAndRefresh (in long long aMilliseconds); */
    pub advanceTimeAndRefresh: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aMilliseconds: libc::int64_t) -> nsresult,

    /* void restoreNormalRefresh (); */
    pub restoreNormalRefresh: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* readonly attribute bool isTestControllingRefreshes; */
    pub get_isTestControllingRefreshes: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIsTestControllingRefreshes: *mut bool) -> nsresult,

    /* readonly attribute bool asyncPanZoomEnabled; */
    pub get_asyncPanZoomEnabled: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aAsyncPanZoomEnabled: *mut bool) -> nsresult,

    /* void setAsyncScrollOffset (in nsIDOMNode aNode, in float aX, in float aY); */
    pub setAsyncScrollOffset: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aNode: *const nsIDOMNode, aX: libc::c_float, aY: libc::c_float) -> nsresult,

    /* void setAsyncZoom (in nsIDOMNode aRootElement, in float aValue); */
    pub setAsyncZoom: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aRootElement: *const nsIDOMNode, aValue: libc::c_float) -> nsresult,

    /* bool flushApzRepaints (); */
    pub flushApzRepaints: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> nsresult,

    /* void zoomToFocusedInput (); */
    pub zoomToFocusedInput: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* double computeAnimationDistance (in nsIDOMElement element, in AString property, in AString value1, in AString value2); */
    pub computeAnimationDistance: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, element: *const nsIDOMElement, property: *const nsAString, value1: *const nsAString, value2: *const nsAString, _retval: *mut libc::c_double) -> nsresult,

    /* AString getAnimationTypeForLonghand (in AString aProperty); */
    pub getAnimationTypeForLonghand: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aProperty: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString getUnanimatedComputedStyle (in nsIDOMElement aElement, in AString aPseudoElement, in AString aProperty); */
    pub getUnanimatedComputedStyle: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement, aPseudoElement: *const nsAString, aProperty: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute string focusedInputType; */
    pub get_focusedInputType: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aFocusedInputType: *mut *const libc::c_char) -> nsresult,

    /* nsViewID getViewId (in nsIDOMElement aElement); */
    pub getViewId: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement, _retval: *mut nsViewID) -> nsresult,

    /* boolean leafLayersPartitionWindow (); */
    pub leafLayersPartitionWindow: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> nsresult,

    /* boolean checkAndClearPaintedState (in nsIDOMElement aElement); */
    pub checkAndClearPaintedState: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement, _retval: *mut bool) -> nsresult,

    /* boolean isPartOfOpaqueLayer (in nsIDOMElement aElement); */
    pub isPartOfOpaqueLayer: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement, _retval: *mut bool) -> nsresult,

    /* unsigned long numberOfAssignedPaintedLayers ([array, size_is (count)] in nsIDOMElement aElements, in uint32_t count); */
    /// Unable to call function as its signature contains a non-rust type
    pub numberOfAssignedPaintedLayers: *const ::libc::c_void,

    /* [implicit_jscontext] long long getFileId (in jsval aFile); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFileId: *const ::libc::c_void,

    /* [implicit_jscontext] AString getFilePath (in jsval aFile); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFilePath: *const ::libc::c_void,

    /* [implicit_jscontext] boolean getFileReferences (in AString aDatabaseName, in long long aId, [optional] in jsval aOptions, [optional] out long aRefCnt, [optional] out long aDBRefCnt, [optional] out long aSliceRefCnt); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFileReferences: *const ::libc::c_void,

    /* void flushPendingFileDeletions (); */
    pub flushPendingFileDeletions: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* [implicit_jscontext] boolean isIncrementalGCEnabled (); */
    /// Unable to call function as its signature contains a non-rust type
    pub isIncrementalGCEnabled: *const ::libc::c_void,

    /* [implicit_jscontext] void startPCCountProfiling (); */
    /// Unable to call function as its signature contains a non-rust type
    pub startPCCountProfiling: *const ::libc::c_void,

    /* [implicit_jscontext] void stopPCCountProfiling (); */
    /// Unable to call function as its signature contains a non-rust type
    pub stopPCCountProfiling: *const ::libc::c_void,

    /* [implicit_jscontext] void purgePCCounts (); */
    /// Unable to call function as its signature contains a non-rust type
    pub purgePCCounts: *const ::libc::c_void,

    /* [implicit_jscontext] long getPCCountScriptCount (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPCCountScriptCount: *const ::libc::c_void,

    /* [implicit_jscontext] AString getPCCountScriptSummary (in long script); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPCCountScriptSummary: *const ::libc::c_void,

    /* [implicit_jscontext] AString getPCCountScriptContents (in long script); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPCCountScriptContents: *const ::libc::c_void,

    /* readonly attribute boolean paintingSuppressed; */
    pub get_paintingSuppressed: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aPaintingSuppressed: *mut bool) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval plugins; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_plugins: *const ::libc::c_void,

    /* void setScrollPositionClampingScrollPortSize (in float aWidth, in float aHeight); */
    pub setScrollPositionClampingScrollPortSize: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aWidth: libc::c_float, aHeight: libc::c_float) -> nsresult,

    /* void disableDialogs (); */
    pub disableDialogs: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* void enableDialogs (); */
    pub enableDialogs: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* bool areDialogsEnabled (); */
    pub areDialogsEnabled: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> nsresult,

    /* void loadSheet (in nsIURI sheetURI, in unsigned long type); */
    pub loadSheet: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, sheetURI: *const nsIURI, type_: libc::uint32_t) -> nsresult,

    /* void loadSheetUsingURIString (in ACString sheetURI, in unsigned long type); */
    pub loadSheetUsingURIString: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, sheetURI: *const nsACString, type_: libc::uint32_t) -> nsresult,

    /* void addSheet (in nsIPreloadedStyleSheet sheet, in unsigned long type); */
    pub addSheet: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, sheet: *const nsIPreloadedStyleSheet, type_: libc::uint32_t) -> nsresult,

    /* void removeSheet (in nsIURI sheetURI, in unsigned long type); */
    pub removeSheet: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, sheetURI: *const nsIURI, type_: libc::uint32_t) -> nsresult,

    /* void removeSheetUsingURIString (in ACString sheetURI, in unsigned long type); */
    pub removeSheetUsingURIString: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, sheetURI: *const nsACString, type_: libc::uint32_t) -> nsresult,

    /* readonly attribute boolean isHandlingUserInput; */
    pub get_isHandlingUserInput: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIsHandlingUserInput: *mut bool) -> nsresult,

    /* readonly attribute double millisSinceLastUserInput; */
    pub get_millisSinceLastUserInput: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aMillisSinceLastUserInput: *mut libc::c_double) -> nsresult,

    /* void allowScriptsToClose (); */
    pub allowScriptsToClose: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* readonly attribute boolean isParentWindowMainWidgetVisible; */
    pub get_isParentWindowMainWidgetVisible: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aIsParentWindowMainWidgetVisible: *mut bool) -> nsresult,

    /* boolean isNodeDisabledForEvents (in nsIDOMNode aNode); */
    pub isNodeDisabledForEvents: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aNode: *const nsIDOMNode, _retval: *mut bool) -> nsresult,

    /* attribute boolean paintFlashing; */
    pub get_paintFlashing: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aPaintFlashing: *mut bool) -> nsresult,
    pub set_paintFlashing: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aPaintFlashing: bool) -> nsresult,

    /* AString getOMTAStyle (in nsIDOMElement aElement, in AString aProperty, [optional] in AString aPseudoElement); */
    pub getOMTAStyle: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement, aProperty: *const nsAString, aPseudoElement: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* float requestCompositorProperty (in AString aProperty); */
    pub requestCompositorProperty: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aProperty: *const nsAString, _retval: *mut libc::c_float) -> nsresult,

    /* nsIJSRAIIHelper setHandlingUserInput (in boolean aHandlingInput); */
    pub setHandlingUserInput: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aHandlingInput: bool, _retval: *mut *const nsIJSRAIIHelper) -> nsresult,

    /* [implicit_jscontext] jsval getContentAPZTestData (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getContentAPZTestData: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getCompositorAPZTestData (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCompositorAPZTestData: *const ::libc::c_void,

    /* void postRestyleSelfEvent (in nsIDOMElement aElement); */
    pub postRestyleSelfEvent: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElement: *const nsIDOMElement) -> nsresult,

    /* attribute uint32_t mediaSuspend; */
    pub get_mediaSuspend: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aMediaSuspend: *mut uint32_t) -> nsresult,
    pub set_mediaSuspend: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aMediaSuspend: uint32_t) -> nsresult,

    /* attribute boolean audioMuted; */
    pub get_audioMuted: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aAudioMuted: *mut bool) -> nsresult,
    pub set_audioMuted: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aAudioMuted: bool) -> nsresult,

    /* attribute float audioVolume; */
    pub get_audioVolume: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aAudioVolume: *mut libc::c_float) -> nsresult,
    pub set_audioVolume: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aAudioVolume: libc::c_float) -> nsresult,

    /* void xpconnectArgument (in nsIDOMWindowUtils aThis); */
    pub xpconnectArgument: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aThis: *const nsIDOMWindowUtils) -> nsresult,

    /* void askPermission (in nsIContentPermissionRequest aRequest); */
    pub askPermission: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aRequest: *const nsIContentPermissionRequest) -> nsresult,

    /* readonly attribute unsigned long long elementsRestyled; */
    pub get_elementsRestyled: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aElementsRestyled: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long framesConstructed; */
    pub get_framesConstructed: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aFramesConstructed: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long framesReflowed; */
    pub get_framesReflowed: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aFramesReflowed: *mut libc::uint64_t) -> nsresult,

    /* void setChromeMargin (in int32_t aTop, in int32_t aRight, in int32_t aBottom, in int32_t aLeft); */
    pub setChromeMargin: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aTop: int32_t, aRight: int32_t, aBottom: int32_t, aLeft: int32_t) -> nsresult,

    /* attribute boolean serviceWorkersTestingEnabled; */
    pub get_serviceWorkersTestingEnabled: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aServiceWorkersTestingEnabled: *mut bool) -> nsresult,
    pub set_serviceWorkersTestingEnabled: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aServiceWorkersTestingEnabled: bool) -> nsresult,

    /* [implicit_jscontext] jsval getFrameUniformityTestData (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFrameUniformityTestData: *const ::libc::c_void,

    /* void enterChaosMode (); */
    pub enterChaosMode: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* void leaveChaosMode (); */
    pub leaveChaosMode: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* bool hasRuleProcessorUsedByMultipleStyleSets (in unsigned long aSheetType); */
    pub hasRuleProcessorUsedByMultipleStyleSets: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aSheetType: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* void forceUseCounterFlush (in nsIDOMNode aNode); */
    pub forceUseCounterFlush: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aNode: *const nsIDOMNode) -> nsresult,

    /* void respectDisplayPortSuppression (in boolean aEnabled); */
    pub respectDisplayPortSuppression: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aEnabled: bool) -> nsresult,

    /* void forceReflowInterrupt (); */
    pub forceReflowInterrupt: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* void terminateGPUProcess (); */
    pub terminateGPUProcess: unsafe extern "C" fn (this: *const nsIDOMWindowUtils) -> nsresult,

    /* readonly attribute int32_t gpuProcessPid; */
    pub get_gpuProcessPid: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aGpuProcessPid: *mut int32_t) -> nsresult,

    /* boolean isTimeoutTracking (in unsigned long timeoutId); */
    pub isTimeoutTracking: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, timeoutId: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* void addManuallyManagedState (in nsIDOMElement element, in AString state); */
    pub addManuallyManagedState: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, element: *const nsIDOMElement, state: *const nsAString) -> nsresult,

    /* void removeManuallyManagedState (in nsIDOMElement element, in AString state); */
    pub removeManuallyManagedState: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, element: *const nsIDOMElement, state: *const nsAString) -> nsresult,

    /* int64_t getStorageUsage (in nsIDOMStorage aStorage); */
    pub getStorageUsage: unsafe extern "C" fn (this: *const nsIDOMWindowUtils, aStorage: *const nsIDOMStorage, _retval: *mut int64_t) -> nsresult,

}


impl nsIDOMWindowUtils {
    /* attribute unsigned short imageAnimationMode; */
    #[inline]
    pub unsafe fn get_imageAnimationMode(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_imageAnimationMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_imageAnimationMode(&self, aImageAnimationMode: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_imageAnimationMode)(self as *const _, aImageAnimationMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean docCharsetIsForced; */
    #[inline]
    pub unsafe fn get_docCharsetIsForced(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_docCharsetIsForced)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* short getCursorType (); */
    #[inline]
    pub unsafe fn getCursorType(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getCursorType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getDocumentMetadata (in AString aName); */
    #[inline]
    pub unsafe fn getDocumentMetadata(&self, aName: &[u16]) -> Result<nsString, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getDocumentMetadata)(self as *const _, &*aName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long redraw ([optional] in unsigned long aCount); */
    #[inline]
    pub unsafe fn redraw(&self, aCount: libc::uint32_t) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).redraw)(self as *const _, aCount, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void updateLayerTree (); */
    #[inline]
    pub unsafe fn updateLayerTree(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).updateLayerTree)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long long lastTransactionId; */
    #[inline]
    pub unsafe fn get_lastTransactionId(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastTransactionId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getViewportInfo (in uint32_t aDisplayWidth, in uint32_t aDisplayHeight, out double aDefaultZoom, out boolean aAllowZoom, out double aMinZoom, out double aMaxZoom, out uint32_t aWidth, out uint32_t aHeight, out boolean aAutoSize); */
    #[inline]
    pub unsafe fn getViewportInfo(&self, aDisplayWidth: uint32_t, aDisplayHeight: uint32_t) -> Result<(libc::c_double, bool, libc::c_double, libc::c_double, uint32_t, uint32_t, bool), nsresult> {
        let mut aDefaultZoom: libc::c_double = ::std::mem::zeroed();
        let mut aAllowZoom: bool = ::std::mem::zeroed();
        let mut aMinZoom: libc::c_double = ::std::mem::zeroed();
        let mut aMaxZoom: libc::c_double = ::std::mem::zeroed();
        let mut aWidth: uint32_t = ::std::mem::zeroed();
        let mut aHeight: uint32_t = ::std::mem::zeroed();
        let mut aAutoSize: bool = ::std::mem::zeroed();
        match ((*self.vtable).getViewportInfo)(self as *const _, aDisplayWidth, aDisplayHeight, &mut aDefaultZoom as *mut _, &mut aAllowZoom as *mut _, &mut aMinZoom as *mut _, &mut aMaxZoom as *mut _, &mut aWidth as *mut _, &mut aHeight as *mut _, &mut aAutoSize as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aDefaultZoom, aAllowZoom, aMinZoom, aMaxZoom, aWidth, aHeight, aAutoSize))
    }

    /* void getContentViewerSize (out uint32_t aDisplayWidth, out uint32_t aDisplayHeight); */
    #[inline]
    pub unsafe fn getContentViewerSize(&self, ) -> Result<(uint32_t, uint32_t), nsresult> {
        let mut aDisplayWidth: uint32_t = ::std::mem::zeroed();
        let mut aDisplayHeight: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getContentViewerSize)(self as *const _, &mut aDisplayWidth as *mut _, &mut aDisplayHeight as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aDisplayWidth, aDisplayHeight))
    }

    /* void setDisplayPortForElement (in float aXPx, in float aYPx, in float aWidthPx, in float aHeightPx, in nsIDOMElement aElement, in uint32_t aPriority); */
    #[inline]
    pub unsafe fn setDisplayPortForElement(&self, aXPx: libc::c_float, aYPx: libc::c_float, aWidthPx: libc::c_float, aHeightPx: libc::c_float, aElement: Option<&nsIDOMElement>, aPriority: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDisplayPortForElement)(self as *const _, aXPx, aYPx, aWidthPx, aHeightPx, aElement.map_or(::std::ptr::null(), |x| x as *const _), aPriority) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDisplayPortMarginsForElement (in float aLeftMargin, in float aTopMargin, in float aRightMargin, in float aBottomMargin, in nsIDOMElement aElement, in uint32_t aPriority); */
    #[inline]
    pub unsafe fn setDisplayPortMarginsForElement(&self, aLeftMargin: libc::c_float, aTopMargin: libc::c_float, aRightMargin: libc::c_float, aBottomMargin: libc::c_float, aElement: Option<&nsIDOMElement>, aPriority: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDisplayPortMarginsForElement)(self as *const _, aLeftMargin, aTopMargin, aRightMargin, aBottomMargin, aElement.map_or(::std::ptr::null(), |x| x as *const _), aPriority) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDisplayPortBaseForElement (in int32_t aX, in int32_t aY, in int32_t aWidth, in int32_t aHeight, in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn setDisplayPortBaseForElement(&self, aX: int32_t, aY: int32_t, aWidth: int32_t, aHeight: int32_t, aElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).setDisplayPortBaseForElement)(self as *const _, aX, aY, aWidth, aHeight, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setResolution (in float aResolution); */
    #[inline]
    pub unsafe fn setResolution(&self, aResolution: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setResolution)(self as *const _, aResolution) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getResolution (out float aResolution); */
    #[inline]
    pub unsafe fn getResolution(&self, ) -> Result<libc::c_float, nsresult> {
        let mut aResolution: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).getResolution)(self as *const _, &mut aResolution as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aResolution)
    }

    /* void setResolutionAndScaleTo (in float aResolution); */
    #[inline]
    pub unsafe fn setResolutionAndScaleTo(&self, aResolution: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setResolutionAndScaleTo)(self as *const _, aResolution) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setRestoreResolution (in float aResolution, in uint32_t aDisplayWidth, in uint32_t aDisplayHeight); */
    #[inline]
    pub unsafe fn setRestoreResolution(&self, aResolution: libc::c_float, aDisplayWidth: uint32_t, aDisplayHeight: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setRestoreResolution)(self as *const _, aResolution, aDisplayWidth, aDisplayHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isResolutionSet; */
    #[inline]
    pub unsafe fn get_isResolutionSet(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isResolutionSet)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean isFirstPaint; */
    #[inline]
    pub unsafe fn get_isFirstPaint(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isFirstPaint)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isFirstPaint(&self, aIsFirstPaint: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isFirstPaint)(self as *const _, aIsFirstPaint) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getPresShellId (out uint32_t aPresShellId); */
    #[inline]
    pub unsafe fn getPresShellId(&self, ) -> Result<uint32_t, nsresult> {
        let mut aPresShellId: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPresShellId)(self as *const _, &mut aPresShellId as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aPresShellId)
    }

    /* [optional_argc] boolean sendMouseEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier); */


    /* [optional_argc] boolean sendPointerEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in long aPointerId, [optional] in long aWidth, [optional] in long aHeight, [optional] in long aTiltX, [optional] in long aTiltY, [optional] in boolean aIsPrimary, [optional] in boolean aIsSynthesized); */


    /* boolean sendTouchEvent (in AString aType, [array, size_is (count)] in uint32_t aIdentifiers, [array, size_is (count)] in int32_t aXs, [array, size_is (count)] in int32_t aYs, [array, size_is (count)] in uint32_t aRxs, [array, size_is (count)] in uint32_t aRys, [array, size_is (count)] in float aRotationAngles, [array, size_is (count)] in float aForces, in uint32_t count, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */


    /* [optional_argc] void sendMouseEventToWindow (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier); */


    /* [optional_argc] void sendPointerEventToWindow (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in long aPointerId, [optional] in long aWidth, [optional] in long aHeight, [optional] in long aTiltX, [optional] in long aTiltY, [optional] in boolean aIsPrimary, [optional] in boolean aIsSynthesized); */


    /* boolean sendTouchEventToWindow (in AString aType, [array, size_is (count)] in uint32_t aIdentifiers, [array, size_is (count)] in int32_t aXs, [array, size_is (count)] in int32_t aYs, [array, size_is (count)] in uint32_t aRxs, [array, size_is (count)] in uint32_t aRys, [array, size_is (count)] in float aRotationAngles, [array, size_is (count)] in float aForces, in uint32_t count, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */


    /* void sendWheelEvent (in float aX, in float aY, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aDeltaMode, in long aModifiers, in long aLineOrPageDeltaX, in long aLineOrPageDeltaY, in unsigned long aOptions); */
    #[inline]
    pub unsafe fn sendWheelEvent(&self, aX: libc::c_float, aY: libc::c_float, aDeltaX: libc::c_double, aDeltaY: libc::c_double, aDeltaZ: libc::c_double, aDeltaMode: libc::uint32_t, aModifiers: libc::int32_t, aLineOrPageDeltaX: libc::int32_t, aLineOrPageDeltaY: libc::int32_t, aOptions: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).sendWheelEvent)(self as *const _, aX, aY, aDeltaX, aDeltaY, aDeltaZ, aDeltaMode, aModifiers, aLineOrPageDeltaX, aLineOrPageDeltaY, aOptions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean sendKeyEvent (in AString aType, in long aKeyCode, in long aCharCode, in long aModifiers, [optional] in unsigned long aAdditionalFlags); */
    #[inline]
    pub unsafe fn sendKeyEvent(&self, aType: &[u16], aKeyCode: libc::int32_t, aCharCode: libc::int32_t, aModifiers: libc::int32_t, aAdditionalFlags: libc::uint32_t) -> Result<bool, nsresult> {
        let aType = nsString::from(aType);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).sendKeyEvent)(self as *const _, &*aType, aKeyCode, aCharCode, aModifiers, aAdditionalFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void sendNativeKeyEvent (in long aNativeKeyboardLayout, in long aNativeKeyCode, in long aModifierFlags, in AString aCharacters, in AString aUnmodifiedCharacters, [optional] in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn sendNativeKeyEvent(&self, aNativeKeyboardLayout: libc::int32_t, aNativeKeyCode: libc::int32_t, aModifierFlags: libc::int32_t, aCharacters: &[u16], aUnmodifiedCharacters: &[u16], aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {
        let aCharacters = nsString::from(aCharacters);
        let aUnmodifiedCharacters = nsString::from(aUnmodifiedCharacters);
        match ((*self.vtable).sendNativeKeyEvent)(self as *const _, aNativeKeyboardLayout, aNativeKeyCode, aModifierFlags, &*aCharacters, &*aUnmodifiedCharacters, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendNativeMouseEvent (in long aScreenX, in long aScreenY, in long aNativeMessage, in long aModifierFlags, in nsIDOMElement aElement, [optional] in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn sendNativeMouseEvent(&self, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aNativeMessage: libc::int32_t, aModifierFlags: libc::int32_t, aElement: Option<&nsIDOMElement>, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).sendNativeMouseEvent)(self as *const _, aScreenX, aScreenY, aNativeMessage, aModifierFlags, aElement.map_or(::std::ptr::null(), |x| x as *const _), aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendNativeMouseMove (in long aScreenX, in long aScreenY, in nsIDOMElement aElement, [optional] in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn sendNativeMouseMove(&self, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aElement: Option<&nsIDOMElement>, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).sendNativeMouseMove)(self as *const _, aScreenX, aScreenY, aElement.map_or(::std::ptr::null(), |x| x as *const _), aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendNativeMouseScrollEvent (in long aScreenX, in long aScreenY, in unsigned long aNativeMessage, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aModifierFlags, in unsigned long aAdditionalFlags, in nsIDOMElement aElement, [optional] in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn sendNativeMouseScrollEvent(&self, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aNativeMessage: libc::uint32_t, aDeltaX: libc::c_double, aDeltaY: libc::c_double, aDeltaZ: libc::c_double, aModifierFlags: libc::uint32_t, aAdditionalFlags: libc::uint32_t, aElement: Option<&nsIDOMElement>, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).sendNativeMouseScrollEvent)(self as *const _, aScreenX, aScreenY, aNativeMessage, aDeltaX, aDeltaY, aDeltaZ, aModifierFlags, aAdditionalFlags, aElement.map_or(::std::ptr::null(), |x| x as *const _), aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendNativeTouchPoint (in unsigned long aPointerId, in unsigned long aTouchState, in long aScreenX, in long aScreenY, in double aPressure, in unsigned long aOrientation, [optional] in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn sendNativeTouchPoint(&self, aPointerId: libc::uint32_t, aTouchState: libc::uint32_t, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aPressure: libc::c_double, aOrientation: libc::uint32_t, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).sendNativeTouchPoint)(self as *const _, aPointerId, aTouchState, aScreenX, aScreenY, aPressure, aOrientation, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendNativeTouchTap (in long aScreenX, in long aScreenY, in boolean aLongTap, [optional] in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn sendNativeTouchTap(&self, aScreenX: libc::int32_t, aScreenY: libc::int32_t, aLongTap: bool, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).sendNativeTouchTap)(self as *const _, aScreenX, aScreenY, aLongTap, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearNativeTouchSequence ([optional] in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn clearNativeTouchSequence(&self, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).clearNativeTouchSequence)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void activateNativeMenuItemAt (in AString indexString); */
    #[inline]
    pub unsafe fn activateNativeMenuItemAt(&self, indexString: &[u16]) -> Result<(), nsresult> {
        let indexString = nsString::from(indexString);
        match ((*self.vtable).activateNativeMenuItemAt)(self as *const _, &*indexString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceUpdateNativeMenuAt (in AString indexString); */
    #[inline]
    pub unsafe fn forceUpdateNativeMenuAt(&self, indexString: &[u16]) -> Result<(), nsresult> {
        let indexString = nsString::from(indexString);
        match ((*self.vtable).forceUpdateNativeMenuAt)(self as *const _, &*indexString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString GetSelectionAsPlaintext (); */
    #[inline]
    pub unsafe fn GetSelectionAsPlaintext(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).GetSelectionAsPlaintext)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void focus (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn focus(&self, aElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).focus)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void garbageCollect ([optional] in nsICycleCollectorListener aListener, [optional] in long aExtraForgetSkippableCalls); */
    #[inline]
    pub unsafe fn garbageCollect(&self, aListener: Option<&nsICycleCollectorListener>, aExtraForgetSkippableCalls: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).garbageCollect)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), aExtraForgetSkippableCalls) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cycleCollect ([optional] in nsICycleCollectorListener aListener, [optional] in long aExtraForgetSkippableCalls); */
    #[inline]
    pub unsafe fn cycleCollect(&self, aListener: Option<&nsICycleCollectorListener>, aExtraForgetSkippableCalls: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).cycleCollect)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), aExtraForgetSkippableCalls) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void runNextCollectorTimer (); */
    #[inline]
    pub unsafe fn runNextCollectorTimer(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).runNextCollectorTimer)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendSimpleGestureEvent (in AString aType, in float aX, in float aY, in unsigned long aDirection, in double aDelta, in long aModifiers, [optional] in unsigned long aClickCount); */
    #[inline]
    pub unsafe fn sendSimpleGestureEvent(&self, aType: &[u16], aX: libc::c_float, aY: libc::c_float, aDirection: libc::uint32_t, aDelta: libc::c_double, aModifiers: libc::int32_t, aClickCount: libc::uint32_t) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).sendSimpleGestureEvent)(self as *const _, &*aType, aX, aY, aDirection, aDelta, aModifiers, aClickCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement elementFromPoint (in float aX, in float aY, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout); */
    #[inline]
    pub unsafe fn elementFromPoint(&self, aX: libc::c_float, aY: libc::c_float, aIgnoreRootScrollFrame: bool, aFlushLayout: bool) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).elementFromPoint)(self as *const _, aX, aY, aIgnoreRootScrollFrame, aFlushLayout, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNodeList nodesFromRect (in float aX, in float aY, in float aTopSize, in float aRightSize, in float aBottomSize, in float aLeftSize, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout); */
    #[inline]
    pub unsafe fn nodesFromRect(&self, aX: libc::c_float, aY: libc::c_float, aTopSize: libc::c_float, aRightSize: libc::c_float, aBottomSize: libc::c_float, aLeftSize: libc::c_float, aIgnoreRootScrollFrame: bool, aFlushLayout: bool) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).nodesFromRect)(self as *const _, aX, aY, aTopSize, aRightSize, aBottomSize, aLeftSize, aIgnoreRootScrollFrame, aFlushLayout, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITranslationNodeList getTranslationNodes (in nsIDOMNode aRoot); */
    #[inline]
    pub unsafe fn getTranslationNodes(&self, aRoot: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsITranslationNodeList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getTranslationNodes)(self as *const _, aRoot.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* uint32_t compareCanvases (in nsIDOMHTMLCanvasElement aCanvas1, in nsIDOMHTMLCanvasElement aCanvas2, out unsigned long aMaxDifference); */
    #[inline]
    pub unsafe fn compareCanvases(&self, aCanvas1: Option<&nsIDOMHTMLCanvasElement>, aCanvas2: Option<&nsIDOMHTMLCanvasElement>) -> Result<(libc::uint32_t, uint32_t), nsresult> {
        let mut aMaxDifference: libc::uint32_t = ::std::mem::zeroed();
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).compareCanvases)(self as *const _, aCanvas1.map_or(::std::ptr::null(), |x| x as *const _), aCanvas2.map_or(::std::ptr::null(), |x| x as *const _), &mut aMaxDifference as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMaxDifference, _retval))
    }

    /* readonly attribute boolean isMozAfterPaintPending; */
    #[inline]
    pub unsafe fn get_isMozAfterPaintPending(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isMozAfterPaintPending)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void suppressEventHandling (in boolean aSuppress); */
    #[inline]
    pub unsafe fn suppressEventHandling(&self, aSuppress: bool) -> Result<(), nsresult> {

        match ((*self.vtable).suppressEventHandling)(self as *const _, aSuppress) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void disableNonTestMouseEvents (in boolean aDisable); */
    #[inline]
    pub unsafe fn disableNonTestMouseEvents(&self, aDisable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).disableNonTestMouseEvents)(self as *const _, aDisable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getScrollXY (in boolean aFlushLayout, out long aScrollX, out long aScrollY); */
    #[inline]
    pub unsafe fn getScrollXY(&self, aFlushLayout: bool) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut aScrollX: libc::int32_t = ::std::mem::zeroed();
        let mut aScrollY: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getScrollXY)(self as *const _, aFlushLayout, &mut aScrollX as *mut _, &mut aScrollY as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aScrollX, aScrollY))
    }

    /* void getScrollXYFloat (in boolean aFlushLayout, out float aScrollX, out float aScrollY); */
    #[inline]
    pub unsafe fn getScrollXYFloat(&self, aFlushLayout: bool) -> Result<(libc::c_float, libc::c_float), nsresult> {
        let mut aScrollX: libc::c_float = ::std::mem::zeroed();
        let mut aScrollY: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).getScrollXYFloat)(self as *const _, aFlushLayout, &mut aScrollX as *mut _, &mut aScrollY as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aScrollX, aScrollY))
    }

    /* void getScrollbarSize (in boolean aFlushLayout, out long aWidth, out long aHeight); */
    #[inline]
    pub unsafe fn getScrollbarSize(&self, aFlushLayout: bool) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut aWidth: libc::int32_t = ::std::mem::zeroed();
        let mut aHeight: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getScrollbarSize)(self as *const _, aFlushLayout, &mut aWidth as *mut _, &mut aHeight as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aWidth, aHeight))
    }

    /* nsIDOMClientRect getBoundsWithoutFlushing (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn getBoundsWithoutFlushing(&self, aElement: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIDOMClientRect>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBoundsWithoutFlushing)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMClientRect getRootBounds (); */
    #[inline]
    pub unsafe fn getRootBounds(&self, ) -> Result<Option<RefPtr<nsIDOMClientRect>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRootBounds)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean IMEIsOpen; */
    #[inline]
    pub unsafe fn get_IMEIsOpen(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_IMEIsOpen)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long IMEStatus; */
    #[inline]
    pub unsafe fn get_IMEStatus(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_IMEStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float screenPixelsPerCSSPixel; */
    #[inline]
    pub unsafe fn get_screenPixelsPerCSSPixel(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_screenPixelsPerCSSPixel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float fullZoom; */
    #[inline]
    pub unsafe fn get_fullZoom(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_fullZoom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean dispatchDOMEventViaPresShell (in nsIDOMNode aTarget, in nsIDOMEvent aEvent, in boolean aTrusted); */
    #[inline]
    pub unsafe fn dispatchDOMEventViaPresShell(&self, aTarget: Option<&nsIDOMNode>, aEvent: Option<&nsIDOMEvent>, aTrusted: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).dispatchDOMEventViaPresShell)(self as *const _, aTarget.map_or(::std::ptr::null(), |x| x as *const _), aEvent.map_or(::std::ptr::null(), |x| x as *const _), aTrusted, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean dispatchEventToChromeOnly (in nsIDOMEventTarget aTarget, in nsIDOMEvent aEvent); */
    #[inline]
    pub unsafe fn dispatchEventToChromeOnly(&self, aTarget: Option<&nsIDOMEventTarget>, aEvent: Option<&nsIDOMEvent>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).dispatchEventToChromeOnly)(self as *const _, aTarget.map_or(::std::ptr::null(), |x| x as *const _), aEvent.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] string getClassName (in jsval aObject); */


    /* void sendContentCommandEvent (in AString aType, [optional] in nsITransferable aTransferable); */
    #[inline]
    pub unsafe fn sendContentCommandEvent(&self, aType: &[u16], aTransferable: Option<&nsITransferable>) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).sendContentCommandEvent)(self as *const _, &*aType, aTransferable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIQueryContentEventResult sendQueryContentEvent (in unsigned long aType, in long long aOffset, in unsigned long aLength, in long aX, in long aY, [optional] in unsigned long aAdditionalFlags); */
    #[inline]
    pub unsafe fn sendQueryContentEvent(&self, aType: libc::uint32_t, aOffset: libc::int64_t, aLength: libc::uint32_t, aX: libc::int32_t, aY: libc::int32_t, aAdditionalFlags: libc::uint32_t) -> Result<Option<RefPtr<nsIQueryContentEventResult>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).sendQueryContentEvent)(self as *const _, aType, aOffset, aLength, aX, aY, aAdditionalFlags, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void remoteFrameFullscreenChanged (in nsIDOMElement aFrameElement); */
    #[inline]
    pub unsafe fn remoteFrameFullscreenChanged(&self, aFrameElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).remoteFrameFullscreenChanged)(self as *const _, aFrameElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void remoteFrameFullscreenReverted (); */
    #[inline]
    pub unsafe fn remoteFrameFullscreenReverted(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).remoteFrameFullscreenReverted)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean handleFullscreenRequests (); */
    #[inline]
    pub unsafe fn handleFullscreenRequests(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).handleFullscreenRequests)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void exitFullscreen (); */
    #[inline]
    pub unsafe fn exitFullscreen(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).exitFullscreen)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean sendSelectionSetEvent (in unsigned long aOffset, in unsigned long aLength, [optional] in unsigned long aAdditionalFlags); */
    #[inline]
    pub unsafe fn sendSelectionSetEvent(&self, aOffset: libc::uint32_t, aLength: libc::uint32_t, aAdditionalFlags: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).sendSelectionSetEvent)(self as *const _, aOffset, aLength, aAdditionalFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean selectAtPoint (in float aX, in float aY, in unsigned long aSelectBehavior); */
    #[inline]
    pub unsafe fn selectAtPoint(&self, aX: libc::c_float, aY: libc::c_float, aSelectBehavior: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).selectAtPoint)(self as *const _, aX, aY, aSelectBehavior, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getVisitedDependentComputedStyle (in nsIDOMElement aElement, in AString aPseudoElement, in AString aPropertyName); */
    #[inline]
    pub unsafe fn getVisitedDependentComputedStyle(&self, aElement: Option<&nsIDOMElement>, aPseudoElement: &[u16], aPropertyName: &[u16]) -> Result<nsString, nsresult> {
        let aPseudoElement = nsString::from(aPseudoElement);
        let aPropertyName = nsString::from(aPropertyName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getVisitedDependentComputedStyle)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*aPseudoElement, &*aPropertyName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long outerWindowID; */
    #[inline]
    pub unsafe fn get_outerWindowID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_outerWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long currentInnerWindowID; */
    #[inline]
    pub unsafe fn get_currentInnerWindowID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_currentInnerWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void enterModalState (); */
    #[inline]
    pub unsafe fn enterModalState(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enterModalState)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void leaveModalState (); */
    #[inline]
    pub unsafe fn leaveModalState(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).leaveModalState)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] boolean isInModalState (); */
    #[inline]
    pub unsafe fn isInModalState(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isInModalState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setDesktopModeViewport (in boolean aDesktopModeViewport); */
    #[inline]
    pub unsafe fn setDesktopModeViewport(&self, aDesktopModeViewport: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setDesktopModeViewport)(self as *const _, aDesktopModeViewport) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void suspendTimeouts (); */
    #[inline]
    pub unsafe fn suspendTimeouts(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).suspendTimeouts)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resumeTimeouts (); */
    #[inline]
    pub unsafe fn resumeTimeouts(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resumeTimeouts)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute AString layerManagerType; */
    #[inline]
    pub unsafe fn get_layerManagerType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_layerManagerType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean layerManagerRemote; */
    #[inline]
    pub unsafe fn get_layerManagerRemote(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_layerManagerRemote)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute jsval supportsHardwareH264Decoding; */


    /* readonly attribute AString currentAudioBackend; */
    #[inline]
    pub unsafe fn get_currentAudioBackend(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_currentAudioBackend)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void startFrameTimeRecording ([retval] out unsigned long startIndex); */
    #[inline]
    pub unsafe fn startFrameTimeRecording(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut startIndex: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).startFrameTimeRecording)(self as *const _, &mut startIndex as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(startIndex)
    }

    /* void stopFrameTimeRecording (in unsigned long startIndex, [optional] out unsigned long frameCount, [array, size_is (frameCount), retval] out float frameIntervals); */


    /* readonly attribute float displayDPI; */
    #[inline]
    pub unsafe fn get_displayDPI(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_displayDPI)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMWindow getOuterWindowWithId (in unsigned long long aOuterWindowID); */
    #[inline]
    pub unsafe fn getOuterWindowWithId(&self, aOuterWindowID: libc::uint64_t) -> Result<Option<RefPtr<nsIDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getOuterWindowWithId)(self as *const _, aOuterWindowID, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement containerElement; */
    #[inline]
    pub unsafe fn get_containerElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_containerElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void RenderDocument (in nsConstRect aRect, in uint32_t aFlags, in nscolor aBackgroundColor, in gfxContext aThebesContext); */


    /* void advanceTimeAndRefresh (in long long aMilliseconds); */
    #[inline]
    pub unsafe fn advanceTimeAndRefresh(&self, aMilliseconds: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).advanceTimeAndRefresh)(self as *const _, aMilliseconds) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void restoreNormalRefresh (); */
    #[inline]
    pub unsafe fn restoreNormalRefresh(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).restoreNormalRefresh)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute bool isTestControllingRefreshes; */
    #[inline]
    pub unsafe fn get_isTestControllingRefreshes(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTestControllingRefreshes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool asyncPanZoomEnabled; */
    #[inline]
    pub unsafe fn get_asyncPanZoomEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_asyncPanZoomEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setAsyncScrollOffset (in nsIDOMNode aNode, in float aX, in float aY); */
    #[inline]
    pub unsafe fn setAsyncScrollOffset(&self, aNode: Option<&nsIDOMNode>, aX: libc::c_float, aY: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setAsyncScrollOffset)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aX, aY) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsyncZoom (in nsIDOMNode aRootElement, in float aValue); */
    #[inline]
    pub unsafe fn setAsyncZoom(&self, aRootElement: Option<&nsIDOMNode>, aValue: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setAsyncZoom)(self as *const _, aRootElement.map_or(::std::ptr::null(), |x| x as *const _), aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool flushApzRepaints (); */
    #[inline]
    pub unsafe fn flushApzRepaints(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).flushApzRepaints)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void zoomToFocusedInput (); */
    #[inline]
    pub unsafe fn zoomToFocusedInput(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).zoomToFocusedInput)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* double computeAnimationDistance (in nsIDOMElement element, in AString property, in AString value1, in AString value2); */
    #[inline]
    pub unsafe fn computeAnimationDistance(&self, element: Option<&nsIDOMElement>, property: &[u16], value1: &[u16], value2: &[u16]) -> Result<libc::c_double, nsresult> {
        let property = nsString::from(property);
        let value1 = nsString::from(value1);
        let value2 = nsString::from(value2);
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).computeAnimationDistance)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), &*property, &*value1, &*value2, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getAnimationTypeForLonghand (in AString aProperty); */
    #[inline]
    pub unsafe fn getAnimationTypeForLonghand(&self, aProperty: &[u16]) -> Result<nsString, nsresult> {
        let aProperty = nsString::from(aProperty);
        let mut _retval = nsString::new();
        match ((*self.vtable).getAnimationTypeForLonghand)(self as *const _, &*aProperty, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getUnanimatedComputedStyle (in nsIDOMElement aElement, in AString aPseudoElement, in AString aProperty); */
    #[inline]
    pub unsafe fn getUnanimatedComputedStyle(&self, aElement: Option<&nsIDOMElement>, aPseudoElement: &[u16], aProperty: &[u16]) -> Result<nsString, nsresult> {
        let aPseudoElement = nsString::from(aPseudoElement);
        let aProperty = nsString::from(aProperty);
        let mut _retval = nsString::new();
        match ((*self.vtable).getUnanimatedComputedStyle)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*aPseudoElement, &*aProperty, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string focusedInputType; */
    #[inline]
    pub unsafe fn get_focusedInputType(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_focusedInputType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsViewID getViewId (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn getViewId(&self, aElement: Option<&nsIDOMElement>) -> Result<nsViewID, nsresult> {
        let mut _retval: nsViewID = ::std::mem::zeroed();
        match ((*self.vtable).getViewId)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean leafLayersPartitionWindow (); */
    #[inline]
    pub unsafe fn leafLayersPartitionWindow(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).leafLayersPartitionWindow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean checkAndClearPaintedState (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn checkAndClearPaintedState(&self, aElement: Option<&nsIDOMElement>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).checkAndClearPaintedState)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isPartOfOpaqueLayer (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn isPartOfOpaqueLayer(&self, aElement: Option<&nsIDOMElement>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isPartOfOpaqueLayer)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long numberOfAssignedPaintedLayers ([array, size_is (count)] in nsIDOMElement aElements, in uint32_t count); */


    /* [implicit_jscontext] long long getFileId (in jsval aFile); */


    /* [implicit_jscontext] AString getFilePath (in jsval aFile); */


    /* [implicit_jscontext] boolean getFileReferences (in AString aDatabaseName, in long long aId, [optional] in jsval aOptions, [optional] out long aRefCnt, [optional] out long aDBRefCnt, [optional] out long aSliceRefCnt); */


    /* void flushPendingFileDeletions (); */
    #[inline]
    pub unsafe fn flushPendingFileDeletions(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).flushPendingFileDeletions)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] boolean isIncrementalGCEnabled (); */


    /* [implicit_jscontext] void startPCCountProfiling (); */


    /* [implicit_jscontext] void stopPCCountProfiling (); */


    /* [implicit_jscontext] void purgePCCounts (); */


    /* [implicit_jscontext] long getPCCountScriptCount (); */


    /* [implicit_jscontext] AString getPCCountScriptSummary (in long script); */


    /* [implicit_jscontext] AString getPCCountScriptContents (in long script); */


    /* readonly attribute boolean paintingSuppressed; */
    #[inline]
    pub unsafe fn get_paintingSuppressed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_paintingSuppressed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval plugins; */


    /* void setScrollPositionClampingScrollPortSize (in float aWidth, in float aHeight); */
    #[inline]
    pub unsafe fn setScrollPositionClampingScrollPortSize(&self, aWidth: libc::c_float, aHeight: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setScrollPositionClampingScrollPortSize)(self as *const _, aWidth, aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void disableDialogs (); */
    #[inline]
    pub unsafe fn disableDialogs(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).disableDialogs)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void enableDialogs (); */
    #[inline]
    pub unsafe fn enableDialogs(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enableDialogs)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool areDialogsEnabled (); */
    #[inline]
    pub unsafe fn areDialogsEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).areDialogsEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void loadSheet (in nsIURI sheetURI, in unsigned long type); */
    #[inline]
    pub unsafe fn loadSheet(&self, sheetURI: Option<&nsIURI>, type_: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).loadSheet)(self as *const _, sheetURI.map_or(::std::ptr::null(), |x| x as *const _), type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void loadSheetUsingURIString (in ACString sheetURI, in unsigned long type); */
    #[inline]
    pub unsafe fn loadSheetUsingURIString(&self, sheetURI: &[u8], type_: libc::uint32_t) -> Result<(), nsresult> {
        let sheetURI = nsCString::from(sheetURI);
        match ((*self.vtable).loadSheetUsingURIString)(self as *const _, &*sheetURI, type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addSheet (in nsIPreloadedStyleSheet sheet, in unsigned long type); */
    #[inline]
    pub unsafe fn addSheet(&self, sheet: Option<&nsIPreloadedStyleSheet>, type_: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).addSheet)(self as *const _, sheet.map_or(::std::ptr::null(), |x| x as *const _), type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeSheet (in nsIURI sheetURI, in unsigned long type); */
    #[inline]
    pub unsafe fn removeSheet(&self, sheetURI: Option<&nsIURI>, type_: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeSheet)(self as *const _, sheetURI.map_or(::std::ptr::null(), |x| x as *const _), type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeSheetUsingURIString (in ACString sheetURI, in unsigned long type); */
    #[inline]
    pub unsafe fn removeSheetUsingURIString(&self, sheetURI: &[u8], type_: libc::uint32_t) -> Result<(), nsresult> {
        let sheetURI = nsCString::from(sheetURI);
        match ((*self.vtable).removeSheetUsingURIString)(self as *const _, &*sheetURI, type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isHandlingUserInput; */
    #[inline]
    pub unsafe fn get_isHandlingUserInput(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isHandlingUserInput)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double millisSinceLastUserInput; */
    #[inline]
    pub unsafe fn get_millisSinceLastUserInput(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_millisSinceLastUserInput)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void allowScriptsToClose (); */
    #[inline]
    pub unsafe fn allowScriptsToClose(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).allowScriptsToClose)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isParentWindowMainWidgetVisible; */
    #[inline]
    pub unsafe fn get_isParentWindowMainWidgetVisible(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isParentWindowMainWidgetVisible)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isNodeDisabledForEvents (in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn isNodeDisabledForEvents(&self, aNode: Option<&nsIDOMNode>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isNodeDisabledForEvents)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean paintFlashing; */
    #[inline]
    pub unsafe fn get_paintFlashing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_paintFlashing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_paintFlashing(&self, aPaintFlashing: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_paintFlashing)(self as *const _, aPaintFlashing) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getOMTAStyle (in nsIDOMElement aElement, in AString aProperty, [optional] in AString aPseudoElement); */
    #[inline]
    pub unsafe fn getOMTAStyle(&self, aElement: Option<&nsIDOMElement>, aProperty: &[u16], aPseudoElement: &[u16]) -> Result<nsString, nsresult> {
        let aProperty = nsString::from(aProperty);
        let aPseudoElement = nsString::from(aPseudoElement);
        let mut _retval = nsString::new();
        match ((*self.vtable).getOMTAStyle)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*aProperty, &*aPseudoElement, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* float requestCompositorProperty (in AString aProperty); */
    #[inline]
    pub unsafe fn requestCompositorProperty(&self, aProperty: &[u16]) -> Result<libc::c_float, nsresult> {
        let aProperty = nsString::from(aProperty);
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).requestCompositorProperty)(self as *const _, &*aProperty, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIJSRAIIHelper setHandlingUserInput (in boolean aHandlingInput); */
    #[inline]
    pub unsafe fn setHandlingUserInput(&self, aHandlingInput: bool) -> Result<Option<RefPtr<nsIJSRAIIHelper>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).setHandlingUserInput)(self as *const _, aHandlingInput, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] jsval getContentAPZTestData (); */


    /* [implicit_jscontext] jsval getCompositorAPZTestData (); */


    /* void postRestyleSelfEvent (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn postRestyleSelfEvent(&self, aElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).postRestyleSelfEvent)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute uint32_t mediaSuspend; */
    #[inline]
    pub unsafe fn get_mediaSuspend(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mediaSuspend)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mediaSuspend(&self, aMediaSuspend: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_mediaSuspend)(self as *const _, aMediaSuspend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean audioMuted; */
    #[inline]
    pub unsafe fn get_audioMuted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_audioMuted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_audioMuted(&self, aAudioMuted: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_audioMuted)(self as *const _, aAudioMuted) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute float audioVolume; */
    #[inline]
    pub unsafe fn get_audioVolume(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_audioVolume)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_audioVolume(&self, aAudioVolume: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_audioVolume)(self as *const _, aAudioVolume) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void xpconnectArgument (in nsIDOMWindowUtils aThis); */
    #[inline]
    pub unsafe fn xpconnectArgument(&self, aThis: Option<&nsIDOMWindowUtils>) -> Result<(), nsresult> {

        match ((*self.vtable).xpconnectArgument)(self as *const _, aThis.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void askPermission (in nsIContentPermissionRequest aRequest); */
    #[inline]
    pub unsafe fn askPermission(&self, aRequest: Option<&nsIContentPermissionRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).askPermission)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long long elementsRestyled; */
    #[inline]
    pub unsafe fn get_elementsRestyled(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_elementsRestyled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long framesConstructed; */
    #[inline]
    pub unsafe fn get_framesConstructed(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_framesConstructed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long framesReflowed; */
    #[inline]
    pub unsafe fn get_framesReflowed(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_framesReflowed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setChromeMargin (in int32_t aTop, in int32_t aRight, in int32_t aBottom, in int32_t aLeft); */
    #[inline]
    pub unsafe fn setChromeMargin(&self, aTop: int32_t, aRight: int32_t, aBottom: int32_t, aLeft: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setChromeMargin)(self as *const _, aTop, aRight, aBottom, aLeft) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean serviceWorkersTestingEnabled; */
    #[inline]
    pub unsafe fn get_serviceWorkersTestingEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_serviceWorkersTestingEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_serviceWorkersTestingEnabled(&self, aServiceWorkersTestingEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_serviceWorkersTestingEnabled)(self as *const _, aServiceWorkersTestingEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval getFrameUniformityTestData (); */


    /* void enterChaosMode (); */
    #[inline]
    pub unsafe fn enterChaosMode(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enterChaosMode)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void leaveChaosMode (); */
    #[inline]
    pub unsafe fn leaveChaosMode(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).leaveChaosMode)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool hasRuleProcessorUsedByMultipleStyleSets (in unsigned long aSheetType); */
    #[inline]
    pub unsafe fn hasRuleProcessorUsedByMultipleStyleSets(&self, aSheetType: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasRuleProcessorUsedByMultipleStyleSets)(self as *const _, aSheetType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void forceUseCounterFlush (in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn forceUseCounterFlush(&self, aNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).forceUseCounterFlush)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void respectDisplayPortSuppression (in boolean aEnabled); */
    #[inline]
    pub unsafe fn respectDisplayPortSuppression(&self, aEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).respectDisplayPortSuppression)(self as *const _, aEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceReflowInterrupt (); */
    #[inline]
    pub unsafe fn forceReflowInterrupt(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).forceReflowInterrupt)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void terminateGPUProcess (); */
    #[inline]
    pub unsafe fn terminateGPUProcess(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).terminateGPUProcess)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute int32_t gpuProcessPid; */
    #[inline]
    pub unsafe fn get_gpuProcessPid(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_gpuProcessPid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isTimeoutTracking (in unsigned long timeoutId); */
    #[inline]
    pub unsafe fn isTimeoutTracking(&self, timeoutId: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isTimeoutTracking)(self as *const _, timeoutId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addManuallyManagedState (in nsIDOMElement element, in AString state); */
    #[inline]
    pub unsafe fn addManuallyManagedState(&self, element: Option<&nsIDOMElement>, state: &[u16]) -> Result<(), nsresult> {
        let state = nsString::from(state);
        match ((*self.vtable).addManuallyManagedState)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), &*state) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeManuallyManagedState (in nsIDOMElement element, in AString state); */
    #[inline]
    pub unsafe fn removeManuallyManagedState(&self, element: Option<&nsIDOMElement>, state: &[u16]) -> Result<(), nsresult> {
        let state = nsString::from(state);
        match ((*self.vtable).removeManuallyManagedState)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), &*state) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* int64_t getStorageUsage (in nsIDOMStorage aStorage); */
    #[inline]
    pub unsafe fn getStorageUsage(&self, aStorage: Option<&nsIDOMStorage>) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).getStorageUsage)(self as *const _, aStorage.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsITranslationNodeList {
    vtable: *const nsITranslationNodeListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITranslationNodeList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc694e359, 0x7227, 0x4392,
            [0xa1, 0x38, 0x33, 0xc0, 0xcc, 0x1f, 0x15, 0xa6])
    }
}

unsafe impl RefCounted for nsITranslationNodeList {
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
pub trait nsITranslationNodeListCoerce {
    fn coerce_from(v: &nsITranslationNodeList) -> &Self;
}

impl nsITranslationNodeListCoerce for nsITranslationNodeList {
    #[inline]
    fn coerce_from(v: &nsITranslationNodeList) -> &Self {
        v
    }
}

impl nsITranslationNodeList {
    #[inline]
    pub fn coerce<T: nsITranslationNodeListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITranslationNodeList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITranslationNodeListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITranslationNodeList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITranslationNodeListVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsITranslationNodeList, aLength: *mut libc::uint32_t) -> nsresult,

    /* nsIDOMNode item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsITranslationNodeList, index: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* boolean isTranslationRootAtIndex (in unsigned long index); */
    pub isTranslationRootAtIndex: unsafe extern "C" fn (this: *const nsITranslationNodeList, index: libc::uint32_t, _retval: *mut bool) -> nsresult,

}


impl nsITranslationNodeList {
    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMNode item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isTranslationRootAtIndex (in unsigned long index); */
    #[inline]
    pub unsafe fn isTranslationRootAtIndex(&self, index: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isTranslationRootAtIndex)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIJSRAIIHelper {
    vtable: *const nsIJSRAIIHelperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJSRAIIHelper {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x52e5a996, 0xd0a9, 0x4efc,
            [0xa6, 0xfa, 0x24, 0x48, 0x9c, 0x53, 0x2b, 0x19])
    }
}

unsafe impl RefCounted for nsIJSRAIIHelper {
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
pub trait nsIJSRAIIHelperCoerce {
    fn coerce_from(v: &nsIJSRAIIHelper) -> &Self;
}

impl nsIJSRAIIHelperCoerce for nsIJSRAIIHelper {
    #[inline]
    fn coerce_from(v: &nsIJSRAIIHelper) -> &Self {
        v
    }
}

impl nsIJSRAIIHelper {
    #[inline]
    pub fn coerce<T: nsIJSRAIIHelperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJSRAIIHelper {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIJSRAIIHelperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSRAIIHelper) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJSRAIIHelperVTable {
    pub __base: nsISupportsVTable,

    /* void destruct (); */
    pub destruct: unsafe extern "C" fn (this: *const nsIJSRAIIHelper) -> nsresult,

}


impl nsIJSRAIIHelper {
    /* void destruct (); */
    #[inline]
    pub unsafe fn destruct(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destruct)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


