//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserElementAPI.idl
//


#[repr(C)]
pub struct nsIBrowserElementNextPaintListener {
    vtable: *const nsIBrowserElementNextPaintListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserElementNextPaintListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x00d0e19d, 0xbd67, 0x491f,
            [0x8e, 0x85, 0xb9, 0x90, 0x52, 0x24, 0xd3, 0xbb])
    }
}

unsafe impl RefCounted for nsIBrowserElementNextPaintListener {
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
pub trait nsIBrowserElementNextPaintListenerCoerce {
    fn coerce_from(v: &nsIBrowserElementNextPaintListener) -> &Self;
}

impl nsIBrowserElementNextPaintListenerCoerce for nsIBrowserElementNextPaintListener {
    #[inline]
    fn coerce_from(v: &nsIBrowserElementNextPaintListener) -> &Self {
        v
    }
}

impl nsIBrowserElementNextPaintListener {
    #[inline]
    pub fn coerce<T: nsIBrowserElementNextPaintListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserElementNextPaintListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserElementNextPaintListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserElementNextPaintListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserElementNextPaintListenerVTable {
    pub __base: nsISupportsVTable,

    /* void recvNextPaint (); */
    pub recvNextPaint: unsafe extern "C" fn (this: *const nsIBrowserElementNextPaintListener) -> nsresult,

}


impl nsIBrowserElementNextPaintListener {
    /* void recvNextPaint (); */
    #[inline]
    pub unsafe fn recvNextPaint(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).recvNextPaint)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIBrowserElementAPI_consts {
    pub const FIND_CASE_SENSITIVE: i64 = 0;
    pub const FIND_CASE_INSENSITIVE: i64 = 1;
    pub const FIND_FORWARD: i64 = 0;
    pub const FIND_BACKWARD: i64 = 1;
}


#[repr(C)]
pub struct nsIBrowserElementAPI {
    vtable: *const nsIBrowserElementAPIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserElementAPI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x57758c10, 0x6036, 0x11e5,
            [0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIBrowserElementAPI {
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
pub trait nsIBrowserElementAPICoerce {
    fn coerce_from(v: &nsIBrowserElementAPI) -> &Self;
}

impl nsIBrowserElementAPICoerce for nsIBrowserElementAPI {
    #[inline]
    fn coerce_from(v: &nsIBrowserElementAPI) -> &Self {
        v
    }
}

impl nsIBrowserElementAPI {
    #[inline]
    pub fn coerce<T: nsIBrowserElementAPICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserElementAPI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserElementAPICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserElementAPI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserElementAPIVTable {
    pub __base: nsISupportsVTable,

    /* void destroyFrameScripts (); */
    pub destroyFrameScripts: unsafe extern "C" fn (this: *const nsIBrowserElementAPI) -> nsresult,

    /* void setFrameLoader (in nsIFrameLoader frameLoader); */
    pub setFrameLoader: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, frameLoader: *const nsIFrameLoader) -> nsresult,

    /* void setVisible (in boolean visible); */
    pub setVisible: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, visible: bool) -> nsresult,

    /* nsIDOMDOMRequest getVisible (); */
    pub getVisible: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* void setActive (in boolean active); */
    pub setActive: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, active: bool) -> nsresult,

    /* boolean getActive (); */
    pub getActive: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut bool) -> nsresult,

    /* void sendMouseEvent (in DOMString type, in uint32_t x, in uint32_t y, in uint32_t button, in uint32_t clickCount, in uint32_t mifiers); */
    pub sendMouseEvent: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, type_: *const nsAString, x: uint32_t, y: uint32_t, button: uint32_t, clickCount: uint32_t, mifiers: uint32_t) -> nsresult,

    /* void sendTouchEvent (in DOMString aType, [array, size_is (count), const] in uint32_t aIdentifiers, [array, size_is (count), const] in int32_t aXs, [array, size_is (count), const] in int32_t aYs, [array, size_is (count), const] in uint32_t aRxs, [array, size_is (count), const] in uint32_t aRys, [array, size_is (count), const] in float aRotationAngles, [array, size_is (count), const] in float aForces, in uint32_t count, in long aModifiers); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendTouchEvent: *const ::libc::c_void,

    /* void goBack (); */
    pub goBack: unsafe extern "C" fn (this: *const nsIBrowserElementAPI) -> nsresult,

    /* void goForward (); */
    pub goForward: unsafe extern "C" fn (this: *const nsIBrowserElementAPI) -> nsresult,

    /* void reload (in boolean hardReload); */
    pub reload: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, hardReload: bool) -> nsresult,

    /* void stop (); */
    pub stop: unsafe extern "C" fn (this: *const nsIBrowserElementAPI) -> nsresult,

    /* nsIDOMDOMRequest download (in DOMString url, [optional] in jsval options); */
    /// Unable to call function as its signature contains a non-rust type
    pub download: *const ::libc::c_void,

