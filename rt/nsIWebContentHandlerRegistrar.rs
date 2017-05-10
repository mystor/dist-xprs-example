//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebContentHandlerRegistrar.idl
//


#[repr(C)]
pub struct nsIWebContentHandlerRegistrar {
    vtable: *const nsIWebContentHandlerRegistrarVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebContentHandlerRegistrar {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x65a3fafd, 0x0e4a, 0x4b06,
            [0x8b, 0x4e, 0x6a, 0x61, 0x1d, 0xa6, 0x3d, 0x98])
    }
}

unsafe impl RefCounted for nsIWebContentHandlerRegistrar {
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
pub trait nsIWebContentHandlerRegistrarCoerce {
    fn coerce_from(v: &nsIWebContentHandlerRegistrar) -> &Self;
}

impl nsIWebContentHandlerRegistrarCoerce for nsIWebContentHandlerRegistrar {
    #[inline]
    fn coerce_from(v: &nsIWebContentHandlerRegistrar) -> &Self {
        v
    }
}

impl nsIWebContentHandlerRegistrar {
    #[inline]
    pub fn coerce<T: nsIWebContentHandlerRegistrarCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebContentHandlerRegistrar {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebContentHandlerRegistrarCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebContentHandlerRegistrar) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebContentHandlerRegistrarVTable {
    pub __base: nsISupportsVTable,

    /* void registerContentHandler (in DOMString mimeType, in DOMString uri, in DOMString title, in nsISupports windowOrBrowser); */
    pub registerContentHandler: unsafe extern "C" fn (this: *const nsIWebContentHandlerRegistrar, mimeType: *const nsAString, uri: *const nsAString, title: *const nsAString, windowOrBrowser: *const nsISupports) -> nsresult,

    /* void registerProtocolHandler (in DOMString protocol, in DOMString uri, in DOMString title, in nsISupports windowOrBrowser); */
    pub registerProtocolHandler: unsafe extern "C" fn (this: *const nsIWebContentHandlerRegistrar, protocol: *const nsAString, uri: *const nsAString, title: *const nsAString, windowOrBrowser: *const nsISupports) -> nsresult,

}


impl nsIWebContentHandlerRegistrar {
    /* void registerContentHandler (in DOMString mimeType, in DOMString uri, in DOMString title, in nsISupports windowOrBrowser); */
    #[inline]
    pub unsafe fn registerContentHandler(&self, mimeType: &[u16], uri: &[u16], title: &[u16], windowOrBrowser: Option<&nsISupports>) -> Result<(), nsresult> {
        let mimeType = nsString::from(mimeType);
        let uri = nsString::from(uri);
        let title = nsString::from(title);
        match ((*self.vtable).registerContentHandler)(self as *const _, &*mimeType, &*uri, &*title, windowOrBrowser.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerProtocolHandler (in DOMString protocol, in DOMString uri, in DOMString title, in nsISupports windowOrBrowser); */
    #[inline]
    pub unsafe fn registerProtocolHandler(&self, protocol: &[u16], uri: &[u16], title: &[u16], windowOrBrowser: Option<&nsISupports>) -> Result<(), nsresult> {
        let protocol = nsString::from(protocol);
        let uri = nsString::from(uri);
        let title = nsString::from(title);
        match ((*self.vtable).registerProtocolHandler)(self as *const _, &*protocol, &*uri, &*title, windowOrBrowser.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


