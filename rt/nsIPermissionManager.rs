//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPermissionManager.idl
//


pub mod nsIPermissionManager_consts {
    pub const UNKNOWN_ACTION: i64 = 0;
    pub const ALLOW_ACTION: i64 = 1;
    pub const DENY_ACTION: i64 = 2;
    pub const PROMPT_ACTION: i64 = 3;
    pub const EXPIRE_NEVER: i64 = 0;
    pub const EXPIRE_SESSION: i64 = 1;
    pub const EXPIRE_TIME: i64 = 2;
}


#[repr(C)]
pub struct nsIPermissionManager {
    vtable: *const nsIPermissionManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPermissionManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4dcb3851, 0xeba2, 0x4e42,
            [0xb2, 0x36, 0x82, 0xd2, 0x59, 0x6f, 0xca, 0x22])
    }
}

unsafe impl RefCounted for nsIPermissionManager {
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
pub trait nsIPermissionManagerCoerce {
    fn coerce_from(v: &nsIPermissionManager) -> &Self;
}

impl nsIPermissionManagerCoerce for nsIPermissionManager {
    #[inline]
    fn coerce_from(v: &nsIPermissionManager) -> &Self {
        v
    }
}

impl nsIPermissionManager {
    #[inline]
    pub fn coerce<T: nsIPermissionManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPermissionManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPermissionManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPermissionManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPermissionManagerVTable {
    pub __base: nsISupportsVTable,

    /* void add (in nsIURI uri, in string type, in uint32_t permission, [optional] in uint32_t expireType, [optional] in int64_t expireTime); */
    pub add: unsafe extern "C" fn (this: *const nsIPermissionManager, uri: *const nsIURI, type_: *const libc::c_char, permission: uint32_t, expireType: uint32_t, expireTime: int64_t) -> nsresult,

    /* nsISimpleEnumerator getAllForURI (in nsIURI uri); */
    pub getAllForURI: unsafe extern "C" fn (this: *const nsIPermissionManager, uri: *const nsIURI, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void addFromPrincipal (in nsIPrincipal principal, in string typed, in uint32_t permission, [optional] in uint32_t expireType, [optional] in int64_t expireTime); */
    pub addFromPrincipal: unsafe extern "C" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, typed: *const libc::c_char, permission: uint32_t, expireType: uint32_t, expireTime: int64_t) -> nsresult,

    /* void remove (in nsIURI uri, in string type); */
    pub remove: unsafe extern "C" fn (this: *const nsIPermissionManager, uri: *const nsIURI, type_: *const libc::c_char) -> nsresult,

    /* void removeFromPrincipal (in nsIPrincipal principal, in string type); */
    pub removeFromPrincipal: unsafe extern "C" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const libc::c_char) -> nsresult,

    /* void removePermission (in nsIPermission perm); */
    pub removePermission: unsafe extern "C" fn (this: *const nsIPermissionManager, perm: *const nsIPermission) -> nsresult,

    /* void removeAll (); */
    pub removeAll: unsafe extern "C" fn (this: *const nsIPermissionManager) -> nsresult,

    /* void removeAllSince (in int64_t since); */
    pub removeAllSince: unsafe extern "C" fn (this: *const nsIPermissionManager, since: int64_t) -> nsresult,

    /* uint32_t testPermission (in nsIURI uri, in string type); */
    pub testPermission: unsafe extern "C" fn (this: *const nsIPermissionManager, uri: *const nsIURI, type_: *const libc::c_char, _retval: *mut uint32_t) -> nsresult,

    /* uint32_t testPermissionFromPrincipal (in nsIPrincipal principal, in string type); */
    pub testPermissionFromPrincipal: unsafe extern "C" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const libc::c_char, _retval: *mut uint32_t) -> nsresult,

    /* uint32_t testPermissionFromWindow (in mozIDOMWindow window, in string type); */
    pub testPermissionFromWindow: unsafe extern "C" fn (this: *const nsIPermissionManager, window: *const mozIDOMWindow, type_: *const libc::c_char, _retval: *mut uint32_t) -> nsresult,

    /* uint32_t testExactPermission (in nsIURI uri, in string type); */
    pub testExactPermission: unsafe extern "C" fn (this: *const nsIPermissionManager, uri: *const nsIURI, type_: *const libc::c_char, _retval: *mut uint32_t) -> nsresult,

