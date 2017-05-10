//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransport.idl
//


pub mod nsITransport_consts {
    pub const OPEN_BLOCKING: i64 = 1;
    pub const OPEN_UNBUFFERED: i64 = 2;
    pub const STATUS_READING: i64 = 2152398856;
    pub const STATUS_WRITING: i64 = 2152398857;
}


#[repr(C)]
pub struct nsITransport {
    vtable: *const nsITransportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransport {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2a8c6334, 0xa5e6, 0x4ec3,
            [0x98, 0x65, 0x12, 0x56, 0x54, 0x14, 0x46, 0xfb])
    }
}

unsafe impl RefCounted for nsITransport {
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
pub trait nsITransportCoerce {
    fn coerce_from(v: &nsITransport) -> &Self;
}

impl nsITransportCoerce for nsITransport {
    #[inline]
    fn coerce_from(v: &nsITransport) -> &Self {
        v
    }
}

impl nsITransport {
    #[inline]
    pub fn coerce<T: nsITransportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransport {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransport) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransportVTable {
    pub __base: nsISupportsVTable,

    /* nsIInputStream openInputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
    pub openInputStream: unsafe extern "C" fn (this: *const nsITransport, aFlags: libc::uint32_t, aSegmentSize: libc::uint32_t, aSegmentCount: libc::uint32_t, _retval: *mut *const nsIInputStream) -> nsresult,

    /* nsIOutputStream openOutputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
    pub openOutputStream: unsafe extern "C" fn (this: *const nsITransport, aFlags: libc::uint32_t, aSegmentSize: libc::uint32_t, aSegmentCount: libc::uint32_t, _retval: *mut *const nsIOutputStream) -> nsresult,

    /* void close (in nsresult aReason); */
    pub close: unsafe extern "C" fn (this: *const nsITransport, aReason: nsresult) -> nsresult,

    /* void setEventSink (in nsITransportEventSink aSink, in nsIEventTarget aEventTarget); */
    pub setEventSink: unsafe extern "C" fn (this: *const nsITransport, aSink: *const nsITransportEventSink, aEventTarget: *const nsIEventTarget) -> nsresult,

}


impl nsITransport {
    /* nsIInputStream openInputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
    #[inline]
    pub unsafe fn openInputStream(&self, aFlags: libc::uint32_t, aSegmentSize: libc::uint32_t, aSegmentCount: libc::uint32_t) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openInputStream)(self as *const _, aFlags, aSegmentSize, aSegmentCount, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIOutputStream openOutputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
    #[inline]
    pub unsafe fn openOutputStream(&self, aFlags: libc::uint32_t, aSegmentSize: libc::uint32_t, aSegmentCount: libc::uint32_t) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openOutputStream)(self as *const _, aFlags, aSegmentSize, aSegmentCount, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void close (in nsresult aReason); */
    #[inline]
    pub unsafe fn close(&self, aReason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setEventSink (in nsITransportEventSink aSink, in nsIEventTarget aEventTarget); */
    #[inline]
    pub unsafe fn setEventSink(&self, aSink: Option<&nsITransportEventSink>, aEventTarget: Option<&nsIEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).setEventSink)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _), aEventTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsITransportEventSink {
    vtable: *const nsITransportEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransportEventSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeda4f520, 0x67f7, 0x484b,
            [0xa6, 0x91, 0x8c, 0x32, 0x26, 0xa5, 0xb0, 0xa6])
    }
}

unsafe impl RefCounted for nsITransportEventSink {
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
pub trait nsITransportEventSinkCoerce {
    fn coerce_from(v: &nsITransportEventSink) -> &Self;
}

impl nsITransportEventSinkCoerce for nsITransportEventSink {
    #[inline]
    fn coerce_from(v: &nsITransportEventSink) -> &Self {
        v
    }
}

impl nsITransportEventSink {
    #[inline]
    pub fn coerce<T: nsITransportEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransportEventSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransportEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransportEventSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransportEventSinkVTable {
    pub __base: nsISupportsVTable,

    /* void onTransportStatus (in nsITransport aTransport, in nsresult aStatus, in long long aProgress, in long long aProgressMax); */
    pub onTransportStatus: unsafe extern "C" fn (this: *const nsITransportEventSink, aTransport: *const nsITransport, aStatus: nsresult, aProgress: libc::int64_t, aProgressMax: libc::int64_t) -> nsresult,

}


impl nsITransportEventSink {
    /* void onTransportStatus (in nsITransport aTransport, in nsresult aStatus, in long long aProgress, in long long aProgressMax); */
    #[inline]
    pub unsafe fn onTransportStatus(&self, aTransport: Option<&nsITransport>, aStatus: nsresult, aProgress: libc::int64_t, aProgressMax: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).onTransportStatus)(self as *const _, aTransport.map_or(::std::ptr::null(), |x| x as *const _), aStatus, aProgress, aProgressMax) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


