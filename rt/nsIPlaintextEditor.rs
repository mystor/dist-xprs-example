//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPlaintextEditor.idl
//


pub mod nsIPlaintextEditor_consts {
    pub const eEditorPlaintextMask: i64 = 1;
    pub const eEditorSingleLineMask: i64 = 2;
    pub const eEditorPasswordMask: i64 = 4;
    pub const eEditorReadonlyMask: i64 = 8;
    pub const eEditorDisabledMask: i64 = 16;
    pub const eEditorFilterInputMask: i64 = 32;
    pub const eEditorMailMask: i64 = 64;
    pub const eEditorEnableWrapHackMask: i64 = 128;
    pub const eEditorWidgetMask: i64 = 256;
    pub const eEditorNoCSSMask: i64 = 512;
    pub const eEditorAllowInteraction: i64 = 1024;
    pub const eEditorDontEchoPassword: i64 = 2048;
    pub const eEditorRightToLeft: i64 = 4096;
    pub const eEditorLeftToRight: i64 = 8192;
    pub const eEditorSkipSpellCheck: i64 = 16384;
    pub const eNewlinesPasteIntact: i64 = 0;
    pub const eNewlinesPasteToFirst: i64 = 1;
    pub const eNewlinesReplaceWithSpaces: i64 = 2;
    pub const eNewlinesStrip: i64 = 3;
    pub const eNewlinesReplaceWithCommas: i64 = 4;
    pub const eNewlinesStripSurroundingWhitespace: i64 = 5;
}


#[repr(C)]
pub struct nsIPlaintextEditor {
    vtable: *const nsIPlaintextEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPlaintextEditor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb74fb158, 0x1265, 0x4102,
            [0x91, 0xeb, 0xed, 0xd0, 0x13, 0x6b, 0x49, 0xf8])
    }
}

unsafe impl RefCounted for nsIPlaintextEditor {
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
pub trait nsIPlaintextEditorCoerce {
    fn coerce_from(v: &nsIPlaintextEditor) -> &Self;
}

impl nsIPlaintextEditorCoerce for nsIPlaintextEditor {
    #[inline]
    fn coerce_from(v: &nsIPlaintextEditor) -> &Self {
        v
    }
}

impl nsIPlaintextEditor {
    #[inline]
    pub fn coerce<T: nsIPlaintextEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPlaintextEditor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPlaintextEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPlaintextEditor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPlaintextEditorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long textLength; */
    pub get_textLength: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aTextLength: *mut libc::int32_t) -> nsresult,

    /* attribute long maxTextLength; */
    pub get_maxTextLength: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aMaxTextLength: *mut libc::int32_t) -> nsresult,
    pub set_maxTextLength: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aMaxTextLength: libc::int32_t) -> nsresult,

    /* attribute long wrapWidth; */
    pub get_wrapWidth: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aWrapWidth: *mut libc::int32_t) -> nsresult,
    pub set_wrapWidth: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aWrapWidth: libc::int32_t) -> nsresult,

    /* void setWrapColumn (in long aWrapColumn); */
    pub setWrapColumn: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aWrapColumn: libc::int32_t) -> nsresult,

    /* attribute long newlineHandling; */
    pub get_newlineHandling: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aNewlineHandling: *mut libc::int32_t) -> nsresult,
    pub set_newlineHandling: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aNewlineHandling: libc::int32_t) -> nsresult,

    /* void insertText (in DOMString aStringToInsert); */
    pub insertText: unsafe extern "C" fn (this: *const nsIPlaintextEditor, aStringToInsert: *const nsAString) -> nsresult,

    /* void insertLineBreak (); */
    pub insertLineBreak: unsafe extern "C" fn (this: *const nsIPlaintextEditor) -> nsresult,

}


impl nsIPlaintextEditor {
    /* readonly attribute long textLength; */
    #[inline]
    pub unsafe fn get_textLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_textLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute long maxTextLength; */
    #[inline]
    pub unsafe fn get_maxTextLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maxTextLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_maxTextLength(&self, aMaxTextLength: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_maxTextLength)(self as *const _, aMaxTextLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long wrapWidth; */
    #[inline]
    pub unsafe fn get_wrapWidth(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_wrapWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_wrapWidth(&self, aWrapWidth: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_wrapWidth)(self as *const _, aWrapWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setWrapColumn (in long aWrapColumn); */
    #[inline]
    pub unsafe fn setWrapColumn(&self, aWrapColumn: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setWrapColumn)(self as *const _, aWrapColumn) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long newlineHandling; */
    #[inline]
    pub unsafe fn get_newlineHandling(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_newlineHandling)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_newlineHandling(&self, aNewlineHandling: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_newlineHandling)(self as *const _, aNewlineHandling) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertText (in DOMString aStringToInsert); */
    #[inline]
    pub unsafe fn insertText(&self, aStringToInsert: &[u16]) -> Result<(), nsresult> {
        let aStringToInsert = nsString::from(aStringToInsert);
        match ((*self.vtable).insertText)(self as *const _, &*aStringToInsert) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertLineBreak (); */
    #[inline]
    pub unsafe fn insertLineBreak(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).insertLineBreak)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


