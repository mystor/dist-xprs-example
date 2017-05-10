//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPackageKitService.idl
//


pub mod nsIPackageKitService_consts {
    pub const PK_INSTALL_PACKAGE_NAMES: i64 = 0;
    pub const PK_INSTALL_MIME_TYPES: i64 = 1;
    pub const PK_INSTALL_FONTCONFIG_RESOURCES: i64 = 2;
    pub const PK_INSTALL_GSTREAMER_RESOURCES: i64 = 3;
    pub const PK_INSTALL_METHOD_COUNT: i64 = 4;
}


#[repr(C)]
pub struct nsIPackageKitService {
    vtable: *const nsIPackageKitServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPackageKitService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x89bb04f6, 0xce2a, 0x11e3,
            [0x8f, 0x4f, 0x60, 0xa4, 0x4c, 0x71, 0x70, 0x42])
    }
}

unsafe impl RefCounted for nsIPackageKitService {
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
pub trait nsIPackageKitServiceCoerce {
    fn coerce_from(v: &nsIPackageKitService) -> &Self;
}

impl nsIPackageKitServiceCoerce for nsIPackageKitService {
    #[inline]
    fn coerce_from(v: &nsIPackageKitService) -> &Self {
        v
    }
}

impl nsIPackageKitService {
    #[inline]
    pub fn coerce<T: nsIPackageKitServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPackageKitService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPackageKitServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPackageKitService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPackageKitServiceVTable {
    pub __base: nsISupportsVTable,

    /* void installPackages (in unsigned long packageKitMethod, in nsIArray packageArray, in nsIObserver observer); */
    pub installPackages: unsafe extern "C" fn (this: *const nsIPackageKitService, packageKitMethod: libc::uint32_t, packageArray: *const nsIArray, observer: *const nsIObserver) -> nsresult,

}


impl nsIPackageKitService {
    /* void installPackages (in unsigned long packageKitMethod, in nsIArray packageArray, in nsIObserver observer); */
    #[inline]
    pub unsafe fn installPackages(&self, packageKitMethod: libc::uint32_t, packageArray: Option<&nsIArray>, observer: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).installPackages)(self as *const _, packageKitMethod, packageArray.map_or(::std::ptr::null(), |x| x as *const _), observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


