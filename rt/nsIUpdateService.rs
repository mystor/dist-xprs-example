//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUpdateService.idl
//


#[repr(C)]
pub struct nsIUpdatePatch {
    vtable: *const nsIUpdatePatchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUpdatePatch {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdc8fb8a9, 0x3a53, 0x4031,
            [0x94, 0x69, 0x2a, 0x51, 0x97, 0xea, 0x30, 0xe7])
    }
}

unsafe impl RefCounted for nsIUpdatePatch {
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
pub trait nsIUpdatePatchCoerce {
    fn coerce_from(v: &nsIUpdatePatch) -> &Self;
}

impl nsIUpdatePatchCoerce for nsIUpdatePatch {
    #[inline]
    fn coerce_from(v: &nsIUpdatePatch) -> &Self {
        v
    }
}

impl nsIUpdatePatch {
    #[inline]
    pub fn coerce<T: nsIUpdatePatchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUpdatePatch {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUpdatePatchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdatePatch) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUpdatePatchVTable {
    pub __base: nsISupportsVTable,

    /* attribute AString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIUpdatePatch, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIUpdatePatch, aType: *const nsAString) -> nsresult,

    /* attribute AString URL; */
    pub get_URL: unsafe extern "C" fn (this: *const nsIUpdatePatch, aURL: *mut nsAString) -> nsresult,
    pub set_URL: unsafe extern "C" fn (this: *const nsIUpdatePatch, aURL: *const nsAString) -> nsresult,

    /* attribute AString finalURL; */
    pub get_finalURL: unsafe extern "C" fn (this: *const nsIUpdatePatch, aFinalURL: *mut nsAString) -> nsresult,
    pub set_finalURL: unsafe extern "C" fn (this: *const nsIUpdatePatch, aFinalURL: *const nsAString) -> nsresult,

    /* attribute AString hashFunction; */
    pub get_hashFunction: unsafe extern "C" fn (this: *const nsIUpdatePatch, aHashFunction: *mut nsAString) -> nsresult,
    pub set_hashFunction: unsafe extern "C" fn (this: *const nsIUpdatePatch, aHashFunction: *const nsAString) -> nsresult,

    /* attribute AString hashValue; */
    pub get_hashValue: unsafe extern "C" fn (this: *const nsIUpdatePatch, aHashValue: *mut nsAString) -> nsresult,
    pub set_hashValue: unsafe extern "C" fn (this: *const nsIUpdatePatch, aHashValue: *const nsAString) -> nsresult,

    /* attribute unsigned long size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIUpdatePatch, aSize: *mut libc::uint32_t) -> nsresult,
    pub set_size: unsafe extern "C" fn (this: *const nsIUpdatePatch, aSize: libc::uint32_t) -> nsresult,

    /* attribute AString state; */
    pub get_state: unsafe extern "C" fn (this: *const nsIUpdatePatch, aState: *mut nsAString) -> nsresult,
    pub set_state: unsafe extern "C" fn (this: *const nsIUpdatePatch, aState: *const nsAString) -> nsresult,

    /* attribute boolean selected; */
    pub get_selected: unsafe extern "C" fn (this: *const nsIUpdatePatch, aSelected: *mut bool) -> nsresult,
    pub set_selected: unsafe extern "C" fn (this: *const nsIUpdatePatch, aSelected: bool) -> nsresult,

    /* nsIDOMElement serialize (in nsIDOMDocument updates); */
    pub serialize: unsafe extern "C" fn (this: *const nsIUpdatePatch, updates: *const nsIDOMDocument, _retval: *mut *const nsIDOMElement) -> nsresult,

}


impl nsIUpdatePatch {
    /* attribute AString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: &[u16]) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).set_type_)(self as *const _, &*aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString URL; */
    #[inline]
    pub unsafe fn get_URL(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_URL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_URL(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).set_URL)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString finalURL; */
    #[inline]
    pub unsafe fn get_finalURL(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_finalURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_finalURL(&self, aFinalURL: &[u16]) -> Result<(), nsresult> {
        let aFinalURL = nsString::from(aFinalURL);
        match ((*self.vtable).set_finalURL)(self as *const _, &*aFinalURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString hashFunction; */
    #[inline]
    pub unsafe fn get_hashFunction(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_hashFunction)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hashFunction(&self, aHashFunction: &[u16]) -> Result<(), nsresult> {
        let aHashFunction = nsString::from(aHashFunction);
        match ((*self.vtable).set_hashFunction)(self as *const _, &*aHashFunction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString hashValue; */
    #[inline]
    pub unsafe fn get_hashValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_hashValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hashValue(&self, aHashValue: &[u16]) -> Result<(), nsresult> {
        let aHashValue = nsString::from(aHashValue);
        match ((*self.vtable).set_hashValue)(self as *const _, &*aHashValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_size)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_size(&self, aSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_size)(self as *const _, aSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_state)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_state(&self, aState: &[u16]) -> Result<(), nsresult> {
        let aState = nsString::from(aState);
        match ((*self.vtable).set_state)(self as *const _, &*aState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean selected; */
    #[inline]
    pub unsafe fn get_selected(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_selected)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selected(&self, aSelected: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_selected)(self as *const _, aSelected) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement serialize (in nsIDOMDocument updates); */
    #[inline]
    pub unsafe fn serialize(&self, updates: Option<&nsIDOMDocument>) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).serialize)(self as *const _, updates.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIUpdate {
    vtable: *const nsIUpdateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUpdate {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe094c045, 0xf4ff, 0x41fd,
            [0x92, 0xda, 0xcd, 0x2e, 0xff, 0xd2, 0xc7, 0xc9])
    }
}

unsafe impl RefCounted for nsIUpdate {
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
pub trait nsIUpdateCoerce {
    fn coerce_from(v: &nsIUpdate) -> &Self;
}

impl nsIUpdateCoerce for nsIUpdate {
    #[inline]
    fn coerce_from(v: &nsIUpdate) -> &Self {
        v
    }
}

impl nsIUpdate {
    #[inline]
    pub fn coerce<T: nsIUpdateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUpdate {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUpdateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdate) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUpdateVTable {
    pub __base: nsISupportsVTable,

    /* attribute AString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIUpdate, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIUpdate, aType: *const nsAString) -> nsresult,

    /* attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIUpdate, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIUpdate, aName: *const nsAString) -> nsresult,

    /* attribute AString displayVersion; */
    pub get_displayVersion: unsafe extern "C" fn (this: *const nsIUpdate, aDisplayVersion: *mut nsAString) -> nsresult,
    pub set_displayVersion: unsafe extern "C" fn (this: *const nsIUpdate, aDisplayVersion: *const nsAString) -> nsresult,

    /* attribute AString appVersion; */
    pub get_appVersion: unsafe extern "C" fn (this: *const nsIUpdate, aAppVersion: *mut nsAString) -> nsresult,
    pub set_appVersion: unsafe extern "C" fn (this: *const nsIUpdate, aAppVersion: *const nsAString) -> nsresult,

    /* attribute AString previousAppVersion; */
    pub get_previousAppVersion: unsafe extern "C" fn (this: *const nsIUpdate, aPreviousAppVersion: *mut nsAString) -> nsresult,
    pub set_previousAppVersion: unsafe extern "C" fn (this: *const nsIUpdate, aPreviousAppVersion: *const nsAString) -> nsresult,

    /* attribute AString buildID; */
    pub get_buildID: unsafe extern "C" fn (this: *const nsIUpdate, aBuildID: *mut nsAString) -> nsresult,
    pub set_buildID: unsafe extern "C" fn (this: *const nsIUpdate, aBuildID: *const nsAString) -> nsresult,

    /* attribute AString detailsURL; */
    pub get_detailsURL: unsafe extern "C" fn (this: *const nsIUpdate, aDetailsURL: *mut nsAString) -> nsresult,
    pub set_detailsURL: unsafe extern "C" fn (this: *const nsIUpdate, aDetailsURL: *const nsAString) -> nsresult,

    /* attribute AString serviceURL; */
    pub get_serviceURL: unsafe extern "C" fn (this: *const nsIUpdate, aServiceURL: *mut nsAString) -> nsresult,
    pub set_serviceURL: unsafe extern "C" fn (this: *const nsIUpdate, aServiceURL: *const nsAString) -> nsresult,

    /* attribute AString channel; */
    pub get_channel: unsafe extern "C" fn (this: *const nsIUpdate, aChannel: *mut nsAString) -> nsresult,
    pub set_channel: unsafe extern "C" fn (this: *const nsIUpdate, aChannel: *const nsAString) -> nsresult,

    /* attribute boolean showPrompt; */
    pub get_showPrompt: unsafe extern "C" fn (this: *const nsIUpdate, aShowPrompt: *mut bool) -> nsresult,
    pub set_showPrompt: unsafe extern "C" fn (this: *const nsIUpdate, aShowPrompt: bool) -> nsresult,

    /* attribute boolean showNeverForVersion; */
    pub get_showNeverForVersion: unsafe extern "C" fn (this: *const nsIUpdate, aShowNeverForVersion: *mut bool) -> nsresult,
    pub set_showNeverForVersion: unsafe extern "C" fn (this: *const nsIUpdate, aShowNeverForVersion: bool) -> nsresult,

    /* attribute boolean unsupported; */
    pub get_unsupported: unsafe extern "C" fn (this: *const nsIUpdate, aUnsupported: *mut bool) -> nsresult,
    pub set_unsupported: unsafe extern "C" fn (this: *const nsIUpdate, aUnsupported: bool) -> nsresult,

    /* attribute long long promptWaitTime; */
    pub get_promptWaitTime: unsafe extern "C" fn (this: *const nsIUpdate, aPromptWaitTime: *mut libc::int64_t) -> nsresult,
    pub set_promptWaitTime: unsafe extern "C" fn (this: *const nsIUpdate, aPromptWaitTime: libc::int64_t) -> nsresult,

    /* attribute boolean isCompleteUpdate; */
    pub get_isCompleteUpdate: unsafe extern "C" fn (this: *const nsIUpdate, aIsCompleteUpdate: *mut bool) -> nsresult,
    pub set_isCompleteUpdate: unsafe extern "C" fn (this: *const nsIUpdate, aIsCompleteUpdate: bool) -> nsresult,

    /* attribute boolean isSecurityUpdate; */
    pub get_isSecurityUpdate: unsafe extern "C" fn (this: *const nsIUpdate, aIsSecurityUpdate: *mut bool) -> nsresult,
    pub set_isSecurityUpdate: unsafe extern "C" fn (this: *const nsIUpdate, aIsSecurityUpdate: bool) -> nsresult,

    /* attribute long long installDate; */
    pub get_installDate: unsafe extern "C" fn (this: *const nsIUpdate, aInstallDate: *mut libc::int64_t) -> nsresult,
    pub set_installDate: unsafe extern "C" fn (this: *const nsIUpdate, aInstallDate: libc::int64_t) -> nsresult,

    /* attribute AString statusText; */
    pub get_statusText: unsafe extern "C" fn (this: *const nsIUpdate, aStatusText: *mut nsAString) -> nsresult,
    pub set_statusText: unsafe extern "C" fn (this: *const nsIUpdate, aStatusText: *const nsAString) -> nsresult,

    /* readonly attribute nsIUpdatePatch selectedPatch; */
    pub get_selectedPatch: unsafe extern "C" fn (this: *const nsIUpdate, aSelectedPatch: *mut *const nsIUpdatePatch) -> nsresult,

    /* attribute AString state; */
    pub get_state: unsafe extern "C" fn (this: *const nsIUpdate, aState: *mut nsAString) -> nsresult,
    pub set_state: unsafe extern "C" fn (this: *const nsIUpdate, aState: *const nsAString) -> nsresult,

    /* attribute long errorCode; */
    pub get_errorCode: unsafe extern "C" fn (this: *const nsIUpdate, aErrorCode: *mut libc::int32_t) -> nsresult,
    pub set_errorCode: unsafe extern "C" fn (this: *const nsIUpdate, aErrorCode: libc::int32_t) -> nsresult,

    /* attribute boolean elevationFailure; */
    pub get_elevationFailure: unsafe extern "C" fn (this: *const nsIUpdate, aElevationFailure: *mut bool) -> nsresult,
    pub set_elevationFailure: unsafe extern "C" fn (this: *const nsIUpdate, aElevationFailure: bool) -> nsresult,

    /* readonly attribute unsigned long patchCount; */
    pub get_patchCount: unsafe extern "C" fn (this: *const nsIUpdate, aPatchCount: *mut libc::uint32_t) -> nsresult,

    /* nsIUpdatePatch getPatchAt (in unsigned long index); */
    pub getPatchAt: unsafe extern "C" fn (this: *const nsIUpdate, index: libc::uint32_t, _retval: *mut *const nsIUpdatePatch) -> nsresult,

    /* nsIDOMElement serialize (in nsIDOMDocument updates); */
    pub serialize: unsafe extern "C" fn (this: *const nsIUpdate, updates: *const nsIDOMDocument, _retval: *mut *const nsIDOMElement) -> nsresult,

}


impl nsIUpdate {
    /* attribute AString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: &[u16]) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).set_type_)(self as *const _, &*aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString displayVersion; */
    #[inline]
    pub unsafe fn get_displayVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_displayVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_displayVersion(&self, aDisplayVersion: &[u16]) -> Result<(), nsresult> {
        let aDisplayVersion = nsString::from(aDisplayVersion);
        match ((*self.vtable).set_displayVersion)(self as *const _, &*aDisplayVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString appVersion; */
    #[inline]
    pub unsafe fn get_appVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_appVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_appVersion(&self, aAppVersion: &[u16]) -> Result<(), nsresult> {
        let aAppVersion = nsString::from(aAppVersion);
        match ((*self.vtable).set_appVersion)(self as *const _, &*aAppVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString previousAppVersion; */
    #[inline]
    pub unsafe fn get_previousAppVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_previousAppVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_previousAppVersion(&self, aPreviousAppVersion: &[u16]) -> Result<(), nsresult> {
        let aPreviousAppVersion = nsString::from(aPreviousAppVersion);
        match ((*self.vtable).set_previousAppVersion)(self as *const _, &*aPreviousAppVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString buildID; */
    #[inline]
    pub unsafe fn get_buildID(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_buildID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_buildID(&self, aBuildID: &[u16]) -> Result<(), nsresult> {
        let aBuildID = nsString::from(aBuildID);
        match ((*self.vtable).set_buildID)(self as *const _, &*aBuildID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString detailsURL; */
    #[inline]
    pub unsafe fn get_detailsURL(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_detailsURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_detailsURL(&self, aDetailsURL: &[u16]) -> Result<(), nsresult> {
        let aDetailsURL = nsString::from(aDetailsURL);
        match ((*self.vtable).set_detailsURL)(self as *const _, &*aDetailsURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString serviceURL; */
    #[inline]
    pub unsafe fn get_serviceURL(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_serviceURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_serviceURL(&self, aServiceURL: &[u16]) -> Result<(), nsresult> {
        let aServiceURL = nsString::from(aServiceURL);
        match ((*self.vtable).set_serviceURL)(self as *const _, &*aServiceURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString channel; */
    #[inline]
    pub unsafe fn get_channel(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_channel)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_channel(&self, aChannel: &[u16]) -> Result<(), nsresult> {
        let aChannel = nsString::from(aChannel);
        match ((*self.vtable).set_channel)(self as *const _, &*aChannel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean showPrompt; */
    #[inline]
    pub unsafe fn get_showPrompt(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showPrompt)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showPrompt(&self, aShowPrompt: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showPrompt)(self as *const _, aShowPrompt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean showNeverForVersion; */
    #[inline]
    pub unsafe fn get_showNeverForVersion(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showNeverForVersion)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showNeverForVersion(&self, aShowNeverForVersion: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showNeverForVersion)(self as *const _, aShowNeverForVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean unsupported; */
    #[inline]
    pub unsafe fn get_unsupported(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_unsupported)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_unsupported(&self, aUnsupported: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_unsupported)(self as *const _, aUnsupported) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long long promptWaitTime; */
    #[inline]
    pub unsafe fn get_promptWaitTime(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_promptWaitTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_promptWaitTime(&self, aPromptWaitTime: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_promptWaitTime)(self as *const _, aPromptWaitTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isCompleteUpdate; */
    #[inline]
    pub unsafe fn get_isCompleteUpdate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isCompleteUpdate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isCompleteUpdate(&self, aIsCompleteUpdate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isCompleteUpdate)(self as *const _, aIsCompleteUpdate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isSecurityUpdate; */
    #[inline]
    pub unsafe fn get_isSecurityUpdate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSecurityUpdate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isSecurityUpdate(&self, aIsSecurityUpdate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isSecurityUpdate)(self as *const _, aIsSecurityUpdate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long long installDate; */
    #[inline]
    pub unsafe fn get_installDate(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_installDate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_installDate(&self, aInstallDate: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_installDate)(self as *const _, aInstallDate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString statusText; */
    #[inline]
    pub unsafe fn get_statusText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_statusText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_statusText(&self, aStatusText: &[u16]) -> Result<(), nsresult> {
        let aStatusText = nsString::from(aStatusText);
        match ((*self.vtable).set_statusText)(self as *const _, &*aStatusText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIUpdatePatch selectedPatch; */
    #[inline]
    pub unsafe fn get_selectedPatch(&self, ) -> Result<Option<RefPtr<nsIUpdatePatch>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectedPatch)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute AString state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_state)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_state(&self, aState: &[u16]) -> Result<(), nsresult> {
        let aState = nsString::from(aState);
        match ((*self.vtable).set_state)(self as *const _, &*aState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long errorCode; */
    #[inline]
    pub unsafe fn get_errorCode(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_errorCode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_errorCode(&self, aErrorCode: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_errorCode)(self as *const _, aErrorCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean elevationFailure; */
    #[inline]
    pub unsafe fn get_elevationFailure(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_elevationFailure)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_elevationFailure(&self, aElevationFailure: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_elevationFailure)(self as *const _, aElevationFailure) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long patchCount; */
    #[inline]
    pub unsafe fn get_patchCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_patchCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIUpdatePatch getPatchAt (in unsigned long index); */
    #[inline]
    pub unsafe fn getPatchAt(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIUpdatePatch>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPatchAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMElement serialize (in nsIDOMDocument updates); */
    #[inline]
    pub unsafe fn serialize(&self, updates: Option<&nsIDOMDocument>) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).serialize)(self as *const _, updates.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIUpdateCheckListener {
    vtable: *const nsIUpdateCheckListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUpdateCheckListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4aa2b4bb, 0x39ea, 0x407b,
            [0x98, 0xff, 0x89, 0xf1, 0x91, 0x34, 0xd4, 0xc0])
    }
}

unsafe impl RefCounted for nsIUpdateCheckListener {
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
pub trait nsIUpdateCheckListenerCoerce {
    fn coerce_from(v: &nsIUpdateCheckListener) -> &Self;
}

impl nsIUpdateCheckListenerCoerce for nsIUpdateCheckListener {
    #[inline]
    fn coerce_from(v: &nsIUpdateCheckListener) -> &Self {
        v
    }
}

impl nsIUpdateCheckListener {
    #[inline]
    pub fn coerce<T: nsIUpdateCheckListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUpdateCheckListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUpdateCheckListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateCheckListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUpdateCheckListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onCheckComplete (in jsval request, [array, size_is (updateCount)] in nsIUpdate updates, in unsigned long updateCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub onCheckComplete: *const ::libc::c_void,

    /* void onError (in jsval request, in nsIUpdate update); */
    /// Unable to call function as its signature contains a non-rust type
    pub onError: *const ::libc::c_void,

}


impl nsIUpdateCheckListener {
    /* void onCheckComplete (in jsval request, [array, size_is (updateCount)] in nsIUpdate updates, in unsigned long updateCount); */


    /* void onError (in jsval request, in nsIUpdate update); */


}


pub mod nsIUpdateChecker_consts {
    pub const CURRENT_CHECK: i64 = 1;
    pub const CURRENT_SESSION: i64 = 2;
    pub const ANY_CHECKS: i64 = 3;
}


#[repr(C)]
pub struct nsIUpdateChecker {
    vtable: *const nsIUpdateCheckerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUpdateChecker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x877ace25, 0x8bc5, 0x452a,
            [0x85, 0x86, 0x9c, 0x1c, 0xf2, 0x87, 0x19, 0x94])
    }
}

unsafe impl RefCounted for nsIUpdateChecker {
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
pub trait nsIUpdateCheckerCoerce {
    fn coerce_from(v: &nsIUpdateChecker) -> &Self;
}

impl nsIUpdateCheckerCoerce for nsIUpdateChecker {
    #[inline]
    fn coerce_from(v: &nsIUpdateChecker) -> &Self {
        v
    }
}

impl nsIUpdateChecker {
    #[inline]
    pub fn coerce<T: nsIUpdateCheckerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUpdateChecker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUpdateCheckerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateChecker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUpdateCheckerVTable {
    pub __base: nsISupportsVTable,

    /* void checkForUpdates (in nsIUpdateCheckListener listener, in boolean force); */
    pub checkForUpdates: unsafe extern "C" fn (this: *const nsIUpdateChecker, listener: *const nsIUpdateCheckListener, force: bool) -> nsresult,

    /* void stopChecking (in unsigned short duration); */
    pub stopChecking: unsafe extern "C" fn (this: *const nsIUpdateChecker, duration: libc::uint16_t) -> nsresult,

}


impl nsIUpdateChecker {
    /* void checkForUpdates (in nsIUpdateCheckListener listener, in boolean force); */
    #[inline]
    pub unsafe fn checkForUpdates(&self, listener: Option<&nsIUpdateCheckListener>, force: bool) -> Result<(), nsresult> {

        match ((*self.vtable).checkForUpdates)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _), force) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopChecking (in unsigned short duration); */
    #[inline]
    pub unsafe fn stopChecking(&self, duration: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).stopChecking)(self as *const _, duration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIApplicationUpdateService {
    vtable: *const nsIApplicationUpdateServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationUpdateService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1107d207, 0xa263, 0x403a,
            [0xb2, 0x68, 0x05, 0x77, 0x2e, 0xc1, 0x07, 0x57])
    }
}

unsafe impl RefCounted for nsIApplicationUpdateService {
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
pub trait nsIApplicationUpdateServiceCoerce {
    fn coerce_from(v: &nsIApplicationUpdateService) -> &Self;
}

impl nsIApplicationUpdateServiceCoerce for nsIApplicationUpdateService {
    #[inline]
    fn coerce_from(v: &nsIApplicationUpdateService) -> &Self {
        v
    }
}

impl nsIApplicationUpdateService {
    #[inline]
    pub fn coerce<T: nsIApplicationUpdateServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationUpdateService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIApplicationUpdateServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationUpdateService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationUpdateServiceVTable {
    pub __base: nsISupportsVTable,

    /* void checkForBackgroundUpdates (); */
    pub checkForBackgroundUpdates: unsafe extern "C" fn (this: *const nsIApplicationUpdateService) -> nsresult,

    /* readonly attribute nsIUpdateChecker backgroundChecker; */
    pub get_backgroundChecker: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, aBackgroundChecker: *mut *const nsIUpdateChecker) -> nsresult,

    /* nsIUpdate selectUpdate ([array, size_is (updateCount)] in nsIUpdate updates, in unsigned long updateCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub selectUpdate: *const ::libc::c_void,

    /* void addDownloadListener (in nsIRequestObserver listener); */
    pub addDownloadListener: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, listener: *const nsIRequestObserver) -> nsresult,

    /* void removeDownloadListener (in nsIRequestObserver listener); */
    pub removeDownloadListener: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, listener: *const nsIRequestObserver) -> nsresult,

    /* AString downloadUpdate (in nsIUpdate update, in boolean background); */
    pub downloadUpdate: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, update: *const nsIUpdate, background: bool, _retval: *mut nsAString) -> nsresult,

    /* nsIFile getUpdatesDirectory (); */
    pub getUpdatesDirectory: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, _retval: *mut *const nsIFile) -> nsresult,

    /* void pauseDownload (); */
    pub pauseDownload: unsafe extern "C" fn (this: *const nsIApplicationUpdateService) -> nsresult,

    /* readonly attribute boolean isDownloading; */
    pub get_isDownloading: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, aIsDownloading: *mut bool) -> nsresult,

    /* readonly attribute boolean canCheckForUpdates; */
    pub get_canCheckForUpdates: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, aCanCheckForUpdates: *mut bool) -> nsresult,

    /* readonly attribute boolean elevationRequired; */
    pub get_elevationRequired: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, aElevationRequired: *mut bool) -> nsresult,

    /* readonly attribute boolean canApplyUpdates; */
    pub get_canApplyUpdates: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, aCanApplyUpdates: *mut bool) -> nsresult,

    /* readonly attribute boolean isOtherInstanceHandlingUpdates; */
    pub get_isOtherInstanceHandlingUpdates: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, aIsOtherInstanceHandlingUpdates: *mut bool) -> nsresult,

    /* readonly attribute boolean canStageUpdates; */
    pub get_canStageUpdates: unsafe extern "C" fn (this: *const nsIApplicationUpdateService, aCanStageUpdates: *mut bool) -> nsresult,

}


impl nsIApplicationUpdateService {
    /* void checkForBackgroundUpdates (); */
    #[inline]
    pub unsafe fn checkForBackgroundUpdates(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).checkForBackgroundUpdates)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIUpdateChecker backgroundChecker; */
    #[inline]
    pub unsafe fn get_backgroundChecker(&self, ) -> Result<Option<RefPtr<nsIUpdateChecker>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_backgroundChecker)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIUpdate selectUpdate ([array, size_is (updateCount)] in nsIUpdate updates, in unsigned long updateCount); */


    /* void addDownloadListener (in nsIRequestObserver listener); */
    #[inline]
    pub unsafe fn addDownloadListener(&self, listener: Option<&nsIRequestObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addDownloadListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDownloadListener (in nsIRequestObserver listener); */
    #[inline]
    pub unsafe fn removeDownloadListener(&self, listener: Option<&nsIRequestObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeDownloadListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString downloadUpdate (in nsIUpdate update, in boolean background); */
    #[inline]
    pub unsafe fn downloadUpdate(&self, update: Option<&nsIUpdate>, background: bool) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).downloadUpdate)(self as *const _, update.map_or(::std::ptr::null(), |x| x as *const _), background, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIFile getUpdatesDirectory (); */
    #[inline]
    pub unsafe fn getUpdatesDirectory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getUpdatesDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void pauseDownload (); */
    #[inline]
    pub unsafe fn pauseDownload(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).pauseDownload)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isDownloading; */
    #[inline]
    pub unsafe fn get_isDownloading(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDownloading)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean canCheckForUpdates; */
    #[inline]
    pub unsafe fn get_canCheckForUpdates(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canCheckForUpdates)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean elevationRequired; */
    #[inline]
    pub unsafe fn get_elevationRequired(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_elevationRequired)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean canApplyUpdates; */
    #[inline]
    pub unsafe fn get_canApplyUpdates(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canApplyUpdates)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isOtherInstanceHandlingUpdates; */
    #[inline]
    pub unsafe fn get_isOtherInstanceHandlingUpdates(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isOtherInstanceHandlingUpdates)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean canStageUpdates; */
    #[inline]
    pub unsafe fn get_canStageUpdates(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canStageUpdates)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIUpdateProcessor {
    vtable: *const nsIUpdateProcessorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUpdateProcessor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x74439497, 0xd796, 0x4915,
            [0x8c, 0xef, 0x3d, 0xfe, 0x43, 0x02, 0x7e, 0x4d])
    }
}

unsafe impl RefCounted for nsIUpdateProcessor {
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
pub trait nsIUpdateProcessorCoerce {
    fn coerce_from(v: &nsIUpdateProcessor) -> &Self;
}

impl nsIUpdateProcessorCoerce for nsIUpdateProcessor {
    #[inline]
    fn coerce_from(v: &nsIUpdateProcessor) -> &Self {
        v
    }
}

impl nsIUpdateProcessor {
    #[inline]
    pub fn coerce<T: nsIUpdateProcessorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUpdateProcessor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUpdateProcessorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateProcessor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUpdateProcessorVTable {
    pub __base: nsISupportsVTable,

    /* void processUpdate (in nsIUpdate update); */
    pub processUpdate: unsafe extern "C" fn (this: *const nsIUpdateProcessor, update: *const nsIUpdate) -> nsresult,

}


impl nsIUpdateProcessor {
    /* void processUpdate (in nsIUpdate update); */
    #[inline]
    pub unsafe fn processUpdate(&self, update: Option<&nsIUpdate>) -> Result<(), nsresult> {

        match ((*self.vtable).processUpdate)(self as *const _, update.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUpdateManager {
    vtable: *const nsIUpdateManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUpdateManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0f1098e9, 0xa447, 0x4af9,
            [0xb0, 0x30, 0x6f, 0x8f, 0x35, 0xc8, 0x5f, 0x89])
    }
}

unsafe impl RefCounted for nsIUpdateManager {
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
pub trait nsIUpdateManagerCoerce {
    fn coerce_from(v: &nsIUpdateManager) -> &Self;
}

impl nsIUpdateManagerCoerce for nsIUpdateManager {
    #[inline]
    fn coerce_from(v: &nsIUpdateManager) -> &Self {
        v
    }
}

impl nsIUpdateManager {
    #[inline]
    pub fn coerce<T: nsIUpdateManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUpdateManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUpdateManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUpdateManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsIUpdate getUpdateAt (in long index); */
    pub getUpdateAt: unsafe extern "C" fn (this: *const nsIUpdateManager, index: libc::int32_t, _retval: *mut *const nsIUpdate) -> nsresult,

    /* readonly attribute long updateCount; */
    pub get_updateCount: unsafe extern "C" fn (this: *const nsIUpdateManager, aUpdateCount: *mut libc::int32_t) -> nsresult,

    /* attribute nsIUpdate activeUpdate; */
    pub get_activeUpdate: unsafe extern "C" fn (this: *const nsIUpdateManager, aActiveUpdate: *mut *const nsIUpdate) -> nsresult,
    pub set_activeUpdate: unsafe extern "C" fn (this: *const nsIUpdateManager, aActiveUpdate: *const nsIUpdate) -> nsresult,

    /* void saveUpdates (); */
    pub saveUpdates: unsafe extern "C" fn (this: *const nsIUpdateManager) -> nsresult,

    /* void refreshUpdateStatus (); */
    pub refreshUpdateStatus: unsafe extern "C" fn (this: *const nsIUpdateManager) -> nsresult,

    /* void elevationOptedIn (); */
    pub elevationOptedIn: unsafe extern "C" fn (this: *const nsIUpdateManager) -> nsresult,

    /* void cleanupActiveUpdate (); */
    pub cleanupActiveUpdate: unsafe extern "C" fn (this: *const nsIUpdateManager) -> nsresult,

}


impl nsIUpdateManager {
    /* nsIUpdate getUpdateAt (in long index); */
    #[inline]
    pub unsafe fn getUpdateAt(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIUpdate>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getUpdateAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long updateCount; */
    #[inline]
    pub unsafe fn get_updateCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_updateCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIUpdate activeUpdate; */
    #[inline]
    pub unsafe fn get_activeUpdate(&self, ) -> Result<Option<RefPtr<nsIUpdate>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activeUpdate)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_activeUpdate(&self, aActiveUpdate: Option<&nsIUpdate>) -> Result<(), nsresult> {

        match ((*self.vtable).set_activeUpdate)(self as *const _, aActiveUpdate.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void saveUpdates (); */
    #[inline]
    pub unsafe fn saveUpdates(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).saveUpdates)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void refreshUpdateStatus (); */
    #[inline]
    pub unsafe fn refreshUpdateStatus(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).refreshUpdateStatus)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void elevationOptedIn (); */
    #[inline]
    pub unsafe fn elevationOptedIn(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).elevationOptedIn)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cleanupActiveUpdate (); */
    #[inline]
    pub unsafe fn cleanupActiveUpdate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cleanupActiveUpdate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUpdatePrompt {
    vtable: *const nsIUpdatePromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUpdatePrompt {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcee3bd60, 0xc564, 0x42ff,
            [0xa2, 0xbf, 0xd4, 0x42, 0xcb, 0x15, 0xf7, 0x5c])
    }
}

unsafe impl RefCounted for nsIUpdatePrompt {
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
pub trait nsIUpdatePromptCoerce {
    fn coerce_from(v: &nsIUpdatePrompt) -> &Self;
}

impl nsIUpdatePromptCoerce for nsIUpdatePrompt {
    #[inline]
    fn coerce_from(v: &nsIUpdatePrompt) -> &Self {
        v
    }
}

impl nsIUpdatePrompt {
    #[inline]
    pub fn coerce<T: nsIUpdatePromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUpdatePrompt {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUpdatePromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdatePrompt) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUpdatePromptVTable {
    pub __base: nsISupportsVTable,

    /* void checkForUpdates (); */
    pub checkForUpdates: unsafe extern "C" fn (this: *const nsIUpdatePrompt) -> nsresult,

    /* void showUpdateAvailable (in nsIUpdate update); */
    pub showUpdateAvailable: unsafe extern "C" fn (this: *const nsIUpdatePrompt, update: *const nsIUpdate) -> nsresult,

    /* void showUpdateDownloaded (in nsIUpdate update, [optional] in boolean background); */
    pub showUpdateDownloaded: unsafe extern "C" fn (this: *const nsIUpdatePrompt, update: *const nsIUpdate, background: bool) -> nsresult,

    /* void showUpdateError (in nsIUpdate update); */
    pub showUpdateError: unsafe extern "C" fn (this: *const nsIUpdatePrompt, update: *const nsIUpdate) -> nsresult,

    /* void showUpdateHistory (in nsIDOMWindow parent); */
    pub showUpdateHistory: unsafe extern "C" fn (this: *const nsIUpdatePrompt, parent: *const nsIDOMWindow) -> nsresult,

    /* void showUpdateElevationRequired (); */
    pub showUpdateElevationRequired: unsafe extern "C" fn (this: *const nsIUpdatePrompt) -> nsresult,

}


impl nsIUpdatePrompt {
    /* void checkForUpdates (); */
    #[inline]
    pub unsafe fn checkForUpdates(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).checkForUpdates)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showUpdateAvailable (in nsIUpdate update); */
    #[inline]
    pub unsafe fn showUpdateAvailable(&self, update: Option<&nsIUpdate>) -> Result<(), nsresult> {

        match ((*self.vtable).showUpdateAvailable)(self as *const _, update.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showUpdateDownloaded (in nsIUpdate update, [optional] in boolean background); */
    #[inline]
    pub unsafe fn showUpdateDownloaded(&self, update: Option<&nsIUpdate>, background: bool) -> Result<(), nsresult> {

        match ((*self.vtable).showUpdateDownloaded)(self as *const _, update.map_or(::std::ptr::null(), |x| x as *const _), background) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showUpdateError (in nsIUpdate update); */
    #[inline]
    pub unsafe fn showUpdateError(&self, update: Option<&nsIUpdate>) -> Result<(), nsresult> {

        match ((*self.vtable).showUpdateError)(self as *const _, update.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showUpdateHistory (in nsIDOMWindow parent); */
    #[inline]
    pub unsafe fn showUpdateHistory(&self, parent: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).showUpdateHistory)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showUpdateElevationRequired (); */
    #[inline]
    pub unsafe fn showUpdateElevationRequired(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).showUpdateElevationRequired)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


