//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputStream.idl
//


#[repr(C)]
pub struct nsIInputStream {
    vtable: *const nsIInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x53cdbc97, 0xc2d7, 0x4e30,
            [0xb2, 0xc3, 0x45, 0xb2, 0xee, 0x79, 0xdb, 0x18])
    }
}

unsafe impl RefCounted for nsIInputStream {
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
pub trait nsIInputStreamCoerce {
    fn coerce_from(v: &nsIInputStream) -> &Self;
}

impl nsIInputStreamCoerce for nsIInputStream {
    #[inline]
    fn coerce_from(v: &nsIInputStream) -> &Self {
        v
    }
}

impl nsIInputStream {
    #[inline]
    pub fn coerce<T: nsIInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInputStreamVTable {
    pub __base: nsISupportsVTable,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIInputStream) -> nsresult,

    /* unsigned long long available (); */
    pub available: unsafe extern "C" fn (this: *const nsIInputStream, _retval: *mut libc::uint64_t) -> nsresult,

    /* [noscript] unsigned long read (in charPtr aBuf, in unsigned long aCount); */
    pub read: unsafe extern "C" fn (this: *const nsIInputStream, aBuf: *const u8, aCount: libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* [noscript] unsigned long readSegments (in nsWriteSegmentFun aWriter, in voidPtr aClosure, in unsigned long aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub readSegments: *const ::libc::c_void,

    /* boolean isNonBlocking (); */
    pub isNonBlocking: unsafe extern "C" fn (this: *const nsIInputStream, _retval: *mut bool) -> nsresult,

}


impl nsIInputStream {
    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long long available (); */
    #[inline]
    pub unsafe fn available(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).available)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] unsigned long read (in charPtr aBuf, in unsigned long aCount); */
    #[inline]
    pub unsafe fn read(&self, aBuf: *const u8, aCount: libc::uint32_t) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).read)(self as *const _, aBuf, aCount, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] unsigned long readSegments (in nsWriteSegmentFun aWriter, in voidPtr aClosure, in unsigned long aCount); */


    /* boolean isNonBlocking (); */
    #[inline]
    pub unsafe fn isNonBlocking(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isNonBlocking)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


