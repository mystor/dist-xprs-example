//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHelperAppLauncherDialog.idl
//


pub mod nsIHelperAppLauncherDialog_consts {
    pub const REASON_CANTHANDLE: i64 = 0;
    pub const REASON_SERVERREQUEST: i64 = 1;
    pub const REASON_TYPESNIFFED: i64 = 2;
}


#[repr(C)]
pub struct nsIHelperAppLauncherDialog {
    vtable: *const nsIHelperAppLauncherDialogVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHelperAppLauncherDialog {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbfc739f3, 0x8d75, 0x4034,
            [0xa6, 0xf8, 0x10, 0x39, 0xa5, 0x99, 0x6b, 0xad])
    }
}

unsafe impl RefCounted for nsIHelperAppLauncherDialog {
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
pub trait nsIHelperAppLauncherDialogCoerce {
    fn coerce_from(v: &nsIHelperAppLauncherDialog) -> &Self;
}

impl nsIHelperAppLauncherDialogCoerce for nsIHelperAppLauncherDialog {
    #[inline]
    fn coerce_from(v: &nsIHelperAppLauncherDialog) -> &Self {
        v
    }
}

impl nsIHelperAppLauncherDialog {
    #[inline]
    pub fn coerce<T: nsIHelperAppLauncherDialogCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHelperAppLauncherDialog {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHelperAppLauncherDialogCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHelperAppLauncherDialog) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHelperAppLauncherDialogVTable {
    pub __base: nsISupportsVTable,

    /* void show (in nsIHelperAppLauncher aLauncher, in nsISupports aWindowContext, in unsigned long aReason); */
    pub show: unsafe extern "C" fn (this: *const nsIHelperAppLauncherDialog, aLauncher: *const nsIHelperAppLauncher, aWindowContext: *const nsISupports, aReason: libc::uint32_t) -> nsresult,

    /* void promptForSaveToFileAsync (in nsIHelperAppLauncher aLauncher, in nsISupports aWindowContext, in wstring aDefaultFileName, in wstring aSuggestedFileExtension, in boolean aForcePrompt); */
    pub promptForSaveToFileAsync: unsafe extern "C" fn (this: *const nsIHelperAppLauncherDialog, aLauncher: *const nsIHelperAppLauncher, aWindowContext: *const nsISupports, aDefaultFileName: *const libc::int16_t, aSuggestedFileExtension: *const libc::int16_t, aForcePrompt: bool) -> nsresult,

}


impl nsIHelperAppLauncherDialog {
    /* void show (in nsIHelperAppLauncher aLauncher, in nsISupports aWindowContext, in unsigned long aReason); */
    #[inline]
    pub unsafe fn show(&self, aLauncher: Option<&nsIHelperAppLauncher>, aWindowContext: Option<&nsISupports>, aReason: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).show)(self as *const _, aLauncher.map_or(::std::ptr::null(), |x| x as *const _), aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void promptForSaveToFileAsync (in nsIHelperAppLauncher aLauncher, in nsISupports aWindowContext, in wstring aDefaultFileName, in wstring aSuggestedFileExtension, in boolean aForcePrompt); */
    #[inline]
    pub unsafe fn promptForSaveToFileAsync(&self, aLauncher: Option<&nsIHelperAppLauncher>, aWindowContext: Option<&nsISupports>, aDefaultFileName: *const libc::int16_t, aSuggestedFileExtension: *const libc::int16_t, aForcePrompt: bool) -> Result<(), nsresult> {

        match ((*self.vtable).promptForSaveToFileAsync)(self as *const _, aLauncher.map_or(::std::ptr::null(), |x| x as *const _), aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), aDefaultFileName, aSuggestedFileExtension, aForcePrompt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


