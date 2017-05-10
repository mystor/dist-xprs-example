//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAudioChannelService.idl
//


#[repr(C)]
pub struct nsIAudioChannelService {
    vtable: *const nsIAudioChannelServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAudioChannelService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5cb24dbc, 0x36c7, 0x46a4,
            [0x99, 0x66, 0xac, 0x73, 0x14, 0x1d, 0xc7, 0x95])
    }
}

unsafe impl RefCounted for nsIAudioChannelService {
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
pub trait nsIAudioChannelServiceCoerce {
    fn coerce_from(v: &nsIAudioChannelService) -> &Self;
}

impl nsIAudioChannelServiceCoerce for nsIAudioChannelService {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelService) -> &Self {
        v
    }
}

impl nsIAudioChannelService {
    #[inline]
    pub fn coerce<T: nsIAudioChannelServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAudioChannelService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAudioChannelServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAudioChannelServiceVTable {
    pub __base: nsISupportsVTable,

    /* float getAudioChannelVolume (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
    pub getAudioChannelVolume: unsafe extern "C" fn (this: *const nsIAudioChannelService, window: *const mozIDOMWindowProxy, audioChannel: libc::uint16_t, _retval: *mut libc::c_float) -> nsresult,

    /* void setAudioChannelVolume (in mozIDOMWindowProxy window, in unsigned short audioChannel, in float volume); */
    pub setAudioChannelVolume: unsafe extern "C" fn (this: *const nsIAudioChannelService, window: *const mozIDOMWindowProxy, audioChannel: libc::uint16_t, volume: libc::c_float) -> nsresult,

    /* boolean getAudioChannelMuted (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
    pub getAudioChannelMuted: unsafe extern "C" fn (this: *const nsIAudioChannelService, window: *const mozIDOMWindowProxy, audioChannel: libc::uint16_t, _retval: *mut bool) -> nsresult,

    /* void setAudioChannelMuted (in mozIDOMWindowProxy window, in unsigned short audioChannel, in boolean muted); */
    pub setAudioChannelMuted: unsafe extern "C" fn (this: *const nsIAudioChannelService, window: *const mozIDOMWindowProxy, audioChannel: libc::uint16_t, muted: bool) -> nsresult,

    /* boolean isAudioChannelActive (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
    pub isAudioChannelActive: unsafe extern "C" fn (this: *const nsIAudioChannelService, window: *const mozIDOMWindowProxy, audioChannel: libc::uint16_t, _retval: *mut bool) -> nsresult,

}


impl nsIAudioChannelService {
    /* float getAudioChannelVolume (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
    #[inline]
    pub unsafe fn getAudioChannelVolume(&self, window: Option<&mozIDOMWindowProxy>, audioChannel: libc::uint16_t) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).getAudioChannelVolume)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), audioChannel, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setAudioChannelVolume (in mozIDOMWindowProxy window, in unsigned short audioChannel, in float volume); */
    #[inline]
    pub unsafe fn setAudioChannelVolume(&self, window: Option<&mozIDOMWindowProxy>, audioChannel: libc::uint16_t, volume: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setAudioChannelVolume)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), audioChannel, volume) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean getAudioChannelMuted (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
    #[inline]
    pub unsafe fn getAudioChannelMuted(&self, window: Option<&mozIDOMWindowProxy>, audioChannel: libc::uint16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getAudioChannelMuted)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), audioChannel, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setAudioChannelMuted (in mozIDOMWindowProxy window, in unsigned short audioChannel, in boolean muted); */
    #[inline]
    pub unsafe fn setAudioChannelMuted(&self, window: Option<&mozIDOMWindowProxy>, audioChannel: libc::uint16_t, muted: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setAudioChannelMuted)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), audioChannel, muted) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isAudioChannelActive (in mozIDOMWindowProxy window, in unsigned short audioChannel); */
    #[inline]
    pub unsafe fn isAudioChannelActive(&self, window: Option<&mozIDOMWindowProxy>, audioChannel: libc::uint16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isAudioChannelActive)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), audioChannel, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


