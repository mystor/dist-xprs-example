//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFile.idl
//


pub mod nsIFile_consts {
    pub const NORMAL_FILE_TYPE: i64 = 0;
    pub const DIRECTORY_TYPE: i64 = 1;
    pub const OS_READAHEAD: i64 = 1073741824;
    pub const DELETE_ON_CLOSE: i64 = 2147483648;
}


#[repr(C)]
pub struct nsIFile {
    vtable: *const nsIFileVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFile {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2fa6884a, 0xae65, 0x412a,
            [0x9d, 0x4c, 0xce, 0x6e, 0x34, 0x54, 0x4b, 0xa1])
    }
}

unsafe impl RefCounted for nsIFile {
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
pub trait nsIFileCoerce {
    fn coerce_from(v: &nsIFile) -> &Self;
}

impl nsIFileCoerce for nsIFile {
    #[inline]
    fn coerce_from(v: &nsIFile) -> &Self {
        v
    }
}

impl nsIFile {
    #[inline]
    pub fn coerce<T: nsIFileCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFile {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFileCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFile) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFileVTable {
    pub __base: nsISupportsVTable,

    /* void append (in AString node); */
    pub append: unsafe extern "C" fn (this: *const nsIFile, node: *const nsAString) -> nsresult,

    /* [noscript] void appendNative (in ACString node); */
    pub appendNative: unsafe extern "C" fn (this: *const nsIFile, node: *const nsACString) -> nsresult,

    /* void normalize (); */
    pub normalize: unsafe extern "C" fn (this: *const nsIFile) -> nsresult,

    /* [must_use] void create (in unsigned long type, in unsigned long permissions); */
    pub create: unsafe extern "C" fn (this: *const nsIFile, type_: libc::uint32_t, permissions: libc::uint32_t) -> nsresult,

    /* attribute AString leafName; */
    pub get_leafName: unsafe extern "C" fn (this: *const nsIFile, aLeafName: *mut nsAString) -> nsresult,
    pub set_leafName: unsafe extern "C" fn (this: *const nsIFile, aLeafName: *const nsAString) -> nsresult,

    /* [noscript] attribute ACString nativeLeafName; */
    pub get_nativeLeafName: unsafe extern "C" fn (this: *const nsIFile, aNativeLeafName: *mut nsACString) -> nsresult,
    pub set_nativeLeafName: unsafe extern "C" fn (this: *const nsIFile, aNativeLeafName: *const nsACString) -> nsresult,

    /* void copyTo (in nsIFile newParentDir, in AString newName); */
    pub copyTo: unsafe extern "C" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const nsAString) -> nsresult,

    /* [noscript] void CopyToNative (in nsIFile newParentDir, in ACString newName); */
    pub CopyToNative: unsafe extern "C" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const nsACString) -> nsresult,

