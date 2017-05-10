//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIContainer.idl
//


pub mod imgIContainer_consts {
    pub const TYPE_RASTER: i64 = 0;
    pub const TYPE_VECTOR: i64 = 1;
    pub const FLAG_NONE: i64 = 0;
    pub const FLAG_SYNC_DECODE: i64 = 1;
    pub const FLAG_SYNC_DECODE_IF_FAST: i64 = 2;
    pub const FLAG_ASYNC_NOTIFY: i64 = 4;
    pub const FLAG_DECODE_NO_PREMULTIPLY_ALPHA: i64 = 8;
    pub const FLAG_DECODE_NO_COLORSPACE_CONVERSION: i64 = 16;
    pub const FLAG_CLAMP: i64 = 32;
    pub const FLAG_HIGH_QUALITY_SCALING: i64 = 64;
    pub const FLAG_WANT_DATA_SURFACE: i64 = 128;
    pub const FLAG_BYPASS_SURFACE_CACHE: i64 = 256;
    pub const FLAG_FORCE_PRESERVEASPECTRATIO_NONE: i64 = 512;
    pub const FLAG_FORCE_UNIFORM_SCALING: i64 = 1024;
    pub const DECODE_FLAGS_DEFAULT: i64 = 0;
    pub const FRAME_FIRST: i64 = 0;
    pub const FRAME_CURRENT: i64 = 1;
    pub const FRAME_MAX_VALUE: i64 = 1;
    pub const kNormalAnimMode: i64 = 0;
    pub const kDontAnimMode: i64 = 1;
    pub const kLoopOnceAnimMode: i64 = 2;
}


#[repr(C)]
pub struct imgIContainer {
    vtable: *const imgIContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgIContainer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa8dbee24, 0xff86, 0x4755,
            [0xb4, 0x0e, 0x51, 0x17, 0x5c, 0xaf, 0x31, 0xaf])
    }
}

unsafe impl RefCounted for imgIContainer {
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
pub trait imgIContainerCoerce {
    fn coerce_from(v: &imgIContainer) -> &Self;
}

impl imgIContainerCoerce for imgIContainer {
    #[inline]
    fn coerce_from(v: &imgIContainer) -> &Self {
        v
    }
}

impl imgIContainer {
    #[inline]
    pub fn coerce<T: imgIContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgIContainer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> imgIContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIContainer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgIContainerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute int32_t width; */
    pub get_width: unsafe extern "C" fn (this: *const imgIContainer, aWidth: *mut int32_t) -> nsresult,

    /* readonly attribute int32_t height; */
    pub get_height: unsafe extern "C" fn (this: *const imgIContainer, aHeight: *mut int32_t) -> nsresult,

    /* [noscript] readonly attribute nsSize intrinsicSize; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_intrinsicSize: *const ::libc::c_void,

    /* [noscript] readonly attribute nsSize intrinsicRatio; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_intrinsicRatio: *const ::libc::c_void,

    /* [nostdcall,notxpcom] nsIntSizeByVal optimalImageSizeForDest ([const] in gfxSize aDest, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, in uint32_t aFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub optimalImageSizeForDest: *const ::libc::c_void,

    /* [infallible] readonly attribute unsigned short type; */
    pub get_type_: unsafe extern "C" fn (this: *const imgIContainer, aType: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute boolean animated; */
    pub get_animated: unsafe extern "C" fn (this: *const imgIContainer, aAnimated: *mut bool) -> nsresult,

    /* [noscript,notxpcom] TempRefSourceSurface getFrame (in uint32_t aWhichFrame, in uint32_t aFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFrame: *const ::libc::c_void,

    /* [noscript,notxpcom] TempRefSourceSurface getFrameAtSize ([const] in nsIntSize aSize, in uint32_t aWhichFrame, in uint32_t aFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFrameAtSize: *const ::libc::c_void,

    /* [noscript,notxpcom] boolean willDrawOpaqueNow (); */
    pub willDrawOpaqueNow: unsafe extern "C" fn (this: *const imgIContainer) -> bool,

    /* [noscript,notxpcom] boolean isImageContainerAvailable (in LayerManager aManager, in uint32_t aFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub isImageContainerAvailable: *const ::libc::c_void,

    /* [noscript,notxpcom] TempRefImageContainer getImageContainer (in LayerManager aManager, in uint32_t aFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub getImageContainer: *const ::libc::c_void,

    /* [noscript,notxpcom] DrawResult draw (in gfxContext aContext, [const] in nsIntSize aSize, [const] in ImageRegion aRegion, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, [const] in MaybeSVGImageContext aSVGContext, in uint32_t aFlags, in float aOpacity); */
    /// Unable to call function as its signature contains a non-rust type
    pub draw: *const ::libc::c_void,

    /* [noscript] void startDecoding (in uint32_t aFlags); */
    pub startDecoding: unsafe extern "C" fn (this: *const imgIContainer, aFlags: uint32_t) -> nsresult,

    /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags); */
    pub startDecodingWithResult: unsafe extern "C" fn (this: *const imgIContainer, aFlags: uint32_t) -> bool,

