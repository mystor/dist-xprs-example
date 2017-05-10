//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationRequestUIGlue.idl
//


#[repr(C)]
pub struct nsIPresentationRequestUIGlue {
    vtable: *const nsIPresentationRequestUIGlueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationRequestUIGlue {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfaa45119, 0x6fb5, 0x496c,
            [0xaa, 0x4c, 0xf7, 0x40, 0x17, 0x7a, 0x38, 0xb5])
    }
}

unsafe impl RefCounted for nsIPresentationRequestUIGlue {
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
pub trait nsIPresentationRequestUIGlueCoerce {
    fn coerce_from(v: &nsIPresentationRequestUIGlue) -> &Self;
}

impl nsIPresentationRequestUIGlueCoerce for nsIPresentationRequestUIGlue {
    #[inline]
    fn coerce_from(v: &nsIPresentationRequestUIGlue) -> &Self {
        v
    }
}

impl nsIPresentationRequestUIGlue {
    #[inline]
    pub fn coerce<T: nsIPresentationRequestUIGlueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationRequestUIGlue {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationRequestUIGlueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationRequestUIGlue) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationRequestUIGlueVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports sendRequest (in DOMString url, in DOMString sessionId, in nsIPresentationDevice device); */
    pub sendRequest: unsafe extern "C" fn (this: *const nsIPresentationRequestUIGlue, url: *const nsAString, sessionId: *const nsAString, device: *const nsIPresentationDevice, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIPresentationRequestUIGlue {
    /* nsISupports sendRequest (in DOMString url, in DOMString sessionId, in nsIPresentationDevice device); */
    #[inline]
    pub unsafe fn sendRequest(&self, url: &[u16], sessionId: &[u16], device: Option<&nsIPresentationDevice>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let url = nsString::from(url);
        let sessionId = nsString::from(sessionId);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).sendRequest)(self as *const _, &*url, &*sessionId, device.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


