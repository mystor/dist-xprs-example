//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginManagerStorage.idl
//


#[repr(C)]
pub struct nsILoginManagerStorage {
    vtable: *const nsILoginManagerStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoginManagerStorage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5df81a93, 0x25e6, 0x4b45,
            [0xa6, 0x96, 0x08, 0x94, 0x79, 0xe1, 0x5c, 0x7d])
    }
}

unsafe impl RefCounted for nsILoginManagerStorage {
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
pub trait nsILoginManagerStorageCoerce {
    fn coerce_from(v: &nsILoginManagerStorage) -> &Self;
}

impl nsILoginManagerStorageCoerce for nsILoginManagerStorage {
    #[inline]
    fn coerce_from(v: &nsILoginManagerStorage) -> &Self {
        v
    }
}

impl nsILoginManagerStorage {
    #[inline]
    pub fn coerce<T: nsILoginManagerStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoginManagerStorage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoginManagerStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginManagerStorage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoginManagerStorageVTable {
    pub __base: nsISupportsVTable,

    /* jsval initialize (); */
    /// Unable to call function as its signature contains a non-rust type
    pub initialize: *const ::libc::c_void,

    /* jsval terminate (); */
    /// Unable to call function as its signature contains a non-rust type
    pub terminate: *const ::libc::c_void,

    /* nsILoginInfo addLogin (in nsILoginInfo aLogin); */
    pub addLogin: unsafe extern "C" fn (this: *const nsILoginManagerStorage, aLogin: *const nsILoginInfo, _retval: *mut *const nsILoginInfo) -> nsresult,

    /* void removeLogin (in nsILoginInfo aLogin); */
    pub removeLogin: unsafe extern "C" fn (this: *const nsILoginManagerStorage, aLogin: *const nsILoginInfo) -> nsresult,

    /* void modifyLogin (in nsILoginInfo oldLogin, in nsISupports newLoginData); */
    pub modifyLogin: unsafe extern "C" fn (this: *const nsILoginManagerStorage, oldLogin: *const nsILoginInfo, newLoginData: *const nsISupports) -> nsresult,

    /* void removeAllLogins (); */
    pub removeAllLogins: unsafe extern "C" fn (this: *const nsILoginManagerStorage) -> nsresult,

    /* void getAllLogins ([optional] out unsigned long count, [array, size_is (count), retval] out nsILoginInfo logins); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAllLogins: *const ::libc::c_void,

    /* void searchLogins (out unsigned long count, in nsIPropertyBag matchData, [array, size_is (count), retval] out nsILoginInfo logins); */
    /// Unable to call function as its signature contains a non-rust type
    pub searchLogins: *const ::libc::c_void,

    /* void findLogins (out unsigned long count, in AString aHostname, in AString aActionURL, in AString aHttpRealm, [array, size_is (count), retval] out nsILoginInfo logins); */
    /// Unable to call function as its signature contains a non-rust type
    pub findLogins: *const ::libc::c_void,

    /* unsigned long countLogins (in AString aHostname, in AString aActionURL, in AString aHttpRealm); */
    pub countLogins: unsafe extern "C" fn (this: *const nsILoginManagerStorage, aHostname: *const nsAString, aActionURL: *const nsAString, aHttpRealm: *const nsAString, _retval: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean uiBusy; */
    pub get_uiBusy: unsafe extern "C" fn (this: *const nsILoginManagerStorage, aUiBusy: *mut bool) -> nsresult,

    /* readonly attribute boolean isLoggedIn; */
    pub get_isLoggedIn: unsafe extern "C" fn (this: *const nsILoginManagerStorage, aIsLoggedIn: *mut bool) -> nsresult,

}


impl nsILoginManagerStorage {
    /* jsval initialize (); */


    /* jsval terminate (); */


    /* nsILoginInfo addLogin (in nsILoginInfo aLogin); */
    #[inline]
    pub unsafe fn addLogin(&self, aLogin: Option<&nsILoginInfo>) -> Result<Option<RefPtr<nsILoginInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).addLogin)(self as *const _, aLogin.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removeLogin (in nsILoginInfo aLogin); */
    #[inline]
    pub unsafe fn removeLogin(&self, aLogin: Option<&nsILoginInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).removeLogin)(self as *const _, aLogin.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void modifyLogin (in nsILoginInfo oldLogin, in nsISupports newLoginData); */
    #[inline]
    pub unsafe fn modifyLogin(&self, oldLogin: Option<&nsILoginInfo>, newLoginData: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).modifyLogin)(self as *const _, oldLogin.map_or(::std::ptr::null(), |x| x as *const _), newLoginData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAllLogins (); */
    #[inline]
    pub unsafe fn removeAllLogins(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllLogins)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getAllLogins ([optional] out unsigned long count, [array, size_is (count), retval] out nsILoginInfo logins); */


    /* void searchLogins (out unsigned long count, in nsIPropertyBag matchData, [array, size_is (count), retval] out nsILoginInfo logins); */


    /* void findLogins (out unsigned long count, in AString aHostname, in AString aActionURL, in AString aHttpRealm, [array, size_is (count), retval] out nsILoginInfo logins); */


    /* unsigned long countLogins (in AString aHostname, in AString aActionURL, in AString aHttpRealm); */
    #[inline]
    pub unsafe fn countLogins(&self, aHostname: &[u16], aActionURL: &[u16], aHttpRealm: &[u16]) -> Result<libc::uint32_t, nsresult> {
        let aHostname = nsString::from(aHostname);
        let aActionURL = nsString::from(aActionURL);
        let aHttpRealm = nsString::from(aHttpRealm);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).countLogins)(self as *const _, &*aHostname, &*aActionURL, &*aHttpRealm, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean uiBusy; */
    #[inline]
    pub unsafe fn get_uiBusy(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_uiBusy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isLoggedIn; */
    #[inline]
    pub unsafe fn get_isLoggedIn(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isLoggedIn)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


