//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamingProtocolController.idl
//


#[repr(C)]
pub struct nsIStreamingProtocolMetaData {
    vtable: *const nsIStreamingProtocolMetaDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamingProtocolMetaData {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x294adb30, 0x856c, 0x11e2,
            [0x9e, 0x96, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIStreamingProtocolMetaData {
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
pub trait nsIStreamingProtocolMetaDataCoerce {
    fn coerce_from(v: &nsIStreamingProtocolMetaData) -> &Self;
}

impl nsIStreamingProtocolMetaDataCoerce for nsIStreamingProtocolMetaData {
    #[inline]
    fn coerce_from(v: &nsIStreamingProtocolMetaData) -> &Self {
        v
    }
}

impl nsIStreamingProtocolMetaData {
    #[inline]
    pub fn coerce<T: nsIStreamingProtocolMetaDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamingProtocolMetaData {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStreamingProtocolMetaDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamingProtocolMetaData) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamingProtocolMetaDataVTable {
    pub __base: nsISupportsVTable,

    /* attribute uint32_t frameType; */
    pub get_frameType: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aFrameType: *mut uint32_t) -> nsresult,
    pub set_frameType: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aFrameType: uint32_t) -> nsresult,

    /* attribute uint32_t totalTracks; */
    pub get_totalTracks: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aTotalTracks: *mut uint32_t) -> nsresult,
    pub set_totalTracks: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aTotalTracks: uint32_t) -> nsresult,

    /* attribute ACString mimeType; */
    pub get_mimeType: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aMimeType: *mut nsACString) -> nsresult,
    pub set_mimeType: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aMimeType: *const nsACString) -> nsresult,

    /* attribute unsigned long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aWidth: *mut libc::uint32_t) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aWidth: libc::uint32_t) -> nsresult,

    /* attribute unsigned long height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aHeight: *mut libc::uint32_t) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aHeight: libc::uint32_t) -> nsresult,

    /* attribute unsigned long long duration; */
    pub get_duration: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aDuration: *mut libc::uint64_t) -> nsresult,
    pub set_duration: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aDuration: libc::uint64_t) -> nsresult,

    /* attribute unsigned long sampleRate; */
    pub get_sampleRate: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aSampleRate: *mut libc::uint32_t) -> nsresult,
    pub set_sampleRate: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aSampleRate: libc::uint32_t) -> nsresult,

    /* attribute unsigned long long timeStamp; */
    pub get_timeStamp: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aTimeStamp: *mut libc::uint64_t) -> nsresult,
    pub set_timeStamp: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aTimeStamp: libc::uint64_t) -> nsresult,

    /* attribute unsigned long channelCount; */
    pub get_channelCount: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aChannelCount: *mut libc::uint32_t) -> nsresult,
    pub set_channelCount: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aChannelCount: libc::uint32_t) -> nsresult,

    /* attribute ACString esdsData; */
    pub get_esdsData: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aEsdsData: *mut nsACString) -> nsresult,
    pub set_esdsData: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aEsdsData: *const nsACString) -> nsresult,

    /* attribute ACString avccData; */
    pub get_avccData: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aAvccData: *mut nsACString) -> nsresult,
    pub set_avccData: unsafe extern "C" fn (this: *const nsIStreamingProtocolMetaData, aAvccData: *const nsACString) -> nsresult,

}


