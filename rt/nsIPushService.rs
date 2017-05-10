//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPushService.idl
//


#[repr(C)]
pub struct nsIPushSubscription {
    vtable: *const nsIPushSubscriptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushSubscription {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1de32d5c, 0xea88, 0x4c9e,
            [0x96, 0x26, 0xb0, 0x32, 0xbd, 0x87, 0xf4, 0x15])
    }
}

unsafe impl RefCounted for nsIPushSubscription {
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
pub trait nsIPushSubscriptionCoerce {
    fn coerce_from(v: &nsIPushSubscription) -> &Self;
}

impl nsIPushSubscriptionCoerce for nsIPushSubscription {
    #[inline]
    fn coerce_from(v: &nsIPushSubscription) -> &Self {
        v
    }
}

impl nsIPushSubscription {
    #[inline]
    pub fn coerce<T: nsIPushSubscriptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushSubscription {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushSubscriptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushSubscription) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushSubscriptionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString endpoint; */
    pub get_endpoint: unsafe extern "C" fn (this: *const nsIPushSubscription, aEndpoint: *mut nsAString) -> nsresult,

    /* readonly attribute long long pushCount; */
    pub get_pushCount: unsafe extern "C" fn (this: *const nsIPushSubscription, aPushCount: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long lastPush; */
    pub get_lastPush: unsafe extern "C" fn (this: *const nsIPushSubscription, aLastPush: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long quota; */
    pub get_quota: unsafe extern "C" fn (this: *const nsIPushSubscription, aQuota: *mut libc::int32_t) -> nsresult,

    /* readonly attribute bool isSystemSubscription; */
    pub get_isSystemSubscription: unsafe extern "C" fn (this: *const nsIPushSubscription, aIsSystemSubscription: *mut bool) -> nsresult,

    /* readonly attribute jsval p256dhPrivateKey; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_p256dhPrivateKey: *const ::libc::c_void,

    /* bool quotaApplies (); */
    pub quotaApplies: unsafe extern "C" fn (this: *const nsIPushSubscription, _retval: *mut bool) -> nsresult,

    /* bool isExpired (); */
    pub isExpired: unsafe extern "C" fn (this: *const nsIPushSubscription, _retval: *mut bool) -> nsresult,

    /* void getKey (in DOMString name, [optional] out uint32_t keyLen, [array, size_is (keyLen), retval] out uint8_t key); */
    /// Unable to call function as its signature contains a non-rust type
    pub getKey: *const ::libc::c_void,

}


impl nsIPushSubscription {
    /* readonly attribute DOMString endpoint; */
    #[inline]
    pub unsafe fn get_endpoint(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_endpoint)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long pushCount; */
    #[inline]
    pub unsafe fn get_pushCount(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_pushCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long lastPush; */
    #[inline]
    pub unsafe fn get_lastPush(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastPush)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long quota; */
    #[inline]
    pub unsafe fn get_quota(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_quota)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool isSystemSubscription; */
    #[inline]
    pub unsafe fn get_isSystemSubscription(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSystemSubscription)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute jsval p256dhPrivateKey; */


    /* bool quotaApplies (); */
    #[inline]
    pub unsafe fn quotaApplies(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).quotaApplies)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool isExpired (); */
    #[inline]
    pub unsafe fn isExpired(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isExpired)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getKey (in DOMString name, [optional] out uint32_t keyLen, [array, size_is (keyLen), retval] out uint8_t key); */


}


#[repr(C)]
pub struct nsIPushSubscriptionCallback {
    vtable: *const nsIPushSubscriptionCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushSubscriptionCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1799c074, 0x9d52, 0x46b0,
            [0xab, 0x3c, 0xc0, 0x97, 0x90, 0x73, 0x2f, 0x6f])
    }
}

unsafe impl RefCounted for nsIPushSubscriptionCallback {
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
pub trait nsIPushSubscriptionCallbackCoerce {
    fn coerce_from(v: &nsIPushSubscriptionCallback) -> &Self;
}

impl nsIPushSubscriptionCallbackCoerce for nsIPushSubscriptionCallback {
    #[inline]
    fn coerce_from(v: &nsIPushSubscriptionCallback) -> &Self {
        v
    }
}

impl nsIPushSubscriptionCallback {
    #[inline]
    pub fn coerce<T: nsIPushSubscriptionCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushSubscriptionCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushSubscriptionCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushSubscriptionCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushSubscriptionCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onPushSubscription (in nsresult status, in nsIPushSubscription subscription); */
    pub onPushSubscription: unsafe extern "C" fn (this: *const nsIPushSubscriptionCallback, status: nsresult, subscription: *const nsIPushSubscription) -> nsresult,

}


impl nsIPushSubscriptionCallback {
    /* void onPushSubscription (in nsresult status, in nsIPushSubscription subscription); */
    #[inline]
    pub unsafe fn onPushSubscription(&self, status: nsresult, subscription: Option<&nsIPushSubscription>) -> Result<(), nsresult> {

        match ((*self.vtable).onPushSubscription)(self as *const _, status, subscription.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUnsubscribeResultCallback {
    vtable: *const nsIUnsubscribeResultCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUnsubscribeResultCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd574118f, 0x61a9, 0x4270,
            [0xb1, 0xf6, 0x44, 0x61, 0xaa, 0x85, 0xc4, 0xf5])
    }
}

unsafe impl RefCounted for nsIUnsubscribeResultCallback {
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
pub trait nsIUnsubscribeResultCallbackCoerce {
    fn coerce_from(v: &nsIUnsubscribeResultCallback) -> &Self;
}

impl nsIUnsubscribeResultCallbackCoerce for nsIUnsubscribeResultCallback {
    #[inline]
    fn coerce_from(v: &nsIUnsubscribeResultCallback) -> &Self {
        v
    }
}

impl nsIUnsubscribeResultCallback {
    #[inline]
    pub fn coerce<T: nsIUnsubscribeResultCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUnsubscribeResultCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUnsubscribeResultCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnsubscribeResultCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUnsubscribeResultCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onUnsubscribe (in nsresult status, in bool success); */
    pub onUnsubscribe: unsafe extern "C" fn (this: *const nsIUnsubscribeResultCallback, status: nsresult, success: bool) -> nsresult,

}


impl nsIUnsubscribeResultCallback {
    /* void onUnsubscribe (in nsresult status, in bool success); */
    #[inline]
    pub unsafe fn onUnsubscribe(&self, status: nsresult, success: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onUnsubscribe)(self as *const _, status, success) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPushClearResultCallback {
    vtable: *const nsIPushClearResultCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushClearResultCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbd47b38e, 0x8bfa, 0x4f92,
            [0x83, 0x4e, 0x83, 0x2a, 0x44, 0x31, 0xe0, 0x5e])
    }
}

unsafe impl RefCounted for nsIPushClearResultCallback {
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
pub trait nsIPushClearResultCallbackCoerce {
    fn coerce_from(v: &nsIPushClearResultCallback) -> &Self;
}

impl nsIPushClearResultCallbackCoerce for nsIPushClearResultCallback {
    #[inline]
    fn coerce_from(v: &nsIPushClearResultCallback) -> &Self {
        v
    }
}

impl nsIPushClearResultCallback {
    #[inline]
    pub fn coerce<T: nsIPushClearResultCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushClearResultCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushClearResultCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushClearResultCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushClearResultCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onClear (in nsresult status); */
    pub onClear: unsafe extern "C" fn (this: *const nsIPushClearResultCallback, status: nsresult) -> nsresult,

}


impl nsIPushClearResultCallback {
    /* void onClear (in nsresult status); */
    #[inline]
    pub unsafe fn onClear(&self, status: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onClear)(self as *const _, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPushService {
    vtable: *const nsIPushServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x678ef584, 0xbf25, 0x47aa,
            [0xac, 0x84, 0x03, 0xef, 0xc0, 0x86, 0x5b, 0x68])
    }
}

unsafe impl RefCounted for nsIPushService {
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
pub trait nsIPushServiceCoerce {
    fn coerce_from(v: &nsIPushService) -> &Self;
}

impl nsIPushServiceCoerce for nsIPushService {
    #[inline]
    fn coerce_from(v: &nsIPushService) -> &Self {
        v
    }
}

impl nsIPushService {
    #[inline]
    pub fn coerce<T: nsIPushServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString pushTopic; */
    pub get_pushTopic: unsafe extern "C" fn (this: *const nsIPushService, aPushTopic: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString subscriptionChangeTopic; */
    pub get_subscriptionChangeTopic: unsafe extern "C" fn (this: *const nsIPushService, aSubscriptionChangeTopic: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString subscriptionModifiedTopic; */
    pub get_subscriptionModifiedTopic: unsafe extern "C" fn (this: *const nsIPushService, aSubscriptionModifiedTopic: *mut nsAString) -> nsresult,

    /* void subscribe (in DOMString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
    pub subscribe: unsafe extern "C" fn (this: *const nsIPushService, scope: *const nsAString, principal: *const nsIPrincipal, callback: *const nsIPushSubscriptionCallback) -> nsresult,

    /* void subscribeWithKey (in DOMString scope, in nsIPrincipal principal, in uint32_t keyLength, [array, size_is (keyLength), const] in uint8_t key, in nsIPushSubscriptionCallback callback); */
    /// Unable to call function as its signature contains a non-rust type
    pub subscribeWithKey: *const ::libc::c_void,

    /* void unsubscribe (in DOMString scope, in nsIPrincipal principal, in nsIUnsubscribeResultCallback callback); */
    pub unsubscribe: unsafe extern "C" fn (this: *const nsIPushService, scope: *const nsAString, principal: *const nsIPrincipal, callback: *const nsIUnsubscribeResultCallback) -> nsresult,

    /* void getSubscription (in DOMString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
    pub getSubscription: unsafe extern "C" fn (this: *const nsIPushService, scope: *const nsAString, principal: *const nsIPrincipal, callback: *const nsIPushSubscriptionCallback) -> nsresult,

    /* void clearForDomain (in DOMString domain, in nsIPushClearResultCallback callback); */
    pub clearForDomain: unsafe extern "C" fn (this: *const nsIPushService, domain: *const nsAString, callback: *const nsIPushClearResultCallback) -> nsresult,

}


impl nsIPushService {
    /* readonly attribute DOMString pushTopic; */
    #[inline]
    pub unsafe fn get_pushTopic(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_pushTopic)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString subscriptionChangeTopic; */
    #[inline]
    pub unsafe fn get_subscriptionChangeTopic(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_subscriptionChangeTopic)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString subscriptionModifiedTopic; */
    #[inline]
    pub unsafe fn get_subscriptionModifiedTopic(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_subscriptionModifiedTopic)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void subscribe (in DOMString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
    #[inline]
    pub unsafe fn subscribe(&self, scope: &[u16], principal: Option<&nsIPrincipal>, callback: Option<&nsIPushSubscriptionCallback>) -> Result<(), nsresult> {
        let scope = nsString::from(scope);
        match ((*self.vtable).subscribe)(self as *const _, &*scope, principal.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void subscribeWithKey (in DOMString scope, in nsIPrincipal principal, in uint32_t keyLength, [array, size_is (keyLength), const] in uint8_t key, in nsIPushSubscriptionCallback callback); */


    /* void unsubscribe (in DOMString scope, in nsIPrincipal principal, in nsIUnsubscribeResultCallback callback); */
    #[inline]
    pub unsafe fn unsubscribe(&self, scope: &[u16], principal: Option<&nsIPrincipal>, callback: Option<&nsIUnsubscribeResultCallback>) -> Result<(), nsresult> {
        let scope = nsString::from(scope);
        match ((*self.vtable).unsubscribe)(self as *const _, &*scope, principal.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getSubscription (in DOMString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
    #[inline]
    pub unsafe fn getSubscription(&self, scope: &[u16], principal: Option<&nsIPrincipal>, callback: Option<&nsIPushSubscriptionCallback>) -> Result<(), nsresult> {
        let scope = nsString::from(scope);
        match ((*self.vtable).getSubscription)(self as *const _, &*scope, principal.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearForDomain (in DOMString domain, in nsIPushClearResultCallback callback); */
    #[inline]
    pub unsafe fn clearForDomain(&self, domain: &[u16], callback: Option<&nsIPushClearResultCallback>) -> Result<(), nsresult> {
        let domain = nsString::from(domain);
        match ((*self.vtable).clearForDomain)(self as *const _, &*domain, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPushQuotaManager {
    vtable: *const nsIPushQuotaManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushQuotaManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa2555e70, 0x46f8, 0x4b52,
            [0xbf, 0x02, 0xd9, 0x78, 0xb9, 0x79, 0xd1, 0x43])
    }
}

unsafe impl RefCounted for nsIPushQuotaManager {
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
pub trait nsIPushQuotaManagerCoerce {
    fn coerce_from(v: &nsIPushQuotaManager) -> &Self;
}

impl nsIPushQuotaManagerCoerce for nsIPushQuotaManager {
    #[inline]
    fn coerce_from(v: &nsIPushQuotaManager) -> &Self {
        v
    }
}

impl nsIPushQuotaManager {
    #[inline]
    pub fn coerce<T: nsIPushQuotaManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushQuotaManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushQuotaManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushQuotaManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushQuotaManagerVTable {
    pub __base: nsISupportsVTable,

    /* void notificationForOriginShown (in string origin); */
    pub notificationForOriginShown: unsafe extern "C" fn (this: *const nsIPushQuotaManager, origin: *const libc::c_char) -> nsresult,

    /* void notificationForOriginClosed (in string origin); */
    pub notificationForOriginClosed: unsafe extern "C" fn (this: *const nsIPushQuotaManager, origin: *const libc::c_char) -> nsresult,

}


impl nsIPushQuotaManager {
    /* void notificationForOriginShown (in string origin); */
    #[inline]
    pub unsafe fn notificationForOriginShown(&self, origin: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).notificationForOriginShown)(self as *const _, origin) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notificationForOriginClosed (in string origin); */
    #[inline]
    pub unsafe fn notificationForOriginClosed(&self, origin: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).notificationForOriginClosed)(self as *const _, origin) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


