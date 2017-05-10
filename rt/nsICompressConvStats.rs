//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICompressConvStats.idl
//


#[repr(C)]
pub struct nsICompressConvStats {
    vtable: *const nsICompressConvStatsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICompressConvStats {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x58172ad0, 0x46a9, 0x4893,
            [0x8f, 0xde, 0xcd, 0x90, 0x9c, 0x10, 0x79, 0x2a])
    }
}

unsafe impl RefCounted for nsICompressConvStats {
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
pub trait nsICompressConvStatsCoerce {
    fn coerce_from(v: &nsICompressConvStats) -> &Self;
}

impl nsICompressConvStatsCoerce for nsICompressConvStats {
    #[inline]
    fn coerce_from(v: &nsICompressConvStats) -> &Self {
        v
    }
}

impl nsICompressConvStats {
    #[inline]
    pub fn coerce<T: nsICompressConvStatsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICompressConvStats {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICompressConvStatsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICompressConvStats) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICompressConvStatsVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute uint64_t decodedDataLength; */
    pub get_decodedDataLength: unsafe extern "C" fn (this: *const nsICompressConvStats, aDecodedDataLength: *mut uint64_t) -> nsresult,

}


impl nsICompressConvStats {
    /* readonly attribute uint64_t decodedDataLength; */
    #[inline]
    pub unsafe fn get_decodedDataLength(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_decodedDataLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