    /* nsIDOMDOMRequest purgeHistory (); */
    pub purgeHistory: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMRequest getScreenshot (in uint32_t width, in uint32_t height, [optional] in DOMString mimeType); */
    pub getScreenshot: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, width: uint32_t, height: uint32_t, mimeType: *const nsAString, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* void zoom (in float zoom); */
    pub zoom: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, zoom: libc::c_float) -> nsresult,

    /* nsIDOMDOMRequest getCanGoBack (); */
    pub getCanGoBack: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMRequest getCanGoForward (); */
    pub getCanGoForward: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMRequest getContentDimensions (); */
    pub getContentDimensions: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* void findAll (in DOMString searchString, in long caseSensitivity); */
    pub findAll: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, searchString: *const nsAString, caseSensitivity: libc::int32_t) -> nsresult,

    /* void findNext (in long direction); */
    pub findNext: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, direction: libc::int32_t) -> nsresult,

    /* void clearMatch (); */
    pub clearMatch: unsafe extern "C" fn (this: *const nsIBrowserElementAPI) -> nsresult,

    /* void mute (); */
    pub mute: unsafe extern "C" fn (this: *const nsIBrowserElementAPI) -> nsresult,

    /* void unmute (); */
    pub unmute: unsafe extern "C" fn (this: *const nsIBrowserElementAPI) -> nsresult,

    /* nsIDOMDOMRequest getMuted (); */
    pub getMuted: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* void setVolume (in float volume); */
    pub setVolume: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, volume: libc::c_float) -> nsresult,

    /* nsIDOMDOMRequest getVolume (); */
    pub getVolume: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* void addNextPaintListener (in nsIBrowserElementNextPaintListener listener); */
    pub addNextPaintListener: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, listener: *const nsIBrowserElementNextPaintListener) -> nsresult,

    /* void removeNextPaintListener (in nsIBrowserElementNextPaintListener listener); */
    pub removeNextPaintListener: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, listener: *const nsIBrowserElementNextPaintListener) -> nsresult,

    /* nsIDOMDOMRequest getAudioChannelVolume (in uint32_t audioChannel); */
    pub getAudioChannelVolume: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, audioChannel: uint32_t, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMRequest setAudioChannelVolume (in uint32_t audioChannel, in float volume); */
    pub setAudioChannelVolume: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, audioChannel: uint32_t, volume: libc::c_float, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMRequest getAudioChannelMuted (in uint32_t audioChannel); */
    pub getAudioChannelMuted: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, audioChannel: uint32_t, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMRequest setAudioChannelMuted (in uint32_t audioChannel, in bool muted); */
    pub setAudioChannelMuted: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, audioChannel: uint32_t, muted: bool, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMRequest isAudioChannelActive (in uint32_t audioChannel); */
    pub isAudioChannelActive: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, audioChannel: uint32_t, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMRequest executeScript (in DOMString script, in jsval options); */
    /// Unable to call function as its signature contains a non-rust type
    pub executeScript: *const ::libc::c_void,

    /* nsIDOMDOMRequest getWebManifest (); */
    pub getWebManifest: unsafe extern "C" fn (this: *const nsIBrowserElementAPI, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

}


