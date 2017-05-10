//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentHandler.idl
//


#[repr(C)]
pub struct nsIContentHandler {
    vtable: *const nsIContentHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x49439df2, 0xb3d2, 0x441c,
            [0xbf, 0x62, 0x86, 0x6b, 0xda, 0xf5, 0x6f, 0xd2])
    }
}

unsafe impl RefCounted for nsIContentHandler {
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
pub trait nsIContentHandlerCoerce {
    fn coerce_from(v: &nsIContentHandler) -> &Self;
}

impl nsIContentHandlerCoerce for nsIContentHandler {
    #[inline]
    fn coerce_from(v: &nsIContentHandler) -> &Self {
        v
    }
}

impl nsIContentHandler {
    #[inline]
    pub fn coerce<T: nsIContentHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void handleContent (in string aContentType, in nsIInterfaceRequestor aWindowContext, in nsIRequest aRequest); */
    pub handleContent: unsafe extern "C" fn (this: *const nsIContentHandler, aContentType: *const libc::c_char, aWindowContext: *const nsIInterfaceRequestor, aRequest: *const nsIRequest) -> nsresult,

}


impl nsIContentHandler {
    /* void handleContent (in string aContentType, in nsIInterfaceRequestor aWindowContext, in nsIRequest aRequest); */
    #[inline]
    pub unsafe fn handleContent(&self, aContentType: *const libc::c_char, aWindowContext: Option<&nsIInterfaceRequestor>, aRequest: Option<&nsIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).handleContent)(self as *const _, aContentType, aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


