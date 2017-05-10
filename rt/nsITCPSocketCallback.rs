//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITCPSocketCallback.idl
//


pub mod nsITCPSocketCallback_consts {
    pub const BUFFER_SIZE: i64 = 65536;
}


#[repr(C)]
pub struct nsITCPSocketCallback {
    vtable: *const nsITCPSocketCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITCPSocketCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xac2c4b69, 0xcb79, 0x4767,
            [0xb1, 0xce, 0xbc, 0xf6, 0x29, 0x45, 0xcd, 0x39])
    }
}

unsafe impl RefCounted for nsITCPSocketCallback {
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
pub trait nsITCPSocketCallbackCoerce {
    fn coerce_from(v: &nsITCPSocketCallback) -> &Self;
}

impl nsITCPSocketCallbackCoerce for nsITCPSocketCallback {
    #[inline]
    fn coerce_from(v: &nsITCPSocketCallback) -> &Self {
        v
    }
}

impl nsITCPSocketCallback {
    #[inline]
    pub fn coerce<T: nsITCPSocketCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITCPSocketCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITCPSocketCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITCPSocketCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITCPSocketCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void fireErrorEvent (in AString name, in AString type); */
    pub fireErrorEvent: unsafe extern "C" fn (this: *const nsITCPSocketCallback, name: *const nsAString, type_: *const nsAString) -> nsresult,

    /* void fireDataStringEvent (in DOMString type, in ACString data); */
    pub fireDataStringEvent: unsafe extern "C" fn (this: *const nsITCPSocketCallback, type_: *const nsAString, data: *const nsACString) -> nsresult,

    /* void fireDataArrayEvent (in DOMString type, [const] in nsUint8TArrayRef data); */
    /// Unable to call function as its signature contains a non-rust type
    pub fireDataArrayEvent: *const ::libc::c_void,

    /* void fireEvent (in DOMString type); */
    pub fireEvent: unsafe extern "C" fn (this: *const nsITCPSocketCallback, type_: *const nsAString) -> nsresult,

    /* void updateReadyState (in unsigned long readystate); */
    pub updateReadyState: unsafe extern "C" fn (this: *const nsITCPSocketCallback, readystate: libc::uint32_t) -> nsresult,

    /* void updateBufferedAmount (in uint32_t bufferedAmount, in uint32_t trackingNumber); */
    pub updateBufferedAmount: unsafe extern "C" fn (this: *const nsITCPSocketCallback, bufferedAmount: uint32_t, trackingNumber: uint32_t) -> nsresult,

}


impl nsITCPSocketCallback {
    /* void fireErrorEvent (in AString name, in AString type); */
    #[inline]
    pub unsafe fn fireErrorEvent(&self, name: &[u16], type_: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        let type_ = nsString::from(type_);
        match ((*self.vtable).fireErrorEvent)(self as *const _, &*name, &*type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void fireDataStringEvent (in DOMString type, in ACString data); */
    #[inline]
    pub unsafe fn fireDataStringEvent(&self, type_: &[u16], data: &[u8]) -> Result<(), nsresult> {
        let type_ = nsString::from(type_);
        let data = nsCString::from(data);
        match ((*self.vtable).fireDataStringEvent)(self as *const _, &*type_, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void fireDataArrayEvent (in DOMString type, [const] in nsUint8TArrayRef data); */


    /* void fireEvent (in DOMString type); */
    #[inline]
    pub unsafe fn fireEvent(&self, type_: &[u16]) -> Result<(), nsresult> {
        let type_ = nsString::from(type_);
        match ((*self.vtable).fireEvent)(self as *const _, &*type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateReadyState (in unsigned long readystate); */
    #[inline]
    pub unsafe fn updateReadyState(&self, readystate: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).updateReadyState)(self as *const _, readystate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateBufferedAmount (in uint32_t bufferedAmount, in uint32_t trackingNumber); */
    #[inline]
    pub unsafe fn updateBufferedAmount(&self, bufferedAmount: uint32_t, trackingNumber: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).updateBufferedAmount)(self as *const _, bufferedAmount, trackingNumber) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


