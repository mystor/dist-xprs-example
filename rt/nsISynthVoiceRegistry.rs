//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISynthVoiceRegistry.idl
//


#[repr(C)]
pub struct nsISynthVoiceRegistry {
    vtable: *const nsISynthVoiceRegistryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISynthVoiceRegistry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5d7a0b38, 0x77e5, 0x4ee5,
            [0x89, 0x7c, 0xce, 0x5d, 0xb9, 0xb8, 0x5d, 0x44])
    }
}

unsafe impl RefCounted for nsISynthVoiceRegistry {
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
pub trait nsISynthVoiceRegistryCoerce {
    fn coerce_from(v: &nsISynthVoiceRegistry) -> &Self;
}

impl nsISynthVoiceRegistryCoerce for nsISynthVoiceRegistry {
    #[inline]
    fn coerce_from(v: &nsISynthVoiceRegistry) -> &Self {
        v
    }
}

impl nsISynthVoiceRegistry {
    #[inline]
    pub fn coerce<T: nsISynthVoiceRegistryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISynthVoiceRegistry {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISynthVoiceRegistryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISynthVoiceRegistry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISynthVoiceRegistryVTable {
    pub __base: nsISupportsVTable,

    /* void addVoice (in nsISpeechService aService, in DOMString aUri, in DOMString aName, in DOMString aLang, in boolean aLocalService, in boolean aQueuesUtterances); */
    pub addVoice: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aService: *const nsISpeechService, aUri: *const nsAString, aName: *const nsAString, aLang: *const nsAString, aLocalService: bool, aQueuesUtterances: bool) -> nsresult,

    /* void removeVoice (in nsISpeechService aService, in DOMString aUri); */
    pub removeVoice: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aService: *const nsISpeechService, aUri: *const nsAString) -> nsresult,

    /* void notifyVoicesChanged (); */
    pub notifyVoicesChanged: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry) -> nsresult,

    /* void setDefaultVoice (in DOMString aUri, in boolean aIsDefault); */
    pub setDefaultVoice: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aUri: *const nsAString, aIsDefault: bool) -> nsresult,

    /* readonly attribute uint32_t voiceCount; */
    pub get_voiceCount: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aVoiceCount: *mut uint32_t) -> nsresult,

    /* AString getVoice (in uint32_t aIndex); */
    pub getVoice: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aIndex: uint32_t, _retval: *mut nsAString) -> nsresult,

    /* bool isDefaultVoice (in DOMString aUri); */
    pub isDefaultVoice: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aUri: *const nsAString, _retval: *mut bool) -> nsresult,

    /* bool isLocalVoice (in DOMString aUri); */
    pub isLocalVoice: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aUri: *const nsAString, _retval: *mut bool) -> nsresult,

    /* AString getVoiceLang (in DOMString aUri); */
    pub getVoiceLang: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aUri: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString getVoiceName (in DOMString aUri); */
    pub getVoiceName: unsafe extern "C" fn (this: *const nsISynthVoiceRegistry, aUri: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsISynthVoiceRegistry {
    /* void addVoice (in nsISpeechService aService, in DOMString aUri, in DOMString aName, in DOMString aLang, in boolean aLocalService, in boolean aQueuesUtterances); */
    #[inline]
    pub unsafe fn addVoice(&self, aService: Option<&nsISpeechService>, aUri: &[u16], aName: &[u16], aLang: &[u16], aLocalService: bool, aQueuesUtterances: bool) -> Result<(), nsresult> {
        let aUri = nsString::from(aUri);
        let aName = nsString::from(aName);
        let aLang = nsString::from(aLang);
        match ((*self.vtable).addVoice)(self as *const _, aService.map_or(::std::ptr::null(), |x| x as *const _), &*aUri, &*aName, &*aLang, aLocalService, aQueuesUtterances) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeVoice (in nsISpeechService aService, in DOMString aUri); */
    #[inline]
    pub unsafe fn removeVoice(&self, aService: Option<&nsISpeechService>, aUri: &[u16]) -> Result<(), nsresult> {
        let aUri = nsString::from(aUri);
        match ((*self.vtable).removeVoice)(self as *const _, aService.map_or(::std::ptr::null(), |x| x as *const _), &*aUri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyVoicesChanged (); */
    #[inline]
    pub unsafe fn notifyVoicesChanged(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyVoicesChanged)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDefaultVoice (in DOMString aUri, in boolean aIsDefault); */
    #[inline]
    pub unsafe fn setDefaultVoice(&self, aUri: &[u16], aIsDefault: bool) -> Result<(), nsresult> {
        let aUri = nsString::from(aUri);
        match ((*self.vtable).setDefaultVoice)(self as *const _, &*aUri, aIsDefault) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute uint32_t voiceCount; */
    #[inline]
    pub unsafe fn get_voiceCount(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_voiceCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getVoice (in uint32_t aIndex); */
    #[inline]
    pub unsafe fn getVoice(&self, aIndex: uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getVoice)(self as *const _, aIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool isDefaultVoice (in DOMString aUri); */
    #[inline]
    pub unsafe fn isDefaultVoice(&self, aUri: &[u16]) -> Result<bool, nsresult> {
        let aUri = nsString::from(aUri);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isDefaultVoice)(self as *const _, &*aUri, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool isLocalVoice (in DOMString aUri); */
    #[inline]
    pub unsafe fn isLocalVoice(&self, aUri: &[u16]) -> Result<bool, nsresult> {
        let aUri = nsString::from(aUri);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isLocalVoice)(self as *const _, &*aUri, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getVoiceLang (in DOMString aUri); */
    #[inline]
    pub unsafe fn getVoiceLang(&self, aUri: &[u16]) -> Result<nsString, nsresult> {
        let aUri = nsString::from(aUri);
        let mut _retval = nsString::new();
        match ((*self.vtable).getVoiceLang)(self as *const _, &*aUri, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getVoiceName (in DOMString aUri); */
    #[inline]
    pub unsafe fn getVoiceName(&self, aUri: &[u16]) -> Result<nsString, nsresult> {
        let aUri = nsString::from(aUri);
        let mut _retval = nsString::new();
        match ((*self.vtable).getVoiceName)(self as *const _, &*aUri, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


