//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISpeechService.idl
//


pub type SpeechServiceType = libc::uint16_t;


#[repr(C)]
pub struct nsISpeechTaskCallback {
    vtable: *const nsISpeechTaskCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISpeechTaskCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc576de0c, 0x8a3d, 0x4570,
            [0xbe, 0x7e, 0x98, 0x76, 0xd3, 0xe5, 0xbe, 0xd2])
    }
}

unsafe impl RefCounted for nsISpeechTaskCallback {
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
pub trait nsISpeechTaskCallbackCoerce {
    fn coerce_from(v: &nsISpeechTaskCallback) -> &Self;
}

impl nsISpeechTaskCallbackCoerce for nsISpeechTaskCallback {
    #[inline]
    fn coerce_from(v: &nsISpeechTaskCallback) -> &Self {
        v
    }
}

impl nsISpeechTaskCallback {
    #[inline]
    pub fn coerce<T: nsISpeechTaskCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISpeechTaskCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISpeechTaskCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechTaskCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISpeechTaskCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onPause (); */
    pub onPause: unsafe extern "C" fn (this: *const nsISpeechTaskCallback) -> nsresult,

    /* void onResume (); */
    pub onResume: unsafe extern "C" fn (this: *const nsISpeechTaskCallback) -> nsresult,

    /* void onCancel (); */
    pub onCancel: unsafe extern "C" fn (this: *const nsISpeechTaskCallback) -> nsresult,

    /* void onVolumeChanged (in float aVolume); */
    pub onVolumeChanged: unsafe extern "C" fn (this: *const nsISpeechTaskCallback, aVolume: libc::c_float) -> nsresult,

}


impl nsISpeechTaskCallback {
    /* void onPause (); */
    #[inline]
    pub unsafe fn onPause(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onPause)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onResume (); */
    #[inline]
    pub unsafe fn onResume(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onResume)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onCancel (); */
    #[inline]
    pub unsafe fn onCancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onCancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onVolumeChanged (in float aVolume); */
    #[inline]
    pub unsafe fn onVolumeChanged(&self, aVolume: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).onVolumeChanged)(self as *const _, aVolume) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsISpeechTask {
    vtable: *const nsISpeechTaskVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISpeechTask {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xad59949c, 0x2437, 0x4b35,
            [0x8e, 0xeb, 0xd7, 0x60, 0xca, 0xab, 0x75, 0xc5])
    }
}

