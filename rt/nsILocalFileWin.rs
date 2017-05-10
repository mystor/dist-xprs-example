//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILocalFileWin.idl
//


pub mod nsILocalFileWin_consts {
    pub const WFA_SEARCH_INDEXED: i64 = 1;
    pub const WFA_READONLY: i64 = 2;
    pub const WFA_READWRITE: i64 = 4;
}


#[repr(C)]
pub struct nsILocalFileWin {
    vtable: *const nsILocalFileWinVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocalFileWin {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe7a3a954, 0x384b, 0x4aeb,
            [0xa5, 0xf7, 0x55, 0x62, 0x6b, 0x0d, 0xe9, 0xbe])
    }
}

unsafe impl RefCounted for nsILocalFileWin {
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
pub trait nsILocalFileWinCoerce {
    fn coerce_from(v: &nsILocalFileWin) -> &Self;
}

impl nsILocalFileWinCoerce for nsILocalFileWin {
    #[inline]
    fn coerce_from(v: &nsILocalFileWin) -> &Self {
        v
    }
}

impl nsILocalFileWin {
    #[inline]
    pub fn coerce<T: nsILocalFileWinCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocalFileWin {
    type Target = nsILocalFile;
    #[inline]
    fn deref(&self) -> &nsILocalFile {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsILocalFileCoerce> nsILocalFileWinCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalFileWin) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocalFileWinVTable {
    pub __base: nsILocalFileVTable,

    /* void initWithCommandLine (in AString aCommandLine); */
    pub initWithCommandLine: unsafe extern "C" fn (this: *const nsILocalFileWin, aCommandLine: *const nsAString) -> nsresult,

    /* AString getVersionInfoField (in string aField); */
    pub getVersionInfoField: unsafe extern "C" fn (this: *const nsILocalFileWin, aField: *const libc::c_char, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute AString canonicalPath; */
    pub get_canonicalPath: unsafe extern "C" fn (this: *const nsILocalFileWin, aCanonicalPath: *mut nsAString) -> nsresult,

    /* [noscript] readonly attribute ACString nativeCanonicalPath; */
    pub get_nativeCanonicalPath: unsafe extern "C" fn (this: *const nsILocalFileWin, aNativeCanonicalPath: *mut nsACString) -> nsresult,

    /* attribute unsigned long fileAttributesWin; */
    pub get_fileAttributesWin: unsafe extern "C" fn (this: *const nsILocalFileWin, aFileAttributesWin: *mut libc::uint32_t) -> nsresult,
    pub set_fileAttributesWin: unsafe extern "C" fn (this: *const nsILocalFileWin, aFileAttributesWin: libc::uint32_t) -> nsresult,

    /* void setShortcut ([optional] in nsIFile targetFile, [optional] in nsIFile workingDir, [optional] in wstring args, [optional] in wstring description, [optional] in nsIFile iconFile, [optional] in long iconIndex); */
    pub setShortcut: unsafe extern "C" fn (this: *const nsILocalFileWin, targetFile: *const nsIFile, workingDir: *const nsIFile, args: *const libc::int16_t, description: *const libc::int16_t, iconFile: *const nsIFile, iconIndex: libc::int32_t) -> nsresult,

    /* [noscript] PRFileDescStar openNSPRFileDescShareDelete (in long flags, in long mode); */
    /// Unable to call function as its signature contains a non-rust type
    pub openNSPRFileDescShareDelete: *const ::libc::c_void,

}


impl nsILocalFileWin {
    /* void initWithCommandLine (in AString aCommandLine); */
    #[inline]
    pub unsafe fn initWithCommandLine(&self, aCommandLine: &[u16]) -> Result<(), nsresult> {
        let aCommandLine = nsString::from(aCommandLine);
        match ((*self.vtable).initWithCommandLine)(self as *const _, &*aCommandLine) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getVersionInfoField (in string aField); */
    #[inline]
    pub unsafe fn getVersionInfoField(&self, aField: *const libc::c_char) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getVersionInfoField)(self as *const _, aField, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString canonicalPath; */
    #[inline]
    pub unsafe fn get_canonicalPath(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_canonicalPath)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute ACString nativeCanonicalPath; */
    #[inline]
    pub unsafe fn get_nativeCanonicalPath(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_nativeCanonicalPath)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute unsigned long fileAttributesWin; */
    #[inline]
    pub unsafe fn get_fileAttributesWin(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fileAttributesWin)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fileAttributesWin(&self, aFileAttributesWin: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_fileAttributesWin)(self as *const _, aFileAttributesWin) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setShortcut ([optional] in nsIFile targetFile, [optional] in nsIFile workingDir, [optional] in wstring args, [optional] in wstring description, [optional] in nsIFile iconFile, [optional] in long iconIndex); */
    #[inline]
    pub unsafe fn setShortcut(&self, targetFile: Option<&nsIFile>, workingDir: Option<&nsIFile>, args: *const libc::int16_t, description: *const libc::int16_t, iconFile: Option<&nsIFile>, iconIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setShortcut)(self as *const _, targetFile.map_or(::std::ptr::null(), |x| x as *const _), workingDir.map_or(::std::ptr::null(), |x| x as *const _), args, description, iconFile.map_or(::std::ptr::null(), |x| x as *const _), iconIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] PRFileDescStar openNSPRFileDescShareDelete (in long flags, in long mode); */


}


