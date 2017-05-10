//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStandaloneNativeMenu.idl
//


#[repr(C)]
pub struct nsIStandaloneNativeMenu {
    vtable: *const nsIStandaloneNativeMenuVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStandaloneNativeMenu {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7f7201eb, 0x510c, 0x4cef,
            [0xbd, 0xf0, 0x04, 0xa1, 0x5a, 0x7a, 0x4a, 0x8c])
    }
}

unsafe impl RefCounted for nsIStandaloneNativeMenu {
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
pub trait nsIStandaloneNativeMenuCoerce {
    fn coerce_from(v: &nsIStandaloneNativeMenu) -> &Self;
}

impl nsIStandaloneNativeMenuCoerce for nsIStandaloneNativeMenu {
    #[inline]
    fn coerce_from(v: &nsIStandaloneNativeMenu) -> &Self {
        v
    }
}

impl nsIStandaloneNativeMenu {
    #[inline]
    pub fn coerce<T: nsIStandaloneNativeMenuCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStandaloneNativeMenu {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStandaloneNativeMenuCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStandaloneNativeMenu) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStandaloneNativeMenuVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIDOMElement aDOMElement); */
    pub init: unsafe extern "C" fn (this: *const nsIStandaloneNativeMenu, aDOMElement: *const nsIDOMElement) -> nsresult,

    /* boolean menuWillOpen (); */
    pub menuWillOpen: unsafe extern "C" fn (this: *const nsIStandaloneNativeMenu, _retval: *mut bool) -> nsresult,

    /* [noscript] readonly attribute voidPtr nativeMenu; */
    pub get_nativeMenu: unsafe extern "C" fn (this: *const nsIStandaloneNativeMenu, aNativeMenu: *mut *const libc::c_void) -> nsresult,

    /* void activateNativeMenuItemAt (in AString anIndexString); */
    pub activateNativeMenuItemAt: unsafe extern "C" fn (this: *const nsIStandaloneNativeMenu, anIndexString: *const nsAString) -> nsresult,

    /* void forceUpdateNativeMenuAt (in AString anIndexString); */
    pub forceUpdateNativeMenuAt: unsafe extern "C" fn (this: *const nsIStandaloneNativeMenu, anIndexString: *const nsAString) -> nsresult,

}


impl nsIStandaloneNativeMenu {
    /* void init (in nsIDOMElement aDOMElement); */
    #[inline]
    pub unsafe fn init(&self, aDOMElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aDOMElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean menuWillOpen (); */
    #[inline]
    pub unsafe fn menuWillOpen(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).menuWillOpen)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute voidPtr nativeMenu; */
    #[inline]
    pub unsafe fn get_nativeMenu(&self, ) -> Result<*const libc::c_void, nsresult> {
        let mut _retval: *const libc::c_void = ::std::ptr::null();
        match ((*self.vtable).get_nativeMenu)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void activateNativeMenuItemAt (in AString anIndexString); */
    #[inline]
    pub unsafe fn activateNativeMenuItemAt(&self, anIndexString: &[u16]) -> Result<(), nsresult> {
        let anIndexString = nsString::from(anIndexString);
        match ((*self.vtable).activateNativeMenuItemAt)(self as *const _, &*anIndexString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceUpdateNativeMenuAt (in AString anIndexString); */
    #[inline]
    pub unsafe fn forceUpdateNativeMenuAt(&self, anIndexString: &[u16]) -> Result<(), nsresult> {
        let anIndexString = nsString::from(anIndexString);
        match ((*self.vtable).forceUpdateNativeMenuAt)(self as *const _, &*anIndexString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