    /* [noscript] void requestDecodeForSize ([const] in nsIntSize aSize, in uint32_t aFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub requestDecodeForSize: *const ::libc::c_void,

    /* void lockImage (); */
    pub lockImage: unsafe extern "C" fn (this: *const imgIContainer) -> nsresult,

    /* void unlockImage (); */
    pub unlockImage: unsafe extern "C" fn (this: *const imgIContainer) -> nsresult,

    /* void requestDiscard (); */
    pub requestDiscard: unsafe extern "C" fn (this: *const imgIContainer) -> nsresult,

    /* [notxpcom] void requestRefresh ([const] in TimeStamp aTime); */
    /// Unable to call function as its signature contains a non-rust type
    pub requestRefresh: *const ::libc::c_void,

    /* attribute unsigned short animationMode; */
    pub get_animationMode: unsafe extern "C" fn (this: *const imgIContainer, aAnimationMode: *mut libc::uint16_t) -> nsresult,
    pub set_animationMode: unsafe extern "C" fn (this: *const imgIContainer, aAnimationMode: libc::uint16_t) -> nsresult,

    /* void resetAnimation (); */
    pub resetAnimation: unsafe extern "C" fn (this: *const imgIContainer) -> nsresult,

    /* [notxpcom] float getFrameIndex (in uint32_t aWhichFrame); */
    pub getFrameIndex: unsafe extern "C" fn (this: *const imgIContainer, aWhichFrame: uint32_t) -> libc::c_float,

    /* [notxpcom] Orientation getOrientation (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getOrientation: *const ::libc::c_void,

    /* [notxpcom] int32_t getFirstFrameDelay (); */
    pub getFirstFrameDelay: unsafe extern "C" fn (this: *const imgIContainer) -> int32_t,

    /* [notxpcom] void setAnimationStartTime ([const] in TimeStamp aTime); */
    /// Unable to call function as its signature contains a non-rust type
    pub setAnimationStartTime: *const ::libc::c_void,

    /* [notxpcom] nsIntRectByVal getImageSpaceInvalidationRect ([const] in nsIntRect aRect); */
    /// Unable to call function as its signature contains a non-rust type
    pub getImageSpaceInvalidationRect: *const ::libc::c_void,

    /* [nostdcall,notxpcom] TempRefImgIContainer unwrap (); */
    /// Unable to call function as its signature contains a non-rust type
    pub unwrap: *const ::libc::c_void,

    /* [noscript,notxpcom] void propagateUseCounters (in nsIDocument aDocument); */
    /// Unable to call function as its signature contains a non-rust type
    pub propagateUseCounters: *const ::libc::c_void,

}


impl imgIContainer {
    /* readonly attribute int32_t width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int32_t height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_height)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute nsSize intrinsicSize; */


