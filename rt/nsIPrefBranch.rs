//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefBranch.idl
//


pub mod nsIPrefBranch_consts {
    pub const PREF_INVALID: i64 = 0;
    pub const PREF_STRING: i64 = 32;
    pub const PREF_INT: i64 = 64;
    pub const PREF_BOOL: i64 = 128;
}


#[repr(C)]
pub struct nsIPrefBranch {
    vtable: *const nsIPrefBranchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrefBranch {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x55d25e49, 0x793f, 0x4727,
            [0xa6, 0x9f, 0xde, 0x8b, 0x15, 0xf4, 0xb9, 0x85])
    }
}

unsafe impl RefCounted for nsIPrefBranch {
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
pub trait nsIPrefBranchCoerce {
    fn coerce_from(v: &nsIPrefBranch) -> &Self;
}

impl nsIPrefBranchCoerce for nsIPrefBranch {
    #[inline]
    fn coerce_from(v: &nsIPrefBranch) -> &Self {
        v
    }
}

impl nsIPrefBranch {
    #[inline]
    pub fn coerce<T: nsIPrefBranchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrefBranch {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrefBranchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefBranch) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrefBranchVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string root; */
    pub get_root: unsafe extern "C" fn (this: *const nsIPrefBranch, aRoot: *mut *const libc::c_char) -> nsresult,

    /* long getPrefType (in string aPrefName); */
    pub getPrefType: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut libc::int32_t) -> nsresult,

