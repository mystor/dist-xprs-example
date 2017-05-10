//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITextInputProcessor.idl
//


pub mod nsITextInputProcessor_consts {
    pub const ATTR_RAW_CLAUSE: i64 = 2;
    pub const ATTR_SELECTED_RAW_CLAUSE: i64 = 3;
    pub const ATTR_CONVERTED_CLAUSE: i64 = 4;
    pub const ATTR_SELECTED_CLAUSE: i64 = 5;
    pub const KEY_DEFAULT_PREVENTED: i64 = 1;
    pub const KEY_NON_PRINTABLE_KEY: i64 = 2;
    pub const KEY_FORCE_PRINTABLE_KEY: i64 = 4;
    pub const KEY_KEEP_KEY_LOCATION_STANDARD: i64 = 8;
    pub const KEY_KEEP_KEYCODE_ZERO: i64 = 16;
    pub const KEY_DONT_DISPATCH_MODIFIER_KEY_EVENT: i64 = 32;
    pub const KEYEVENT_NOT_CONSUMED: i64 = 0;
    pub const KEYDOWN_IS_CONSUMED: i64 = 1;
    pub const KEYPRESS_IS_CONSUMED: i64 = 2;
}


#[repr(C)]
pub struct nsITextInputProcessor {
    vtable: *const nsITextInputProcessorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITextInputProcessor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x47ae2181, 0x2e98, 0x4d58,
            [0x84, 0xa2, 0xb8, 0xdb, 0x67, 0x64, 0xce, 0x9a])
    }
}

unsafe impl RefCounted for nsITextInputProcessor {
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
pub trait nsITextInputProcessorCoerce {
    fn coerce_from(v: &nsITextInputProcessor) -> &Self;
}

impl nsITextInputProcessorCoerce for nsITextInputProcessor {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessor) -> &Self {
        v
    }
}

impl nsITextInputProcessor {
    #[inline]
    pub fn coerce<T: nsITextInputProcessorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITextInputProcessor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITextInputProcessorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITextInputProcessorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean hasComposition; */
    pub get_hasComposition: unsafe extern "C" fn (this: *const nsITextInputProcessor, aHasComposition: *mut bool) -> nsresult,

    /* boolean beginInputTransaction (in mozIDOMWindow aWindow, in nsITextInputProcessorCallback aCallback); */
    pub beginInputTransaction: unsafe extern "C" fn (this: *const nsITextInputProcessor, aWindow: *const mozIDOMWindow, aCallback: *const nsITextInputProcessorCallback, _retval: *mut bool) -> nsresult,

    /* [optional_argc] boolean beginInputTransactionForTests (in mozIDOMWindow aWindow, [optional] in nsITextInputProcessorCallback aCallback); */
    /// Unable to call function as its signature contains a non-rust type
    pub beginInputTransactionForTests: *const ::libc::c_void,

    /* [optional_argc] boolean startComposition ([optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub startComposition: *const ::libc::c_void,

    /* void setPendingCompositionString (in DOMString aString); */
    pub setPendingCompositionString: unsafe extern "C" fn (this: *const nsITextInputProcessor, aString: *const nsAString) -> nsresult,

    /* void appendClauseToPendingComposition (in unsigned long aLength, in unsigned long aAttribute); */
    pub appendClauseToPendingComposition: unsafe extern "C" fn (this: *const nsITextInputProcessor, aLength: libc::uint32_t, aAttribute: libc::uint32_t) -> nsresult,

    /* void setCaretInPendingComposition (in unsigned long aOffset); */
    pub setCaretInPendingComposition: unsafe extern "C" fn (this: *const nsITextInputProcessor, aOffset: libc::uint32_t) -> nsresult,

    /* [optional_argc] boolean flushPendingComposition ([optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub flushPendingComposition: *const ::libc::c_void,

    /* [optional_argc] void commitComposition ([optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub commitComposition: *const ::libc::c_void,

    /* [optional_argc] boolean commitCompositionWith (in DOMString aCommitString, [optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub commitCompositionWith: *const ::libc::c_void,

    /* [optional_argc] void cancelComposition ([optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub cancelComposition: *const ::libc::c_void,

    /* [optional_argc] unsigned long keydown (in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub keydown: *const ::libc::c_void,

    /* [optional_argc] boolean keyup (in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub keyup: *const ::libc::c_void,

    /* boolean getModifierState (in DOMString aModifierKey); */
    pub getModifierState: unsafe extern "C" fn (this: *const nsITextInputProcessor, aModifierKey: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void shareModifierStateOf (in nsITextInputProcessor aOther); */
    pub shareModifierStateOf: unsafe extern "C" fn (this: *const nsITextInputProcessor, aOther: *const nsITextInputProcessor) -> nsresult,

}


impl nsITextInputProcessor {
    /* readonly attribute boolean hasComposition; */
    #[inline]
    pub unsafe fn get_hasComposition(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasComposition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean beginInputTransaction (in mozIDOMWindow aWindow, in nsITextInputProcessorCallback aCallback); */
    #[inline]
    pub unsafe fn beginInputTransaction(&self, aWindow: Option<&mozIDOMWindow>, aCallback: Option<&nsITextInputProcessorCallback>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).beginInputTransaction)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [optional_argc] boolean beginInputTransactionForTests (in mozIDOMWindow aWindow, [optional] in nsITextInputProcessorCallback aCallback); */


    /* [optional_argc] boolean startComposition ([optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */


    /* void setPendingCompositionString (in DOMString aString); */
    #[inline]
    pub unsafe fn setPendingCompositionString(&self, aString: &[u16]) -> Result<(), nsresult> {
        let aString = nsString::from(aString);
        match ((*self.vtable).setPendingCompositionString)(self as *const _, &*aString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendClauseToPendingComposition (in unsigned long aLength, in unsigned long aAttribute); */
    #[inline]
    pub unsafe fn appendClauseToPendingComposition(&self, aLength: libc::uint32_t, aAttribute: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).appendClauseToPendingComposition)(self as *const _, aLength, aAttribute) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCaretInPendingComposition (in unsigned long aOffset); */
    #[inline]
    pub unsafe fn setCaretInPendingComposition(&self, aOffset: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setCaretInPendingComposition)(self as *const _, aOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [optional_argc] boolean flushPendingComposition ([optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */


    /* [optional_argc] void commitComposition ([optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */


    /* [optional_argc] boolean commitCompositionWith (in DOMString aCommitString, [optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */


    /* [optional_argc] void cancelComposition ([optional] in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */


    /* [optional_argc] unsigned long keydown (in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */


    /* [optional_argc] boolean keyup (in nsIDOMKeyEvent aKeyboardEvent, [optional] in unsigned long aKeyFlags); */


    /* boolean getModifierState (in DOMString aModifierKey); */
    #[inline]
    pub unsafe fn getModifierState(&self, aModifierKey: &[u16]) -> Result<bool, nsresult> {
        let aModifierKey = nsString::from(aModifierKey);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getModifierState)(self as *const _, &*aModifierKey, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void shareModifierStateOf (in nsITextInputProcessor aOther); */
    #[inline]
    pub unsafe fn shareModifierStateOf(&self, aOther: Option<&nsITextInputProcessor>) -> Result<(), nsresult> {

        match ((*self.vtable).shareModifierStateOf)(self as *const _, aOther.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