unsafe impl RefCounted for nsISpeechTask {
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
pub trait nsISpeechTaskCoerce {
    fn coerce_from(v: &nsISpeechTask) -> &Self;
}

impl nsISpeechTaskCoerce for nsISpeechTask {
    #[inline]
    fn coerce_from(v: &nsISpeechTask) -> &Self {
        v
    }
}

impl nsISpeechTask {
    #[inline]
    pub fn coerce<T: nsISpeechTaskCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISpeechTask {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISpeechTaskCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechTask) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISpeechTaskVTable {
    pub __base: nsISupportsVTable,

    /* [optional_argc] void setup (in nsISpeechTaskCallback aCallback, [optional] in uint32_t aChannels, [optional] in uint32_t aRate); */
    /// Unable to call function as its signature contains a non-rust type
    pub setup: *const ::libc::c_void,

    /* [implicit_jscontext] void sendAudio (in jsval aData, in jsval aLandmarks); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendAudio: *const ::libc::c_void,

    /* [noscript] void sendAudioNative ([array, size_is (aDataLen)] in short aData, in unsigned long aDataLen); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendAudioNative: *const ::libc::c_void,

    /* void dispatchStart (); */
    pub dispatchStart: unsafe extern "C" fn (this: *const nsISpeechTask) -> nsresult,

    /* void dispatchEnd (in float aElapsedTime, in unsigned long aCharIndex); */
    pub dispatchEnd: unsafe extern "C" fn (this: *const nsISpeechTask, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> nsresult,

    /* void dispatchPause (in float aElapsedTime, in unsigned long aCharIndex); */
    pub dispatchPause: unsafe extern "C" fn (this: *const nsISpeechTask, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> nsresult,

    /* void dispatchResume (in float aElapsedTime, in unsigned long aCharIndex); */
    pub dispatchResume: unsafe extern "C" fn (this: *const nsISpeechTask, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> nsresult,

    /* void dispatchError (in float aElapsedTime, in unsigned long aCharIndex); */
    pub dispatchError: unsafe extern "C" fn (this: *const nsISpeechTask, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> nsresult,

    /* [optional_argc] void dispatchBoundary (in DOMString aName, in float aElapsedTime, in unsigned long aCharIndex, [optional] in unsigned long aCharLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub dispatchBoundary: *const ::libc::c_void,

    /* void dispatchMark (in DOMString aName, in float aElapsedTime, in unsigned long aCharIndex); */
    pub dispatchMark: unsafe extern "C" fn (this: *const nsISpeechTask, aName: *const nsAString, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> nsresult,

}


impl nsISpeechTask {
    /* [optional_argc] void setup (in nsISpeechTaskCallback aCallback, [optional] in uint32_t aChannels, [optional] in uint32_t aRate); */


    /* [implicit_jscontext] void sendAudio (in jsval aData, in jsval aLandmarks); */


    /* [noscript] void sendAudioNative ([array, size_is (aDataLen)] in short aData, in unsigned long aDataLen); */


    /* void dispatchStart (); */
    #[inline]
    pub unsafe fn dispatchStart(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dispatchStart)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dispatchEnd (in float aElapsedTime, in unsigned long aCharIndex); */
    #[inline]
    pub unsafe fn dispatchEnd(&self, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).dispatchEnd)(self as *const _, aElapsedTime, aCharIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dispatchPause (in float aElapsedTime, in unsigned long aCharIndex); */
    #[inline]
    pub unsafe fn dispatchPause(&self, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).dispatchPause)(self as *const _, aElapsedTime, aCharIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dispatchResume (in float aElapsedTime, in unsigned long aCharIndex); */
    #[inline]
    pub unsafe fn dispatchResume(&self, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).dispatchResume)(self as *const _, aElapsedTime, aCharIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dispatchError (in float aElapsedTime, in unsigned long aCharIndex); */
    #[inline]
    pub unsafe fn dispatchError(&self, aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).dispatchError)(self as *const _, aElapsedTime, aCharIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [optional_argc] void dispatchBoundary (in DOMString aName, in float aElapsedTime, in unsigned long aCharIndex, [optional] in unsigned long aCharLength); */


    /* void dispatchMark (in DOMString aName, in float aElapsedTime, in unsigned long aCharIndex); */
    #[inline]
    pub unsafe fn dispatchMark(&self, aName: &[u16], aElapsedTime: libc::c_float, aCharIndex: libc::uint32_t) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).dispatchMark)(self as *const _, &*aName, aElapsedTime, aCharIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsISpeechService_consts {
    pub const SERVICETYPE_DIRECT_AUDIO: i64 = 1;
    pub const SERVICETYPE_INDIRECT_AUDIO: i64 = 2;
}


#[repr(C)]
pub struct nsISpeechService {
    vtable: *const nsISpeechServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISpeechService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9b7d59db, 0x88ff, 0x43d0,
            [0xb6, 0xee, 0x9f, 0x63, 0xd0, 0x42, 0xd0, 0x8f])
    }
}

unsafe impl RefCounted for nsISpeechService {
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
pub trait nsISpeechServiceCoerce {
    fn coerce_from(v: &nsISpeechService) -> &Self;
}

impl nsISpeechServiceCoerce for nsISpeechService {
    #[inline]
    fn coerce_from(v: &nsISpeechService) -> &Self {
        v
    }
}

impl nsISpeechService {
    #[inline]
    pub fn coerce<T: nsISpeechServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISpeechService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISpeechServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISpeechServiceVTable {
    pub __base: nsISupportsVTable,

    /* void speak (in DOMString aText, in DOMString aUri, in float aVolume, in float aRate, in float aPitch, in nsISpeechTask aTask); */
    pub speak: unsafe extern "C" fn (this: *const nsISpeechService, aText: *const nsAString, aUri: *const nsAString, aVolume: libc::c_float, aRate: libc::c_float, aPitch: libc::c_float, aTask: *const nsISpeechTask) -> nsresult,

    /* readonly attribute SpeechServiceType serviceType; */
    pub get_serviceType: unsafe extern "C" fn (this: *const nsISpeechService, aServiceType: *mut SpeechServiceType) -> nsresult,

}


impl nsISpeechService {
    /* void speak (in DOMString aText, in DOMString aUri, in float aVolume, in float aRate, in float aPitch, in nsISpeechTask aTask); */
    #[inline]
    pub unsafe fn speak(&self, aText: &[u16], aUri: &[u16], aVolume: libc::c_float, aRate: libc::c_float, aPitch: libc::c_float, aTask: Option<&nsISpeechTask>) -> Result<(), nsresult> {
        let aText = nsString::from(aText);
        let aUri = nsString::from(aUri);
        match ((*self.vtable).speak)(self as *const _, &*aText, &*aUri, aVolume, aRate, aPitch, aTask.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute SpeechServiceType serviceType; */
    #[inline]
    pub unsafe fn get_serviceType(&self, ) -> Result<SpeechServiceType, nsresult> {
        let mut _retval: SpeechServiceType = ::std::mem::zeroed();
        match ((*self.vtable).get_serviceType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


