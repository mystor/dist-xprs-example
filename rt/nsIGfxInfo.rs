//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGfxInfo.idl
//


pub mod nsIGfxInfo_consts {
    pub const FEATURE_DIRECT2D: i64 = 1;
    pub const FEATURE_DIRECT3D_9_LAYERS: i64 = 2;
    pub const FEATURE_DIRECT3D_10_LAYERS: i64 = 3;
    pub const FEATURE_DIRECT3D_10_1_LAYERS: i64 = 4;
    pub const FEATURE_OPENGL_LAYERS: i64 = 5;
    pub const FEATURE_WEBGL_OPENGL: i64 = 6;
    pub const FEATURE_WEBGL_ANGLE: i64 = 7;
    pub const FEATURE_WEBGL_MSAA: i64 = 8;
    pub const FEATURE_STAGEFRIGHT: i64 = 9;
    pub const FEATURE_WEBRTC_HW_ACCELERATION: i64 = 10;
    pub const FEATURE_DIRECT3D_11_LAYERS: i64 = 11;
    pub const FEATURE_HARDWARE_VIDEO_DECODING: i64 = 12;
    pub const FEATURE_DIRECT3D_11_ANGLE: i64 = 13;
    pub const FEATURE_WEBRTC_HW_ACCELERATION_ENCODE: i64 = 14;
    pub const FEATURE_WEBRTC_HW_ACCELERATION_DECODE: i64 = 15;
    pub const FEATURE_CANVAS2D_ACCELERATION: i64 = 16;
    pub const FEATURE_VP8_HW_DECODE: i64 = 17;
    pub const FEATURE_VP9_HW_DECODE: i64 = 18;
    pub const FEATURE_DX_INTEROP2: i64 = 19;
    pub const FEATURE_GPU_PROCESS: i64 = 20;
    pub const FEATURE_WEBGL2: i64 = 21;
    pub const FEATURE_MAX_VALUE: i64 = 21;
    pub const FEATURE_STATUS_OK: i64 = 1;
    pub const FEATURE_STATUS_UNKNOWN: i64 = 2;
    pub const FEATURE_BLOCKED_DRIVER_VERSION: i64 = 3;
    pub const FEATURE_BLOCKED_DEVICE: i64 = 4;
    pub const FEATURE_DISCOURAGED: i64 = 5;
    pub const FEATURE_BLOCKED_OS_VERSION: i64 = 6;
    pub const FEATURE_BLOCKED_MISMATCHED_VERSION: i64 = 7;
}


#[repr(C)]
pub struct nsIGfxInfo {
    vtable: *const nsIGfxInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGfxInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1accd618, 0x4c80, 0x4703,
            [0x9d, 0x29, 0xec, 0xf2, 0x57, 0xd3, 0x97, 0xc8])
    }
}

unsafe impl RefCounted for nsIGfxInfo {
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
pub trait nsIGfxInfoCoerce {
    fn coerce_from(v: &nsIGfxInfo) -> &Self;
}

impl nsIGfxInfoCoerce for nsIGfxInfo {
    #[inline]
    fn coerce_from(v: &nsIGfxInfo) -> &Self {
        v
    }
}

impl nsIGfxInfo {
    #[inline]
    pub fn coerce<T: nsIGfxInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGfxInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGfxInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGfxInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGfxInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean D2DEnabled; */
    pub get_D2DEnabled: unsafe extern "C" fn (this: *const nsIGfxInfo, aD2DEnabled: *mut bool) -> nsresult,

    /* readonly attribute boolean DWriteEnabled; */
    pub get_DWriteEnabled: unsafe extern "C" fn (this: *const nsIGfxInfo, aDWriteEnabled: *mut bool) -> nsresult,

    /* readonly attribute boolean usingGPUProcess; */
    pub get_usingGPUProcess: unsafe extern "C" fn (this: *const nsIGfxInfo, aUsingGPUProcess: *mut bool) -> nsresult,

