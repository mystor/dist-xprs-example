//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserPersistDocument.idl
//


#[repr(C)]
pub struct nsIWebBrowserPersistURIMap {
    vtable: *const nsIWebBrowserPersistURIMapVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserPersistURIMap {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd52e8b93, 0x2771, 0x45e8,
            [0xa5, 0xb0, 0x6e, 0x12, 0xb6, 0x67, 0x04, 0x6b])
    }
}

unsafe impl RefCounted for nsIWebBrowserPersistURIMap {
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
pub trait nsIWebBrowserPersistURIMapCoerce {
    fn coerce_from(v: &nsIWebBrowserPersistURIMap) -> &Self;
}

impl nsIWebBrowserPersistURIMapCoerce for nsIWebBrowserPersistURIMap {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistURIMap) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistURIMap {
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistURIMapCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserPersistURIMap {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserPersistURIMapCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistURIMap) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserPersistURIMapVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long numMappedURIs; */
    pub get_numMappedURIs: unsafe extern "C" fn (this: *const nsIWebBrowserPersistURIMap, aNumMappedURIs: *mut libc::uint32_t) -> nsresult,

    /* void getURIMapping (in unsigned long aIndex, out AUTF8String aMapFrom, out AUTF8String aMapTo); */
    pub getURIMapping: unsafe extern "C" fn (this: *const nsIWebBrowserPersistURIMap, aIndex: libc::uint32_t, aMapFrom: *mut nsACString, aMapTo: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String targetBaseURI; */
    pub get_targetBaseURI: unsafe extern "C" fn (this: *const nsIWebBrowserPersistURIMap, aTargetBaseURI: *mut nsACString) -> nsresult,

}


impl nsIWebBrowserPersistURIMap {
    /* readonly attribute unsigned long numMappedURIs; */
    #[inline]
    pub unsafe fn get_numMappedURIs(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numMappedURIs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getURIMapping (in unsigned long aIndex, out AUTF8String aMapFrom, out AUTF8String aMapTo); */
    #[inline]
    pub unsafe fn getURIMapping(&self, aIndex: libc::uint32_t) -> Result<(nsCString, nsCString), nsresult> {
        let mut aMapFrom = nsCString::new();
        let mut aMapTo = nsCString::new();
        match ((*self.vtable).getURIMapping)(self as *const _, aIndex, &mut *aMapFrom, &mut *aMapTo) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMapFrom, aMapTo))
    }

    /* readonly attribute AUTF8String targetBaseURI; */
    #[inline]
    pub unsafe fn get_targetBaseURI(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_targetBaseURI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIWebBrowserPersistDocument {
    vtable: *const nsIWebBrowserPersistDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserPersistDocument {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x74aa4918, 0x5d15, 0x46b6,
            [0x9c, 0xcf, 0x74, 0xf9, 0x69, 0x6d, 0x72, 0x1d])
    }
}

unsafe impl RefCounted for nsIWebBrowserPersistDocument {
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
pub trait nsIWebBrowserPersistDocumentCoerce {
    fn coerce_from(v: &nsIWebBrowserPersistDocument) -> &Self;
}

impl nsIWebBrowserPersistDocumentCoerce for nsIWebBrowserPersistDocument {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistDocument) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistDocument {
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserPersistDocument {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserPersistDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistDocument) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserPersistDocumentVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isPrivate; */
    pub get_isPrivate: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aIsPrivate: *mut bool) -> nsresult,

    /* readonly attribute AUTF8String documentURI; */
    pub get_documentURI: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aDocumentURI: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String baseURI; */
    pub get_baseURI: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aBaseURI: *mut nsACString) -> nsresult,

    /* readonly attribute ACString contentType; */
    pub get_contentType: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aContentType: *mut nsACString) -> nsresult,

    /* readonly attribute ACString characterSet; */
    pub get_characterSet: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aCharacterSet: *mut nsACString) -> nsresult,

    /* readonly attribute AString title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aTitle: *mut nsAString) -> nsresult,

    /* readonly attribute AString referrer; */
    pub get_referrer: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aReferrer: *mut nsAString) -> nsresult,

    /* readonly attribute AString contentDisposition; */
    pub get_contentDisposition: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aContentDisposition: *mut nsAString) -> nsresult,

    /* readonly attribute nsIInputStream postData; */
    pub get_postData: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aPostData: *mut *const nsIInputStream) -> nsresult,

    /* readonly attribute unsigned long cacheKey; */
    pub get_cacheKey: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aCacheKey: *mut libc::uint32_t) -> nsresult,

    /* attribute unsigned long persistFlags; */
    pub get_persistFlags: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aPersistFlags: *mut libc::uint32_t) -> nsresult,
    pub set_persistFlags: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aPersistFlags: libc::uint32_t) -> nsresult,

    /* void readResources (in nsIWebBrowserPersistResourceVisitor aVisitor); */
    pub readResources: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aVisitor: *const nsIWebBrowserPersistResourceVisitor) -> nsresult,

    /* void writeContent (in nsIOutputStream aStream, in nsIWebBrowserPersistURIMap aURIMap, in ACString aRequestedContentType, in unsigned long aEncoderFlags, in unsigned long aWrapColumn, in nsIWebBrowserPersistWriteCompletion aCompletion); */
    pub writeContent: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocument, aStream: *const nsIOutputStream, aURIMap: *const nsIWebBrowserPersistURIMap, aRequestedContentType: *const nsACString, aEncoderFlags: libc::uint32_t, aWrapColumn: libc::uint32_t, aCompletion: *const nsIWebBrowserPersistWriteCompletion) -> nsresult,

}


