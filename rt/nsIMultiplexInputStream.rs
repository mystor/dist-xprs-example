//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMultiplexInputStream.idl
//


#[repr(C)]
pub struct nsIMultiplexInputStream {
    vtable: *const nsIMultiplexInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMultiplexInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa076fd12, 0x1dd1, 0x11b2,
            [0xb1, 0x9a, 0xd5, 0x3b, 0x5d, 0xff, 0xaa, 0xde])
    }
}

unsafe impl RefCounted for nsIMultiplexInputStream {
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
pub trait nsIMultiplexInputStreamCoerce {
    fn coerce_from(v: &nsIMultiplexInputStream) -> &Self;
}

impl nsIMultiplexInputStreamCoerce for nsIMultiplexInputStream {
    #[inline]
    fn coerce_from(v: &nsIMultiplexInputStream) -> &Self {
        v
    }
}

impl nsIMultiplexInputStream {
    #[inline]
    pub fn coerce<T: nsIMultiplexInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMultiplexInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIMultiplexInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMultiplexInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMultiplexInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* readonly attribute unsigned long count; */
    pub get_count: unsafe extern "C" fn (this: *const nsIMultiplexInputStream, aCount: *mut libc::uint32_t) -> nsresult,

    /* void appendStream (in nsIInputStream stream); */
    pub appendStream: unsafe extern "C" fn (this: *const nsIMultiplexInputStream, stream: *const nsIInputStream) -> nsresult,

    /* void insertStream (in nsIInputStream stream, in unsigned long index); */
    pub insertStream: unsafe extern "C" fn (this: *const nsIMultiplexInputStream, stream: *const nsIInputStream, index: libc::uint32_t) -> nsresult,

    /* void removeStream (in unsigned long index); */
    pub removeStream: unsafe extern "C" fn (this: *const nsIMultiplexInputStream, index: libc::uint32_t) -> nsresult,

    /* nsIInputStream getStream (in unsigned long index); */
    pub getStream: unsafe extern "C" fn (this: *const nsIMultiplexInputStream, index: libc::uint32_t, _retval: *mut *const nsIInputStream) -> nsresult,

}


impl nsIMultiplexInputStream {
    /* readonly attribute unsigned long count; */
    #[inline]
    pub unsafe fn get_count(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_count)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void appendStream (in nsIInputStream stream); */
    #[inline]
    pub unsafe fn appendStream(&self, stream: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).appendStream)(self as *const _, stream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertStream (in nsIInputStream stream, in unsigned long index); */
    #[inline]
    pub unsafe fn insertStream(&self, stream: Option<&nsIInputStream>, index: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).insertStream)(self as *const _, stream.map_or(::std::ptr::null(), |x| x as *const _), index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeStream (in unsigned long index); */
    #[inline]
    pub unsafe fn removeStream(&self, index: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeStream)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIInputStream getStream (in unsigned long index); */
    #[inline]
    pub unsafe fn getStream(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getStream)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