impl nsIStreamingProtocolMetaData {
    /* attribute uint32_t frameType; */
    #[inline]
    pub unsafe fn get_frameType(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_frameType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_frameType(&self, aFrameType: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_frameType)(self as *const _, aFrameType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute uint32_t totalTracks; */
    #[inline]
    pub unsafe fn get_totalTracks(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_totalTracks)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_totalTracks(&self, aTotalTracks: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_totalTracks)(self as *const _, aTotalTracks) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString mimeType; */
    #[inline]
    pub unsafe fn get_mimeType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_mimeType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mimeType(&self, aMimeType: &[u8]) -> Result<(), nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        match ((*self.vtable).set_mimeType)(self as *const _, &*aMimeType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_width(&self, aWidth: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_width)(self as *const _, aWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_height)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_height(&self, aHeight: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_height)(self as *const _, aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long long duration; */
    #[inline]
    pub unsafe fn get_duration(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_duration)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_duration(&self, aDuration: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_duration)(self as *const _, aDuration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long sampleRate; */
    #[inline]
    pub unsafe fn get_sampleRate(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_sampleRate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sampleRate(&self, aSampleRate: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_sampleRate)(self as *const _, aSampleRate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long long timeStamp; */
    #[inline]
    pub unsafe fn get_timeStamp(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_timeStamp)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timeStamp(&self, aTimeStamp: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_timeStamp)(self as *const _, aTimeStamp) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long channelCount; */
    #[inline]
    pub unsafe fn get_channelCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_channelCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_channelCount(&self, aChannelCount: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_channelCount)(self as *const _, aChannelCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString esdsData; */
    #[inline]
    pub unsafe fn get_esdsData(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_esdsData)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_esdsData(&self, aEsdsData: &[u8]) -> Result<(), nsresult> {
        let aEsdsData = nsCString::from(aEsdsData);
        match ((*self.vtable).set_esdsData)(self as *const _, &*aEsdsData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString avccData; */
    #[inline]
    pub unsafe fn get_avccData(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_avccData)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_avccData(&self, aAvccData: &[u8]) -> Result<(), nsresult> {
        let aAvccData = nsCString::from(aAvccData);
        match ((*self.vtable).set_avccData)(self as *const _, &*aAvccData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIStreamingProtocolListener {
    vtable: *const nsIStreamingProtocolListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamingProtocolListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc4f6b660, 0x892e, 0x11e2,
            [0x9e, 0x96, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIStreamingProtocolListener {
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
pub trait nsIStreamingProtocolListenerCoerce {
    fn coerce_from(v: &nsIStreamingProtocolListener) -> &Self;
}

impl nsIStreamingProtocolListenerCoerce for nsIStreamingProtocolListener {
    #[inline]
    fn coerce_from(v: &nsIStreamingProtocolListener) -> &Self {
        v
    }
}

impl nsIStreamingProtocolListener {
    #[inline]
    pub fn coerce<T: nsIStreamingProtocolListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamingProtocolListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStreamingProtocolListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamingProtocolListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamingProtocolListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onMediaDataAvailable (in uint8_t index, in ACString data, in uint32_t length, in uint32_t offset, in nsIStreamingProtocolMetaData meta); */
    pub onMediaDataAvailable: unsafe extern "C" fn (this: *const nsIStreamingProtocolListener, index: uint8_t, data: *const nsACString, length: uint32_t, offset: uint32_t, meta: *const nsIStreamingProtocolMetaData) -> nsresult,

    /* void onConnected (in uint8_t index, in nsIStreamingProtocolMetaData meta); */
    pub onConnected: unsafe extern "C" fn (this: *const nsIStreamingProtocolListener, index: uint8_t, meta: *const nsIStreamingProtocolMetaData) -> nsresult,

    /* void onDisconnected (in uint8_t index, in nsresult reason); */
    pub onDisconnected: unsafe extern "C" fn (this: *const nsIStreamingProtocolListener, index: uint8_t, reason: nsresult) -> nsresult,

}


impl nsIStreamingProtocolListener {
    /* void onMediaDataAvailable (in uint8_t index, in ACString data, in uint32_t length, in uint32_t offset, in nsIStreamingProtocolMetaData meta); */
    #[inline]
    pub unsafe fn onMediaDataAvailable(&self, index: uint8_t, data: &[u8], length: uint32_t, offset: uint32_t, meta: Option<&nsIStreamingProtocolMetaData>) -> Result<(), nsresult> {
        let data = nsCString::from(data);
        match ((*self.vtable).onMediaDataAvailable)(self as *const _, index, &*data, length, offset, meta.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onConnected (in uint8_t index, in nsIStreamingProtocolMetaData meta); */
    #[inline]
    pub unsafe fn onConnected(&self, index: uint8_t, meta: Option<&nsIStreamingProtocolMetaData>) -> Result<(), nsresult> {

        match ((*self.vtable).onConnected)(self as *const _, index, meta.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onDisconnected (in uint8_t index, in nsresult reason); */
    #[inline]
    pub unsafe fn onDisconnected(&self, index: uint8_t, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onDisconnected)(self as *const _, index, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIStreamingProtocolController {
    vtable: *const nsIStreamingProtocolControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamingProtocolController {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4ce040f0, 0xc50d, 0x461f,
            [0x94, 0xe2, 0xaf, 0x5a, 0x77, 0xfe, 0x13, 0xa5])
    }
}

unsafe impl RefCounted for nsIStreamingProtocolController {
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
pub trait nsIStreamingProtocolControllerCoerce {
    fn coerce_from(v: &nsIStreamingProtocolController) -> &Self;
}

impl nsIStreamingProtocolControllerCoerce for nsIStreamingProtocolController {
    #[inline]
    fn coerce_from(v: &nsIStreamingProtocolController) -> &Self {
        v
    }
}

impl nsIStreamingProtocolController {
    #[inline]
    pub fn coerce<T: nsIStreamingProtocolControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamingProtocolController {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStreamingProtocolControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamingProtocolController) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamingProtocolControllerVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIURI aUri); */
    pub init: unsafe extern "C" fn (this: *const nsIStreamingProtocolController, aUri: *const nsIURI) -> nsresult,

    /* void asyncOpen (in nsIStreamingProtocolListener aListener); */
    pub asyncOpen: unsafe extern "C" fn (this: *const nsIStreamingProtocolController, aListener: *const nsIStreamingProtocolListener) -> nsresult,

    /* nsIStreamingProtocolMetaData getTrackMetaData (in octet index); */
    pub getTrackMetaData: unsafe extern "C" fn (this: *const nsIStreamingProtocolController, index: libc::uint8_t, _retval: *mut *const nsIStreamingProtocolMetaData) -> nsresult,

    /* void play (); */
    pub play: unsafe extern "C" fn (this: *const nsIStreamingProtocolController) -> nsresult,

    /* void pause (); */
    pub pause: unsafe extern "C" fn (this: *const nsIStreamingProtocolController) -> nsresult,

    /* void resume (); */
    pub resume: unsafe extern "C" fn (this: *const nsIStreamingProtocolController) -> nsresult,

    /* void suspend (); */
    pub suspend: unsafe extern "C" fn (this: *const nsIStreamingProtocolController) -> nsresult,

    /* void seek (in unsigned long long seekTimeUs); */
    pub seek: unsafe extern "C" fn (this: *const nsIStreamingProtocolController, seekTimeUs: libc::uint64_t) -> nsresult,

    /* void stop (); */
    pub stop: unsafe extern "C" fn (this: *const nsIStreamingProtocolController) -> nsresult,

    /* void playbackEnded (); */
    pub playbackEnded: unsafe extern "C" fn (this: *const nsIStreamingProtocolController) -> nsresult,

    /* readonly attribute octet totalTracks; */
    pub get_totalTracks: unsafe extern "C" fn (this: *const nsIStreamingProtocolController, aTotalTracks: *mut libc::uint8_t) -> nsresult,

}


impl nsIStreamingProtocolController {
    /* void init (in nsIURI aUri); */
    #[inline]
    pub unsafe fn init(&self, aUri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aUri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncOpen (in nsIStreamingProtocolListener aListener); */
    #[inline]
    pub unsafe fn asyncOpen(&self, aListener: Option<&nsIStreamingProtocolListener>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncOpen)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIStreamingProtocolMetaData getTrackMetaData (in octet index); */
    #[inline]
    pub unsafe fn getTrackMetaData(&self, index: libc::uint8_t) -> Result<Option<RefPtr<nsIStreamingProtocolMetaData>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getTrackMetaData)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void play (); */
    #[inline]
    pub unsafe fn play(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).play)(self as *const _, ) {
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

    /* void resume (); */
    #[inline]
    pub unsafe fn resume(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resume)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void suspend (); */
    #[inline]
    pub unsafe fn suspend(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).suspend)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void seek (in unsigned long long seekTimeUs); */
    #[inline]
    pub unsafe fn seek(&self, seekTimeUs: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).seek)(self as *const _, seekTimeUs) {
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

    /* void playbackEnded (); */
    #[inline]
    pub unsafe fn playbackEnded(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).playbackEnded)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute octet totalTracks; */
    #[inline]
    pub unsafe fn get_totalTracks(&self, ) -> Result<libc::uint8_t, nsresult> {
        let mut _retval: libc::uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).get_totalTracks)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


