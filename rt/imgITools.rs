//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgITools.idl
//


#[repr(C)]
pub struct imgITools {
    vtable: *const imgIToolsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgITools {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4c2383a4, 0x931c, 0x484d,
            [0x8c, 0x4a, 0x97, 0x35, 0x90, 0xf6, 0x6e, 0x3f])
    }
}

unsafe impl RefCounted for imgITools {
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
pub trait imgIToolsCoerce {
    fn coerce_from(v: &imgITools) -> &Self;
}

impl imgIToolsCoerce for imgITools {
    #[inline]
    fn coerce_from(v: &imgITools) -> &Self {
        v
    }
}

impl imgITools {
    #[inline]
    pub fn coerce<T: imgIToolsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgITools {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> imgIToolsCoerce for T {
    #[inline]
    fn coerce_from(v: &imgITools) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgIToolsVTable {
    pub __base: nsISupportsVTable,

    /* imgIContainer decodeImage (in nsIInputStream aStream, in ACString aMimeType); */
    pub decodeImage: unsafe extern "C" fn (this: *const imgITools, aStream: *const nsIInputStream, aMimeType: *const nsACString, _retval: *mut *const imgIContainer) -> nsresult,

    /* [deprecated] void decodeImageData (in nsIInputStream aStream, in ACString aMimeType, inout imgIContainer aContainer); */
    pub decodeImageData: unsafe extern "C" fn (this: *const imgITools, aStream: *const nsIInputStream, aMimeType: *const nsACString, aContainer: *mut *const imgIContainer) -> nsresult,

    /* nsIInputStream encodeImage (in imgIContainer aContainer, in ACString aMimeType, [optional] in AString outputOptions); */
    pub encodeImage: unsafe extern "C" fn (this: *const imgITools, aContainer: *const imgIContainer, aMimeType: *const nsACString, outputOptions: *const nsAString, _retval: *mut *const nsIInputStream) -> nsresult,

    /* nsIInputStream encodeScaledImage (in imgIContainer aContainer, in ACString aMimeType, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
    pub encodeScaledImage: unsafe extern "C" fn (this: *const imgITools, aContainer: *const imgIContainer, aMimeType: *const nsACString, aWidth: libc::int32_t, aHeight: libc::int32_t, outputOptions: *const nsAString, _retval: *mut *const nsIInputStream) -> nsresult,

    /* imgILoader getImgLoaderForDocument (in nsIDOMDocument doc); */
    pub getImgLoaderForDocument: unsafe extern "C" fn (this: *const imgITools, doc: *const nsIDOMDocument, _retval: *mut *const imgILoader) -> nsresult,

    /* imgICache getImgCacheForDocument (in nsIDOMDocument doc); */
    pub getImgCacheForDocument: unsafe extern "C" fn (this: *const imgITools, doc: *const nsIDOMDocument, _retval: *mut *const imgICache) -> nsresult,

    /* nsIInputStream encodeCroppedImage (in imgIContainer aContainer, in ACString aMimeType, in long aOffsetX, in long aOffsetY, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
    pub encodeCroppedImage: unsafe extern "C" fn (this: *const imgITools, aContainer: *const imgIContainer, aMimeType: *const nsACString, aOffsetX: libc::int32_t, aOffsetY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t, outputOptions: *const nsAString, _retval: *mut *const nsIInputStream) -> nsresult,

    /* imgINotificationObserver createScriptedObserver (in imgIScriptedNotificationObserver aObserver); */
    pub createScriptedObserver: unsafe extern "C" fn (this: *const imgITools, aObserver: *const imgIScriptedNotificationObserver, _retval: *mut *const imgINotificationObserver) -> nsresult,

}


impl imgITools {
    /* imgIContainer decodeImage (in nsIInputStream aStream, in ACString aMimeType); */
    #[inline]
    pub unsafe fn decodeImage(&self, aStream: Option<&nsIInputStream>, aMimeType: &[u8]) -> Result<Option<RefPtr<imgIContainer>>, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).decodeImage)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [deprecated] void decodeImageData (in nsIInputStream aStream, in ACString aMimeType, inout imgIContainer aContainer); */
    #[inline]
    pub unsafe fn decodeImageData(&self, aStream: Option<&nsIInputStream>, aMimeType: &[u8]) -> Result<Option<RefPtr<imgIContainer>>, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let mut aContainer = GetterAddrefs::new();
        match ((*self.vtable).decodeImageData)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeType, aContainer.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aContainer.refptr())
    }

    /* nsIInputStream encodeImage (in imgIContainer aContainer, in ACString aMimeType, [optional] in AString outputOptions); */
    #[inline]
    pub unsafe fn encodeImage(&self, aContainer: Option<&imgIContainer>, aMimeType: &[u8], outputOptions: &[u16]) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let outputOptions = nsString::from(outputOptions);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).encodeImage)(self as *const _, aContainer.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeType, &*outputOptions, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIInputStream encodeScaledImage (in imgIContainer aContainer, in ACString aMimeType, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
    #[inline]
    pub unsafe fn encodeScaledImage(&self, aContainer: Option<&imgIContainer>, aMimeType: &[u8], aWidth: libc::int32_t, aHeight: libc::int32_t, outputOptions: &[u16]) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let outputOptions = nsString::from(outputOptions);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).encodeScaledImage)(self as *const _, aContainer.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeType, aWidth, aHeight, &*outputOptions, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* imgILoader getImgLoaderForDocument (in nsIDOMDocument doc); */
    #[inline]
    pub unsafe fn getImgLoaderForDocument(&self, doc: Option<&nsIDOMDocument>) -> Result<Option<RefPtr<imgILoader>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getImgLoaderForDocument)(self as *const _, doc.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* imgICache getImgCacheForDocument (in nsIDOMDocument doc); */
    #[inline]
    pub unsafe fn getImgCacheForDocument(&self, doc: Option<&nsIDOMDocument>) -> Result<Option<RefPtr<imgICache>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getImgCacheForDocument)(self as *const _, doc.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIInputStream encodeCroppedImage (in imgIContainer aContainer, in ACString aMimeType, in long aOffsetX, in long aOffsetY, in long aWidth, in long aHeight, [optional] in AString outputOptions); */
    #[inline]
    pub unsafe fn encodeCroppedImage(&self, aContainer: Option<&imgIContainer>, aMimeType: &[u8], aOffsetX: libc::int32_t, aOffsetY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t, outputOptions: &[u16]) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let outputOptions = nsString::from(outputOptions);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).encodeCroppedImage)(self as *const _, aContainer.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeType, aOffsetX, aOffsetY, aWidth, aHeight, &*outputOptions, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* imgINotificationObserver createScriptedObserver (in imgIScriptedNotificationObserver aObserver); */
    #[inline]
    pub unsafe fn createScriptedObserver(&self, aObserver: Option<&imgIScriptedNotificationObserver>) -> Result<Option<RefPtr<imgINotificationObserver>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createScriptedObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


