//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMediaElement.idl
//


pub mod nsIDOMHTMLMediaElement_consts {
    pub const NETWORK_EMPTY: i64 = 0;
    pub const NETWORK_IDLE: i64 = 1;
    pub const NETWORK_LOADING: i64 = 2;
    pub const NETWORK_NO_SOURCE: i64 = 3;
    pub const HAVE_NOTHING: i64 = 0;
    pub const HAVE_METADATA: i64 = 1;
    pub const HAVE_CURRENT_DATA: i64 = 2;
    pub const HAVE_FUTURE_DATA: i64 = 3;
    pub const HAVE_ENOUGH_DATA: i64 = 4;
}


#[repr(C)]
pub struct nsIDOMHTMLMediaElement {
    vtable: *const nsIDOMHTMLMediaElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLMediaElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc041d76c, 0x15ce, 0x47ad,
            [0xb6, 0x1d, 0xe8, 0x75, 0x5a, 0x6d, 0xb6, 0x38])
    }
}

unsafe impl RefCounted for nsIDOMHTMLMediaElement {
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
pub trait nsIDOMHTMLMediaElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLMediaElement) -> &Self;
}

impl nsIDOMHTMLMediaElementCoerce for nsIDOMHTMLMediaElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMediaElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLMediaElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLMediaElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLMediaElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLMediaElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMediaElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLMediaElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aSrc: *const nsAString) -> nsresult,

    /* readonly attribute DOMString currentSrc; */
    pub get_currentSrc: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aCurrentSrc: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned short networkState; */
    pub get_networkState: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aNetworkState: *mut libc::uint16_t) -> nsresult,

    /* attribute DOMString preload; */
    pub get_preload: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aPreload: *mut nsAString) -> nsresult,
    pub set_preload: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aPreload: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMTimeRanges buffered; */
    pub get_buffered: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aBuffered: *mut *const nsIDOMTimeRanges) -> nsresult,

    /* void load (); */
    pub load: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement) -> nsresult,

    /* DOMString canPlayType (in DOMString type); */
    pub canPlayType: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, type_: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned short readyState; */
    pub get_readyState: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aReadyState: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute boolean seeking; */
    pub get_seeking: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aSeeking: *mut bool) -> nsresult,

    /* attribute double currentTime; */
    pub get_currentTime: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aCurrentTime: *mut libc::c_double) -> nsresult,
    pub set_currentTime: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aCurrentTime: libc::c_double) -> nsresult,

    /* readonly attribute double duration; */
    pub get_duration: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aDuration: *mut libc::c_double) -> nsresult,

    /* readonly attribute boolean paused; */
    pub get_paused: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aPaused: *mut bool) -> nsresult,

    /* attribute double defaultPlaybackRate; */
    pub get_defaultPlaybackRate: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aDefaultPlaybackRate: *mut libc::c_double) -> nsresult,
    pub set_defaultPlaybackRate: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aDefaultPlaybackRate: libc::c_double) -> nsresult,

    /* attribute double playbackRate; */
    pub get_playbackRate: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aPlaybackRate: *mut libc::c_double) -> nsresult,
    pub set_playbackRate: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aPlaybackRate: libc::c_double) -> nsresult,

    /* attribute boolean mozPreservesPitch; */
    pub get_mozPreservesPitch: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aMozPreservesPitch: *mut bool) -> nsresult,
    pub set_mozPreservesPitch: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aMozPreservesPitch: bool) -> nsresult,

    /* readonly attribute nsIDOMTimeRanges played; */
    pub get_played: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aPlayed: *mut *const nsIDOMTimeRanges) -> nsresult,

    /* readonly attribute nsIDOMTimeRanges seekable; */
    pub get_seekable: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aSeekable: *mut *const nsIDOMTimeRanges) -> nsresult,

    /* readonly attribute boolean ended; */
    pub get_ended: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aEnded: *mut bool) -> nsresult,

    /* readonly attribute boolean mozAutoplayEnabled; */
    pub get_mozAutoplayEnabled: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aMozAutoplayEnabled: *mut bool) -> nsresult,

    /* attribute boolean autoplay; */
    pub get_autoplay: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aAutoplay: *mut bool) -> nsresult,
    pub set_autoplay: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aAutoplay: bool) -> nsresult,

    /* attribute boolean loop; */
    pub get_loop_: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aLoop: *mut bool) -> nsresult,
    pub set_loop_: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aLoop: bool) -> nsresult,

    /* void pause (); */
    pub pause: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement) -> nsresult,

    /* attribute boolean controls; */
    pub get_controls: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aControls: *mut bool) -> nsresult,
    pub set_controls: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aControls: bool) -> nsresult,

    /* attribute double volume; */
    pub get_volume: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aVolume: *mut libc::c_double) -> nsresult,
    pub set_volume: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aVolume: libc::c_double) -> nsresult,

    /* attribute boolean muted; */
    pub get_muted: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aMuted: *mut bool) -> nsresult,
    pub set_muted: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aMuted: bool) -> nsresult,

    /* attribute boolean defaultMuted; */
    pub get_defaultMuted: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aDefaultMuted: *mut bool) -> nsresult,
    pub set_defaultMuted: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aDefaultMuted: bool) -> nsresult,

    /* readonly attribute boolean mozAudioCaptured; */
    pub get_mozAudioCaptured: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aMozAudioCaptured: *mut bool) -> nsresult,

    /* [implicit_jscontext] jsval mozGetMetadata (); */
    /// Unable to call function as its signature contains a non-rust type
    pub mozGetMetadata: *const ::libc::c_void,

    /* readonly attribute double mozFragmentEnd; */
    pub get_mozFragmentEnd: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement, aMozFragmentEnd: *mut libc::c_double) -> nsresult,

    /* [notxpcom] boolean isVideo (); */
    pub isVideo: unsafe extern "C" fn (this: *const nsIDOMHTMLMediaElement) -> bool,

    /* [noscript,notxpcom] void onVisibilityChange (in Visibility aNewVisibility); */
    /// Unable to call function as its signature contains a non-rust type
    pub onVisibilityChange: *const ::libc::c_void,

}


