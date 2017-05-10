//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginManager.idl
//


#[repr(C)]
pub struct nsILoginManager {
    vtable: *const nsILoginManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoginManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x38c7f6af, 0x7df9, 0x49c7,
            [0xb5, 0x58, 0x27, 0x76, 0xb2, 0x4e, 0x6c, 0xc1])
    }
}

unsafe impl RefCounted for nsILoginManager {
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
pub trait nsILoginManagerCoerce {
    fn coerce_from(v: &nsILoginManager) -> &Self;
}

impl nsILoginManagerCoerce for nsILoginManager {
    #[inline]
    fn coerce_from(v: &nsILoginManager) -> &Self {
        v
    }
}

impl nsILoginManager {
    #[inline]
    pub fn coerce<T: nsILoginManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoginManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoginManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoginManagerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute jsval initializationPromise; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_initializationPromise: *const ::libc::c_void,

    /* nsILoginInfo addLogin (in nsILoginInfo aLogin); */
    pub addLogin: unsafe extern "C" fn (this: *const nsILoginManager, aLogin: *const nsILoginInfo, _retval: *mut *const nsILoginInfo) -> nsresult,

    /* void removeLogin (in nsILoginInfo aLogin); */
    pub removeLogin: unsafe extern "C" fn (this: *const nsILoginManager, aLogin: *const nsILoginInfo) -> nsresult,

    /* void modifyLogin (in nsILoginInfo oldLogin, in nsISupports newLoginData); */
    pub modifyLogin: unsafe extern "C" fn (this: *const nsILoginManager, oldLogin: *const nsILoginInfo, newLoginData: *const nsISupports) -> nsresult,

    /* void removeAllLogins (); */
    pub removeAllLogins: unsafe extern "C" fn (this: *const nsILoginManager) -> nsresult,

    /* void getAllLogins ([optional] out unsigned long count, [array, size_is (count), retval] out nsILoginInfo logins); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAllLogins: *const ::libc::c_void,

    /* void getAllDisabledHosts ([optional] out unsigned long count, [array, size_is (count), retval] out wstring hostnames); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAllDisabledHosts: *const ::libc::c_void,

    /* boolean getLoginSavingEnabled (in AString aHost); */
    pub getLoginSavingEnabled: unsafe extern "C" fn (this: *const nsILoginManager, aHost: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void setLoginSavingEnabled (in AString aHost, in boolean isEnabled); */
    pub setLoginSavingEnabled: unsafe extern "C" fn (this: *const nsILoginManager, aHost: *const nsAString, isEnabled: bool) -> nsresult,

    /* void findLogins (out unsigned long count, in AString aHostname, in AString aActionURL, in AString aHttpRealm, [array, size_is (count), retval] out nsILoginInfo logins); */
    /// Unable to call function as its signature contains a non-rust type
    pub findLogins: *const ::libc::c_void,

    /* unsigned long countLogins (in AString aHostname, in AString aActionURL, in AString aHttpRealm); */
    pub countLogins: unsafe extern "C" fn (this: *const nsILoginManager, aHostname: *const nsAString, aActionURL: *const nsAString, aHttpRealm: *const nsAString, _retval: *mut libc::uint32_t) -> nsresult,

    /* void autoCompleteSearchAsync (in AString aSearchString, in nsIAutoCompleteResult aPreviousResult, in nsIDOMHTMLInputElement aElement, in nsIFormAutoCompleteObserver aListener); */
    pub autoCompleteSearchAsync: unsafe extern "C" fn (this: *const nsILoginManager, aSearchString: *const nsAString, aPreviousResult: *const nsIAutoCompleteResult, aElement: *const nsIDOMHTMLInputElement, aListener: *const nsIFormAutoCompleteObserver) -> nsresult,

    /* void stopSearch (); */
    pub stopSearch: unsafe extern "C" fn (this: *const nsILoginManager) -> nsresult,

    /* void searchLogins (out unsigned long count, in nsIPropertyBag matchData, [array, size_is (count), retval] out nsILoginInfo logins); */
    /// Unable to call function as its signature contains a non-rust type
    pub searchLogins: *const ::libc::c_void,

    /* readonly attribute boolean uiBusy; */
    pub get_uiBusy: unsafe extern "C" fn (this: *const nsILoginManager, aUiBusy: *mut bool) -> nsresult,

    /* readonly attribute boolean isLoggedIn; */
    pub get_isLoggedIn: unsafe extern "C" fn (this: *const nsILoginManager, aIsLoggedIn: *mut bool) -> nsresult,

}


impl nsILoginManager {
    /* readonly attribute jsval initializationPromise; */


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


    /* void getAllDisabledHosts ([optional] out unsigned long count, [array, size_is (count), retval] out wstring hostnames); */


    /* boolean getLoginSavingEnabled (in AString aHost); */
    #[inline]
    pub unsafe fn getLoginSavingEnabled(&self, aHost: &[u16]) -> Result<bool, nsresult> {
        let aHost = nsString::from(aHost);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getLoginSavingEnabled)(self as *const _, &*aHost, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setLoginSavingEnabled (in AString aHost, in boolean isEnabled); */
    #[inline]
    pub unsafe fn setLoginSavingEnabled(&self, aHost: &[u16], isEnabled: bool) -> Result<(), nsresult> {
        let aHost = nsString::from(aHost);
        match ((*self.vtable).setLoginSavingEnabled)(self as *const _, &*aHost, isEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* void autoCompleteSearchAsync (in AString aSearchString, in nsIAutoCompleteResult aPreviousResult, in nsIDOMHTMLInputElement aElement, in nsIFormAutoCompleteObserver aListener); */
    #[inline]
    pub unsafe fn autoCompleteSearchAsync(&self, aSearchString: &[u16], aPreviousResult: Option<&nsIAutoCompleteResult>, aElement: Option<&nsIDOMHTMLInputElement>, aListener: Option<&nsIFormAutoCompleteObserver>) -> Result<(), nsresult> {
        let aSearchString = nsString::from(aSearchString);
        match ((*self.vtable).autoCompleteSearchAsync)(self as *const _, &*aSearchString, aPreviousResult.map_or(::std::ptr::null(), |x| x as *const _), aElement.map_or(::std::ptr::null(), |x| x as *const _), aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopSearch (); */
    #[inline]
    pub unsafe fn stopSearch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopSearch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void searchLogins (out unsigned long count, in nsIPropertyBag matchData, [array, size_is (count), retval] out nsILoginInfo logins); */


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