    /* uint32_t testExactPermissionFromPrincipal (in nsIPrincipal principal, in string type); */
    pub testExactPermissionFromPrincipal: unsafe extern "C" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const libc::c_char, _retval: *mut uint32_t) -> nsresult,

    /* uint32_t testExactPermanentPermission (in nsIPrincipal principal, in string type); */
    pub testExactPermanentPermission: unsafe extern "C" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const libc::c_char, _retval: *mut uint32_t) -> nsresult,

    /* nsIPermission getPermissionObjectForURI (in nsIURI uri, in string type, in boolean exactHost); */
    pub getPermissionObjectForURI: unsafe extern "C" fn (this: *const nsIPermissionManager, uri: *const nsIURI, type_: *const libc::c_char, exactHost: bool, _retval: *mut *const nsIPermission) -> nsresult,

    /* nsIPermission getPermissionObject (in nsIPrincipal principal, in string type, in boolean exactHost); */
    pub getPermissionObject: unsafe extern "C" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const libc::c_char, exactHost: bool, _retval: *mut *const nsIPermission) -> nsresult,

    /* readonly attribute nsISimpleEnumerator enumerator; */
    pub get_enumerator: unsafe extern "C" fn (this: *const nsIPermissionManager, aEnumerator: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void removePermissionsWithAttributes (in DOMString patternAsJSON); */
    pub removePermissionsWithAttributes: unsafe extern "C" fn (this: *const nsIPermissionManager, patternAsJSON: *const nsAString) -> nsresult,

    /* void updateExpireTime (in nsIPrincipal principal, in string type, in boolean exactHost, in uint64_t sessionExpireTime, in uint64_t persistentExpireTime); */
    pub updateExpireTime: unsafe extern "C" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const libc::c_char, exactHost: bool, sessionExpireTime: uint64_t, persistentExpireTime: uint64_t) -> nsresult,

    /* void getPermissionsWithKey (in ACString permissionKey, out IPCPermissionArrayRef perms); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPermissionsWithKey: *const ::libc::c_void,

    /* void setPermissionsWithKey (in ACString permissionKey, in IPCPermissionArrayRef perms); */
    /// Unable to call function as its signature contains a non-rust type
    pub setPermissionsWithKey: *const ::libc::c_void,

    /* void broadcastPermissionsForPrincipalToAllContentProcesses (in nsIPrincipal aPrincipal); */
    pub broadcastPermissionsForPrincipalToAllContentProcesses: unsafe extern "C" fn (this: *const nsIPermissionManager, aPrincipal: *const nsIPrincipal) -> nsresult,

    /* void whenPermissionsAvailable (in nsIPrincipal aPrincipal, in nsIRunnable aRunnable); */
    pub whenPermissionsAvailable: unsafe extern "C" fn (this: *const nsIPermissionManager, aPrincipal: *const nsIPrincipal, aRunnable: *const nsIRunnable) -> nsresult,

}


