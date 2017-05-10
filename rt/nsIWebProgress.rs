//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebProgress.idl
//


pub mod nsIWebProgress_consts {
    pub const NOTIFY_STATE_REQUEST: i64 = 1;
    pub const NOTIFY_STATE_DOCUMENT: i64 = 2;
    pub const NOTIFY_STATE_NETWORK: i64 = 4;
    pub const NOTIFY_STATE_WINDOW: i64 = 8;
    pub const NOTIFY_STATE_ALL: i64 = 15;
    pub const NOTIFY_PROGRESS: i64 = 16;
    pub const NOTIFY_STATUS: i64 = 32;
    pub const NOTIFY_SECURITY: i64 = 64;
    pub const NOTIFY_LOCATION: i64 = 128;
    pub const NOTIFY_REFRESH: i64 = 256;
    pub const NOTIFY_ALL: i64 = 511;
}


#[repr(C)]
pub struct nsIWebProgress {
    vtable: *const nsIWebProgressVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebProgress {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc4d64640, 0xb332, 0x4db6,
            [0xa2, 0xa5, 0xe0, 0x85, 0x66, 0x00, 0x0d, 0xc9])
    }
}

unsafe impl RefCounted for nsIWebProgress {
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
pub trait nsIWebProgressCoerce {
    fn coerce_from(v: &nsIWebProgress) -> &Self;
}

impl nsIWebProgressCoerce for nsIWebProgress {
    #[inline]
    fn coerce_from(v: &nsIWebProgress) -> &Self {
        v
    }
}

impl nsIWebProgress {
    #[inline]
    pub fn coerce<T: nsIWebProgressCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebProgress {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebProgressCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebProgress) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebProgressVTable {
    pub __base: nsISupportsVTable,

    /* void addProgressListener (in nsIWebProgressListener aListener, in unsigned long aNotifyMask); */
    pub addProgressListener: unsafe extern "C" fn (this: *const nsIWebProgress, aListener: *const nsIWebProgressListener, aNotifyMask: libc::uint32_t) -> nsresult,

    /* void removeProgressListener (in nsIWebProgressListener aListener); */
    pub removeProgressListener: unsafe extern "C" fn (this: *const nsIWebProgress, aListener: *const nsIWebProgressListener) -> nsresult,

    /* readonly attribute mozIDOMWindowProxy DOMWindow; */
    pub get_DOMWindow: unsafe extern "C" fn (this: *const nsIWebProgress, aDOMWindow: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute uint64_t DOMWindowID; */
    pub get_DOMWindowID: unsafe extern "C" fn (this: *const nsIWebProgress, aDOMWindowID: *mut uint64_t) -> nsresult,

    /* readonly attribute boolean isTopLevel; */
    pub get_isTopLevel: unsafe extern "C" fn (this: *const nsIWebProgress, aIsTopLevel: *mut bool) -> nsresult,

    /* readonly attribute boolean isLoadingDocument; */
    pub get_isLoadingDocument: unsafe extern "C" fn (this: *const nsIWebProgress, aIsLoadingDocument: *mut bool) -> nsresult,

    /* readonly attribute unsigned long loadType; */
    pub get_loadType: unsafe extern "C" fn (this: *const nsIWebProgress, aLoadType: *mut libc::uint32_t) -> nsresult,

}


impl nsIWebProgress {
    /* void addProgressListener (in nsIWebProgressListener aListener, in unsigned long aNotifyMask); */
    #[inline]
    pub unsafe fn addProgressListener(&self, aListener: Option<&nsIWebProgressListener>, aNotifyMask: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).addProgressListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), aNotifyMask) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeProgressListener (in nsIWebProgressListener aListener); */
    #[inline]
    pub unsafe fn removeProgressListener(&self, aListener: Option<&nsIWebProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeProgressListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute mozIDOMWindowProxy DOMWindow; */
    #[inline]
    pub unsafe fn get_DOMWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DOMWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute uint64_t DOMWindowID; */
    #[inline]
    pub unsafe fn get_DOMWindowID(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_DOMWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isTopLevel; */
    #[inline]
    pub unsafe fn get_isTopLevel(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTopLevel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isLoadingDocument; */
    #[inline]
    pub unsafe fn get_isLoadingDocument(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isLoadingDocument)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long loadType; */
    #[inline]
    pub unsafe fn get_loadType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_loadType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


