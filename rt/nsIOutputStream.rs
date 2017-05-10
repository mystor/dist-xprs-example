//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIOutputStream.idl
//


#[repr(C)]
pub struct nsIOutputStream {
    vtable: *const nsIOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0d0acd2a, 0x61b4, 0x11d4,
            [0x98, 0x77, 0x00, 0xc0, 0x4f, 0xa0, 0xcf, 0x4a])
    }
}

unsafe impl RefCounted for nsIOutputStream {
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
pub trait nsIOutputStreamCoerce {
    fn coerce_from(v: &nsIOutputStream) -> &Self;
}

impl nsIOutputStreamCoerce for nsIOutputStream {
    #[inline]
    fn coerce_from(v: &nsIOutputStream) -> &Self {
        v
    }
}

impl nsIOutputStream {
    #[inline]
    pub fn coerce<T: nsIOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOutputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOutputStreamVTable {
    pub __base: nsISupportsVTable,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIOutputStream) -> nsresult,

    /* void flush (); */
    pub flush: unsafe extern "C" fn (this: *const nsIOutputStream) -> nsresult,

    /* unsigned long write (in string aBuf, in unsigned long aCount); */
    pub write: unsafe extern "C" fn (this: *const nsIOutputStream, aBuf: *const libc::c_char, aCount: libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* unsigned long writeFrom (in nsIInputStream aFromStream, in unsigned long aCount); */
    pub writeFrom: unsafe extern "C" fn (this: *const nsIOutputStream, aFromStream: *const nsIInputStream, aCount: libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* [noscript] unsigned long writeSegments (in nsReadSegmentFun aReader, in voidPtr aClosure, in unsigned long aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub writeSegments: *const ::libc::c_void,

    /* boolean isNonBlocking (); */
    pub isNonBlocking: unsafe extern "C" fn (this: *const nsIOutputStream, _retval: *mut bool) -> nsresult,

}


impl nsIOutputStream {
    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void flush (); */
    #[inline]
    pub unsafe fn flush(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).flush)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long write (in string aBuf, in unsigned long aCount); */
    #[inline]
    pub unsafe fn write(&self, aBuf: *const libc::c_char, aCount: libc::uint32_t) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).write)(self as *const _, aBuf, aCount, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long writeFrom (in nsIInputStream aFromStream, in unsigned long aCount); */
    #[inline]
    pub unsafe fn writeFrom(&self, aFromStream: Option<&nsIInputStream>, aCount: libc::uint32_t) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).writeFrom)(self as *const _, aFromStream.map_or(::std::ptr::null(), |x| x as *const _), aCount, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] unsigned long writeSegments (in nsReadSegmentFun aReader, in voidPtr aClosure, in unsigned long aCount); */


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


