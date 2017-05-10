//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFileStreams.idl
//


pub mod nsIFileInputStream_consts {
    pub const DELETE_ON_CLOSE: i64 = 2;
    pub const CLOSE_ON_EOF: i64 = 4;
    pub const REOPEN_ON_REWIND: i64 = 8;
    pub const DEFER_OPEN: i64 = 16;
    pub const SHARE_DELETE: i64 = 32;
}


#[repr(C)]
pub struct nsIFileInputStream {
    vtable: *const nsIFileInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFileInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe3d56a20, 0xc7ec, 0x11d3,
            [0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3])
    }
}

unsafe impl RefCounted for nsIFileInputStream {
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
pub trait nsIFileInputStreamCoerce {
    fn coerce_from(v: &nsIFileInputStream) -> &Self;
}

impl nsIFileInputStreamCoerce for nsIFileInputStream {
    #[inline]
    fn coerce_from(v: &nsIFileInputStream) -> &Self {
        v
    }
}

impl nsIFileInputStream {
    #[inline]
    pub fn coerce<T: nsIFileInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFileInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIFileInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFileInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    pub init: unsafe extern "C" fn (this: *const nsIFileInputStream, file: *const nsIFile, ioFlags: libc::int32_t, perm: libc::int32_t, behaviorFlags: libc::int32_t) -> nsresult,

}


impl nsIFileInputStream {
    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    #[inline]
    pub unsafe fn init(&self, file: Option<&nsIFile>, ioFlags: libc::int32_t, perm: libc::int32_t, behaviorFlags: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, file.map_or(::std::ptr::null(), |x| x as *const _), ioFlags, perm, behaviorFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIFileOutputStream_consts {
    pub const DEFER_OPEN: i64 = 1;
}


#[repr(C)]
pub struct nsIFileOutputStream {
    vtable: *const nsIFileOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFileOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe734cac9, 0x1295, 0x4e6f,
            [0x96, 0x84, 0x3a, 0xc4, 0xe1, 0xf9, 0x10, 0x63])
    }
}

unsafe impl RefCounted for nsIFileOutputStream {
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
pub trait nsIFileOutputStreamCoerce {
    fn coerce_from(v: &nsIFileOutputStream) -> &Self;
}

impl nsIFileOutputStreamCoerce for nsIFileOutputStream {
    #[inline]
    fn coerce_from(v: &nsIFileOutputStream) -> &Self {
        v
    }
}

impl nsIFileOutputStream {
    #[inline]
    pub fn coerce<T: nsIFileOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFileOutputStream {
    type Target = nsIOutputStream;
    #[inline]
    fn deref(&self) -> &nsIOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIOutputStreamCoerce> nsIFileOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFileOutputStreamVTable {
    pub __base: nsIOutputStreamVTable,

    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    pub init: unsafe extern "C" fn (this: *const nsIFileOutputStream, file: *const nsIFile, ioFlags: libc::int32_t, perm: libc::int32_t, behaviorFlags: libc::int32_t) -> nsresult,

    /* [noscript] void preallocate (in long long length); */
    pub preallocate: unsafe extern "C" fn (this: *const nsIFileOutputStream, length: libc::int64_t) -> nsresult,

}


impl nsIFileOutputStream {
    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    #[inline]
    pub unsafe fn init(&self, file: Option<&nsIFile>, ioFlags: libc::int32_t, perm: libc::int32_t, behaviorFlags: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, file.map_or(::std::ptr::null(), |x| x as *const _), ioFlags, perm, behaviorFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void preallocate (in long long length); */
    #[inline]
    pub unsafe fn preallocate(&self, length: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).preallocate)(self as *const _, length) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIFileStream_consts {
    pub const DEFER_OPEN: i64 = 1;
}


#[repr(C)]
pub struct nsIFileStream {
    vtable: *const nsIFileStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFileStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x82cf605a, 0x8393, 0x4550,
            [0x83, 0xab, 0x43, 0xcd, 0x55, 0x78, 0xe0, 0x06])
    }
}

unsafe impl RefCounted for nsIFileStream {
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
pub trait nsIFileStreamCoerce {
    fn coerce_from(v: &nsIFileStream) -> &Self;
}

impl nsIFileStreamCoerce for nsIFileStream {
    #[inline]
    fn coerce_from(v: &nsIFileStream) -> &Self {
        v
    }
}

impl nsIFileStream {
    #[inline]
    pub fn coerce<T: nsIFileStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFileStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFileStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFileStreamVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    pub init: unsafe extern "C" fn (this: *const nsIFileStream, file: *const nsIFile, ioFlags: libc::int32_t, perm: libc::int32_t, behaviorFlags: libc::int32_t) -> nsresult,

}


impl nsIFileStream {
    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    #[inline]
    pub unsafe fn init(&self, file: Option<&nsIFile>, ioFlags: libc::int32_t, perm: libc::int32_t, behaviorFlags: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, file.map_or(::std::ptr::null(), |x| x as *const _), ioFlags, perm, behaviorFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIFileMetadata {
    vtable: *const nsIFileMetadataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFileMetadata {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x07f679e4, 0x9601, 0x4bd1,
            [0xb5, 0x10, 0xcd, 0x38, 0x52, 0xed, 0xb8, 0x81])
    }
}

unsafe impl RefCounted for nsIFileMetadata {
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
pub trait nsIFileMetadataCoerce {
    fn coerce_from(v: &nsIFileMetadata) -> &Self;
}

impl nsIFileMetadataCoerce for nsIFileMetadata {
    #[inline]
    fn coerce_from(v: &nsIFileMetadata) -> &Self {
        v
    }
}

impl nsIFileMetadata {
    #[inline]
    pub fn coerce<T: nsIFileMetadataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFileMetadata {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFileMetadataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileMetadata) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFileMetadataVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long long size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIFileMetadata, aSize: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long lastModified; */
    pub get_lastModified: unsafe extern "C" fn (this: *const nsIFileMetadata, aLastModified: *mut libc::int64_t) -> nsresult,

    /* [noscript] PRFileDescPtr getFileDescriptor (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFileDescriptor: *const ::libc::c_void,

}


impl nsIFileMetadata {
    /* readonly attribute long long size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_size)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long lastModified; */
    #[inline]
    pub unsafe fn get_lastModified(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] PRFileDescPtr getFileDescriptor (); */


}


