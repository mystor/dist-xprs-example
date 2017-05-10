//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMediaManager.idl
//


#[repr(C)]
pub struct nsIMediaManagerService {
    vtable: *const nsIMediaManagerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMediaManagerService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x24b23e01, 0x33fd, 0x401f,
            [0xba, 0x25, 0x6e, 0x52, 0x65, 0x87, 0x50, 0xb0])
    }
}

unsafe impl RefCounted for nsIMediaManagerService {
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
pub trait nsIMediaManagerServiceCoerce {
    fn coerce_from(v: &nsIMediaManagerService) -> &Self;
}

impl nsIMediaManagerServiceCoerce for nsIMediaManagerService {
    #[inline]
    fn coerce_from(v: &nsIMediaManagerService) -> &Self {
        v
    }
}

impl nsIMediaManagerService {
    #[inline]
    pub fn coerce<T: nsIMediaManagerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMediaManagerService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMediaManagerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMediaManagerService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMediaManagerServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIArray activeMediaCaptureWindows; */
    pub get_activeMediaCaptureWindows: unsafe extern "C" fn (this: *const nsIMediaManagerService, aActiveMediaCaptureWindows: *mut *const nsIArray) -> nsresult,

    /* void mediaCaptureWindowState (in nsIDOMWindow aWindow, out boolean aVideo, out boolean aAudio, [optional] out boolean aScreenShare, [optional] out boolean aWindowShare, [optional] out boolean aAppShare, [optional] out boolean aBrowserShare); */
    pub mediaCaptureWindowState: unsafe extern "C" fn (this: *const nsIMediaManagerService, aWindow: *const nsIDOMWindow, aVideo: *mut bool, aAudio: *mut bool, aScreenShare: *mut bool, aWindowShare: *mut bool, aAppShare: *mut bool, aBrowserShare: *mut bool) -> nsresult,

    /* void sanitizeDeviceIds (in long long sinceWhen); */
    pub sanitizeDeviceIds: unsafe extern "C" fn (this: *const nsIMediaManagerService, sinceWhen: libc::int64_t) -> nsresult,

}


impl nsIMediaManagerService {
    /* readonly attribute nsIArray activeMediaCaptureWindows; */
    #[inline]
    pub unsafe fn get_activeMediaCaptureWindows(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activeMediaCaptureWindows)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void mediaCaptureWindowState (in nsIDOMWindow aWindow, out boolean aVideo, out boolean aAudio, [optional] out boolean aScreenShare, [optional] out boolean aWindowShare, [optional] out boolean aAppShare, [optional] out boolean aBrowserShare); */
    #[inline]
    pub unsafe fn mediaCaptureWindowState(&self, aWindow: Option<&nsIDOMWindow>) -> Result<(bool, bool, bool, bool, bool, bool), nsresult> {
        let mut aVideo: bool = ::std::mem::zeroed();
        let mut aAudio: bool = ::std::mem::zeroed();
        let mut aScreenShare: bool = ::std::mem::zeroed();
        let mut aWindowShare: bool = ::std::mem::zeroed();
        let mut aAppShare: bool = ::std::mem::zeroed();
        let mut aBrowserShare: bool = ::std::mem::zeroed();
        match ((*self.vtable).mediaCaptureWindowState)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut aVideo as *mut _, &mut aAudio as *mut _, &mut aScreenShare as *mut _, &mut aWindowShare as *mut _, &mut aAppShare as *mut _, &mut aBrowserShare as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aVideo, aAudio, aScreenShare, aWindowShare, aAppShare, aBrowserShare))
    }

    /* void sanitizeDeviceIds (in long long sinceWhen); */
    #[inline]
    pub unsafe fn sanitizeDeviceIds(&self, sinceWhen: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).sanitizeDeviceIds)(self as *const _, sinceWhen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


