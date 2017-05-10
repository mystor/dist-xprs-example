//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIRequest.idl
//


pub mod imgIRequest_consts {
    pub const STATUS_NONE: i64 = 0;
    pub const STATUS_SIZE_AVAILABLE: i64 = 1;
    pub const STATUS_LOAD_COMPLETE: i64 = 2;
    pub const STATUS_ERROR: i64 = 4;
    pub const STATUS_FRAME_COMPLETE: i64 = 8;
    pub const STATUS_DECODE_COMPLETE: i64 = 16;
    pub const STATUS_IS_ANIMATED: i64 = 32;
    pub const STATUS_HAS_TRANSPARENCY: i64 = 64;
    pub const CORS_NONE: i64 = 1;
    pub const CORS_ANONYMOUS: i64 = 2;
    pub const CORS_USE_CREDENTIALS: i64 = 3;
    pub const CATEGORY_FRAME_INIT: i64 = 1;
    pub const CATEGORY_SIZE_QUERY: i64 = 2;
    pub const CATEGORY_DISPLAY: i64 = 4;
}


#[repr(C)]
pub struct imgIRequest {
    vtable: *const imgIRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgIRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdb0a945c, 0x3883, 0x424a,
            [0x98, 0xd0, 0x2e, 0xe0, 0x52, 0x3b, 0x02, 0x55])
    }
}

