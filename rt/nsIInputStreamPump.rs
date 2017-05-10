//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputStreamPump.idl
//


#[repr(C)]
pub struct nsIInputStreamPump {
    vtable: *const nsIInputStreamPumpVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInputStreamPump {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x400f5468, 0x97e7, 0x4d2b,
            [0x9c, 0x65, 0xa8, 0x2a, 0xec, 0xc7, 0xae, 0x82])
    }
}

unsafe impl RefCounted for nsIInputStreamPump {
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
pub trait nsIInputStreamPumpCoerce {
    fn coerce_from(v: &nsIInputStreamPump) -> &Self;
}

impl nsIInputStreamPumpCoerce for nsIInputStreamPump {
    #[inline]
    fn coerce_from(v: &nsIInputStreamPump) -> &Self {
        v
    }
}

impl nsIInputStreamPump {
    #[inline]
    pub fn coerce<T: nsIInputStreamPumpCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInputStreamPump {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestCoerce> nsIInputStreamPumpCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamPump) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInputStreamPumpVTable {
    pub __base: nsIRequestVTable,

    /* void init (in nsIInputStream aStream, in long long aStreamPos, in long long aStreamLen, in unsigned long aSegmentSize, in unsigned long aSegmentCount, in boolean aCloseWhenDone); */
    pub init: unsafe extern "C" fn (this: *const nsIInputStreamPump, aStream: *const nsIInputStream, aStreamPos: libc::int64_t, aStreamLen: libc::int64_t, aSegmentSize: libc::uint32_t, aSegmentCount: libc::uint32_t, aCloseWhenDone: bool) -> nsresult,

    /* void asyncRead (in nsIStreamListener aListener, in nsISupports aListenerContext); */
    pub asyncRead: unsafe extern "C" fn (this: *const nsIInputStreamPump, aListener: *const nsIStreamListener, aListenerContext: *const nsISupports) -> nsresult,

}


impl nsIInputStreamPump {
    /* void init (in nsIInputStream aStream, in long long aStreamPos, in long long aStreamLen, in unsigned long aSegmentSize, in unsigned long aSegmentCount, in boolean aCloseWhenDone); */
    #[inline]
    pub unsafe fn init(&self, aStream: Option<&nsIInputStream>, aStreamPos: libc::int64_t, aStreamLen: libc::int64_t, aSegmentSize: libc::uint32_t, aSegmentCount: libc::uint32_t, aCloseWhenDone: bool) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), aStreamPos, aStreamLen, aSegmentSize, aSegmentCount, aCloseWhenDone) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncRead (in nsIStreamListener aListener, in nsISupports aListenerContext); */
    #[inline]
    pub unsafe fn asyncRead(&self, aListener: Option<&nsIStreamListener>, aListenerContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncRead)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), aListenerContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


