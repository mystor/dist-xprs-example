//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIImageLoadingContent.idl
//


pub mod nsIImageLoadingContent_consts {
    pub const UNKNOWN_REQUEST: i64 = -1;
    pub const CURRENT_REQUEST: i64 = 0;
    pub const PENDING_REQUEST: i64 = 1;
}


#[repr(C)]
pub struct nsIImageLoadingContent {
    vtable: *const nsIImageLoadingContentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIImageLoadingContent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0357123d, 0x9224, 0x4d12,
            [0xa4, 0x7e, 0x86, 0x8c, 0x32, 0x68, 0x97, 0x77])
    }
}

unsafe impl RefCounted for nsIImageLoadingContent {
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
pub trait nsIImageLoadingContentCoerce {
    fn coerce_from(v: &nsIImageLoadingContent) -> &Self;
}

impl nsIImageLoadingContentCoerce for nsIImageLoadingContent {
    #[inline]
    fn coerce_from(v: &nsIImageLoadingContent) -> &Self {
        v
    }
}

impl nsIImageLoadingContent {
    #[inline]
    pub fn coerce<T: nsIImageLoadingContentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIImageLoadingContent {
    type Target = imgINotificationObserver;
    #[inline]
    fn deref(&self) -> &imgINotificationObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: imgINotificationObserverCoerce> nsIImageLoadingContentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIImageLoadingContent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIImageLoadingContentVTable {
    pub __base: imgINotificationObserverVTable,

    /* attribute boolean loadingEnabled; */
    pub get_loadingEnabled: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aLoadingEnabled: *mut bool) -> nsresult,
    pub set_loadingEnabled: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aLoadingEnabled: bool) -> nsresult,

    /* readonly attribute short imageBlockingStatus; */
    pub get_imageBlockingStatus: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aImageBlockingStatus: *mut libc::int16_t) -> nsresult,

    /* void addObserver (in imgINotificationObserver aObserver); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aObserver: *const imgINotificationObserver) -> nsresult,

    /* void removeObserver (in imgINotificationObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aObserver: *const imgINotificationObserver) -> nsresult,

    /* imgIRequest getRequest (in long aRequestType); */
    pub getRequest: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aRequestType: libc::int32_t, _retval: *mut *const imgIRequest) -> nsresult,

    /* [noscript,notxpcom] boolean currentRequestHasSize (); */
    pub currentRequestHasSize: unsafe extern "C" fn (this: *const nsIImageLoadingContent) -> bool,

    /* [notxpcom] void frameCreated (in nsIFrame aFrame); */
    /// Unable to call function as its signature contains a non-rust type
    pub frameCreated: *const ::libc::c_void,

    /* [notxpcom] void frameDestroyed (in nsIFrame aFrame); */
    /// Unable to call function as its signature contains a non-rust type
    pub frameDestroyed: *const ::libc::c_void,

    /* long getRequestType (in imgIRequest aRequest); */
    pub getRequestType: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aRequest: *const imgIRequest, _retval: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIURI currentURI; */
    pub get_currentURI: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aCurrentURI: *mut *const nsIURI) -> nsresult,

    /* nsIStreamListener loadImageWithChannel (in nsIChannel aChannel); */
    pub loadImageWithChannel: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aChannel: *const nsIChannel, _retval: *mut *const nsIStreamListener) -> nsresult,

    /* [optional_argc] void forceReload ([optional] in boolean aNotify); */
    /// Unable to call function as its signature contains a non-rust type
    pub forceReload: *const ::libc::c_void,

    /* void forceImageState (in boolean aForce, in unsigned long long aState); */
    pub forceImageState: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aForce: bool, aState: libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long naturalWidth; */
    pub get_naturalWidth: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aNaturalWidth: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long naturalHeight; */
    pub get_naturalHeight: unsafe extern "C" fn (this: *const nsIImageLoadingContent, aNaturalHeight: *mut libc::uint32_t) -> nsresult,

    /* [noscript,notxpcom] void onVisibilityChange (in Visibility aNewVisibility, in MaybeOnNonvisible aNonvisibleAction); */
    /// Unable to call function as its signature contains a non-rust type
    pub onVisibilityChange: *const ::libc::c_void,

}


impl nsIImageLoadingContent {
    /* attribute boolean loadingEnabled; */
    #[inline]
    pub unsafe fn get_loadingEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loadingEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_loadingEnabled(&self, aLoadingEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadingEnabled)(self as *const _, aLoadingEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute short imageBlockingStatus; */
    #[inline]
    pub unsafe fn get_imageBlockingStatus(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_imageBlockingStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addObserver (in imgINotificationObserver aObserver); */
    #[inline]
    pub unsafe fn addObserver(&self, aObserver: Option<&imgINotificationObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in imgINotificationObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aObserver: Option<&imgINotificationObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* imgIRequest getRequest (in long aRequestType); */
    #[inline]
    pub unsafe fn getRequest(&self, aRequestType: libc::int32_t) -> Result<Option<RefPtr<imgIRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRequest)(self as *const _, aRequestType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,notxpcom] boolean currentRequestHasSize (); */
    #[inline]
    pub unsafe fn currentRequestHasSize(&self, ) -> bool {

        let _retval = ((*self.vtable).currentRequestHasSize)(self as *const _, );
        _retval
    }

    /* [notxpcom] void frameCreated (in nsIFrame aFrame); */


    /* [notxpcom] void frameDestroyed (in nsIFrame aFrame); */


    /* long getRequestType (in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn getRequestType(&self, aRequest: Option<&imgIRequest>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRequestType)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI currentURI; */
    #[inline]
    pub unsafe fn get_currentURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIStreamListener loadImageWithChannel (in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn loadImageWithChannel(&self, aChannel: Option<&nsIChannel>) -> Result<Option<RefPtr<nsIStreamListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).loadImageWithChannel)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [optional_argc] void forceReload ([optional] in boolean aNotify); */


    /* void forceImageState (in boolean aForce, in unsigned long long aState); */
    #[inline]
    pub unsafe fn forceImageState(&self, aForce: bool, aState: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).forceImageState)(self as *const _, aForce, aState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long naturalWidth; */
    #[inline]
    pub unsafe fn get_naturalWidth(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_naturalWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long naturalHeight; */
    #[inline]
    pub unsafe fn get_naturalHeight(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_naturalHeight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,notxpcom] void onVisibilityChange (in Visibility aNewVisibility, in MaybeOnNonvisible aNonvisibleAction); */


}


