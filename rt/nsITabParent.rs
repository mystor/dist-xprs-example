//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITabParent.idl
//


#[repr(C)]
pub struct nsITabParent {
    vtable: *const nsITabParentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITabParent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8e49f7b0, 0x1f98, 0x4939,
            [0xbf, 0x91, 0xe9, 0xc3, 0x9c, 0xd5, 0x64, 0x34])
    }
}

unsafe impl RefCounted for nsITabParent {
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
pub trait nsITabParentCoerce {
    fn coerce_from(v: &nsITabParent) -> &Self;
}

impl nsITabParentCoerce for nsITabParent {
    #[inline]
    fn coerce_from(v: &nsITabParent) -> &Self {
        v
    }
}

impl nsITabParent {
    #[inline]
    pub fn coerce<T: nsITabParentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITabParent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITabParentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITabParent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITabParentVTable {
    pub __base: nsISupportsVTable,

    /* void getChildProcessOffset (out int32_t aCssX, out int32_t aCssY); */
    pub getChildProcessOffset: unsafe extern "C" fn (this: *const nsITabParent, aCssX: *mut int32_t, aCssY: *mut int32_t) -> nsresult,

    /* readonly attribute boolean useAsyncPanZoom; */
    pub get_useAsyncPanZoom: unsafe extern "C" fn (this: *const nsITabParent, aUseAsyncPanZoom: *mut bool) -> nsresult,

    /* attribute boolean docShellIsActive; */
    pub get_docShellIsActive: unsafe extern "C" fn (this: *const nsITabParent, aDocShellIsActive: *mut bool) -> nsresult,
    pub set_docShellIsActive: unsafe extern "C" fn (this: *const nsITabParent, aDocShellIsActive: bool) -> nsresult,

    /* [infallible] readonly attribute boolean isPrerendered; */
    pub get_isPrerendered: unsafe extern "C" fn (this: *const nsITabParent, aIsPrerendered: *mut bool) -> nsresult,

    /* void preserveLayers (in boolean aPreserveLayers); */
    pub preserveLayers: unsafe extern "C" fn (this: *const nsITabParent, aPreserveLayers: bool) -> nsresult,

    /* void suppressDisplayport (in bool aEnabled); */
    pub suppressDisplayport: unsafe extern "C" fn (this: *const nsITabParent, aEnabled: bool) -> nsresult,

    /* readonly attribute uint64_t tabId; */
    pub get_tabId: unsafe extern "C" fn (this: *const nsITabParent, aTabId: *mut uint64_t) -> nsresult,

    /* readonly attribute int32_t osPid; */
    pub get_osPid: unsafe extern "C" fn (this: *const nsITabParent, aOsPid: *mut int32_t) -> nsresult,

    /* void navigateByKey (in bool aForward, in bool aForDocumentNavigation); */
    pub navigateByKey: unsafe extern "C" fn (this: *const nsITabParent, aForward: bool, aForDocumentNavigation: bool) -> nsresult,

    /* readonly attribute boolean hasContentOpener; */
    pub get_hasContentOpener: unsafe extern "C" fn (this: *const nsITabParent, aHasContentOpener: *mut bool) -> nsresult,

    /* readonly attribute boolean hasPresented; */
    pub get_hasPresented: unsafe extern "C" fn (this: *const nsITabParent, aHasPresented: *mut bool) -> nsresult,

    /* void transmitPermissionsForPrincipal (in nsIPrincipal aPrincipal); */
    pub transmitPermissionsForPrincipal: unsafe extern "C" fn (this: *const nsITabParent, aPrincipal: *const nsIPrincipal) -> nsresult,

    /* readonly attribute boolean hasBeforeUnload; */
    pub get_hasBeforeUnload: unsafe extern "C" fn (this: *const nsITabParent, aHasBeforeUnload: *mut bool) -> nsresult,

}


impl nsITabParent {
    /* void getChildProcessOffset (out int32_t aCssX, out int32_t aCssY); */
    #[inline]
    pub unsafe fn getChildProcessOffset(&self, ) -> Result<(int32_t, int32_t), nsresult> {
        let mut aCssX: int32_t = ::std::mem::zeroed();
        let mut aCssY: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getChildProcessOffset)(self as *const _, &mut aCssX as *mut _, &mut aCssY as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCssX, aCssY))
    }

    /* readonly attribute boolean useAsyncPanZoom; */
    #[inline]
    pub unsafe fn get_useAsyncPanZoom(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_useAsyncPanZoom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean docShellIsActive; */
    #[inline]
    pub unsafe fn get_docShellIsActive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_docShellIsActive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_docShellIsActive(&self, aDocShellIsActive: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_docShellIsActive)(self as *const _, aDocShellIsActive) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean isPrerendered; */
    #[inline]
    pub unsafe fn get_isPrerendered(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPrerendered)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void preserveLayers (in boolean aPreserveLayers); */
    #[inline]
    pub unsafe fn preserveLayers(&self, aPreserveLayers: bool) -> Result<(), nsresult> {

        match ((*self.vtable).preserveLayers)(self as *const _, aPreserveLayers) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void suppressDisplayport (in bool aEnabled); */
    #[inline]
    pub unsafe fn suppressDisplayport(&self, aEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).suppressDisplayport)(self as *const _, aEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute uint64_t tabId; */
    #[inline]
    pub unsafe fn get_tabId(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tabId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int32_t osPid; */
    #[inline]
    pub unsafe fn get_osPid(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_osPid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void navigateByKey (in bool aForward, in bool aForDocumentNavigation); */
    #[inline]
    pub unsafe fn navigateByKey(&self, aForward: bool, aForDocumentNavigation: bool) -> Result<(), nsresult> {

        match ((*self.vtable).navigateByKey)(self as *const _, aForward, aForDocumentNavigation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasContentOpener; */
    #[inline]
    pub unsafe fn get_hasContentOpener(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasContentOpener)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean hasPresented; */
    #[inline]
    pub unsafe fn get_hasPresented(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasPresented)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void transmitPermissionsForPrincipal (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn transmitPermissionsForPrincipal(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).transmitPermissionsForPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasBeforeUnload; */
    #[inline]
    pub unsafe fn get_hasBeforeUnload(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasBeforeUnload)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


