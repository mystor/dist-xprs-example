//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMacDockSupport.idl
//


#[repr(C)]
pub struct nsIMacDockSupport {
    vtable: *const nsIMacDockSupportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMacDockSupport {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8be66b0c, 0x5f71, 0x4b74,
            [0x98, 0xcf, 0x6c, 0x25, 0x51, 0xb9, 0x99, 0xb1])
    }
}

unsafe impl RefCounted for nsIMacDockSupport {
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
pub trait nsIMacDockSupportCoerce {
    fn coerce_from(v: &nsIMacDockSupport) -> &Self;
}

impl nsIMacDockSupportCoerce for nsIMacDockSupport {
    #[inline]
    fn coerce_from(v: &nsIMacDockSupport) -> &Self {
        v
    }
}

impl nsIMacDockSupport {
    #[inline]
    pub fn coerce<T: nsIMacDockSupportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMacDockSupport {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMacDockSupportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMacDockSupport) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMacDockSupportVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIStandaloneNativeMenu dockMenu; */
    pub get_dockMenu: unsafe extern "C" fn (this: *const nsIMacDockSupport, aDockMenu: *mut *const nsIStandaloneNativeMenu) -> nsresult,
    pub set_dockMenu: unsafe extern "C" fn (this: *const nsIMacDockSupport, aDockMenu: *const nsIStandaloneNativeMenu) -> nsresult,

    /* void activateApplication (in boolean aIgnoreOtherApplications); */
    pub activateApplication: unsafe extern "C" fn (this: *const nsIMacDockSupport, aIgnoreOtherApplications: bool) -> nsresult,

    /* attribute AString badgeText; */
    pub get_badgeText: unsafe extern "C" fn (this: *const nsIMacDockSupport, aBadgeText: *mut nsAString) -> nsresult,
    pub set_badgeText: unsafe extern "C" fn (this: *const nsIMacDockSupport, aBadgeText: *const nsAString) -> nsresult,

}


impl nsIMacDockSupport {
    /* attribute nsIStandaloneNativeMenu dockMenu; */
    #[inline]
    pub unsafe fn get_dockMenu(&self, ) -> Result<Option<RefPtr<nsIStandaloneNativeMenu>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_dockMenu)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_dockMenu(&self, aDockMenu: Option<&nsIStandaloneNativeMenu>) -> Result<(), nsresult> {

        match ((*self.vtable).set_dockMenu)(self as *const _, aDockMenu.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void activateApplication (in boolean aIgnoreOtherApplications); */
    #[inline]
    pub unsafe fn activateApplication(&self, aIgnoreOtherApplications: bool) -> Result<(), nsresult> {

        match ((*self.vtable).activateApplication)(self as *const _, aIgnoreOtherApplications) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString badgeText; */
    #[inline]
    pub unsafe fn get_badgeText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_badgeText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_badgeText(&self, aBadgeText: &[u16]) -> Result<(), nsresult> {
        let aBadgeText = nsString::from(aBadgeText);
        match ((*self.vtable).set_badgeText)(self as *const _, &*aBadgeText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