    /* [noscript] readonly attribute nsSize intrinsicRatio; */


    /* [nostdcall,notxpcom] nsIntSizeByVal optimalImageSizeForDest ([const] in gfxSize aDest, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, in uint32_t aFlags); */


    /* [infallible] readonly attribute unsigned short type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean animated; */
    #[inline]
    pub unsafe fn get_animated(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_animated)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,notxpcom] TempRefSourceSurface getFrame (in uint32_t aWhichFrame, in uint32_t aFlags); */


    /* [noscript,notxpcom] TempRefSourceSurface getFrameAtSize ([const] in nsIntSize aSize, in uint32_t aWhichFrame, in uint32_t aFlags); */


    /* [noscript,notxpcom] boolean willDrawOpaqueNow (); */
    #[inline]
    pub unsafe fn willDrawOpaqueNow(&self, ) -> bool {

        let _retval = ((*self.vtable).willDrawOpaqueNow)(self as *const _, );
        _retval
    }

    /* [noscript,notxpcom] boolean isImageContainerAvailable (in LayerManager aManager, in uint32_t aFlags); */


    /* [noscript,notxpcom] TempRefImageContainer getImageContainer (in LayerManager aManager, in uint32_t aFlags); */


    /* [noscript,notxpcom] DrawResult draw (in gfxContext aContext, [const] in nsIntSize aSize, [const] in ImageRegion aRegion, in uint32_t aWhichFrame, in SamplingFilter aSamplingFilter, [const] in MaybeSVGImageContext aSVGContext, in uint32_t aFlags, in float aOpacity); */


    /* [noscript] void startDecoding (in uint32_t aFlags); */
    #[inline]
    pub unsafe fn startDecoding(&self, aFlags: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).startDecoding)(self as *const _, aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags); */
    #[inline]
    pub unsafe fn startDecodingWithResult(&self, aFlags: uint32_t) -> bool {

        let _retval = ((*self.vtable).startDecodingWithResult)(self as *const _, aFlags);
        _retval
    }

    /* [noscript] void requestDecodeForSize ([const] in nsIntSize aSize, in uint32_t aFlags); */


    /* void lockImage (); */
    #[inline]
    pub unsafe fn lockImage(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).lockImage)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unlockImage (); */
    #[inline]
    pub unsafe fn unlockImage(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unlockImage)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestDiscard (); */
    #[inline]
    pub unsafe fn requestDiscard(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).requestDiscard)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] void requestRefresh ([const] in TimeStamp aTime); */


    /* attribute unsigned short animationMode; */
    #[inline]
    pub unsafe fn get_animationMode(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_animationMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_animationMode(&self, aAnimationMode: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_animationMode)(self as *const _, aAnimationMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resetAnimation (); */
    #[inline]
    pub unsafe fn resetAnimation(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetAnimation)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] float getFrameIndex (in uint32_t aWhichFrame); */
    #[inline]
    pub unsafe fn getFrameIndex(&self, aWhichFrame: uint32_t) -> libc::c_float {

        let _retval = ((*self.vtable).getFrameIndex)(self as *const _, aWhichFrame);
        _retval
    }

    /* [notxpcom] Orientation getOrientation (); */


    /* [notxpcom] int32_t getFirstFrameDelay (); */
    #[inline]
    pub unsafe fn getFirstFrameDelay(&self, ) -> int32_t {

        let _retval = ((*self.vtable).getFirstFrameDelay)(self as *const _, );
        _retval
    }

    /* [notxpcom] void setAnimationStartTime ([const] in TimeStamp aTime); */


    /* [notxpcom] nsIntRectByVal getImageSpaceInvalidationRect ([const] in nsIntRect aRect); */


    /* [nostdcall,notxpcom] TempRefImgIContainer unwrap (); */


    /* [noscript,notxpcom] void propagateUseCounters (in nsIDocument aDocument); */


}