impl nsIDOMHTMLMediaElement {
    /* attribute DOMString src; */
    #[inline]
    pub unsafe fn get_src(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_src)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_src(&self, aSrc: &[u16]) -> Result<(), nsresult> {
        let aSrc = nsString::from(aSrc);
        match ((*self.vtable).set_src)(self as *const _, &*aSrc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute DOMString currentSrc; */
    #[inline]
    pub unsafe fn get_currentSrc(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_currentSrc)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short networkState; */
    #[inline]
    pub unsafe fn get_networkState(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_networkState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString preload; */
    #[inline]
    pub unsafe fn get_preload(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_preload)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_preload(&self, aPreload: &[u16]) -> Result<(), nsresult> {
        let aPreload = nsString::from(aPreload);
        match ((*self.vtable).set_preload)(self as *const _, &*aPreload) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMTimeRanges buffered; */
    #[inline]
    pub unsafe fn get_buffered(&self, ) -> Result<Option<RefPtr<nsIDOMTimeRanges>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_buffered)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void load (); */
    #[inline]
    pub unsafe fn load(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).load)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString canPlayType (in DOMString type); */
    #[inline]
    pub unsafe fn canPlayType(&self, type_: &[u16]) -> Result<nsString, nsresult> {
        let type_ = nsString::from(type_);
        let mut _retval = nsString::new();
        match ((*self.vtable).canPlayType)(self as *const _, &*type_, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short readyState; */
    #[inline]
    pub unsafe fn get_readyState(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_readyState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean seeking; */
    #[inline]
    pub unsafe fn get_seeking(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_seeking)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute double currentTime; */
    #[inline]
    pub unsafe fn get_currentTime(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_currentTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_currentTime(&self, aCurrentTime: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentTime)(self as *const _, aCurrentTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute double duration; */
    #[inline]
    pub unsafe fn get_duration(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_duration)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean paused; */
    #[inline]
    pub unsafe fn get_paused(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_paused)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute double defaultPlaybackRate; */
    #[inline]
    pub unsafe fn get_defaultPlaybackRate(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultPlaybackRate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultPlaybackRate(&self, aDefaultPlaybackRate: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultPlaybackRate)(self as *const _, aDefaultPlaybackRate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double playbackRate; */
    #[inline]
    pub unsafe fn get_playbackRate(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_playbackRate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_playbackRate(&self, aPlaybackRate: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_playbackRate)(self as *const _, aPlaybackRate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean mozPreservesPitch; */
    #[inline]
    pub unsafe fn get_mozPreservesPitch(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozPreservesPitch)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mozPreservesPitch(&self, aMozPreservesPitch: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_mozPreservesPitch)(self as *const _, aMozPreservesPitch) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMTimeRanges played; */
    #[inline]
    pub unsafe fn get_played(&self, ) -> Result<Option<RefPtr<nsIDOMTimeRanges>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_played)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMTimeRanges seekable; */
    #[inline]
    pub unsafe fn get_seekable(&self, ) -> Result<Option<RefPtr<nsIDOMTimeRanges>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_seekable)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean ended; */
    #[inline]
    pub unsafe fn get_ended(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_ended)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean mozAutoplayEnabled; */
    #[inline]
    pub unsafe fn get_mozAutoplayEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozAutoplayEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean autoplay; */
    #[inline]
    pub unsafe fn get_autoplay(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_autoplay)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_autoplay(&self, aAutoplay: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_autoplay)(self as *const _, aAutoplay) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean loop; */
    #[inline]
    pub unsafe fn get_loop_(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loop_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_loop_(&self, aLoop: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_loop_)(self as *const _, aLoop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void pause (); */
    #[inline]
    pub unsafe fn pause(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).pause)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean controls; */
    #[inline]
    pub unsafe fn get_controls(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_controls)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_controls(&self, aControls: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_controls)(self as *const _, aControls) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute double volume; */
    #[inline]
    pub unsafe fn get_volume(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_volume)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_volume(&self, aVolume: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_volume)(self as *const _, aVolume) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean muted; */
    #[inline]
    pub unsafe fn get_muted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_muted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_muted(&self, aMuted: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_muted)(self as *const _, aMuted) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean defaultMuted; */
    #[inline]
    pub unsafe fn get_defaultMuted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultMuted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultMuted(&self, aDefaultMuted: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultMuted)(self as *const _, aDefaultMuted) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean mozAudioCaptured; */
    #[inline]
    pub unsafe fn get_mozAudioCaptured(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozAudioCaptured)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval mozGetMetadata (); */


    /* readonly attribute double mozFragmentEnd; */
    #[inline]
    pub unsafe fn get_mozFragmentEnd(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_mozFragmentEnd)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [notxpcom] boolean isVideo (); */
    #[inline]
    pub unsafe fn isVideo(&self, ) -> bool {

        let _retval = ((*self.vtable).isVideo)(self as *const _, );
        _retval
    }

    /* [noscript,notxpcom] void onVisibilityChange (in Visibility aNewVisibility); */


}