    /* void copyToFollowingLinks (in nsIFile newParentDir, in AString newName); */
    pub copyToFollowingLinks: unsafe extern "C" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const nsAString) -> nsresult,

    /* [noscript] void copyToFollowingLinksNative (in nsIFile newParentDir, in ACString newName); */
    pub copyToFollowingLinksNative: unsafe extern "C" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const nsACString) -> nsresult,

    /* void moveTo (in nsIFile newParentDir, in AString newName); */
    pub moveTo: unsafe extern "C" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const nsAString) -> nsresult,

    /* [noscript] void moveToNative (in nsIFile newParentDir, in ACString newName); */
    pub moveToNative: unsafe extern "C" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const nsACString) -> nsresult,

    /* void renameTo (in nsIFile newParentDir, in AString newName); */
    pub renameTo: unsafe extern "C" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const nsAString) -> nsresult,

    /* [noscript] void renameToNative (in nsIFile newParentDir, in ACString newName); */
    pub renameToNative: unsafe extern "C" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const nsACString) -> nsresult,

    /* void remove (in boolean recursive); */
    pub remove: unsafe extern "C" fn (this: *const nsIFile, recursive: bool) -> nsresult,

    /* attribute unsigned long permissions; */
    pub get_permissions: unsafe extern "C" fn (this: *const nsIFile, aPermissions: *mut libc::uint32_t) -> nsresult,
    pub set_permissions: unsafe extern "C" fn (this: *const nsIFile, aPermissions: libc::uint32_t) -> nsresult,

    /* attribute unsigned long permissionsOfLink; */
    pub get_permissionsOfLink: unsafe extern "C" fn (this: *const nsIFile, aPermissionsOfLink: *mut libc::uint32_t) -> nsresult,
    pub set_permissionsOfLink: unsafe extern "C" fn (this: *const nsIFile, aPermissionsOfLink: libc::uint32_t) -> nsresult,

    /* attribute PRTime lastModifiedTime; */
    pub get_lastModifiedTime: unsafe extern "C" fn (this: *const nsIFile, aLastModifiedTime: *mut PRTime) -> nsresult,
    pub set_lastModifiedTime: unsafe extern "C" fn (this: *const nsIFile, aLastModifiedTime: PRTime) -> nsresult,

    /* attribute PRTime lastModifiedTimeOfLink; */
    pub get_lastModifiedTimeOfLink: unsafe extern "C" fn (this: *const nsIFile, aLastModifiedTimeOfLink: *mut PRTime) -> nsresult,
    pub set_lastModifiedTimeOfLink: unsafe extern "C" fn (this: *const nsIFile, aLastModifiedTimeOfLink: PRTime) -> nsresult,

    /* attribute int64_t fileSize; */
    pub get_fileSize: unsafe extern "C" fn (this: *const nsIFile, aFileSize: *mut int64_t) -> nsresult,
    pub set_fileSize: unsafe extern "C" fn (this: *const nsIFile, aFileSize: int64_t) -> nsresult,

    /* readonly attribute int64_t fileSizeOfLink; */
    pub get_fileSizeOfLink: unsafe extern "C" fn (this: *const nsIFile, aFileSizeOfLink: *mut int64_t) -> nsresult,

    /* readonly attribute AString target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIFile, aTarget: *mut nsAString) -> nsresult,

    /* [noscript] readonly attribute ACString nativeTarget; */
    pub get_nativeTarget: unsafe extern "C" fn (this: *const nsIFile, aNativeTarget: *mut nsACString) -> nsresult,

    /* readonly attribute AString path; */
    pub get_path: unsafe extern "C" fn (this: *const nsIFile, aPath: *mut nsAString) -> nsresult,

    /* [noscript] readonly attribute ACString nativePath; */
    pub get_nativePath: unsafe extern "C" fn (this: *const nsIFile, aNativePath: *mut nsACString) -> nsresult,

    /* boolean exists (); */
    pub exists: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean isWritable (); */
    pub isWritable: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean isReadable (); */
    pub isReadable: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean isExecutable (); */
    pub isExecutable: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean isHidden (); */
    pub isHidden: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean isDirectory (); */
    pub isDirectory: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean isFile (); */
    pub isFile: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean isSymlink (); */
    pub isSymlink: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean isSpecial (); */
    pub isSpecial: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* [must_use] void createUnique (in unsigned long type, in unsigned long permissions); */
    pub createUnique: unsafe extern "C" fn (this: *const nsIFile, type_: libc::uint32_t, permissions: libc::uint32_t) -> nsresult,

    /* nsIFile clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsIFile, _retval: *mut *const nsIFile) -> nsresult,

    /* boolean equals (in nsIFile inFile); */
    pub equals: unsafe extern "C" fn (this: *const nsIFile, inFile: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* boolean contains (in nsIFile inFile); */
    pub contains: unsafe extern "C" fn (this: *const nsIFile, inFile: *const nsIFile, _retval: *mut bool) -> nsresult,

    /* readonly attribute nsIFile parent; */
    pub get_parent: unsafe extern "C" fn (this: *const nsIFile, aParent: *mut *const nsIFile) -> nsresult,

    /* readonly attribute nsISimpleEnumerator directoryEntries; */
    pub get_directoryEntries: unsafe extern "C" fn (this: *const nsIFile, aDirectoryEntries: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void initWithPath (in AString filePath); */
    pub initWithPath: unsafe extern "C" fn (this: *const nsIFile, filePath: *const nsAString) -> nsresult,

    /* [noscript] void initWithNativePath (in ACString filePath); */
    pub initWithNativePath: unsafe extern "C" fn (this: *const nsIFile, filePath: *const nsACString) -> nsresult,

    /* void initWithFile (in nsIFile aFile); */
    pub initWithFile: unsafe extern "C" fn (this: *const nsIFile, aFile: *const nsIFile) -> nsresult,

    /* attribute boolean followLinks; */
    pub get_followLinks: unsafe extern "C" fn (this: *const nsIFile, aFollowLinks: *mut bool) -> nsresult,
    pub set_followLinks: unsafe extern "C" fn (this: *const nsIFile, aFollowLinks: bool) -> nsresult,

    /* [must_use,noscript] PRFileDescStar openNSPRFileDesc (in long flags, in long mode); */
    /// Unable to call function as its signature contains a non-rust type
    pub openNSPRFileDesc: *const ::libc::c_void,

    /* [must_use,noscript] FILE openANSIFileDesc (in string mode); */
    /// Unable to call function as its signature contains a non-rust type
    pub openANSIFileDesc: *const ::libc::c_void,

    /* [must_use,noscript] PRLibraryStar load (); */
    /// Unable to call function as its signature contains a non-rust type
    pub load: *const ::libc::c_void,

    /* [must_use] readonly attribute int64_t diskSpaceAvailable; */
    pub get_diskSpaceAvailable: unsafe extern "C" fn (this: *const nsIFile, aDiskSpaceAvailable: *mut int64_t) -> nsresult,

    /* void appendRelativePath (in AString relativeFilePath); */
    pub appendRelativePath: unsafe extern "C" fn (this: *const nsIFile, relativeFilePath: *const nsAString) -> nsresult,

    /* [noscript] void appendRelativeNativePath (in ACString relativeFilePath); */
    pub appendRelativeNativePath: unsafe extern "C" fn (this: *const nsIFile, relativeFilePath: *const nsACString) -> nsresult,

    /* [must_use] attribute ACString persistentDescriptor; */
    pub get_persistentDescriptor: unsafe extern "C" fn (this: *const nsIFile, aPersistentDescriptor: *mut nsACString) -> nsresult,
    pub set_persistentDescriptor: unsafe extern "C" fn (this: *const nsIFile, aPersistentDescriptor: *const nsACString) -> nsresult,

    /* [must_use] void reveal (); */
    pub reveal: unsafe extern "C" fn (this: *const nsIFile) -> nsresult,

    /* [must_use] void launch (); */
    pub launch: unsafe extern "C" fn (this: *const nsIFile) -> nsresult,

    /* [must_use] ACString getRelativeDescriptor (in nsIFile fromFile); */
    pub getRelativeDescriptor: unsafe extern "C" fn (this: *const nsIFile, fromFile: *const nsIFile, _retval: *mut nsACString) -> nsresult,

    /* [must_use] void setRelativeDescriptor (in nsIFile fromFile, in ACString relativeDesc); */
    pub setRelativeDescriptor: unsafe extern "C" fn (this: *const nsIFile, fromFile: *const nsIFile, relativeDesc: *const nsACString) -> nsresult,

    /* [must_use] AUTF8String getRelativePath (in nsIFile fromFile); */
    pub getRelativePath: unsafe extern "C" fn (this: *const nsIFile, fromFile: *const nsIFile, _retval: *mut nsACString) -> nsresult,

    /* [must_use] void setRelativePath (in nsIFile fromFile, in AUTF8String relativeDesc); */
    pub setRelativePath: unsafe extern "C" fn (this: *const nsIFile, fromFile: *const nsIFile, relativeDesc: *const nsACString) -> nsresult,

}


impl nsIFile {
    /* void append (in AString node); */
    #[inline]
    pub unsafe fn append(&self, node: &[u16]) -> Result<(), nsresult> {
        let node = nsString::from(node);
        match ((*self.vtable).append)(self as *const _, &*node) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void appendNative (in ACString node); */
    #[inline]
    pub unsafe fn appendNative(&self, node: &[u8]) -> Result<(), nsresult> {
        let node = nsCString::from(node);
        match ((*self.vtable).appendNative)(self as *const _, &*node) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void normalize (); */
    #[inline]
    pub unsafe fn normalize(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).normalize)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void create (in unsigned long type, in unsigned long permissions); */
    #[inline]
    pub unsafe fn create(&self, type_: libc::uint32_t, permissions: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).create)(self as *const _, type_, permissions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString leafName; */
    #[inline]
    pub unsafe fn get_leafName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_leafName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_leafName(&self, aLeafName: &[u16]) -> Result<(), nsresult> {
        let aLeafName = nsString::from(aLeafName);
        match ((*self.vtable).set_leafName)(self as *const _, &*aLeafName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute ACString nativeLeafName; */
    #[inline]
    pub unsafe fn get_nativeLeafName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_nativeLeafName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_nativeLeafName(&self, aNativeLeafName: &[u8]) -> Result<(), nsresult> {
        let aNativeLeafName = nsCString::from(aNativeLeafName);
        match ((*self.vtable).set_nativeLeafName)(self as *const _, &*aNativeLeafName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copyTo (in nsIFile newParentDir, in AString newName); */
    #[inline]
    pub unsafe fn copyTo(&self, newParentDir: Option<&nsIFile>, newName: &[u16]) -> Result<(), nsresult> {
        let newName = nsString::from(newName);
        match ((*self.vtable).copyTo)(self as *const _, newParentDir.map_or(::std::ptr::null(), |x| x as *const _), &*newName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void CopyToNative (in nsIFile newParentDir, in ACString newName); */
    #[inline]
    pub unsafe fn CopyToNative(&self, newParentDir: Option<&nsIFile>, newName: &[u8]) -> Result<(), nsresult> {
        let newName = nsCString::from(newName);
        match ((*self.vtable).CopyToNative)(self as *const _, newParentDir.map_or(::std::ptr::null(), |x| x as *const _), &*newName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copyToFollowingLinks (in nsIFile newParentDir, in AString newName); */
    #[inline]
    pub unsafe fn copyToFollowingLinks(&self, newParentDir: Option<&nsIFile>, newName: &[u16]) -> Result<(), nsresult> {
        let newName = nsString::from(newName);
        match ((*self.vtable).copyToFollowingLinks)(self as *const _, newParentDir.map_or(::std::ptr::null(), |x| x as *const _), &*newName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void copyToFollowingLinksNative (in nsIFile newParentDir, in ACString newName); */
    #[inline]
    pub unsafe fn copyToFollowingLinksNative(&self, newParentDir: Option<&nsIFile>, newName: &[u8]) -> Result<(), nsresult> {
        let newName = nsCString::from(newName);
        match ((*self.vtable).copyToFollowingLinksNative)(self as *const _, newParentDir.map_or(::std::ptr::null(), |x| x as *const _), &*newName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void moveTo (in nsIFile newParentDir, in AString newName); */
    #[inline]
    pub unsafe fn moveTo(&self, newParentDir: Option<&nsIFile>, newName: &[u16]) -> Result<(), nsresult> {
        let newName = nsString::from(newName);
        match ((*self.vtable).moveTo)(self as *const _, newParentDir.map_or(::std::ptr::null(), |x| x as *const _), &*newName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void moveToNative (in nsIFile newParentDir, in ACString newName); */
    #[inline]
    pub unsafe fn moveToNative(&self, newParentDir: Option<&nsIFile>, newName: &[u8]) -> Result<(), nsresult> {
        let newName = nsCString::from(newName);
        match ((*self.vtable).moveToNative)(self as *const _, newParentDir.map_or(::std::ptr::null(), |x| x as *const _), &*newName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void renameTo (in nsIFile newParentDir, in AString newName); */
    #[inline]
    pub unsafe fn renameTo(&self, newParentDir: Option<&nsIFile>, newName: &[u16]) -> Result<(), nsresult> {
        let newName = nsString::from(newName);
        match ((*self.vtable).renameTo)(self as *const _, newParentDir.map_or(::std::ptr::null(), |x| x as *const _), &*newName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void renameToNative (in nsIFile newParentDir, in ACString newName); */
    #[inline]
    pub unsafe fn renameToNative(&self, newParentDir: Option<&nsIFile>, newName: &[u8]) -> Result<(), nsresult> {
        let newName = nsCString::from(newName);
        match ((*self.vtable).renameToNative)(self as *const _, newParentDir.map_or(::std::ptr::null(), |x| x as *const _), &*newName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void remove (in boolean recursive); */
    #[inline]
    pub unsafe fn remove(&self, recursive: bool) -> Result<(), nsresult> {

        match ((*self.vtable).remove)(self as *const _, recursive) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long permissions; */
    #[inline]
    pub unsafe fn get_permissions(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_permissions)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_permissions(&self, aPermissions: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_permissions)(self as *const _, aPermissions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long permissionsOfLink; */
    #[inline]
    pub unsafe fn get_permissionsOfLink(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_permissionsOfLink)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_permissionsOfLink(&self, aPermissionsOfLink: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_permissionsOfLink)(self as *const _, aPermissionsOfLink) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute PRTime lastModifiedTime; */
    #[inline]
    pub unsafe fn get_lastModifiedTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModifiedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_lastModifiedTime(&self, aLastModifiedTime: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).set_lastModifiedTime)(self as *const _, aLastModifiedTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute PRTime lastModifiedTimeOfLink; */
    #[inline]
    pub unsafe fn get_lastModifiedTimeOfLink(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModifiedTimeOfLink)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_lastModifiedTimeOfLink(&self, aLastModifiedTimeOfLink: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).set_lastModifiedTimeOfLink)(self as *const _, aLastModifiedTimeOfLink) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute int64_t fileSize; */
    #[inline]
    pub unsafe fn get_fileSize(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fileSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fileSize(&self, aFileSize: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_fileSize)(self as *const _, aFileSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute int64_t fileSizeOfLink; */
    #[inline]
    pub unsafe fn get_fileSizeOfLink(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fileSizeOfLink)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_target)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute ACString nativeTarget; */
    #[inline]
    pub unsafe fn get_nativeTarget(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_nativeTarget)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString path; */
    #[inline]
    pub unsafe fn get_path(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_path)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute ACString nativePath; */
    #[inline]
    pub unsafe fn get_nativePath(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_nativePath)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean exists (); */
    #[inline]
    pub unsafe fn exists(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).exists)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isWritable (); */
    #[inline]
    pub unsafe fn isWritable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isWritable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isReadable (); */
    #[inline]
    pub unsafe fn isReadable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isReadable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isExecutable (); */
    #[inline]
    pub unsafe fn isExecutable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isExecutable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isHidden (); */
    #[inline]
    pub unsafe fn isHidden(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isHidden)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isDirectory (); */
    #[inline]
    pub unsafe fn isDirectory(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isDirectory)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isFile (); */
    #[inline]
    pub unsafe fn isFile(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isFile)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isSymlink (); */
    #[inline]
    pub unsafe fn isSymlink(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSymlink)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isSpecial (); */
    #[inline]
    pub unsafe fn isSpecial(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSpecial)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void createUnique (in unsigned long type, in unsigned long permissions); */
    #[inline]
    pub unsafe fn createUnique(&self, type_: libc::uint32_t, permissions: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).createUnique)(self as *const _, type_, permissions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIFile clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean equals (in nsIFile inFile); */
    #[inline]
    pub unsafe fn equals(&self, inFile: Option<&nsIFile>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, inFile.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean contains (in nsIFile inFile); */
    #[inline]
    pub unsafe fn contains(&self, inFile: Option<&nsIFile>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).contains)(self as *const _, inFile.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIFile parent; */
    #[inline]
    pub unsafe fn get_parent(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISimpleEnumerator directoryEntries; */
    #[inline]
    pub unsafe fn get_directoryEntries(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_directoryEntries)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void initWithPath (in AString filePath); */
    #[inline]
    pub unsafe fn initWithPath(&self, filePath: &[u16]) -> Result<(), nsresult> {
        let filePath = nsString::from(filePath);
        match ((*self.vtable).initWithPath)(self as *const _, &*filePath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void initWithNativePath (in ACString filePath); */
    #[inline]
    pub unsafe fn initWithNativePath(&self, filePath: &[u8]) -> Result<(), nsresult> {
        let filePath = nsCString::from(filePath);
        match ((*self.vtable).initWithNativePath)(self as *const _, &*filePath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initWithFile (in nsIFile aFile); */
    #[inline]
    pub unsafe fn initWithFile(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).initWithFile)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean followLinks; */
    #[inline]
    pub unsafe fn get_followLinks(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_followLinks)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_followLinks(&self, aFollowLinks: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_followLinks)(self as *const _, aFollowLinks) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use,noscript] PRFileDescStar openNSPRFileDesc (in long flags, in long mode); */


    /* [must_use,noscript] FILE openANSIFileDesc (in string mode); */


    /* [must_use,noscript] PRLibraryStar load (); */


    /* [must_use] readonly attribute int64_t diskSpaceAvailable; */
    #[inline]
    pub unsafe fn get_diskSpaceAvailable(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_diskSpaceAvailable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void appendRelativePath (in AString relativeFilePath); */
    #[inline]
    pub unsafe fn appendRelativePath(&self, relativeFilePath: &[u16]) -> Result<(), nsresult> {
        let relativeFilePath = nsString::from(relativeFilePath);
        match ((*self.vtable).appendRelativePath)(self as *const _, &*relativeFilePath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void appendRelativeNativePath (in ACString relativeFilePath); */
    #[inline]
    pub unsafe fn appendRelativeNativePath(&self, relativeFilePath: &[u8]) -> Result<(), nsresult> {
        let relativeFilePath = nsCString::from(relativeFilePath);
        match ((*self.vtable).appendRelativeNativePath)(self as *const _, &*relativeFilePath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute ACString persistentDescriptor; */
    #[inline]
    pub unsafe fn get_persistentDescriptor(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_persistentDescriptor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_persistentDescriptor(&self, aPersistentDescriptor: &[u8]) -> Result<(), nsresult> {
        let aPersistentDescriptor = nsCString::from(aPersistentDescriptor);
        match ((*self.vtable).set_persistentDescriptor)(self as *const _, &*aPersistentDescriptor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void reveal (); */
    #[inline]
    pub unsafe fn reveal(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reveal)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void launch (); */
    #[inline]
    pub unsafe fn launch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).launch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] ACString getRelativeDescriptor (in nsIFile fromFile); */
    #[inline]
    pub unsafe fn getRelativeDescriptor(&self, fromFile: Option<&nsIFile>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getRelativeDescriptor)(self as *const _, fromFile.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void setRelativeDescriptor (in nsIFile fromFile, in ACString relativeDesc); */
    #[inline]
    pub unsafe fn setRelativeDescriptor(&self, fromFile: Option<&nsIFile>, relativeDesc: &[u8]) -> Result<(), nsresult> {
        let relativeDesc = nsCString::from(relativeDesc);
        match ((*self.vtable).setRelativeDescriptor)(self as *const _, fromFile.map_or(::std::ptr::null(), |x| x as *const _), &*relativeDesc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] AUTF8String getRelativePath (in nsIFile fromFile); */
    #[inline]
    pub unsafe fn getRelativePath(&self, fromFile: Option<&nsIFile>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getRelativePath)(self as *const _, fromFile.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void setRelativePath (in nsIFile fromFile, in AUTF8String relativeDesc); */
    #[inline]
    pub unsafe fn setRelativePath(&self, fromFile: Option<&nsIFile>, relativeDesc: &[u8]) -> Result<(), nsresult> {
        let relativeDesc = nsCString::from(relativeDesc);
        match ((*self.vtable).setRelativePath)(self as *const _, fromFile.map_or(::std::ptr::null(), |x| x as *const _), &*relativeDesc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


