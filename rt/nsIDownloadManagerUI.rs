//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloadManagerUI.idl
//


pub mod nsIDownloadManagerUI_consts {
    pub const REASON_USER_INTERACTED: i64 = 0;
    pub const REASON_NEW_DOWNLOAD: i64 = 1;
}


#[repr(C)]
pub struct nsIDownloadManagerUI {
    vtable: *const nsIDownloadManagerUIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDownloadManagerUI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0c76d4cf, 0x0b06, 0x4c1a,
            [0x9b, 0xea, 0x52, 0x0c, 0x7b, 0xbd, 0xba, 0x99])
    }
}

unsafe impl RefCounted for nsIDownloadManagerUI {
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
pub trait nsIDownloadManagerUICoerce {
    fn coerce_from(v: &nsIDownloadManagerUI) -> &Self;
}

impl nsIDownloadManagerUICoerce for nsIDownloadManagerUI {
    #[inline]
    fn coerce_from(v: &nsIDownloadManagerUI) -> &Self {
        v
    }
}

impl nsIDownloadManagerUI {
    #[inline]
    pub fn coerce<T: nsIDownloadManagerUICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDownloadManagerUI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDownloadManagerUICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloadManagerUI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDownloadManagerUIVTable {
    pub __base: nsISupportsVTable,

    /* void show ([optional] in nsIInterfaceRequestor aWindowContext, [optional] in nsIDownload aDownload, [optional] in short aReason, [optional] in boolean aUsePrivateUI); */
    pub show: unsafe extern "C" fn (this: *const nsIDownloadManagerUI, aWindowContext: *const nsIInterfaceRequestor, aDownload: *const nsIDownload, aReason: libc::int16_t, aUsePrivateUI: bool) -> nsresult,

    /* readonly attribute boolean visible; */
    pub get_visible: unsafe extern "C" fn (this: *const nsIDownloadManagerUI, aVisible: *mut bool) -> nsresult,

    /* void getAttention (); */
    pub getAttention: unsafe extern "C" fn (this: *const nsIDownloadManagerUI) -> nsresult,

}


impl nsIDownloadManagerUI {
    /* void show ([optional] in nsIInterfaceRequestor aWindowContext, [optional] in nsIDownload aDownload, [optional] in short aReason, [optional] in boolean aUsePrivateUI); */
    #[inline]
    pub unsafe fn show(&self, aWindowContext: Option<&nsIInterfaceRequestor>, aDownload: Option<&nsIDownload>, aReason: libc::int16_t, aUsePrivateUI: bool) -> Result<(), nsresult> {

        match ((*self.vtable).show)(self as *const _, aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), aDownload.map_or(::std::ptr::null(), |x| x as *const _), aReason, aUsePrivateUI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean visible; */
    #[inline]
    pub unsafe fn get_visible(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_visible)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getAttention (); */
    #[inline]
    pub unsafe fn getAttention(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).getAttention)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