impl nsIPermissionManager {
    /* void add (in nsIURI uri, in string type, in uint32_t permission, [optional] in uint32_t expireType, [optional] in int64_t expireTime); */
    #[inline]
    pub unsafe fn add(&self, uri: Option<&nsIURI>, type_: *const libc::c_char, permission: uint32_t, expireType: uint32_t, expireTime: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).add)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), type_, permission, expireType, expireTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator getAllForURI (in nsIURI uri); */
    #[inline]
    pub unsafe fn getAllForURI(&self, uri: Option<&nsIURI>) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAllForURI)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addFromPrincipal (in nsIPrincipal principal, in string typed, in uint32_t permission, [optional] in uint32_t expireType, [optional] in int64_t expireTime); */
    #[inline]
    pub unsafe fn addFromPrincipal(&self, principal: Option<&nsIPrincipal>, typed: *const libc::c_char, permission: uint32_t, expireType: uint32_t, expireTime: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).addFromPrincipal)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), typed, permission, expireType, expireTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void remove (in nsIURI uri, in string type); */
    #[inline]
    pub unsafe fn remove(&self, uri: Option<&nsIURI>, type_: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).remove)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeFromPrincipal (in nsIPrincipal principal, in string type); */
    #[inline]
    pub unsafe fn removeFromPrincipal(&self, principal: Option<&nsIPrincipal>, type_: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).removeFromPrincipal)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removePermission (in nsIPermission perm); */
    #[inline]
    pub unsafe fn removePermission(&self, perm: Option<&nsIPermission>) -> Result<(), nsresult> {

        match ((*self.vtable).removePermission)(self as *const _, perm.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAll (); */
    #[inline]
    pub unsafe fn removeAll(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeAll)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAllSince (in int64_t since); */
    #[inline]
    pub unsafe fn removeAllSince(&self, since: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllSince)(self as *const _, since) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* uint32_t testPermission (in nsIURI uri, in string type); */
    #[inline]
    pub unsafe fn testPermission(&self, uri: Option<&nsIURI>, type_: *const libc::c_char) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).testPermission)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), type_, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t testPermissionFromPrincipal (in nsIPrincipal principal, in string type); */
    #[inline]
    pub unsafe fn testPermissionFromPrincipal(&self, principal: Option<&nsIPrincipal>, type_: *const libc::c_char) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).testPermissionFromPrincipal)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), type_, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t testPermissionFromWindow (in mozIDOMWindow window, in string type); */
    #[inline]
    pub unsafe fn testPermissionFromWindow(&self, window: Option<&mozIDOMWindow>, type_: *const libc::c_char) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).testPermissionFromWindow)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), type_, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t testExactPermission (in nsIURI uri, in string type); */
    #[inline]
    pub unsafe fn testExactPermission(&self, uri: Option<&nsIURI>, type_: *const libc::c_char) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).testExactPermission)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), type_, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t testExactPermissionFromPrincipal (in nsIPrincipal principal, in string type); */
    #[inline]
    pub unsafe fn testExactPermissionFromPrincipal(&self, principal: Option<&nsIPrincipal>, type_: *const libc::c_char) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).testExactPermissionFromPrincipal)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), type_, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t testExactPermanentPermission (in nsIPrincipal principal, in string type); */
    #[inline]
    pub unsafe fn testExactPermanentPermission(&self, principal: Option<&nsIPrincipal>, type_: *const libc::c_char) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).testExactPermanentPermission)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), type_, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIPermission getPermissionObjectForURI (in nsIURI uri, in string type, in boolean exactHost); */
    #[inline]
    pub unsafe fn getPermissionObjectForURI(&self, uri: Option<&nsIURI>, type_: *const libc::c_char, exactHost: bool) -> Result<Option<RefPtr<nsIPermission>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPermissionObjectForURI)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), type_, exactHost, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPermission getPermissionObject (in nsIPrincipal principal, in string type, in boolean exactHost); */
    #[inline]
    pub unsafe fn getPermissionObject(&self, principal: Option<&nsIPrincipal>, type_: *const libc::c_char, exactHost: bool) -> Result<Option<RefPtr<nsIPermission>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPermissionObject)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), type_, exactHost, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISimpleEnumerator enumerator; */
    #[inline]
    pub unsafe fn get_enumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_enumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removePermissionsWithAttributes (in DOMString patternAsJSON); */
    #[inline]
    pub unsafe fn removePermissionsWithAttributes(&self, patternAsJSON: &[u16]) -> Result<(), nsresult> {
        let patternAsJSON = nsString::from(patternAsJSON);
        match ((*self.vtable).removePermissionsWithAttributes)(self as *const _, &*patternAsJSON) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateExpireTime (in nsIPrincipal principal, in string type, in boolean exactHost, in uint64_t sessionExpireTime, in uint64_t persistentExpireTime); */
    #[inline]
    pub unsafe fn updateExpireTime(&self, principal: Option<&nsIPrincipal>, type_: *const libc::c_char, exactHost: bool, sessionExpireTime: uint64_t, persistentExpireTime: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).updateExpireTime)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), type_, exactHost, sessionExpireTime, persistentExpireTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getPermissionsWithKey (in ACString permissionKey, out IPCPermissionArrayRef perms); */


    /* void setPermissionsWithKey (in ACString permissionKey, in IPCPermissionArrayRef perms); */


    /* void broadcastPermissionsForPrincipalToAllContentProcesses (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn broadcastPermissionsForPrincipalToAllContentProcesses(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).broadcastPermissionsForPrincipalToAllContentProcesses)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void whenPermissionsAvailable (in nsIPrincipal aPrincipal, in nsIRunnable aRunnable); */
    #[inline]
    pub unsafe fn whenPermissionsAvailable(&self, aPrincipal: Option<&nsIPrincipal>, aRunnable: Option<&nsIRunnable>) -> Result<(), nsresult> {

        match ((*self.vtable).whenPermissionsAvailable)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aRunnable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


