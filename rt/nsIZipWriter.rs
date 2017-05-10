//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIZipWriter.idl
//


pub mod nsIZipWriter_consts {
    pub const COMPRESSION_NONE: i64 = 0;
    pub const COMPRESSION_FASTEST: i64 = 1;
    pub const COMPRESSION_DEFAULT: i64 = 6;
    pub const COMPRESSION_BEST: i64 = 9;
}


#[repr(C)]
pub struct nsIZipWriter {
    vtable: *const nsIZipWriterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIZipWriter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3ca10750, 0x797e, 0x4a22,
            [0xbc, 0xfe, 0x66, 0x17, 0x0b, 0x5e, 0x96, 0xdd])
    }
}

unsafe impl RefCounted for nsIZipWriter {
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
pub trait nsIZipWriterCoerce {
    fn coerce_from(v: &nsIZipWriter) -> &Self;
}

impl nsIZipWriterCoerce for nsIZipWriter {
    #[inline]
    fn coerce_from(v: &nsIZipWriter) -> &Self {
        v
    }
}

impl nsIZipWriter {
    #[inline]
    pub fn coerce<T: nsIZipWriterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIZipWriter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIZipWriterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIZipWriter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIZipWriterVTable {
    pub __base: nsISupportsVTable,

    /* attribute ACString comment; */
    pub get_comment: unsafe extern "C" fn (this: *const nsIZipWriter, aComment: *mut nsACString) -> nsresult,
    pub set_comment: unsafe extern "C" fn (this: *const nsIZipWriter, aComment: *const nsACString) -> nsresult,

    /* readonly attribute boolean inQueue; */
    pub get_inQueue: unsafe extern "C" fn (this: *const nsIZipWriter, aInQueue: *mut bool) -> nsresult,

    /* readonly attribute nsIFile file; */
    pub get_file: unsafe extern "C" fn (this: *const nsIZipWriter, aFile: *mut *const nsIFile) -> nsresult,

    /* void open (in nsIFile aFile, in int32_t aIoFlags); */
    pub open: unsafe extern "C" fn (this: *const nsIZipWriter, aFile: *const nsIFile, aIoFlags: int32_t) -> nsresult,

    /* nsIZipEntry getEntry (in AUTF8String aZipEntry); */
    pub getEntry: unsafe extern "C" fn (this: *const nsIZipWriter, aZipEntry: *const nsACString, _retval: *mut *const nsIZipEntry) -> nsresult,

    /* boolean hasEntry (in AUTF8String aZipEntry); */
    pub hasEntry: unsafe extern "C" fn (this: *const nsIZipWriter, aZipEntry: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void addEntryDirectory (in AUTF8String aZipEntry, in PRTime aModTime, in boolean aQueue); */
    pub addEntryDirectory: unsafe extern "C" fn (this: *const nsIZipWriter, aZipEntry: *const nsACString, aModTime: PRTime, aQueue: bool) -> nsresult,

    /* void addEntryFile (in AUTF8String aZipEntry, in int32_t aCompression, in nsIFile aFile, in boolean aQueue); */
    pub addEntryFile: unsafe extern "C" fn (this: *const nsIZipWriter, aZipEntry: *const nsACString, aCompression: int32_t, aFile: *const nsIFile, aQueue: bool) -> nsresult,

    /* void addEntryChannel (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIChannel aChannel, in boolean aQueue); */
    pub addEntryChannel: unsafe extern "C" fn (this: *const nsIZipWriter, aZipEntry: *const nsACString, aModTime: PRTime, aCompression: int32_t, aChannel: *const nsIChannel, aQueue: bool) -> nsresult,

    /* void addEntryStream (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIInputStream aStream, in boolean aQueue); */
    pub addEntryStream: unsafe extern "C" fn (this: *const nsIZipWriter, aZipEntry: *const nsACString, aModTime: PRTime, aCompression: int32_t, aStream: *const nsIInputStream, aQueue: bool) -> nsresult,

    /* void removeEntry (in AUTF8String aZipEntry, in boolean aQueue); */
    pub removeEntry: unsafe extern "C" fn (this: *const nsIZipWriter, aZipEntry: *const nsACString, aQueue: bool) -> nsresult,

    /* void processQueue (in nsIRequestObserver aObserver, in nsISupports aContext); */
    pub processQueue: unsafe extern "C" fn (this: *const nsIZipWriter, aObserver: *const nsIRequestObserver, aContext: *const nsISupports) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIZipWriter) -> nsresult,

    /* void alignStoredFiles (in uint16_t aAlignSize); */
    pub alignStoredFiles: unsafe extern "C" fn (this: *const nsIZipWriter, aAlignSize: uint16_t) -> nsresult,

}


impl nsIZipWriter {
    /* attribute ACString comment; */
    #[inline]
    pub unsafe fn get_comment(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_comment)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_comment(&self, aComment: &[u8]) -> Result<(), nsresult> {
        let aComment = nsCString::from(aComment);
        match ((*self.vtable).set_comment)(self as *const _, &*aComment) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean inQueue; */
    #[inline]
    pub unsafe fn get_inQueue(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inQueue)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

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

    /* void open (in nsIFile aFile, in int32_t aIoFlags); */
    #[inline]
    pub unsafe fn open(&self, aFile: Option<&nsIFile>, aIoFlags: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).open)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _), aIoFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIZipEntry getEntry (in AUTF8String aZipEntry); */
    #[inline]
    pub unsafe fn getEntry(&self, aZipEntry: &[u8]) -> Result<Option<RefPtr<nsIZipEntry>>, nsresult> {
        let aZipEntry = nsCString::from(aZipEntry);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEntry)(self as *const _, &*aZipEntry, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean hasEntry (in AUTF8String aZipEntry); */
    #[inline]
    pub unsafe fn hasEntry(&self, aZipEntry: &[u8]) -> Result<bool, nsresult> {
        let aZipEntry = nsCString::from(aZipEntry);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasEntry)(self as *const _, &*aZipEntry, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addEntryDirectory (in AUTF8String aZipEntry, in PRTime aModTime, in boolean aQueue); */
    #[inline]
    pub unsafe fn addEntryDirectory(&self, aZipEntry: &[u8], aModTime: PRTime, aQueue: bool) -> Result<(), nsresult> {
        let aZipEntry = nsCString::from(aZipEntry);
        match ((*self.vtable).addEntryDirectory)(self as *const _, &*aZipEntry, aModTime, aQueue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addEntryFile (in AUTF8String aZipEntry, in int32_t aCompression, in nsIFile aFile, in boolean aQueue); */
    #[inline]
    pub unsafe fn addEntryFile(&self, aZipEntry: &[u8], aCompression: int32_t, aFile: Option<&nsIFile>, aQueue: bool) -> Result<(), nsresult> {
        let aZipEntry = nsCString::from(aZipEntry);
        match ((*self.vtable).addEntryFile)(self as *const _, &*aZipEntry, aCompression, aFile.map_or(::std::ptr::null(), |x| x as *const _), aQueue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addEntryChannel (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIChannel aChannel, in boolean aQueue); */
    #[inline]
    pub unsafe fn addEntryChannel(&self, aZipEntry: &[u8], aModTime: PRTime, aCompression: int32_t, aChannel: Option<&nsIChannel>, aQueue: bool) -> Result<(), nsresult> {
        let aZipEntry = nsCString::from(aZipEntry);
        match ((*self.vtable).addEntryChannel)(self as *const _, &*aZipEntry, aModTime, aCompression, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aQueue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addEntryStream (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIInputStream aStream, in boolean aQueue); */
    #[inline]
    pub unsafe fn addEntryStream(&self, aZipEntry: &[u8], aModTime: PRTime, aCompression: int32_t, aStream: Option<&nsIInputStream>, aQueue: bool) -> Result<(), nsresult> {
        let aZipEntry = nsCString::from(aZipEntry);
        match ((*self.vtable).addEntryStream)(self as *const _, &*aZipEntry, aModTime, aCompression, aStream.map_or(::std::ptr::null(), |x| x as *const _), aQueue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeEntry (in AUTF8String aZipEntry, in boolean aQueue); */
    #[inline]
    pub unsafe fn removeEntry(&self, aZipEntry: &[u8], aQueue: bool) -> Result<(), nsresult> {
        let aZipEntry = nsCString::from(aZipEntry);
        match ((*self.vtable).removeEntry)(self as *const _, &*aZipEntry, aQueue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void processQueue (in nsIRequestObserver aObserver, in nsISupports aContext); */
    #[inline]
    pub unsafe fn processQueue(&self, aObserver: Option<&nsIRequestObserver>, aContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).processQueue)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void alignStoredFiles (in uint16_t aAlignSize); */
    #[inline]
    pub unsafe fn alignStoredFiles(&self, aAlignSize: uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).alignStoredFiles)(self as *const _, aAlignSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


