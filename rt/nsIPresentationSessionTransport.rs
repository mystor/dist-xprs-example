//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationSessionTransport.idl
//


#[repr(C)]
pub struct nsIPresentationSessionTransportCallback {
    vtable: *const nsIPresentationSessionTransportCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationSessionTransportCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9f158786, 0x41a6, 0x4a10,
            [0xb2, 0x9b, 0x94, 0x97, 0xf2, 0x5d, 0x4b, 0x67])
    }
}

unsafe impl RefCounted for nsIPresentationSessionTransportCallback {
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
pub trait nsIPresentationSessionTransportCallbackCoerce {
    fn coerce_from(v: &nsIPresentationSessionTransportCallback) -> &Self;
}

impl nsIPresentationSessionTransportCallbackCoerce for nsIPresentationSessionTransportCallback {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportCallback) -> &Self {
        v
    }
}

impl nsIPresentationSessionTransportCallback {
    #[inline]
    pub fn coerce<T: nsIPresentationSessionTransportCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationSessionTransportCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationSessionTransportCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationSessionTransportCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void notifyTransportReady (); */
    pub notifyTransportReady: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportCallback) -> nsresult,

    /* void notifyTransportClosed (in nsresult reason); */
    pub notifyTransportClosed: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportCallback, reason: nsresult) -> nsresult,

    /* void notifyData (in ACString data, in boolean isBinary); */
    pub notifyData: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportCallback, data: *const nsACString, isBinary: bool) -> nsresult,

}


impl nsIPresentationSessionTransportCallback {
    /* void notifyTransportReady (); */
    #[inline]
    pub unsafe fn notifyTransportReady(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyTransportReady)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyTransportClosed (in nsresult reason); */
    #[inline]
    pub unsafe fn notifyTransportClosed(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).notifyTransportClosed)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyData (in ACString data, in boolean isBinary); */
    #[inline]
    pub unsafe fn notifyData(&self, data: &[u8], isBinary: bool) -> Result<(), nsresult> {
        let data = nsCString::from(data);
        match ((*self.vtable).notifyData)(self as *const _, &*data, isBinary) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPresentationSessionTransport {
    vtable: *const nsIPresentationSessionTransportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationSessionTransport {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x670b7e1b, 0x65be, 0x42b6,
            [0xa5, 0x96, 0xbe, 0x57, 0x19, 0x07, 0xfa, 0x18])
    }
}

unsafe impl RefCounted for nsIPresentationSessionTransport {
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
pub trait nsIPresentationSessionTransportCoerce {
    fn coerce_from(v: &nsIPresentationSessionTransport) -> &Self;
}

impl nsIPresentationSessionTransportCoerce for nsIPresentationSessionTransport {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransport) -> &Self {
        v
    }
}

impl nsIPresentationSessionTransport {
    #[inline]
    pub fn coerce<T: nsIPresentationSessionTransportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationSessionTransport {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationSessionTransportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransport) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationSessionTransportVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIPresentationSessionTransportCallback callback; */
    pub get_callback: unsafe extern "C" fn (this: *const nsIPresentationSessionTransport, aCallback: *mut *const nsIPresentationSessionTransportCallback) -> nsresult,
    pub set_callback: unsafe extern "C" fn (this: *const nsIPresentationSessionTransport, aCallback: *const nsIPresentationSessionTransportCallback) -> nsresult,

    /* readonly attribute nsINetAddr selfAddress; */
    pub get_selfAddress: unsafe extern "C" fn (this: *const nsIPresentationSessionTransport, aSelfAddress: *mut *const nsINetAddr) -> nsresult,

    /* void enableDataNotification (); */
    pub enableDataNotification: unsafe extern "C" fn (this: *const nsIPresentationSessionTransport) -> nsresult,

    /* void send (in DOMString data); */
    pub send: unsafe extern "C" fn (this: *const nsIPresentationSessionTransport, data: *const nsAString) -> nsresult,

    /* void sendBinaryMsg (in ACString data); */
    pub sendBinaryMsg: unsafe extern "C" fn (this: *const nsIPresentationSessionTransport, data: *const nsACString) -> nsresult,

    /* void sendBlob (in nsIDOMBlob blob); */
    pub sendBlob: unsafe extern "C" fn (this: *const nsIPresentationSessionTransport, blob: *const nsIDOMBlob) -> nsresult,

    /* void close (in nsresult reason); */
    pub close: unsafe extern "C" fn (this: *const nsIPresentationSessionTransport, reason: nsresult) -> nsresult,

}


impl nsIPresentationSessionTransport {
    /* attribute nsIPresentationSessionTransportCallback callback; */
    #[inline]
    pub unsafe fn get_callback(&self, ) -> Result<Option<RefPtr<nsIPresentationSessionTransportCallback>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_callback)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_callback(&self, aCallback: Option<&nsIPresentationSessionTransportCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).set_callback)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsINetAddr selfAddress; */
    #[inline]
    pub unsafe fn get_selfAddress(&self, ) -> Result<Option<RefPtr<nsINetAddr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selfAddress)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void enableDataNotification (); */
    #[inline]
    pub unsafe fn enableDataNotification(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enableDataNotification)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void send (in DOMString data); */
    #[inline]
    pub unsafe fn send(&self, data: &[u16]) -> Result<(), nsresult> {
        let data = nsString::from(data);
        match ((*self.vtable).send)(self as *const _, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendBinaryMsg (in ACString data); */
    #[inline]
    pub unsafe fn sendBinaryMsg(&self, data: &[u8]) -> Result<(), nsresult> {
        let data = nsCString::from(data);
        match ((*self.vtable).sendBinaryMsg)(self as *const _, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendBlob (in nsIDOMBlob blob); */
    #[inline]
    pub unsafe fn sendBlob(&self, blob: Option<&nsIDOMBlob>) -> Result<(), nsresult> {

        match ((*self.vtable).sendBlob)(self as *const _, blob.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void close (in nsresult reason); */
    #[inline]
    pub unsafe fn close(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


