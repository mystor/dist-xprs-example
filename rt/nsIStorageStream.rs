//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStorageStream.idl
//


#[repr(C)]
pub struct nsIStorageStream {
    vtable: *const nsIStorageStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStorageStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x44a200fe, 0x6c2b, 0x4b41,
            [0xb4, 0xe3, 0x63, 0xe8, 0xc1, 0x4e, 0x7c, 0x0d])
    }
}

unsafe impl RefCounted for nsIStorageStream {
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
pub trait nsIStorageStreamCoerce {
    fn coerce_from(v: &nsIStorageStream) -> &Self;
}

impl nsIStorageStreamCoerce for nsIStorageStream {
    #[inline]
    fn coerce_from(v: &nsIStorageStream) -> &Self {
        v
    }
}

impl nsIStorageStream {
    #[inline]
    pub fn coerce<T: nsIStorageStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStorageStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStorageStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStorageStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStorageStreamVTable {
    pub __base: nsISupportsVTable,

    /* void init (in uint32_t segmentSize, in uint32_t maxSize); */
    pub init: unsafe extern "C" fn (this: *const nsIStorageStream, segmentSize: uint32_t, maxSize: uint32_t) -> nsresult,

    /* nsIOutputStream getOutputStream (in int32_t startPosition); */
    pub getOutputStream: unsafe extern "C" fn (this: *const nsIStorageStream, startPosition: int32_t, _retval: *mut *const nsIOutputStream) -> nsresult,

    /* nsIInputStream newInputStream (in int32_t startPosition); */
    pub newInputStream: unsafe extern "C" fn (this: *const nsIStorageStream, startPosition: int32_t, _retval: *mut *const nsIInputStream) -> nsresult,

    /* attribute uint32_t length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIStorageStream, aLength: *mut uint32_t) -> nsresult,
    pub set_length: unsafe extern "C" fn (this: *const nsIStorageStream, aLength: uint32_t) -> nsresult,

    /* readonly attribute boolean writeInProgress; */
    pub get_writeInProgress: unsafe extern "C" fn (this: *const nsIStorageStream, aWriteInProgress: *mut bool) -> nsresult,

}


impl nsIStorageStream {
    /* void init (in uint32_t segmentSize, in uint32_t maxSize); */
    #[inline]
    pub unsafe fn init(&self, segmentSize: uint32_t, maxSize: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, segmentSize, maxSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIOutputStream getOutputStream (in int32_t startPosition); */
    #[inline]
    pub unsafe fn getOutputStream(&self, startPosition: int32_t) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getOutputStream)(self as *const _, startPosition, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIInputStream newInputStream (in int32_t startPosition); */
    #[inline]
    pub unsafe fn newInputStream(&self, startPosition: int32_t) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newInputStream)(self as *const _, startPosition, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute uint32_t length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_length(&self, aLength: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_length)(self as *const _, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean writeInProgress; */
    #[inline]
    pub unsafe fn get_writeInProgress(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_writeInProgress)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