    /* [binaryname(GetBoolPrefWithDefault),optional_argc] boolean getBoolPref (in string aPrefName, [optional] in boolean aDefaultValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetBoolPrefWithDefault: *const ::libc::c_void,

    /* [binaryname(GetBoolPref),noscript] boolean getBoolPrefXPCOM (in string aPrefName); */
    pub GetBoolPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* void setBoolPref (in string aPrefName, in boolean aValue); */
    pub setBoolPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aValue: bool) -> nsresult,

    /* [binaryname(GetFloatPrefWithDefault),optional_argc] float getFloatPref (in string aPrefName, [optional] in float aDefaultValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetFloatPrefWithDefault: *const ::libc::c_void,

    /* [binaryname(GetFloatPref),noscript] float getFloatPrefXPCOM (in string aPrefName); */
    pub GetFloatPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut libc::c_float) -> nsresult,

    /* [binaryname(GetCharPrefWithDefault),optional_argc] string getCharPref (in string aPrefName, [optional] in string aDefaultValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetCharPrefWithDefault: *const ::libc::c_void,

    /* [binaryname(GetCharPref),noscript] string getCharPrefXPCOM (in string aPrefName); */
    pub GetCharPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* void setCharPref (in string aPrefName, in string aValue); */
    pub setCharPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aValue: *const libc::c_char) -> nsresult,

    /* [optional_argc] AUTF8String getStringPref (in string aPrefName, [optional] in AUTF8String aDefaultValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub getStringPref: *const ::libc::c_void,

    /* void setStringPref (in string aPrefName, in AUTF8String aValue); */
    pub setStringPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aValue: *const nsACString) -> nsresult,

    /* [binaryname(GetIntPrefWithDefault),optional_argc] long getIntPref (in string aPrefName, [optional] in long aDefaultValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetIntPrefWithDefault: *const ::libc::c_void,

    /* [binaryname(GetIntPref),noscript] long getIntPrefXPCOM (in string aPrefName); */
    pub GetIntPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut libc::int32_t) -> nsresult,

    /* void setIntPref (in string aPrefName, in long aValue); */
    pub setIntPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aValue: libc::int32_t) -> nsresult,

    /* void getComplexValue (in string aPrefName, in nsIIDRef aType, [iid_is (aType), retval] out nsQIResult aValue); */
    pub getComplexValue: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aType: *const nsIID, aValue: *mut *const libc::c_void) -> nsresult,

    /* void setComplexValue (in string aPrefName, in nsIIDRef aType, in nsISupports aValue); */
    pub setComplexValue: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, aType: *const nsIID, aValue: *const nsISupports) -> nsresult,

    /* void clearUserPref (in string aPrefName); */
    pub clearUserPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char) -> nsresult,

    /* void lockPref (in string aPrefName); */
    pub lockPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char) -> nsresult,

    /* boolean prefHasUserValue (in string aPrefName); */
    pub prefHasUserValue: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* boolean prefIsLocked (in string aPrefName); */
    pub prefIsLocked: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* void unlockPref (in string aPrefName); */
    pub unlockPref: unsafe extern "C" fn (this: *const nsIPrefBranch, aPrefName: *const libc::c_char) -> nsresult,

    /* void deleteBranch (in string aStartingAt); */
    pub deleteBranch: unsafe extern "C" fn (this: *const nsIPrefBranch, aStartingAt: *const libc::c_char) -> nsresult,

    /* void getChildList (in string aStartingAt, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aChildArray); */
    /// Unable to call function as its signature contains a non-rust type
    pub getChildList: *const ::libc::c_void,

    /* void resetBranch (in string aStartingAt); */
    pub resetBranch: unsafe extern "C" fn (this: *const nsIPrefBranch, aStartingAt: *const libc::c_char) -> nsresult,

    /* void addObserver (in string aDomain, in nsIObserver aObserver, [optional] in boolean aHoldWeak); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIPrefBranch, aDomain: *const libc::c_char, aObserver: *const nsIObserver, aHoldWeak: bool) -> nsresult,

    /* void removeObserver (in string aDomain, in nsIObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIPrefBranch, aDomain: *const libc::c_char, aObserver: *const nsIObserver) -> nsresult,

}


impl nsIPrefBranch {
    /* readonly attribute string root; */
    #[inline]
    pub unsafe fn get_root(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_root)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getPrefType (in string aPrefName); */
    #[inline]
    pub unsafe fn getPrefType(&self, aPrefName: *const libc::c_char) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPrefType)(self as *const _, aPrefName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(GetBoolPrefWithDefault),optional_argc] boolean getBoolPref (in string aPrefName, [optional] in boolean aDefaultValue); */


    /* [binaryname(GetBoolPref),noscript] boolean getBoolPrefXPCOM (in string aPrefName); */
    #[inline]
    pub unsafe fn GetBoolPref(&self, aPrefName: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).GetBoolPref)(self as *const _, aPrefName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setBoolPref (in string aPrefName, in boolean aValue); */
    #[inline]
    pub unsafe fn setBoolPref(&self, aPrefName: *const libc::c_char, aValue: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setBoolPref)(self as *const _, aPrefName, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(GetFloatPrefWithDefault),optional_argc] float getFloatPref (in string aPrefName, [optional] in float aDefaultValue); */


    /* [binaryname(GetFloatPref),noscript] float getFloatPrefXPCOM (in string aPrefName); */
    #[inline]
    pub unsafe fn GetFloatPref(&self, aPrefName: *const libc::c_char) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).GetFloatPref)(self as *const _, aPrefName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(GetCharPrefWithDefault),optional_argc] string getCharPref (in string aPrefName, [optional] in string aDefaultValue); */


    /* [binaryname(GetCharPref),noscript] string getCharPrefXPCOM (in string aPrefName); */
    #[inline]
    pub unsafe fn GetCharPref(&self, aPrefName: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).GetCharPref)(self as *const _, aPrefName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setCharPref (in string aPrefName, in string aValue); */
    #[inline]
    pub unsafe fn setCharPref(&self, aPrefName: *const libc::c_char, aValue: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setCharPref)(self as *const _, aPrefName, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [optional_argc] AUTF8String getStringPref (in string aPrefName, [optional] in AUTF8String aDefaultValue); */


    /* void setStringPref (in string aPrefName, in AUTF8String aValue); */
    #[inline]
    pub unsafe fn setStringPref(&self, aPrefName: *const libc::c_char, aValue: &[u8]) -> Result<(), nsresult> {
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).setStringPref)(self as *const _, aPrefName, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(GetIntPrefWithDefault),optional_argc] long getIntPref (in string aPrefName, [optional] in long aDefaultValue); */


    /* [binaryname(GetIntPref),noscript] long getIntPrefXPCOM (in string aPrefName); */
    #[inline]
    pub unsafe fn GetIntPref(&self, aPrefName: *const libc::c_char) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetIntPref)(self as *const _, aPrefName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setIntPref (in string aPrefName, in long aValue); */
    #[inline]
    pub unsafe fn setIntPref(&self, aPrefName: *const libc::c_char, aValue: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setIntPref)(self as *const _, aPrefName, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getComplexValue (in string aPrefName, in nsIIDRef aType, [iid_is (aType), retval] out nsQIResult aValue); */
    #[inline]
    pub unsafe fn getComplexValue<T: XpCom>(&self, aPrefName: *const libc::c_char) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut aValue : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getComplexValue)(self as *const _, aPrefName, &T::iid(), aValue.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aValue.refptr())
    }

    /* void setComplexValue (in string aPrefName, in nsIIDRef aType, in nsISupports aValue); */
    #[inline]
    pub unsafe fn setComplexValue(&self, aPrefName: *const libc::c_char, aType: *const nsIID, aValue: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).setComplexValue)(self as *const _, aPrefName, aType, aValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearUserPref (in string aPrefName); */
    #[inline]
    pub unsafe fn clearUserPref(&self, aPrefName: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).clearUserPref)(self as *const _, aPrefName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void lockPref (in string aPrefName); */
    #[inline]
    pub unsafe fn lockPref(&self, aPrefName: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).lockPref)(self as *const _, aPrefName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean prefHasUserValue (in string aPrefName); */
    #[inline]
    pub unsafe fn prefHasUserValue(&self, aPrefName: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).prefHasUserValue)(self as *const _, aPrefName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean prefIsLocked (in string aPrefName); */
    #[inline]
    pub unsafe fn prefIsLocked(&self, aPrefName: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).prefIsLocked)(self as *const _, aPrefName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void unlockPref (in string aPrefName); */
    #[inline]
    pub unsafe fn unlockPref(&self, aPrefName: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).unlockPref)(self as *const _, aPrefName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteBranch (in string aStartingAt); */
    #[inline]
    pub unsafe fn deleteBranch(&self, aStartingAt: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).deleteBranch)(self as *const _, aStartingAt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getChildList (in string aStartingAt, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aChildArray); */


    /* void resetBranch (in string aStartingAt); */
    #[inline]
    pub unsafe fn resetBranch(&self, aStartingAt: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).resetBranch)(self as *const _, aStartingAt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addObserver (in string aDomain, in nsIObserver aObserver, [optional] in boolean aHoldWeak); */
    #[inline]
    pub unsafe fn addObserver(&self, aDomain: *const libc::c_char, aObserver: Option<&nsIObserver>, aHoldWeak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, aDomain, aObserver.map_or(::std::ptr::null(), |x| x as *const _), aHoldWeak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in string aDomain, in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aDomain: *const libc::c_char, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, aDomain, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