unsafe impl RefCounted for imgIRequest {
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
pub trait imgIRequestCoerce {
    fn coerce_from(v: &imgIRequest) -> &Self;
}

impl imgIRequestCoerce for imgIRequest {
    #[inline]
    fn coerce_from(v: &imgIRequest) -> &Self {
        v
    }
}

impl imgIRequest {
    #[inline]
    pub fn coerce<T: imgIRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgIRequest {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestCoerce> imgIRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgIRequestVTable {
    pub __base: nsIRequestVTable,

    /* readonly attribute imgIContainer image; */
    pub get_image: unsafe extern "C" fn (this: *const imgIRequest, aImage: *mut *const imgIContainer) -> nsresult,

    /* readonly attribute unsigned long imageStatus; */
    pub get_imageStatus: unsafe extern "C" fn (this: *const imgIRequest, aImageStatus: *mut libc::uint32_t) -> nsresult,

    /* [noscript] readonly attribute nsresult imageErrorCode; */
    pub get_imageErrorCode: unsafe extern "C" fn (this: *const imgIRequest, aImageErrorCode: *mut nsresult) -> nsresult,

    /* readonly attribute nsIURI URI; */
    pub get_URI: unsafe extern "C" fn (this: *const imgIRequest, aURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI currentURI; */
    pub get_currentURI: unsafe extern "C" fn (this: *const imgIRequest, aCurrentURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute imgINotificationObserver notificationObserver; */
    pub get_notificationObserver: unsafe extern "C" fn (this: *const imgIRequest, aNotificationObserver: *mut *const imgINotificationObserver) -> nsresult,

    /* readonly attribute string mimeType; */
    pub get_mimeType: unsafe extern "C" fn (this: *const imgIRequest, aMimeType: *mut *const libc::c_char) -> nsresult,

    /* imgIRequest clone (in imgINotificationObserver aObserver); */
    pub clone: unsafe extern "C" fn (this: *const imgIRequest, aObserver: *const imgINotificationObserver, _retval: *mut *const imgIRequest) -> nsresult,

    /* readonly attribute nsIPrincipal imagePrincipal; */
    pub get_imagePrincipal: unsafe extern "C" fn (this: *const imgIRequest, aImagePrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* readonly attribute bool multipart; */
    pub get_multipart: unsafe extern "C" fn (this: *const imgIRequest, aMultipart: *mut bool) -> nsresult,

    /* readonly attribute long CORSMode; */
    pub get_CORSMode: unsafe extern "C" fn (this: *const imgIRequest, aCORSMode: *mut libc::int32_t) -> nsresult,

    /* void cancelAndForgetObserver (in nsresult aStatus); */
    pub cancelAndForgetObserver: unsafe extern "C" fn (this: *const imgIRequest, aStatus: nsresult) -> nsresult,

    /* void startDecoding (in uint32_t aFlags); */
    pub startDecoding: unsafe extern "C" fn (this: *const imgIRequest, aFlags: uint32_t) -> nsresult,

    /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags); */
    pub startDecodingWithResult: unsafe extern "C" fn (this: *const imgIRequest, aFlags: uint32_t) -> bool,

    /* void lockImage (); */
    pub lockImage: unsafe extern "C" fn (this: *const imgIRequest) -> nsresult,

    /* void unlockImage (); */
    pub unlockImage: unsafe extern "C" fn (this: *const imgIRequest) -> nsresult,

    /* void requestDiscard (); */
    pub requestDiscard: unsafe extern "C" fn (this: *const imgIRequest) -> nsresult,

    /* imgIRequest getStaticRequest (); */
    pub getStaticRequest: unsafe extern "C" fn (this: *const imgIRequest, _retval: *mut *const imgIRequest) -> nsresult,

    /* void incrementAnimationConsumers (); */
    pub incrementAnimationConsumers: unsafe extern "C" fn (this: *const imgIRequest) -> nsresult,

    /* void decrementAnimationConsumers (); */
    pub decrementAnimationConsumers: unsafe extern "C" fn (this: *const imgIRequest) -> nsresult,

    /* void boostPriority (in uint32_t aCategory); */
    pub boostPriority: unsafe extern "C" fn (this: *const imgIRequest, aCategory: uint32_t) -> nsresult,

}


impl imgIRequest {
    /* readonly attribute imgIContainer image; */
    #[inline]
    pub unsafe fn get_image(&self, ) -> Result<Option<RefPtr<imgIContainer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_image)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long imageStatus; */
    #[inline]
    pub unsafe fn get_imageStatus(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_imageStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute nsresult imageErrorCode; */
    #[inline]
    pub unsafe fn get_imageErrorCode(&self, ) -> Result<nsresult, nsresult> {
        let mut _retval: nsresult = ::std::mem::zeroed();
        match ((*self.vtable).get_imageErrorCode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI URI; */
    #[inline]
    pub unsafe fn get_URI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_URI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
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

    /* readonly attribute imgINotificationObserver notificationObserver; */
    #[inline]
    pub unsafe fn get_notificationObserver(&self, ) -> Result<Option<RefPtr<imgINotificationObserver>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_notificationObserver)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute string mimeType; */
    #[inline]
    pub unsafe fn get_mimeType(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_mimeType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* imgIRequest clone (in imgINotificationObserver aObserver); */
    #[inline]
    pub unsafe fn clone(&self, aObserver: Option<&imgINotificationObserver>) -> Result<Option<RefPtr<imgIRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIPrincipal imagePrincipal; */
    #[inline]
    pub unsafe fn get_imagePrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_imagePrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute bool multipart; */
    #[inline]
    pub unsafe fn get_multipart(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_multipart)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long CORSMode; */
    #[inline]
    pub unsafe fn get_CORSMode(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_CORSMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void cancelAndForgetObserver (in nsresult aStatus); */
    #[inline]
    pub unsafe fn cancelAndForgetObserver(&self, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).cancelAndForgetObserver)(self as *const _, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void startDecoding (in uint32_t aFlags); */
    #[inline]
    pub unsafe fn startDecoding(&self, aFlags: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).startDecoding)(self as *const _, aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags); */
    #[inline]
    pub unsafe fn startDecodingWithResult(&self, aFlags: uint32_t) -> bool {

        let _retval = ((*self.vtable).startDecodingWithResult)(self as *const _, aFlags);
        _retval
    }

    /* void lockImage (); */
    #[inline]
    pub unsafe fn lockImage(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).lockImage)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unlockImage (); */
    #[inline]
    pub unsafe fn unlockImage(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unlockImage)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestDiscard (); */
    #[inline]
    pub unsafe fn requestDiscard(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).requestDiscard)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* imgIRequest getStaticRequest (); */
    #[inline]
    pub unsafe fn getStaticRequest(&self, ) -> Result<Option<RefPtr<imgIRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getStaticRequest)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void incrementAnimationConsumers (); */
    #[inline]
    pub unsafe fn incrementAnimationConsumers(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).incrementAnimationConsumers)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void decrementAnimationConsumers (); */
    #[inline]
    pub unsafe fn decrementAnimationConsumers(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).decrementAnimationConsumers)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void boostPriority (in uint32_t aCategory); */
    #[inline]
    pub unsafe fn boostPriority(&self, aCategory: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).boostPriority)(self as *const _, aCategory) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