impl nsIBrowserElementAPI {
    /* void destroyFrameScripts (); */
    #[inline]
    pub unsafe fn destroyFrameScripts(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destroyFrameScripts)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setFrameLoader (in nsIFrameLoader frameLoader); */
    #[inline]
    pub unsafe fn setFrameLoader(&self, frameLoader: Option<&nsIFrameLoader>) -> Result<(), nsresult> {

        match ((*self.vtable).setFrameLoader)(self as *const _, frameLoader.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setVisible (in boolean visible); */
    #[inline]
    pub unsafe fn setVisible(&self, visible: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setVisible)(self as *const _, visible) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDOMRequest getVisible (); */
    #[inline]
    pub unsafe fn getVisible(&self, ) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getVisible)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setActive (in boolean active); */
    #[inline]
    pub unsafe fn setActive(&self, active: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setActive)(self as *const _, active) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean getActive (); */
    #[inline]
    pub unsafe fn getActive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getActive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void sendMouseEvent (in DOMString type, in uint32_t x, in uint32_t y, in uint32_t button, in uint32_t clickCount, in uint32_t mifiers); */
    #[inline]
    pub unsafe fn sendMouseEvent(&self, type_: &[u16], x: uint32_t, y: uint32_t, button: uint32_t, clickCount: uint32_t, mifiers: uint32_t) -> Result<(), nsresult> {
        let type_ = nsString::from(type_);
        match ((*self.vtable).sendMouseEvent)(self as *const _, &*type_, x, y, button, clickCount, mifiers) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendTouchEvent (in DOMString aType, [array, size_is (count), const] in uint32_t aIdentifiers, [array, size_is (count), const] in int32_t aXs, [array, size_is (count), const] in int32_t aYs, [array, size_is (count), const] in uint32_t aRxs, [array, size_is (count), const] in uint32_t aRys, [array, size_is (count), const] in float aRotationAngles, [array, size_is (count), const] in float aForces, in uint32_t count, in long aModifiers); */


    /* void goBack (); */
    #[inline]
    pub unsafe fn goBack(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).goBack)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void goForward (); */
    #[inline]
    pub unsafe fn goForward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).goForward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reload (in boolean hardReload); */
    #[inline]
    pub unsafe fn reload(&self, hardReload: bool) -> Result<(), nsresult> {

        match ((*self.vtable).reload)(self as *const _, hardReload) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stop (); */
    #[inline]
    pub unsafe fn stop(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stop)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDOMRequest download (in DOMString url, [optional] in jsval options); */


    /* nsIDOMDOMRequest purgeHistory (); */
    #[inline]
    pub unsafe fn purgeHistory(&self, ) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).purgeHistory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMRequest getScreenshot (in uint32_t width, in uint32_t height, [optional] in DOMString mimeType); */
    #[inline]
    pub unsafe fn getScreenshot(&self, width: uint32_t, height: uint32_t, mimeType: &[u16]) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mimeType = nsString::from(mimeType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getScreenshot)(self as *const _, width, height, &*mimeType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void zoom (in float zoom); */
    #[inline]
    pub unsafe fn zoom(&self, zoom: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).zoom)(self as *const _, zoom) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDOMRequest getCanGoBack (); */
    #[inline]
    pub unsafe fn getCanGoBack(&self, ) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCanGoBack)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMRequest getCanGoForward (); */
    #[inline]
    pub unsafe fn getCanGoForward(&self, ) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCanGoForward)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMRequest getContentDimensions (); */
    #[inline]
    pub unsafe fn getContentDimensions(&self, ) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getContentDimensions)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void findAll (in DOMString searchString, in long caseSensitivity); */
    #[inline]
    pub unsafe fn findAll(&self, searchString: &[u16], caseSensitivity: libc::int32_t) -> Result<(), nsresult> {
        let searchString = nsString::from(searchString);
        match ((*self.vtable).findAll)(self as *const _, &*searchString, caseSensitivity) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void findNext (in long direction); */
    #[inline]
    pub unsafe fn findNext(&self, direction: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).findNext)(self as *const _, direction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearMatch (); */
    #[inline]
    pub unsafe fn clearMatch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearMatch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void mute (); */
    #[inline]
    pub unsafe fn mute(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).mute)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unmute (); */
    #[inline]
    pub unsafe fn unmute(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unmute)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDOMRequest getMuted (); */
    #[inline]
    pub unsafe fn getMuted(&self, ) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getMuted)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setVolume (in float volume); */
    #[inline]
    pub unsafe fn setVolume(&self, volume: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setVolume)(self as *const _, volume) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDOMRequest getVolume (); */
    #[inline]
    pub unsafe fn getVolume(&self, ) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getVolume)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addNextPaintListener (in nsIBrowserElementNextPaintListener listener); */
    #[inline]
    pub unsafe fn addNextPaintListener(&self, listener: Option<&nsIBrowserElementNextPaintListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addNextPaintListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeNextPaintListener (in nsIBrowserElementNextPaintListener listener); */
    #[inline]
    pub unsafe fn removeNextPaintListener(&self, listener: Option<&nsIBrowserElementNextPaintListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeNextPaintListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDOMRequest getAudioChannelVolume (in uint32_t audioChannel); */
    #[inline]
    pub unsafe fn getAudioChannelVolume(&self, audioChannel: uint32_t) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAudioChannelVolume)(self as *const _, audioChannel, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMRequest setAudioChannelVolume (in uint32_t audioChannel, in float volume); */
    #[inline]
    pub unsafe fn setAudioChannelVolume(&self, audioChannel: uint32_t, volume: libc::c_float) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).setAudioChannelVolume)(self as *const _, audioChannel, volume, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMRequest getAudioChannelMuted (in uint32_t audioChannel); */
    #[inline]
    pub unsafe fn getAudioChannelMuted(&self, audioChannel: uint32_t) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAudioChannelMuted)(self as *const _, audioChannel, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMRequest setAudioChannelMuted (in uint32_t audioChannel, in bool muted); */
    #[inline]
    pub unsafe fn setAudioChannelMuted(&self, audioChannel: uint32_t, muted: bool) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).setAudioChannelMuted)(self as *const _, audioChannel, muted, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMRequest isAudioChannelActive (in uint32_t audioChannel); */
    #[inline]
    pub unsafe fn isAudioChannelActive(&self, audioChannel: uint32_t) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).isAudioChannelActive)(self as *const _, audioChannel, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMRequest executeScript (in DOMString script, in jsval options); */


    /* nsIDOMDOMRequest getWebManifest (); */
    #[inline]
    pub unsafe fn getWebManifest(&self, ) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getWebManifest)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