impl nsIWebBrowserPersistDocument {
    /* readonly attribute boolean isPrivate; */
    #[inline]
    pub unsafe fn get_isPrivate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPrivate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String documentURI; */
    #[inline]
    pub unsafe fn get_documentURI(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_documentURI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String baseURI; */
    #[inline]
    pub unsafe fn get_baseURI(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_baseURI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString contentType; */
    #[inline]
    pub unsafe fn get_contentType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_contentType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString characterSet; */
    #[inline]
    pub unsafe fn get_characterSet(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_characterSet)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_title)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString referrer; */
    #[inline]
    pub unsafe fn get_referrer(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_referrer)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString contentDisposition; */
    #[inline]
    pub unsafe fn get_contentDisposition(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_contentDisposition)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIInputStream postData; */
    #[inline]
    pub unsafe fn get_postData(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_postData)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long cacheKey; */
    #[inline]
    pub unsafe fn get_cacheKey(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_cacheKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute unsigned long persistFlags; */
    #[inline]
    pub unsafe fn get_persistFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_persistFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_persistFlags(&self, aPersistFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_persistFlags)(self as *const _, aPersistFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void readResources (in nsIWebBrowserPersistResourceVisitor aVisitor); */
    #[inline]
    pub unsafe fn readResources(&self, aVisitor: Option<&nsIWebBrowserPersistResourceVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).readResources)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeContent (in nsIOutputStream aStream, in nsIWebBrowserPersistURIMap aURIMap, in ACString aRequestedContentType, in unsigned long aEncoderFlags, in unsigned long aWrapColumn, in nsIWebBrowserPersistWriteCompletion aCompletion); */
    #[inline]
    pub unsafe fn writeContent(&self, aStream: Option<&nsIOutputStream>, aURIMap: Option<&nsIWebBrowserPersistURIMap>, aRequestedContentType: &[u8], aEncoderFlags: libc::uint32_t, aWrapColumn: libc::uint32_t, aCompletion: Option<&nsIWebBrowserPersistWriteCompletion>) -> Result<(), nsresult> {
        let aRequestedContentType = nsCString::from(aRequestedContentType);
        match ((*self.vtable).writeContent)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), aURIMap.map_or(::std::ptr::null(), |x| x as *const _), &*aRequestedContentType, aEncoderFlags, aWrapColumn, aCompletion.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIWebBrowserPersistResourceVisitor {
    vtable: *const nsIWebBrowserPersistResourceVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserPersistResourceVisitor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8ce37706, 0xb7d3, 0x481a,
            [0xbe, 0x68, 0x54, 0xf1, 0x74, 0xfc, 0x0d, 0x0a])
    }
}

unsafe impl RefCounted for nsIWebBrowserPersistResourceVisitor {
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
pub trait nsIWebBrowserPersistResourceVisitorCoerce {
    fn coerce_from(v: &nsIWebBrowserPersistResourceVisitor) -> &Self;
}

impl nsIWebBrowserPersistResourceVisitorCoerce for nsIWebBrowserPersistResourceVisitor {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistResourceVisitor) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistResourceVisitor {
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistResourceVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserPersistResourceVisitor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserPersistResourceVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistResourceVisitor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserPersistResourceVisitorVTable {
    pub __base: nsISupportsVTable,

    /* void visitResource (in nsIWebBrowserPersistDocument aDocument, in AUTF8String aURI); */
    pub visitResource: unsafe extern "C" fn (this: *const nsIWebBrowserPersistResourceVisitor, aDocument: *const nsIWebBrowserPersistDocument, aURI: *const nsACString) -> nsresult,

    /* void visitDocument (in nsIWebBrowserPersistDocument aDocument, in nsIWebBrowserPersistDocument aSubDocument); */
    pub visitDocument: unsafe extern "C" fn (this: *const nsIWebBrowserPersistResourceVisitor, aDocument: *const nsIWebBrowserPersistDocument, aSubDocument: *const nsIWebBrowserPersistDocument) -> nsresult,

    /* void endVisit (in nsIWebBrowserPersistDocument aDocument, in nsresult aStatus); */
    pub endVisit: unsafe extern "C" fn (this: *const nsIWebBrowserPersistResourceVisitor, aDocument: *const nsIWebBrowserPersistDocument, aStatus: nsresult) -> nsresult,

}


impl nsIWebBrowserPersistResourceVisitor {
    /* void visitResource (in nsIWebBrowserPersistDocument aDocument, in AUTF8String aURI); */
    #[inline]
    pub unsafe fn visitResource(&self, aDocument: Option<&nsIWebBrowserPersistDocument>, aURI: &[u8]) -> Result<(), nsresult> {
        let aURI = nsCString::from(aURI);
        match ((*self.vtable).visitResource)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), &*aURI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void visitDocument (in nsIWebBrowserPersistDocument aDocument, in nsIWebBrowserPersistDocument aSubDocument); */
    #[inline]
    pub unsafe fn visitDocument(&self, aDocument: Option<&nsIWebBrowserPersistDocument>, aSubDocument: Option<&nsIWebBrowserPersistDocument>) -> Result<(), nsresult> {

        match ((*self.vtable).visitDocument)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), aSubDocument.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endVisit (in nsIWebBrowserPersistDocument aDocument, in nsresult aStatus); */
    #[inline]
    pub unsafe fn endVisit(&self, aDocument: Option<&nsIWebBrowserPersistDocument>, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).endVisit)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIWebBrowserPersistWriteCompletion {
    vtable: *const nsIWebBrowserPersistWriteCompletionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserPersistWriteCompletion {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa07e6892, 0x38ae, 0x4207,
            [0x83, 0x40, 0x7f, 0xa6, 0xec, 0x44, 0x6e, 0xd6])
    }
}

unsafe impl RefCounted for nsIWebBrowserPersistWriteCompletion {
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
pub trait nsIWebBrowserPersistWriteCompletionCoerce {
    fn coerce_from(v: &nsIWebBrowserPersistWriteCompletion) -> &Self;
}

impl nsIWebBrowserPersistWriteCompletionCoerce for nsIWebBrowserPersistWriteCompletion {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistWriteCompletion) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistWriteCompletion {
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistWriteCompletionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserPersistWriteCompletion {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserPersistWriteCompletionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistWriteCompletion) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserPersistWriteCompletionVTable {
    pub __base: nsISupportsVTable,

    /* void onFinish (in nsIWebBrowserPersistDocument aDocument, in nsIOutputStream aStream, in ACString aContentType, in nsresult aStatus); */
    pub onFinish: unsafe extern "C" fn (this: *const nsIWebBrowserPersistWriteCompletion, aDocument: *const nsIWebBrowserPersistDocument, aStream: *const nsIOutputStream, aContentType: *const nsACString, aStatus: nsresult) -> nsresult,

}


impl nsIWebBrowserPersistWriteCompletion {
    /* void onFinish (in nsIWebBrowserPersistDocument aDocument, in nsIOutputStream aStream, in ACString aContentType, in nsresult aStatus); */
    #[inline]
    pub unsafe fn onFinish(&self, aDocument: Option<&nsIWebBrowserPersistDocument>, aStream: Option<&nsIOutputStream>, aContentType: &[u8], aStatus: nsresult) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        match ((*self.vtable).onFinish)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), aStream.map_or(::std::ptr::null(), |x| x as *const _), &*aContentType, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIWebBrowserPersistDocumentReceiver {
    vtable: *const nsIWebBrowserPersistDocumentReceiverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserPersistDocumentReceiver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x321e3174, 0x594f, 0x4036,
            [0xb7, 0xbe, 0x79, 0x1b, 0x82, 0x1b, 0xd3, 0x76])
    }
}

unsafe impl RefCounted for nsIWebBrowserPersistDocumentReceiver {
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
pub trait nsIWebBrowserPersistDocumentReceiverCoerce {
    fn coerce_from(v: &nsIWebBrowserPersistDocumentReceiver) -> &Self;
}

impl nsIWebBrowserPersistDocumentReceiverCoerce for nsIWebBrowserPersistDocumentReceiver {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistDocumentReceiver) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistDocumentReceiver {
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistDocumentReceiverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserPersistDocumentReceiver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserPersistDocumentReceiverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistDocumentReceiver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserPersistDocumentReceiverVTable {
    pub __base: nsISupportsVTable,

    /* void onDocumentReady (in nsIWebBrowserPersistDocument aDocument); */
    pub onDocumentReady: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocumentReceiver, aDocument: *const nsIWebBrowserPersistDocument) -> nsresult,

    /* void onError (in nsresult aFailure); */
    pub onError: unsafe extern "C" fn (this: *const nsIWebBrowserPersistDocumentReceiver, aFailure: nsresult) -> nsresult,

}


impl nsIWebBrowserPersistDocumentReceiver {
    /* void onDocumentReady (in nsIWebBrowserPersistDocument aDocument); */
    #[inline]
    pub unsafe fn onDocumentReady(&self, aDocument: Option<&nsIWebBrowserPersistDocument>) -> Result<(), nsresult> {

        match ((*self.vtable).onDocumentReady)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onError (in nsresult aFailure); */
    #[inline]
    pub unsafe fn onError(&self, aFailure: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onError)(self as *const _, aFailure) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


