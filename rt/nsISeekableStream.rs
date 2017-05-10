//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISeekableStream.idl
//


pub mod nsISeekableStream_consts {
    pub const NS_SEEK_SET: i64 = 0;
    pub const NS_SEEK_CUR: i64 = 1;
    pub const NS_SEEK_END: i64 = 2;
}


#[repr(C)]
pub struct nsISeekableStream {
    vtable: *const nsISeekableStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISeekableStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8429d350, 0x1040, 0x4661,
            [0x8b, 0x71, 0xf2, 0xa6, 0xba, 0x45, 0x59, 0x80])
    }
}

unsafe impl RefCounted for nsISeekableStream {
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
pub trait nsISeekableStreamCoerce {
    fn coerce_from(v: &nsISeekableStream) -> &Self;
}

impl nsISeekableStreamCoerce for nsISeekableStream {
    #[inline]
    fn coerce_from(v: &nsISeekableStream) -> &Self {
        v
    }
}

impl nsISeekableStream {
    #[inline]
    pub fn coerce<T: nsISeekableStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISeekableStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISeekableStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISeekableStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISeekableStreamVTable {
    pub __base: nsISupportsVTable,

    /* void seek (in long whence, in long long offset); */
    pub seek: unsafe extern "C" fn (this: *const nsISeekableStream, whence: libc::int32_t, offset: libc::int64_t) -> nsresult,

    /* long long tell (); */
    pub tell: unsafe extern "C" fn (this: *const nsISeekableStream, _retval: *mut libc::int64_t) -> nsresult,

    /* void setEOF (); */
    pub setEOF: unsafe extern "C" fn (this: *const nsISeekableStream) -> nsresult,

}


impl nsISeekableStream {
    /* void seek (in long whence, in long long offset); */
    #[inline]
    pub unsafe fn seek(&self, whence: libc::int32_t, offset: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).seek)(self as *const _, whence, offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long long tell (); */
    #[inline]
    pub unsafe fn tell(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).tell)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setEOF (); */
    #[inline]
    pub unsafe fn setEOF(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setEOF)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


