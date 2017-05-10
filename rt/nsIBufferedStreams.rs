//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBufferedStreams.idl
//


#[repr(C)]
pub struct nsIBufferedInputStream {
    vtable: *const nsIBufferedInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBufferedInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x616f5b48, 0xda09, 0x11d3,
            [0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3])
    }
}

unsafe impl RefCounted for nsIBufferedInputStream {
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
pub trait nsIBufferedInputStreamCoerce {
    fn coerce_from(v: &nsIBufferedInputStream) -> &Self;
}

impl nsIBufferedInputStreamCoerce for nsIBufferedInputStream {
    #[inline]
    fn coerce_from(v: &nsIBufferedInputStream) -> &Self {
        v
    }
}

impl nsIBufferedInputStream {
    #[inline]
    pub fn coerce<T: nsIBufferedInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBufferedInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIBufferedInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBufferedInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBufferedInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* void init (in nsIInputStream fillFromStream, in unsigned long bufferSize); */
    pub init: unsafe extern "C" fn (this: *const nsIBufferedInputStream, fillFromStream: *const nsIInputStream, bufferSize: libc::uint32_t) -> nsresult,

}


impl nsIBufferedInputStream {
    /* void init (in nsIInputStream fillFromStream, in unsigned long bufferSize); */
    #[inline]
    pub unsafe fn init(&self, fillFromStream: Option<&nsIInputStream>, bufferSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, fillFromStream.map_or(::std::ptr::null(), |x| x as *const _), bufferSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIBufferedOutputStream {
    vtable: *const nsIBufferedOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBufferedOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6476378a, 0xda09, 0x11d3,
            [0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3])
    }
}

unsafe impl RefCounted for nsIBufferedOutputStream {
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
pub trait nsIBufferedOutputStreamCoerce {
    fn coerce_from(v: &nsIBufferedOutputStream) -> &Self;
}

impl nsIBufferedOutputStreamCoerce for nsIBufferedOutputStream {
    #[inline]
    fn coerce_from(v: &nsIBufferedOutputStream) -> &Self {
        v
    }
}

impl nsIBufferedOutputStream {
    #[inline]
    pub fn coerce<T: nsIBufferedOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBufferedOutputStream {
    type Target = nsIOutputStream;
    #[inline]
    fn deref(&self) -> &nsIOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIOutputStreamCoerce> nsIBufferedOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBufferedOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBufferedOutputStreamVTable {
    pub __base: nsIOutputStreamVTable,

    /* void init (in nsIOutputStream sinkToStream, in unsigned long bufferSize); */
    pub init: unsafe extern "C" fn (this: *const nsIBufferedOutputStream, sinkToStream: *const nsIOutputStream, bufferSize: libc::uint32_t) -> nsresult,

}


impl nsIBufferedOutputStream {
    /* void init (in nsIOutputStream sinkToStream, in unsigned long bufferSize); */
    #[inline]
    pub unsafe fn init(&self, sinkToStream: Option<&nsIOutputStream>, bufferSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, sinkToStream.map_or(::std::ptr::null(), |x| x as *const _), bufferSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


