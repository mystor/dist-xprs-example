//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAudioChannelAgent.idl
//


pub type nsSuspendedTypes = uint32_t;


pub mod nsISuspendedTypes_consts {
    pub const NONE_SUSPENDED: i64 = 0;
    pub const SUSPENDED_PAUSE: i64 = 1;
    pub const SUSPENDED_BLOCK: i64 = 2;
    pub const SUSPENDED_PAUSE_DISPOSABLE: i64 = 3;
    pub const SUSPENDED_STOP_DISPOSABLE: i64 = 4;
}


#[repr(C)]
pub struct nsISuspendedTypes {
    vtable: *const nsISuspendedTypesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISuspendedTypes {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2822a840, 0xf009, 0x11e5,
            [0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsISuspendedTypes {
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
pub trait nsISuspendedTypesCoerce {
    fn coerce_from(v: &nsISuspendedTypes) -> &Self;
}

impl nsISuspendedTypesCoerce for nsISuspendedTypes {
    #[inline]
    fn coerce_from(v: &nsISuspendedTypes) -> &Self {
        v
    }
}

impl nsISuspendedTypes {
    #[inline]
    pub fn coerce<T: nsISuspendedTypesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISuspendedTypes {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISuspendedTypesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISuspendedTypes) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISuspendedTypesVTable {
    pub __base: nsISupportsVTable,

}


impl nsISuspendedTypes {
}


#[repr(C)]
pub struct nsIAudioChannelAgentCallback {
    vtable: *const nsIAudioChannelAgentCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAudioChannelAgentCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x15c05894, 0x408e, 0x4798,
            [0xb5, 0x27, 0xa8, 0xc3, 0x2d, 0x9c, 0x5f, 0x8c])
    }
}

unsafe impl RefCounted for nsIAudioChannelAgentCallback {
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
pub trait nsIAudioChannelAgentCallbackCoerce {
    fn coerce_from(v: &nsIAudioChannelAgentCallback) -> &Self;
}

impl nsIAudioChannelAgentCallbackCoerce for nsIAudioChannelAgentCallback {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelAgentCallback) -> &Self {
        v
    }
}

impl nsIAudioChannelAgentCallback {
    #[inline]
    pub fn coerce<T: nsIAudioChannelAgentCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAudioChannelAgentCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAudioChannelAgentCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelAgentCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAudioChannelAgentCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void windowVolumeChanged (in float aVolume, in bool aMuted); */
    pub windowVolumeChanged: unsafe extern "C" fn (this: *const nsIAudioChannelAgentCallback, aVolume: libc::c_float, aMuted: bool) -> nsresult,

    /* void windowSuspendChanged (in uint32_t aSuspend); */
    pub windowSuspendChanged: unsafe extern "C" fn (this: *const nsIAudioChannelAgentCallback, aSuspend: uint32_t) -> nsresult,

    /* void windowAudioCaptureChanged (in bool aCapture); */
    pub windowAudioCaptureChanged: unsafe extern "C" fn (this: *const nsIAudioChannelAgentCallback, aCapture: bool) -> nsresult,

}


impl nsIAudioChannelAgentCallback {
    /* void windowVolumeChanged (in float aVolume, in bool aMuted); */
    #[inline]
    pub unsafe fn windowVolumeChanged(&self, aVolume: libc::c_float, aMuted: bool) -> Result<(), nsresult> {

        match ((*self.vtable).windowVolumeChanged)(self as *const _, aVolume, aMuted) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void windowSuspendChanged (in uint32_t aSuspend); */
    #[inline]
    pub unsafe fn windowSuspendChanged(&self, aSuspend: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).windowSuspendChanged)(self as *const _, aSuspend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void windowAudioCaptureChanged (in bool aCapture); */
    #[inline]
    pub unsafe fn windowAudioCaptureChanged(&self, aCapture: bool) -> Result<(), nsresult> {

        match ((*self.vtable).windowAudioCaptureChanged)(self as *const _, aCapture) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIAudioChannelAgent_consts {
    pub const AUDIO_AGENT_CHANNEL_NORMAL: i64 = 0;
    pub const AUDIO_AGENT_CHANNEL_CONTENT: i64 = 1;
    pub const AUDIO_AGENT_CHANNEL_NOTIFICATION: i64 = 2;
    pub const AUDIO_AGENT_CHANNEL_ALARM: i64 = 3;
    pub const AUDIO_AGENT_CHANNEL_TELEPHONY: i64 = 4;
    pub const AUDIO_AGENT_CHANNEL_RINGER: i64 = 5;
    pub const AUDIO_AGENT_CHANNEL_PUBLICNOTIFICATION: i64 = 6;
    pub const AUDIO_AGENT_CHANNEL_SYSTEM: i64 = 7;
    pub const AUDIO_AGENT_CHANNEL_ERROR: i64 = 1000;
    pub const AUDIO_AGENT_STATE_NORMAL: i64 = 0;
    pub const AUDIO_AGENT_STATE_MUTED: i64 = 1;
    pub const AUDIO_AGENT_STATE_FADED: i64 = 2;
}


#[repr(C)]
pub struct nsIAudioChannelAgent {
    vtable: *const nsIAudioChannelAgentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAudioChannelAgent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xab7e21c0, 0x970c, 0x11e5,
            [0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIAudioChannelAgent {
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
pub trait nsIAudioChannelAgentCoerce {
    fn coerce_from(v: &nsIAudioChannelAgent) -> &Self;
}

impl nsIAudioChannelAgentCoerce for nsIAudioChannelAgent {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelAgent) -> &Self {
        v
    }
}

impl nsIAudioChannelAgent {
    #[inline]
    pub fn coerce<T: nsIAudioChannelAgentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAudioChannelAgent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAudioChannelAgentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelAgent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAudioChannelAgentVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long audioChannelType; */
    pub get_audioChannelType: unsafe extern "C" fn (this: *const nsIAudioChannelAgent, aAudioChannelType: *mut libc::int32_t) -> nsresult,

    /* void init (in mozIDOMWindow window, in long channelType, in nsIAudioChannelAgentCallback callback); */
    pub init: unsafe extern "C" fn (this: *const nsIAudioChannelAgent, window: *const mozIDOMWindow, channelType: libc::int32_t, callback: *const nsIAudioChannelAgentCallback) -> nsresult,

    /* void initWithWeakCallback (in mozIDOMWindow window, in long channelType, in nsIAudioChannelAgentCallback callback); */
    pub initWithWeakCallback: unsafe extern "C" fn (this: *const nsIAudioChannelAgent, window: *const mozIDOMWindow, channelType: libc::int32_t, callback: *const nsIAudioChannelAgentCallback) -> nsresult,

    /* void notifyStartedPlaying (in AudioPlaybackConfig config, in uint8_t audible); */
    /// Unable to call function as its signature contains a non-rust type
    pub notifyStartedPlaying: *const ::libc::c_void,

    /* void notifyStoppedPlaying (); */
    pub notifyStoppedPlaying: unsafe extern "C" fn (this: *const nsIAudioChannelAgent) -> nsresult,

    /* void notifyStartedAudible (in uint8_t audible, in uint32_t reason); */
    pub notifyStartedAudible: unsafe extern "C" fn (this: *const nsIAudioChannelAgent, audible: uint8_t, reason: uint32_t) -> nsresult,

}


impl nsIAudioChannelAgent {
    /* readonly attribute long audioChannelType; */
    #[inline]
    pub unsafe fn get_audioChannelType(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_audioChannelType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void init (in mozIDOMWindow window, in long channelType, in nsIAudioChannelAgentCallback callback); */
    #[inline]
    pub unsafe fn init(&self, window: Option<&mozIDOMWindow>, channelType: libc::int32_t, callback: Option<&nsIAudioChannelAgentCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), channelType, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initWithWeakCallback (in mozIDOMWindow window, in long channelType, in nsIAudioChannelAgentCallback callback); */
    #[inline]
    pub unsafe fn initWithWeakCallback(&self, window: Option<&mozIDOMWindow>, channelType: libc::int32_t, callback: Option<&nsIAudioChannelAgentCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).initWithWeakCallback)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), channelType, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyStartedPlaying (in AudioPlaybackConfig config, in uint8_t audible); */


    /* void notifyStoppedPlaying (); */
    #[inline]
    pub unsafe fn notifyStoppedPlaying(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyStoppedPlaying)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyStartedAudible (in uint8_t audible, in uint32_t reason); */
    #[inline]
    pub unsafe fn notifyStartedAudible(&self, audible: uint8_t, reason: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).notifyStartedAudible)(self as *const _, audible, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


