//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIResumableChannel.idl
//


#[repr(C)]
pub struct nsIResumableChannel {
    vtable: *const nsIResumableChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIResumableChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4ad136fa, 0x83af, 0x4a22,
            [0xa7, 0x6e, 0x50, 0x36, 0x42, 0xc0, 0xf4, 0xa8])
    }
}

unsafe impl RefCounted for nsIResumableChannel {
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
pub trait nsIResumableChannelCoerce {
    fn coerce_from(v: &nsIResumableChannel) -> &Self;
}

impl nsIResumableChannelCoerce for nsIResumableChannel {
    #[inline]
    fn coerce_from(v: &nsIResumableChannel) -> &Self {
        v
    }
}

impl nsIResumableChannel {
    #[inline]
    pub fn coerce<T: nsIResumableChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIResumableChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIResumableChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIResumableChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIResumableChannelVTable {
    pub __base: nsISupportsVTable,

    /* void resumeAt (in unsigned long long startPos, in ACString entityID); */
    pub resumeAt: unsafe extern "C" fn (this: *const nsIResumableChannel, startPos: libc::uint64_t, entityID: *const nsACString) -> nsresult,

    /* readonly attribute ACString entityID; */
    pub get_entityID: unsafe extern "C" fn (this: *const nsIResumableChannel, aEntityID: *mut nsACString) -> nsresult,

}


impl nsIResumableChannel {
    /* void resumeAt (in unsigned long long startPos, in ACString entityID); */
    #[inline]
    pub unsafe fn resumeAt(&self, startPos: libc::uint64_t, entityID: &[u8]) -> Result<(), nsresult> {
        let entityID = nsCString::from(entityID);
        match ((*self.vtable).resumeAt)(self as *const _, startPos, &*entityID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute ACString entityID; */
    #[inline]
    pub unsafe fn get_entityID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_entityID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


