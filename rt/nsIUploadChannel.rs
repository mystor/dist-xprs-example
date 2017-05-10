//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUploadChannel.idl
//


#[repr(C)]
pub struct nsIUploadChannel {
    vtable: *const nsIUploadChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUploadChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5cfe15bd, 0x5adb, 0x4a7f,
            [0x9e, 0x55, 0x4f, 0x5a, 0x67, 0xd1, 0x57, 0x94])
    }
}

unsafe impl RefCounted for nsIUploadChannel {
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
pub trait nsIUploadChannelCoerce {
    fn coerce_from(v: &nsIUploadChannel) -> &Self;
}

impl nsIUploadChannelCoerce for nsIUploadChannel {
    #[inline]
    fn coerce_from(v: &nsIUploadChannel) -> &Self {
        v
    }
}

impl nsIUploadChannel {
    #[inline]
    pub fn coerce<T: nsIUploadChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUploadChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUploadChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUploadChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUploadChannelVTable {
    pub __base: nsISupportsVTable,

    /* void setUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength); */
    pub setUploadStream: unsafe extern "C" fn (this: *const nsIUploadChannel, aStream: *const nsIInputStream, aContentType: *const nsACString, aContentLength: libc::int64_t) -> nsresult,

    /* readonly attribute nsIInputStream uploadStream; */
    pub get_uploadStream: unsafe extern "C" fn (this: *const nsIUploadChannel, aUploadStream: *mut *const nsIInputStream) -> nsresult,

}


impl nsIUploadChannel {
    /* void setUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength); */
    #[inline]
    pub unsafe fn setUploadStream(&self, aStream: Option<&nsIInputStream>, aContentType: &[u8], aContentLength: libc::int64_t) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        match ((*self.vtable).setUploadStream)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), &*aContentType, aContentLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIInputStream uploadStream; */
    #[inline]
    pub unsafe fn get_uploadStream(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_uploadStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


