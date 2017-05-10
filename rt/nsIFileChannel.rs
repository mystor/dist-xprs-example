//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFileChannel.idl
//


#[repr(C)]
pub struct nsIFileChannel {
    vtable: *const nsIFileChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFileChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x06169120, 0x136d, 0x45a5,
            [0xb5, 0x35, 0x49, 0x8f, 0x1f, 0x75, 0x5a, 0xb7])
    }
}

unsafe impl RefCounted for nsIFileChannel {
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
pub trait nsIFileChannelCoerce {
    fn coerce_from(v: &nsIFileChannel) -> &Self;
}

impl nsIFileChannelCoerce for nsIFileChannel {
    #[inline]
    fn coerce_from(v: &nsIFileChannel) -> &Self {
        v
    }
}

impl nsIFileChannel {
    #[inline]
    pub fn coerce<T: nsIFileChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFileChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFileChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFileChannelVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFile file; */
    pub get_file: unsafe extern "C" fn (this: *const nsIFileChannel, aFile: *mut *const nsIFile) -> nsresult,

}


impl nsIFileChannel {
    /* readonly attribute nsIFile file; */
    #[inline]
    pub unsafe fn get_file(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_file)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


