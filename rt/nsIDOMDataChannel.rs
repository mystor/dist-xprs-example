//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDataChannel.idl
//


#[repr(C)]
pub struct nsIDOMDataChannel {
    vtable: *const nsIDOMDataChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDataChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb00a4ca7, 0x312e, 0x4926,
            [0x84, 0xf6, 0x8e, 0xbb, 0x43, 0xe5, 0x3d, 0x83])
    }
}

unsafe impl RefCounted for nsIDOMDataChannel {
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
pub trait nsIDOMDataChannelCoerce {
    fn coerce_from(v: &nsIDOMDataChannel) -> &Self;
}

impl nsIDOMDataChannelCoerce for nsIDOMDataChannel {
    #[inline]
    fn coerce_from(v: &nsIDOMDataChannel) -> &Self {
        v
    }
}

impl nsIDOMDataChannel {
    #[inline]
    pub fn coerce<T: nsIDOMDataChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDataChannel {
    type Target = nsIDOMEventTarget;
    #[inline]
    fn deref(&self) -> &nsIDOMEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMEventTargetCoerce> nsIDOMDataChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDataChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDataChannelVTable {
    pub __base: nsIDOMEventTargetVTable,

    /* readonly attribute DOMString label; */
    pub get_label: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aLabel: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString protocol; */
    pub get_protocol: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aProtocol: *mut nsAString) -> nsresult,

    /* readonly attribute boolean reliable; */
    pub get_reliable: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aReliable: *mut bool) -> nsresult,

    /* readonly attribute boolean ordered; */
    pub get_ordered: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aOrdered: *mut bool) -> nsresult,

    /* readonly attribute DOMString readyState; */
    pub get_readyState: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aReadyState: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long bufferedAmount; */
    pub get_bufferedAmount: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aBufferedAmount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned short id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aId: *mut libc::uint16_t) -> nsresult,

    /* [implicit_jscontext] attribute jsval onopen; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onopen: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onopen: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onerror; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onerror: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onerror: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onclose; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onclose: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onclose: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onmessage; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onmessage: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onmessage: *const ::libc::c_void,

    /* attribute DOMString binaryType; */
    pub get_binaryType: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aBinaryType: *mut nsAString) -> nsresult,
    pub set_binaryType: unsafe extern "C" fn (this: *const nsIDOMDataChannel, aBinaryType: *const nsAString) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIDOMDataChannel) -> nsresult,

}


impl nsIDOMDataChannel {
    /* readonly attribute DOMString label; */
    #[inline]
    pub unsafe fn get_label(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_label)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString protocol; */
    #[inline]
    pub unsafe fn get_protocol(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_protocol)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean reliable; */
    #[inline]
    pub unsafe fn get_reliable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_reliable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean ordered; */
    #[inline]
    pub unsafe fn get_ordered(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_ordered)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString readyState; */
    #[inline]
    pub unsafe fn get_readyState(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_readyState)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long bufferedAmount; */
    #[inline]
    pub unsafe fn get_bufferedAmount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_bufferedAmount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_id)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] attribute jsval onopen; */



    /* [implicit_jscontext] attribute jsval onerror; */



    /* [implicit_jscontext] attribute jsval onclose; */



    /* [implicit_jscontext] attribute jsval onmessage; */



    /* attribute DOMString binaryType; */
    #[inline]
    pub unsafe fn get_binaryType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_binaryType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_binaryType(&self, aBinaryType: &[u16]) -> Result<(), nsresult> {
        let aBinaryType = nsString::from(aBinaryType);
        match ((*self.vtable).set_binaryType)(self as *const _, &*aBinaryType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


