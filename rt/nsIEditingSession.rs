//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditingSession.idl
//


pub mod nsIEditingSession_consts {
    pub const eEditorOK: i64 = 0;
    pub const eEditorCreationInProgress: i64 = 1;
    pub const eEditorErrorCantEditMimeType: i64 = 2;
    pub const eEditorErrorFileNotFound: i64 = 3;
    pub const eEditorErrorCantEditFramesets: i64 = 8;
    pub const eEditorErrorUnknown: i64 = 9;
}


#[repr(C)]
pub struct nsIEditingSession {
    vtable: *const nsIEditingSessionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditingSession {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x24f963d1, 0xe6fc, 0x43ea,
            [0xa2, 0x06, 0x99, 0xac, 0x5f, 0xcc, 0x52, 0x65])
    }
}

unsafe impl RefCounted for nsIEditingSession {
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
pub trait nsIEditingSessionCoerce {
    fn coerce_from(v: &nsIEditingSession) -> &Self;
}

impl nsIEditingSessionCoerce for nsIEditingSession {
    #[inline]
    fn coerce_from(v: &nsIEditingSession) -> &Self {
        v
    }
}

impl nsIEditingSession {
    #[inline]
    pub fn coerce<T: nsIEditingSessionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditingSession {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditingSessionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditingSession) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditingSessionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long editorStatus; */
    pub get_editorStatus: unsafe extern "C" fn (this: *const nsIEditingSession, aEditorStatus: *mut libc::uint32_t) -> nsresult,

    /* void makeWindowEditable (in mozIDOMWindowProxy window, in string aEditorType, in boolean doAfterUriLoad, in boolean aMakeWholeDocumentEditable, in boolean aInteractive); */
    pub makeWindowEditable: unsafe extern "C" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy, aEditorType: *const libc::c_char, doAfterUriLoad: bool, aMakeWholeDocumentEditable: bool, aInteractive: bool) -> nsresult,

    /* boolean windowIsEditable (in mozIDOMWindowProxy window); */
    pub windowIsEditable: unsafe extern "C" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy, _retval: *mut bool) -> nsresult,

    /* nsIEditor getEditorForWindow (in mozIDOMWindowProxy window); */
    pub getEditorForWindow: unsafe extern "C" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy, _retval: *mut *const nsIEditor) -> nsresult,

    /* void setupEditorOnWindow (in mozIDOMWindowProxy window); */
    pub setupEditorOnWindow: unsafe extern "C" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy) -> nsresult,

    /* void tearDownEditorOnWindow (in mozIDOMWindowProxy window); */
    pub tearDownEditorOnWindow: unsafe extern "C" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy) -> nsresult,

    /* void setEditorOnControllers (in mozIDOMWindowProxy aWindow, in nsIEditor aEditor); */
    pub setEditorOnControllers: unsafe extern "C" fn (this: *const nsIEditingSession, aWindow: *const mozIDOMWindowProxy, aEditor: *const nsIEditor) -> nsresult,

    /* void disableJSAndPlugins (in mozIDOMWindowProxy aWindow); */
    pub disableJSAndPlugins: unsafe extern "C" fn (this: *const nsIEditingSession, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* void restoreJSAndPlugins (in mozIDOMWindowProxy aWindow); */
    pub restoreJSAndPlugins: unsafe extern "C" fn (this: *const nsIEditingSession, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* void detachFromWindow (in mozIDOMWindowProxy aWindow); */
    pub detachFromWindow: unsafe extern "C" fn (this: *const nsIEditingSession, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* void reattachToWindow (in mozIDOMWindowProxy aWindow); */
    pub reattachToWindow: unsafe extern "C" fn (this: *const nsIEditingSession, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute boolean jsAndPluginsDisabled; */
    pub get_jsAndPluginsDisabled: unsafe extern "C" fn (this: *const nsIEditingSession, aJsAndPluginsDisabled: *mut bool) -> nsresult,

}


impl nsIEditingSession {
    /* readonly attribute unsigned long editorStatus; */
    #[inline]
    pub unsafe fn get_editorStatus(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_editorStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void makeWindowEditable (in mozIDOMWindowProxy window, in string aEditorType, in boolean doAfterUriLoad, in boolean aMakeWholeDocumentEditable, in boolean aInteractive); */
    #[inline]
    pub unsafe fn makeWindowEditable(&self, window: Option<&mozIDOMWindowProxy>, aEditorType: *const libc::c_char, doAfterUriLoad: bool, aMakeWholeDocumentEditable: bool, aInteractive: bool) -> Result<(), nsresult> {

        match ((*self.vtable).makeWindowEditable)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), aEditorType, doAfterUriLoad, aMakeWholeDocumentEditable, aInteractive) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean windowIsEditable (in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn windowIsEditable(&self, window: Option<&mozIDOMWindowProxy>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).windowIsEditable)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIEditor getEditorForWindow (in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn getEditorForWindow(&self, window: Option<&mozIDOMWindowProxy>) -> Result<Option<RefPtr<nsIEditor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEditorForWindow)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setupEditorOnWindow (in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn setupEditorOnWindow(&self, window: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).setupEditorOnWindow)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void tearDownEditorOnWindow (in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn tearDownEditorOnWindow(&self, window: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).tearDownEditorOnWindow)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setEditorOnControllers (in mozIDOMWindowProxy aWindow, in nsIEditor aEditor); */
    #[inline]
    pub unsafe fn setEditorOnControllers(&self, aWindow: Option<&mozIDOMWindowProxy>, aEditor: Option<&nsIEditor>) -> Result<(), nsresult> {

        match ((*self.vtable).setEditorOnControllers)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aEditor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void disableJSAndPlugins (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn disableJSAndPlugins(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).disableJSAndPlugins)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void restoreJSAndPlugins (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn restoreJSAndPlugins(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).restoreJSAndPlugins)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void detachFromWindow (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn detachFromWindow(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).detachFromWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reattachToWindow (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn reattachToWindow(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).reattachToWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean jsAndPluginsDisabled; */
    #[inline]
    pub unsafe fn get_jsAndPluginsDisabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_jsAndPluginsDisabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


