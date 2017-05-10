//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPluginTag.idl
//


pub mod nsIPluginTag_consts {
    pub const STATE_DISABLED: i64 = 0;
    pub const STATE_CLICKTOPLAY: i64 = 1;
    pub const STATE_ENABLED: i64 = 2;
}


#[repr(C)]
pub struct nsIPluginTag {
    vtable: *const nsIPluginTagVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPluginTag {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5daa99d5, 0x265a, 0x4397,
            [0xb4, 0x29, 0xc9, 0x43, 0x80, 0x3e, 0x26, 0x19])
    }
}

unsafe impl RefCounted for nsIPluginTag {
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
pub trait nsIPluginTagCoerce {
    fn coerce_from(v: &nsIPluginTag) -> &Self;
}

impl nsIPluginTagCoerce for nsIPluginTag {
    #[inline]
    fn coerce_from(v: &nsIPluginTag) -> &Self {
        v
    }
}

impl nsIPluginTag {
    #[inline]
    pub fn coerce<T: nsIPluginTagCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPluginTag {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPluginTagCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginTag) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPluginTagVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String description; */
    pub get_description: unsafe extern "C" fn (this: *const nsIPluginTag, aDescription: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String filename; */
    pub get_filename: unsafe extern "C" fn (this: *const nsIPluginTag, aFilename: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String fullpath; */
    pub get_fullpath: unsafe extern "C" fn (this: *const nsIPluginTag, aFullpath: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String version; */
    pub get_version: unsafe extern "C" fn (this: *const nsIPluginTag, aVersion: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIPluginTag, aName: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String niceName; */
    pub get_niceName: unsafe extern "C" fn (this: *const nsIPluginTag, aNiceName: *mut nsACString) -> nsresult,

    /* readonly attribute boolean blocklisted; */
    pub get_blocklisted: unsafe extern "C" fn (this: *const nsIPluginTag, aBlocklisted: *mut bool) -> nsresult,

    /* readonly attribute boolean isEnabledStateLocked; */
    pub get_isEnabledStateLocked: unsafe extern "C" fn (this: *const nsIPluginTag, aIsEnabledStateLocked: *mut bool) -> nsresult,

    /* readonly attribute boolean active; */
    pub get_active: unsafe extern "C" fn (this: *const nsIPluginTag, aActive: *mut bool) -> nsresult,

    /* readonly attribute unsigned long blocklistState; */
    pub get_blocklistState: unsafe extern "C" fn (this: *const nsIPluginTag, aBlocklistState: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIPluginTag, aDisabled: *mut bool) -> nsresult,

    /* readonly attribute boolean clicktoplay; */
    pub get_clicktoplay: unsafe extern "C" fn (this: *const nsIPluginTag, aClicktoplay: *mut bool) -> nsresult,

    /* readonly attribute boolean loaded; */
    pub get_loaded: unsafe extern "C" fn (this: *const nsIPluginTag, aLoaded: *mut bool) -> nsresult,

    /* attribute unsigned long enabledState; */
    pub get_enabledState: unsafe extern "C" fn (this: *const nsIPluginTag, aEnabledState: *mut libc::uint32_t) -> nsresult,
    pub set_enabledState: unsafe extern "C" fn (this: *const nsIPluginTag, aEnabledState: libc::uint32_t) -> nsresult,

    /* readonly attribute PRTime lastModifiedTime; */
    pub get_lastModifiedTime: unsafe extern "C" fn (this: *const nsIPluginTag, aLastModifiedTime: *mut PRTime) -> nsresult,

    /* void getMimeTypes ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aResults); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMimeTypes: *const ::libc::c_void,

    /* void getMimeDescriptions ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aResults); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMimeDescriptions: *const ::libc::c_void,

    /* void getExtensions ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aResults); */
    /// Unable to call function as its signature contains a non-rust type
    pub getExtensions: *const ::libc::c_void,

}


impl nsIPluginTag {
    /* readonly attribute AUTF8String description; */
    #[inline]
    pub unsafe fn get_description(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_description)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String filename; */
    #[inline]
    pub unsafe fn get_filename(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_filename)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String fullpath; */
    #[inline]
    pub unsafe fn get_fullpath(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_fullpath)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String version; */
    #[inline]
    pub unsafe fn get_version(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String niceName; */
    #[inline]
    pub unsafe fn get_niceName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_niceName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean blocklisted; */
    #[inline]
    pub unsafe fn get_blocklisted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_blocklisted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isEnabledStateLocked; */
    #[inline]
    pub unsafe fn get_isEnabledStateLocked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isEnabledStateLocked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean active; */
    #[inline]
    pub unsafe fn get_active(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_active)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long blocklistState; */
    #[inline]
    pub unsafe fn get_blocklistState(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_blocklistState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean disabled; */
    #[inline]
    pub unsafe fn get_disabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_disabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean clicktoplay; */
    #[inline]
    pub unsafe fn get_clicktoplay(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_clicktoplay)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean loaded; */
    #[inline]
    pub unsafe fn get_loaded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loaded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute unsigned long enabledState; */
    #[inline]
    pub unsafe fn get_enabledState(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_enabledState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_enabledState(&self, aEnabledState: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_enabledState)(self as *const _, aEnabledState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* void getMimeTypes ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aResults); */


    /* void getMimeDescriptions ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aResults); */


    /* void getExtensions ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aResults); */


}


#[repr(C)]
pub struct nsIFakePluginTag {
    vtable: *const nsIFakePluginTagVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFakePluginTag {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6d22c968, 0x226d, 0x4156,
            [0xb2, 0x30, 0xda, 0x6a, 0xd6, 0xbb, 0xf6, 0xe8])
    }
}

unsafe impl RefCounted for nsIFakePluginTag {
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
pub trait nsIFakePluginTagCoerce {
    fn coerce_from(v: &nsIFakePluginTag) -> &Self;
}

impl nsIFakePluginTagCoerce for nsIFakePluginTag {
    #[inline]
    fn coerce_from(v: &nsIFakePluginTag) -> &Self {
        v
    }
}

impl nsIFakePluginTag {
    #[inline]
    pub fn coerce<T: nsIFakePluginTagCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFakePluginTag {
    type Target = nsIPluginTag;
    #[inline]
    fn deref(&self) -> &nsIPluginTag {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPluginTagCoerce> nsIFakePluginTagCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFakePluginTag) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFakePluginTagVTable {
    pub __base: nsIPluginTagVTable,

    /* readonly attribute nsIURI handlerURI; */
    pub get_handlerURI: unsafe extern "C" fn (this: *const nsIFakePluginTag, aHandlerURI: *mut *const nsIURI) -> nsresult,

}


impl nsIFakePluginTag {
    /* readonly attribute nsIURI handlerURI; */
    #[inline]
    pub unsafe fn get_handlerURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_handlerURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


