//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginManagerPrompter.idl
//


#[repr(C)]
pub struct nsILoginManagerPrompter {
    vtable: *const nsILoginManagerPrompterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoginManagerPrompter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x425f73b9, 0xb2db, 0x4e8a,
            [0x88, 0xc5, 0x9a, 0xc2, 0x51, 0x29, 0x34, 0xce])
    }
}

unsafe impl RefCounted for nsILoginManagerPrompter {
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
pub trait nsILoginManagerPrompterCoerce {
    fn coerce_from(v: &nsILoginManagerPrompter) -> &Self;
}

impl nsILoginManagerPrompterCoerce for nsILoginManagerPrompter {
    #[inline]
    fn coerce_from(v: &nsILoginManagerPrompter) -> &Self {
        v
    }
}

impl nsILoginManagerPrompter {
    #[inline]
    pub fn coerce<T: nsILoginManagerPrompterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoginManagerPrompter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoginManagerPrompterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginManagerPrompter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoginManagerPrompterVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIDOMWindow aWindow); */
    pub init: unsafe extern "C" fn (this: *const nsILoginManagerPrompter, aWindow: *const nsIDOMWindow) -> nsresult,

    /* attribute nsIDOMElement browser; */
    pub get_browser: unsafe extern "C" fn (this: *const nsILoginManagerPrompter, aBrowser: *mut *const nsIDOMElement) -> nsresult,
    pub set_browser: unsafe extern "C" fn (this: *const nsILoginManagerPrompter, aBrowser: *const nsIDOMElement) -> nsresult,

    /* attribute nsIDOMWindow opener; */
    pub get_opener: unsafe extern "C" fn (this: *const nsILoginManagerPrompter, aOpener: *mut *const nsIDOMWindow) -> nsresult,
    pub set_opener: unsafe extern "C" fn (this: *const nsILoginManagerPrompter, aOpener: *const nsIDOMWindow) -> nsresult,

    /* void promptToSavePassword (in nsILoginInfo aLogin); */
    pub promptToSavePassword: unsafe extern "C" fn (this: *const nsILoginManagerPrompter, aLogin: *const nsILoginInfo) -> nsresult,

    /* void promptToChangePassword (in nsILoginInfo aOldLogin, in nsILoginInfo aNewLogin); */
    pub promptToChangePassword: unsafe extern "C" fn (this: *const nsILoginManagerPrompter, aOldLogin: *const nsILoginInfo, aNewLogin: *const nsILoginInfo) -> nsresult,

    /* void promptToChangePasswordWithUsernames ([array, size_is (count)] in nsILoginInfo logins, in uint32_t count, in nsILoginInfo aNewLogin); */
    /// Unable to call function as its signature contains a non-rust type
    pub promptToChangePasswordWithUsernames: *const ::libc::c_void,

}


impl nsILoginManagerPrompter {
    /* void init (in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn init(&self, aWindow: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIDOMElement browser; */
    #[inline]
    pub unsafe fn get_browser(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_browser)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_browser(&self, aBrowser: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).set_browser)(self as *const _, aBrowser.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIDOMWindow opener; */
    #[inline]
    pub unsafe fn get_opener(&self, ) -> Result<Option<RefPtr<nsIDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_opener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_opener(&self, aOpener: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).set_opener)(self as *const _, aOpener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void promptToSavePassword (in nsILoginInfo aLogin); */
    #[inline]
    pub unsafe fn promptToSavePassword(&self, aLogin: Option<&nsILoginInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).promptToSavePassword)(self as *const _, aLogin.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void promptToChangePassword (in nsILoginInfo aOldLogin, in nsILoginInfo aNewLogin); */
    #[inline]
    pub unsafe fn promptToChangePassword(&self, aOldLogin: Option<&nsILoginInfo>, aNewLogin: Option<&nsILoginInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).promptToChangePassword)(self as *const _, aOldLogin.map_or(::std::ptr::null(), |x| x as *const _), aNewLogin.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void promptToChangePasswordWithUsernames ([array, size_is (count)] in nsILoginInfo logins, in uint32_t count, in nsILoginInfo aNewLogin); */


}


