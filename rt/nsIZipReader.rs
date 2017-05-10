//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIZipReader.idl
//


#[repr(C)]
pub struct nsIZipEntry {
    vtable: *const nsIZipEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIZipEntry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfad6f72f, 0x13d8, 0x4e26,
            [0x91, 0x73, 0x53, 0x00, 0x7a, 0x4a, 0xfe, 0x71])
    }
}

unsafe impl RefCounted for nsIZipEntry {
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
pub trait nsIZipEntryCoerce {
    fn coerce_from(v: &nsIZipEntry) -> &Self;
}

impl nsIZipEntryCoerce for nsIZipEntry {
    #[inline]
    fn coerce_from(v: &nsIZipEntry) -> &Self {
        v
    }
}

impl nsIZipEntry {
    #[inline]
    pub fn coerce<T: nsIZipEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIZipEntry {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIZipEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIZipEntry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIZipEntryVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short compression; */
    pub get_compression: unsafe extern "C" fn (this: *const nsIZipEntry, aCompression: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute unsigned long size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIZipEntry, aSize: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long realSize; */
    pub get_realSize: unsafe extern "C" fn (this: *const nsIZipEntry, aRealSize: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long CRC32; */
    pub get_CRC32: unsafe extern "C" fn (this: *const nsIZipEntry, aCRC32: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean isDirectory; */
    pub get_isDirectory: unsafe extern "C" fn (this: *const nsIZipEntry, aIsDirectory: *mut bool) -> nsresult,

    /* readonly attribute PRTime lastModifiedTime; */
    pub get_lastModifiedTime: unsafe extern "C" fn (this: *const nsIZipEntry, aLastModifiedTime: *mut PRTime) -> nsresult,

    /* readonly attribute boolean isSynthetic; */
    pub get_isSynthetic: unsafe extern "C" fn (this: *const nsIZipEntry, aIsSynthetic: *mut bool) -> nsresult,

    /* readonly attribute unsigned long permissions; */
    pub get_permissions: unsafe extern "C" fn (this: *const nsIZipEntry, aPermissions: *mut libc::uint32_t) -> nsresult,

}


impl nsIZipEntry {
    /* readonly attribute unsigned short compression; */
    #[inline]
    pub unsafe fn get_compression(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_compression)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_size)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long realSize; */
    #[inline]
    pub unsafe fn get_realSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_realSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long CRC32; */
    #[inline]
    pub unsafe fn get_CRC32(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_CRC32)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isDirectory; */
    #[inline]
    pub unsafe fn get_isDirectory(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDirectory)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime lastModifiedTime; */
    #[inline]
    pub unsafe fn get_lastModifiedTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModifiedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isSynthetic; */
    #[inline]
    pub unsafe fn get_isSynthetic(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSynthetic)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long permissions; */
    #[inline]
    pub unsafe fn get_permissions(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_permissions)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIZipReader {
    vtable: *const nsIZipReaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIZipReader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9ba4ef54, 0xe0a0, 0x4f65,
            [0x9d, 0x23, 0x12, 0x84, 0x82, 0x44, 0x88, 0x85])
    }
}

unsafe impl RefCounted for nsIZipReader {
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
pub trait nsIZipReaderCoerce {
    fn coerce_from(v: &nsIZipReader) -> &Self;
}

impl nsIZipReaderCoerce for nsIZipReader {
    #[inline]
    fn coerce_from(v: &nsIZipReader) -> &Self {
        v
    }
}

impl nsIZipReader {
    #[inline]
    pub fn coerce<T: nsIZipReaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIZipReader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIZipReaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIZipReader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIZipReaderVTable {
    pub __base: nsISupportsVTable,

    /* void open (in nsIFile zipFile); */
    pub open: unsafe extern "C" fn (this: *const nsIZipReader, zipFile: *const nsIFile) -> nsresult,

    /* void openInner (in nsIZipReader zipReader, in AUTF8String zipEntry); */
    pub openInner: unsafe extern "C" fn (this: *const nsIZipReader, zipReader: *const nsIZipReader, zipEntry: *const nsACString) -> nsresult,

    /* void openMemory (in voidPtr aData, in unsigned long aLength); */
    pub openMemory: unsafe extern "C" fn (this: *const nsIZipReader, aData: *const libc::c_void, aLength: libc::uint32_t) -> nsresult,

    /* readonly attribute nsIFile file; */
    pub get_file: unsafe extern "C" fn (this: *const nsIZipReader, aFile: *mut *const nsIFile) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIZipReader) -> nsresult,

    /* void test (in AUTF8String aEntryName); */
    pub test: unsafe extern "C" fn (this: *const nsIZipReader, aEntryName: *const nsACString) -> nsresult,

    /* void extract (in AUTF8String zipEntry, in nsIFile outFile); */
    pub extract: unsafe extern "C" fn (this: *const nsIZipReader, zipEntry: *const nsACString, outFile: *const nsIFile) -> nsresult,

    /* nsIZipEntry getEntry (in AUTF8String zipEntry); */
    pub getEntry: unsafe extern "C" fn (this: *const nsIZipReader, zipEntry: *const nsACString, _retval: *mut *const nsIZipEntry) -> nsresult,

    /* boolean hasEntry (in AUTF8String zipEntry); */
    pub hasEntry: unsafe extern "C" fn (this: *const nsIZipReader, zipEntry: *const nsACString, _retval: *mut bool) -> nsresult,

    /* nsIUTF8StringEnumerator findEntries (in AUTF8String aPattern); */
    pub findEntries: unsafe extern "C" fn (this: *const nsIZipReader, aPattern: *const nsACString, _retval: *mut *const nsIUTF8StringEnumerator) -> nsresult,

    /* nsIInputStream getInputStream (in AUTF8String zipEntry); */
    pub getInputStream: unsafe extern "C" fn (this: *const nsIZipReader, zipEntry: *const nsACString, _retval: *mut *const nsIInputStream) -> nsresult,

    /* nsIInputStream getInputStreamWithSpec (in AUTF8String aJarSpec, in AUTF8String zipEntry); */
    pub getInputStreamWithSpec: unsafe extern "C" fn (this: *const nsIZipReader, aJarSpec: *const nsACString, zipEntry: *const nsACString, _retval: *mut *const nsIInputStream) -> nsresult,

    /* nsIX509Cert getSigningCert (in AUTF8String aEntryName); */
    pub getSigningCert: unsafe extern "C" fn (this: *const nsIZipReader, aEntryName: *const nsACString, _retval: *mut *const nsIX509Cert) -> nsresult,

    /* readonly attribute uint32_t manifestEntriesCount; */
    pub get_manifestEntriesCount: unsafe extern "C" fn (this: *const nsIZipReader, aManifestEntriesCount: *mut uint32_t) -> nsresult,

}


impl nsIZipReader {
    /* void open (in nsIFile zipFile); */
    #[inline]
    pub unsafe fn open(&self, zipFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).open)(self as *const _, zipFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void openInner (in nsIZipReader zipReader, in AUTF8String zipEntry); */
    #[inline]
    pub unsafe fn openInner(&self, zipReader: Option<&nsIZipReader>, zipEntry: &[u8]) -> Result<(), nsresult> {
        let zipEntry = nsCString::from(zipEntry);
        match ((*self.vtable).openInner)(self as *const _, zipReader.map_or(::std::ptr::null(), |x| x as *const _), &*zipEntry) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void openMemory (in voidPtr aData, in unsigned long aLength); */
    #[inline]
    pub unsafe fn openMemory(&self, aData: *const libc::c_void, aLength: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).openMemory)(self as *const _, aData, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void test (in AUTF8String aEntryName); */
    #[inline]
    pub unsafe fn test(&self, aEntryName: &[u8]) -> Result<(), nsresult> {
        let aEntryName = nsCString::from(aEntryName);
        match ((*self.vtable).test)(self as *const _, &*aEntryName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void extract (in AUTF8String zipEntry, in nsIFile outFile); */
    #[inline]
    pub unsafe fn extract(&self, zipEntry: &[u8], outFile: Option<&nsIFile>) -> Result<(), nsresult> {
        let zipEntry = nsCString::from(zipEntry);
        match ((*self.vtable).extract)(self as *const _, &*zipEntry, outFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIZipEntry getEntry (in AUTF8String zipEntry); */
    #[inline]
    pub unsafe fn getEntry(&self, zipEntry: &[u8]) -> Result<Option<RefPtr<nsIZipEntry>>, nsresult> {
        let zipEntry = nsCString::from(zipEntry);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEntry)(self as *const _, &*zipEntry, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean hasEntry (in AUTF8String zipEntry); */
    #[inline]
    pub unsafe fn hasEntry(&self, zipEntry: &[u8]) -> Result<bool, nsresult> {
        let zipEntry = nsCString::from(zipEntry);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasEntry)(self as *const _, &*zipEntry, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIUTF8StringEnumerator findEntries (in AUTF8String aPattern); */
    #[inline]
    pub unsafe fn findEntries(&self, aPattern: &[u8]) -> Result<Option<RefPtr<nsIUTF8StringEnumerator>>, nsresult> {
        let aPattern = nsCString::from(aPattern);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findEntries)(self as *const _, &*aPattern, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIInputStream getInputStream (in AUTF8String zipEntry); */
    #[inline]
    pub unsafe fn getInputStream(&self, zipEntry: &[u8]) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let zipEntry = nsCString::from(zipEntry);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInputStream)(self as *const _, &*zipEntry, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIInputStream getInputStreamWithSpec (in AUTF8String aJarSpec, in AUTF8String zipEntry); */
    #[inline]
    pub unsafe fn getInputStreamWithSpec(&self, aJarSpec: &[u8], zipEntry: &[u8]) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let aJarSpec = nsCString::from(aJarSpec);
        let zipEntry = nsCString::from(zipEntry);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInputStreamWithSpec)(self as *const _, &*aJarSpec, &*zipEntry, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIX509Cert getSigningCert (in AUTF8String aEntryName); */
    #[inline]
    pub unsafe fn getSigningCert(&self, aEntryName: &[u8]) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let aEntryName = nsCString::from(aEntryName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSigningCert)(self as *const _, &*aEntryName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute uint32_t manifestEntriesCount; */
    #[inline]
    pub unsafe fn get_manifestEntriesCount(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_manifestEntriesCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIZipReaderCache {
    vtable: *const nsIZipReaderCacheVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIZipReaderCache {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x31179807, 0x9fcd, 0x46c4,
            [0xbe, 0xfa, 0x2a, 0xde, 0x20, 0x9a, 0x39, 0x4b])
    }
}

unsafe impl RefCounted for nsIZipReaderCache {
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
pub trait nsIZipReaderCacheCoerce {
    fn coerce_from(v: &nsIZipReaderCache) -> &Self;
}

impl nsIZipReaderCacheCoerce for nsIZipReaderCache {
    #[inline]
    fn coerce_from(v: &nsIZipReaderCache) -> &Self {
        v
    }
}

impl nsIZipReaderCache {
    #[inline]
    pub fn coerce<T: nsIZipReaderCacheCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIZipReaderCache {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIZipReaderCacheCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIZipReaderCache) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIZipReaderCacheVTable {
    pub __base: nsISupportsVTable,

    /* void init (in unsigned long cacheSize); */
    pub init: unsafe extern "C" fn (this: *const nsIZipReaderCache, cacheSize: libc::uint32_t) -> nsresult,

    /* nsIZipReader getZip (in nsIFile zipFile); */
    pub getZip: unsafe extern "C" fn (this: *const nsIZipReaderCache, zipFile: *const nsIFile, _retval: *mut *const nsIZipReader) -> nsresult,

    /* bool isCached (in nsIFile zipFile); */
    pub isCached: unsafe extern "C" fn (this: *const nsIZipReaderCache, zipFile: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* nsIZipReader getInnerZip (in nsIFile zipFile, in AUTF8String zipEntry); */
    pub getInnerZip: unsafe extern "C" fn (this: *const nsIZipReaderCache, zipFile: *const nsIFile, zipEntry: *const nsACString, _retval: *mut *const nsIZipReader) -> nsresult,

    /* PRFileDescStar getFd (in nsIFile zipFile); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFd: *const ::libc::c_void,

}


impl nsIZipReaderCache {
    /* void init (in unsigned long cacheSize); */
    #[inline]
    pub unsafe fn init(&self, cacheSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, cacheSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIZipReader getZip (in nsIFile zipFile); */
    #[inline]
    pub unsafe fn getZip(&self, zipFile: Option<&nsIFile>) -> Result<Option<RefPtr<nsIZipReader>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getZip)(self as *const _, zipFile.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* bool isCached (in nsIFile zipFile); */
    #[inline]
    pub unsafe fn isCached(&self, zipFile: Option<&nsIFile>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCached)(self as *const _, zipFile.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIZipReader getInnerZip (in nsIFile zipFile, in AUTF8String zipEntry); */
    #[inline]
    pub unsafe fn getInnerZip(&self, zipFile: Option<&nsIFile>, zipEntry: &[u8]) -> Result<Option<RefPtr<nsIZipReader>>, nsresult> {
        let zipEntry = nsCString::from(zipEntry);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInnerZip)(self as *const _, zipFile.map_or(::std::ptr::null(), |x| x as *const _), &*zipEntry, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* PRFileDescStar getFd (in nsIFile zipFile); */


}


