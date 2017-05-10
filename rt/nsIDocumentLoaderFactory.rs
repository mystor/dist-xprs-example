//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocumentLoaderFactory.idl
//


#[repr(C)]
pub struct nsIDocumentLoaderFactory {
    vtable: *const nsIDocumentLoaderFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocumentLoaderFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe795239e, 0x9d3c, 0x47c4,
            [0xb0, 0x63, 0x9e, 0x60, 0x0f, 0xb3, 0xb2, 0x87])
    }
}

unsafe impl RefCounted for nsIDocumentLoaderFactory {
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
pub trait nsIDocumentLoaderFactoryCoerce {
    fn coerce_from(v: &nsIDocumentLoaderFactory) -> &Self;
}

impl nsIDocumentLoaderFactoryCoerce for nsIDocumentLoaderFactory {
    #[inline]
    fn coerce_from(v: &nsIDocumentLoaderFactory) -> &Self {
        v
    }
}

impl nsIDocumentLoaderFactory {
    #[inline]
    pub fn coerce<T: nsIDocumentLoaderFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocumentLoaderFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocumentLoaderFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentLoaderFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocumentLoaderFactoryVTable {
    pub __base: nsISupportsVTable,

    /* nsIContentViewer createInstance (in string aCommand, in nsIChannel aChannel, in nsILoadGroup aLoadGroup, in ACString aContentType, in nsIDocShell aContainer, in nsISupports aExtraInfo, out nsIStreamListener aDocListenerResult); */
    pub createInstance: unsafe extern "C" fn (this: *const nsIDocumentLoaderFactory, aCommand: *const libc::c_char, aChannel: *const nsIChannel, aLoadGroup: *const nsILoadGroup, aContentType: *const nsACString, aContainer: *const nsIDocShell, aExtraInfo: *const nsISupports, aDocListenerResult: *mut *const nsIStreamListener, _retval: *mut *const nsIContentViewer) -> nsresult,

    /* nsIContentViewer createInstanceForDocument (in nsISupports aContainer, in nsIDocument aDocument, in string aCommand); */
    pub createInstanceForDocument: unsafe extern "C" fn (this: *const nsIDocumentLoaderFactory, aContainer: *const nsISupports, aDocument: *const nsIDocument, aCommand: *const libc::c_char, _retval: *mut *const nsIContentViewer) -> nsresult,

    /* nsIDocument createBlankDocument (in nsILoadGroup aLoadGroup, in nsIPrincipal aPrincipal); */
    pub createBlankDocument: unsafe extern "C" fn (this: *const nsIDocumentLoaderFactory, aLoadGroup: *const nsILoadGroup, aPrincipal: *const nsIPrincipal, _retval: *mut *const nsIDocument) -> nsresult,

}


impl nsIDocumentLoaderFactory {
    /* nsIContentViewer createInstance (in string aCommand, in nsIChannel aChannel, in nsILoadGroup aLoadGroup, in ACString aContentType, in nsIDocShell aContainer, in nsISupports aExtraInfo, out nsIStreamListener aDocListenerResult); */
    #[inline]
    pub unsafe fn createInstance(&self, aCommand: *const libc::c_char, aChannel: Option<&nsIChannel>, aLoadGroup: Option<&nsILoadGroup>, aContentType: &[u8], aContainer: Option<&nsIDocShell>, aExtraInfo: Option<&nsISupports>) -> Result<(Option<RefPtr<nsIStreamListener>>, Option<RefPtr<nsIContentViewer>>), nsresult> {
        let aContentType = nsCString::from(aContentType);
        let mut aDocListenerResult = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createInstance)(self as *const _, aCommand, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aLoadGroup.map_or(::std::ptr::null(), |x| x as *const _), &*aContentType, aContainer.map_or(::std::ptr::null(), |x| x as *const _), aExtraInfo.map_or(::std::ptr::null(), |x| x as *const _), aDocListenerResult.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aDocListenerResult.refptr(), _retval.refptr()))
    }

    /* nsIContentViewer createInstanceForDocument (in nsISupports aContainer, in nsIDocument aDocument, in string aCommand); */
    #[inline]
    pub unsafe fn createInstanceForDocument(&self, aContainer: Option<&nsISupports>, aDocument: Option<&nsIDocument>, aCommand: *const libc::c_char) -> Result<Option<RefPtr<nsIContentViewer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createInstanceForDocument)(self as *const _, aContainer.map_or(::std::ptr::null(), |x| x as *const _), aDocument.map_or(::std::ptr::null(), |x| x as *const _), aCommand, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDocument createBlankDocument (in nsILoadGroup aLoadGroup, in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn createBlankDocument(&self, aLoadGroup: Option<&nsILoadGroup>, aPrincipal: Option<&nsIPrincipal>) -> Result<Option<RefPtr<nsIDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createBlankDocument)(self as *const _, aLoadGroup.map_or(::std::ptr::null(), |x| x as *const _), aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


