//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFTPChannel.idl
//


#[repr(C)]
pub struct nsIFTPChannel {
    vtable: *const nsIFTPChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFTPChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x07f0d5cd, 0x1fd5, 0x4aa3,
            [0xb6, 0xfc, 0x66, 0x5b, 0xdc, 0x5d, 0xbf, 0x9f])
    }
}

unsafe impl RefCounted for nsIFTPChannel {
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
pub trait nsIFTPChannelCoerce {
    fn coerce_from(v: &nsIFTPChannel) -> &Self;
}

impl nsIFTPChannelCoerce for nsIFTPChannel {
    #[inline]
    fn coerce_from(v: &nsIFTPChannel) -> &Self {
        v
    }
}

impl nsIFTPChannel {
    #[inline]
    pub fn coerce<T: nsIFTPChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFTPChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFTPChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFTPChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFTPChannelVTable {
    pub __base: nsISupportsVTable,

    /* attribute PRTime lastModifiedTime; */
    pub get_lastModifiedTime: unsafe extern "C" fn (this: *const nsIFTPChannel, aLastModifiedTime: *mut PRTime) -> nsresult,
    pub set_lastModifiedTime: unsafe extern "C" fn (this: *const nsIFTPChannel, aLastModifiedTime: PRTime) -> nsresult,

}


impl nsIFTPChannel {
    /* attribute PRTime lastModifiedTime; */
    #[inline]
    pub unsafe fn get_lastModifiedTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModifiedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_lastModifiedTime(&self, aLastModifiedTime: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).set_lastModifiedTime)(self as *const _, aLastModifiedTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIFTPEventSink {
    vtable: *const nsIFTPEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFTPEventSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x455d4234, 0x0330, 0x43d2,
            [0xbb, 0xfb, 0x99, 0xaf, 0xbe, 0xcb, 0xfe, 0xb0])
    }
}

unsafe impl RefCounted for nsIFTPEventSink {
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
pub trait nsIFTPEventSinkCoerce {
    fn coerce_from(v: &nsIFTPEventSink) -> &Self;
}

impl nsIFTPEventSinkCoerce for nsIFTPEventSink {
    #[inline]
    fn coerce_from(v: &nsIFTPEventSink) -> &Self {
        v
    }
}

impl nsIFTPEventSink {
    #[inline]
    pub fn coerce<T: nsIFTPEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFTPEventSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFTPEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFTPEventSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFTPEventSinkVTable {
    pub __base: nsISupportsVTable,

    /* void OnFTPControlLog (in boolean server, in string msg); */
    pub OnFTPControlLog: unsafe extern "C" fn (this: *const nsIFTPEventSink, server: bool, msg: *const libc::c_char) -> nsresult,

}


impl nsIFTPEventSink {
    /* void OnFTPControlLog (in boolean server, in string msg); */
    #[inline]
    pub unsafe fn OnFTPControlLog(&self, server: bool, msg: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).OnFTPControlLog)(self as *const _, server, msg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


