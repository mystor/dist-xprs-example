//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIJARChannel.idl
//


#[repr(C)]
pub struct nsIJARChannel {
    vtable: *const nsIJARChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJARChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe72b179b, 0xd5df, 0x4d87,
            [0xb5, 0xde, 0xfd, 0x73, 0xa6, 0x5c, 0x60, 0xf6])
    }
}

unsafe impl RefCounted for nsIJARChannel {
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
pub trait nsIJARChannelCoerce {
    fn coerce_from(v: &nsIJARChannel) -> &Self;
}

impl nsIJARChannelCoerce for nsIJARChannel {
    #[inline]
    fn coerce_from(v: &nsIJARChannel) -> &Self {
        v
    }
}

impl nsIJARChannel {
    #[inline]
    pub fn coerce<T: nsIJARChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJARChannel {
    type Target = nsIChannel;
    #[inline]
    fn deref(&self) -> &nsIChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIChannelCoerce> nsIJARChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJARChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJARChannelVTable {
    pub __base: nsIChannelVTable,

    /* [infallible] readonly attribute boolean isUnsafe; */
    pub get_isUnsafe: unsafe extern "C" fn (this: *const nsIJARChannel, aIsUnsafe: *mut bool) -> nsresult,

    /* readonly attribute nsIFile jarFile; */
    pub get_jarFile: unsafe extern "C" fn (this: *const nsIJARChannel, aJarFile: *mut *const nsIFile) -> nsresult,

    /* readonly attribute nsIZipEntry zipEntry; */
    pub get_zipEntry: unsafe extern "C" fn (this: *const nsIJARChannel, aZipEntry: *mut *const nsIZipEntry) -> nsresult,

}


impl nsIJARChannel {
    /* [infallible] readonly attribute boolean isUnsafe; */
    #[inline]
    pub unsafe fn get_isUnsafe(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isUnsafe)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIFile jarFile; */
    #[inline]
    pub unsafe fn get_jarFile(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_jarFile)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIZipEntry zipEntry; */
    #[inline]
    pub unsafe fn get_zipEntry(&self, ) -> Result<Option<RefPtr<nsIZipEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_zipEntry)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


