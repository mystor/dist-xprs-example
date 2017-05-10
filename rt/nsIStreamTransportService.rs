//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamTransportService.idl
//


#[repr(C)]
pub struct nsIStreamTransportService {
    vtable: *const nsIStreamTransportServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamTransportService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5e0adf7d, 0x9785, 0x45c3,
            [0xa1, 0x93, 0x04, 0xf2, 0x5a, 0x75, 0xda, 0x8f])
    }
}

unsafe impl RefCounted for nsIStreamTransportService {
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
pub trait nsIStreamTransportServiceCoerce {
    fn coerce_from(v: &nsIStreamTransportService) -> &Self;
}

impl nsIStreamTransportServiceCoerce for nsIStreamTransportService {
    #[inline]
    fn coerce_from(v: &nsIStreamTransportService) -> &Self {
        v
    }
}

impl nsIStreamTransportService {
    #[inline]
    pub fn coerce<T: nsIStreamTransportServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamTransportService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStreamTransportServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamTransportService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamTransportServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsITransport createInputTransport (in nsIInputStream aStream, in long long aStartOffset, in long long aReadLimit, in boolean aCloseWhenDone); */
    pub createInputTransport: unsafe extern "C" fn (this: *const nsIStreamTransportService, aStream: *const nsIInputStream, aStartOffset: libc::int64_t, aReadLimit: libc::int64_t, aCloseWhenDone: bool, _retval: *mut *const nsITransport) -> nsresult,

    /* nsITransport createOutputTransport (in nsIOutputStream aStream, in long long aStartOffset, in long long aWriteLimit, in boolean aCloseWhenDone); */
    pub createOutputTransport: unsafe extern "C" fn (this: *const nsIStreamTransportService, aStream: *const nsIOutputStream, aStartOffset: libc::int64_t, aWriteLimit: libc::int64_t, aCloseWhenDone: bool, _retval: *mut *const nsITransport) -> nsresult,

}


impl nsIStreamTransportService {
    /* nsITransport createInputTransport (in nsIInputStream aStream, in long long aStartOffset, in long long aReadLimit, in boolean aCloseWhenDone); */
    #[inline]
    pub unsafe fn createInputTransport(&self, aStream: Option<&nsIInputStream>, aStartOffset: libc::int64_t, aReadLimit: libc::int64_t, aCloseWhenDone: bool) -> Result<Option<RefPtr<nsITransport>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createInputTransport)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), aStartOffset, aReadLimit, aCloseWhenDone, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITransport createOutputTransport (in nsIOutputStream aStream, in long long aStartOffset, in long long aWriteLimit, in boolean aCloseWhenDone); */
    #[inline]
    pub unsafe fn createOutputTransport(&self, aStream: Option<&nsIOutputStream>, aStartOffset: libc::int64_t, aWriteLimit: libc::int64_t, aCloseWhenDone: bool) -> Result<Option<RefPtr<nsITransport>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createOutputTransport)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), aStartOffset, aWriteLimit, aCloseWhenDone, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