    /* readonly attribute DOMString DWriteVersion; */
    pub get_DWriteVersion: unsafe extern "C" fn (this: *const nsIGfxInfo, aDWriteVersion: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString cleartypeParameters; */
    pub get_cleartypeParameters: unsafe extern "C" fn (this: *const nsIGfxInfo, aCleartypeParameters: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString ContentBackend; */
    pub get_ContentBackend: unsafe extern "C" fn (this: *const nsIGfxInfo, aContentBackend: *mut nsAString) -> nsresult,

    /* readonly attribute boolean WebRenderEnabled; */
    pub get_WebRenderEnabled: unsafe extern "C" fn (this: *const nsIGfxInfo, aWebRenderEnabled: *mut bool) -> nsresult,

    /* readonly attribute DOMString adapterDescription; */
    pub get_adapterDescription: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDescription: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDescription2; */
    pub get_adapterDescription2: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDescription2: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDriver; */
    pub get_adapterDriver: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDriver: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDriver2; */
    pub get_adapterDriver2: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDriver2: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterVendorID; */
    pub get_adapterVendorID: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterVendorID: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterVendorID2; */
    pub get_adapterVendorID2: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterVendorID2: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDeviceID; */
    pub get_adapterDeviceID: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDeviceID: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDeviceID2; */
    pub get_adapterDeviceID2: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDeviceID2: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterSubsysID; */
    pub get_adapterSubsysID: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterSubsysID: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterSubsysID2; */
    pub get_adapterSubsysID2: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterSubsysID2: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterRAM; */
    pub get_adapterRAM: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterRAM: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterRAM2; */
    pub get_adapterRAM2: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterRAM2: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDriverVersion; */
    pub get_adapterDriverVersion: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDriverVersion: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDriverVersion2; */
    pub get_adapterDriverVersion2: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDriverVersion2: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDriverDate; */
    pub get_adapterDriverDate: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDriverDate: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString adapterDriverDate2; */
    pub get_adapterDriverDate2: unsafe extern "C" fn (this: *const nsIGfxInfo, aAdapterDriverDate2: *mut nsAString) -> nsresult,

    /* readonly attribute boolean isGPU2Active; */
    pub get_isGPU2Active: unsafe extern "C" fn (this: *const nsIGfxInfo, aIsGPU2Active: *mut bool) -> nsresult,

    /* [implicit_jscontext] jsval getMonitors (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMonitors: *const ::libc::c_void,

    /* void getFailures (out unsigned long failureCount, [array, size_is (failureCount), optional] out long indices, [array, size_is (failureCount), retval] out string failures); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFailures: *const ::libc::c_void,

    /* [noscript,notxpcom] void logFailure (in ACString failure); */
    pub logFailure: unsafe extern "C" fn (this: *const nsIGfxInfo, failure: *const nsACString) -> libc::c_void,

    /* long getFeatureStatus (in long aFeature, [optional] out ACString aFailureId); */
    pub getFeatureStatus: unsafe extern "C" fn (this: *const nsIGfxInfo, aFeature: libc::int32_t, aFailureId: *mut nsACString, _retval: *mut libc::int32_t) -> nsresult,

    /* DOMString getFeatureSuggestedDriverVersion (in long aFeature); */
    pub getFeatureSuggestedDriverVersion: unsafe extern "C" fn (this: *const nsIGfxInfo, aFeature: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* DOMString getWebGLParameter (in DOMString aParam); */
    pub getWebGLParameter: unsafe extern "C" fn (this: *const nsIGfxInfo, aParam: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* [noscript,notxpcom] void GetData (); */
    pub GetData: unsafe extern "C" fn (this: *const nsIGfxInfo) -> libc::c_void,

    /* [implicit_jscontext] jsval getInfo (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getInfo: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getFeatureLog (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFeatureLog: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getFeatures (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFeatures: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getActiveCrashGuards (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getActiveCrashGuards: *const ::libc::c_void,

    /* boolean controlGPUProcessForXPCShell (in boolean aEnable); */
    pub controlGPUProcessForXPCShell: unsafe extern "C" fn (this: *const nsIGfxInfo, aEnable: bool, _retval: *mut bool) -> nsresult,

}


impl nsIGfxInfo {
    /* readonly attribute boolean D2DEnabled; */
    #[inline]
    pub unsafe fn get_D2DEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_D2DEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean DWriteEnabled; */
    #[inline]
    pub unsafe fn get_DWriteEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_DWriteEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean usingGPUProcess; */
    #[inline]
    pub unsafe fn get_usingGPUProcess(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_usingGPUProcess)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString DWriteVersion; */
    #[inline]
    pub unsafe fn get_DWriteVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_DWriteVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString cleartypeParameters; */
    #[inline]
    pub unsafe fn get_cleartypeParameters(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cleartypeParameters)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString ContentBackend; */
    #[inline]
    pub unsafe fn get_ContentBackend(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_ContentBackend)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean WebRenderEnabled; */
    #[inline]
    pub unsafe fn get_WebRenderEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_WebRenderEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDescription; */
    #[inline]
    pub unsafe fn get_adapterDescription(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDescription)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDescription2; */
    #[inline]
    pub unsafe fn get_adapterDescription2(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDescription2)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDriver; */
    #[inline]
    pub unsafe fn get_adapterDriver(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDriver)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDriver2; */
    #[inline]
    pub unsafe fn get_adapterDriver2(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDriver2)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterVendorID; */
    #[inline]
    pub unsafe fn get_adapterVendorID(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterVendorID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterVendorID2; */
    #[inline]
    pub unsafe fn get_adapterVendorID2(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterVendorID2)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDeviceID; */
    #[inline]
    pub unsafe fn get_adapterDeviceID(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDeviceID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDeviceID2; */
    #[inline]
    pub unsafe fn get_adapterDeviceID2(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDeviceID2)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterSubsysID; */
    #[inline]
    pub unsafe fn get_adapterSubsysID(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterSubsysID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterSubsysID2; */
    #[inline]
    pub unsafe fn get_adapterSubsysID2(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterSubsysID2)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterRAM; */
    #[inline]
    pub unsafe fn get_adapterRAM(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterRAM)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterRAM2; */
    #[inline]
    pub unsafe fn get_adapterRAM2(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterRAM2)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDriverVersion; */
    #[inline]
    pub unsafe fn get_adapterDriverVersion(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDriverVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDriverVersion2; */
    #[inline]
    pub unsafe fn get_adapterDriverVersion2(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDriverVersion2)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDriverDate; */
    #[inline]
    pub unsafe fn get_adapterDriverDate(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDriverDate)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString adapterDriverDate2; */
    #[inline]
    pub unsafe fn get_adapterDriverDate2(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_adapterDriverDate2)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isGPU2Active; */
    #[inline]
    pub unsafe fn get_isGPU2Active(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isGPU2Active)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval getMonitors (); */


    /* void getFailures (out unsigned long failureCount, [array, size_is (failureCount), optional] out long indices, [array, size_is (failureCount), retval] out string failures); */


    /* [noscript,notxpcom] void logFailure (in ACString failure); */
    #[inline]
    pub unsafe fn logFailure(&self, failure: &[u8]) -> () {
        let failure = nsCString::from(failure);
        let _retval = ((*self.vtable).logFailure)(self as *const _, &*failure);
        ()
    }

    /* long getFeatureStatus (in long aFeature, [optional] out ACString aFailureId); */
    #[inline]
    pub unsafe fn getFeatureStatus(&self, aFeature: libc::int32_t) -> Result<(nsCString, libc::int32_t), nsresult> {
        let mut aFailureId = nsCString::new();
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getFeatureStatus)(self as *const _, aFeature, &mut *aFailureId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFailureId, _retval))
    }

    /* DOMString getFeatureSuggestedDriverVersion (in long aFeature); */
    #[inline]
    pub unsafe fn getFeatureSuggestedDriverVersion(&self, aFeature: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getFeatureSuggestedDriverVersion)(self as *const _, aFeature, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* DOMString getWebGLParameter (in DOMString aParam); */
    #[inline]
    pub unsafe fn getWebGLParameter(&self, aParam: &[u16]) -> Result<nsString, nsresult> {
        let aParam = nsString::from(aParam);
        let mut _retval = nsString::new();
        match ((*self.vtable).getWebGLParameter)(self as *const _, &*aParam, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,notxpcom] void GetData (); */
    #[inline]
    pub unsafe fn GetData(&self, ) -> () {

        let _retval = ((*self.vtable).GetData)(self as *const _, );
        ()
    }

    /* [implicit_jscontext] jsval getInfo (); */


    /* [implicit_jscontext] jsval getFeatureLog (); */


    /* [implicit_jscontext] jsval getFeatures (); */


    /* [implicit_jscontext] jsval getActiveCrashGuards (); */


    /* boolean controlGPUProcessForXPCShell (in boolean aEnable); */
    #[inline]
    pub unsafe fn controlGPUProcessForXPCShell(&self, aEnable: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).controlGPUProcessForXPCShell)(self as *const _, aEnable, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


