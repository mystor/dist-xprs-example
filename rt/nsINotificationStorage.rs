//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINotificationStorage.idl
//


#[repr(C)]
pub struct nsINotificationStorageCallback {
    vtable: *const nsINotificationStorageCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINotificationStorageCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc1622232, 0x259c, 0x43b0,
            [0xb5, 0x2e, 0x89, 0xc3, 0x9d, 0xcd, 0x97, 0x96])
    }
}

unsafe impl RefCounted for nsINotificationStorageCallback {
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
pub trait nsINotificationStorageCallbackCoerce {
    fn coerce_from(v: &nsINotificationStorageCallback) -> &Self;
}

impl nsINotificationStorageCallbackCoerce for nsINotificationStorageCallback {
    #[inline]
    fn coerce_from(v: &nsINotificationStorageCallback) -> &Self {
        v
    }
}

impl nsINotificationStorageCallback {
    #[inline]
    pub fn coerce<T: nsINotificationStorageCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINotificationStorageCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINotificationStorageCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINotificationStorageCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINotificationStorageCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handle (in DOMString id, in DOMString title, in DOMString dir, in DOMString lang, in DOMString body, in DOMString tag, in DOMString icon, in DOMString data, in DOMString behavior, in DOMString serviceWorkerRegistrationScope); */
    pub handle: unsafe extern "C" fn (this: *const nsINotificationStorageCallback, id: *const nsAString, title: *const nsAString, dir: *const nsAString, lang: *const nsAString, body: *const nsAString, tag: *const nsAString, icon: *const nsAString, data: *const nsAString, behavior: *const nsAString, serviceWorkerRegistrationScope: *const nsAString) -> nsresult,

    /* void done (); */
    pub done: unsafe extern "C" fn (this: *const nsINotificationStorageCallback) -> nsresult,

}


impl nsINotificationStorageCallback {
    /* void handle (in DOMString id, in DOMString title, in DOMString dir, in DOMString lang, in DOMString body, in DOMString tag, in DOMString icon, in DOMString data, in DOMString behavior, in DOMString serviceWorkerRegistrationScope); */
    #[inline]
    pub unsafe fn handle(&self, id: &[u16], title: &[u16], dir: &[u16], lang: &[u16], body: &[u16], tag: &[u16], icon: &[u16], data: &[u16], behavior: &[u16], serviceWorkerRegistrationScope: &[u16]) -> Result<(), nsresult> {
        let id = nsString::from(id);
        let title = nsString::from(title);
        let dir = nsString::from(dir);
        let lang = nsString::from(lang);
        let body = nsString::from(body);
        let tag = nsString::from(tag);
        let icon = nsString::from(icon);
        let data = nsString::from(data);
        let behavior = nsString::from(behavior);
        let serviceWorkerRegistrationScope = nsString::from(serviceWorkerRegistrationScope);
        match ((*self.vtable).handle)(self as *const _, &*id, &*title, &*dir, &*lang, &*body, &*tag, &*icon, &*data, &*behavior, &*serviceWorkerRegistrationScope) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void done (); */
    #[inline]
    pub unsafe fn done(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).done)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINotificationStorage {
    vtable: *const nsINotificationStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINotificationStorage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x17f85e52, 0xfe57, 0x440e,
            [0x9b, 0xa1, 0x5c, 0x31, 0x2c, 0xa0, 0x2b, 0x95])
    }
}

unsafe impl RefCounted for nsINotificationStorage {
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
pub trait nsINotificationStorageCoerce {
    fn coerce_from(v: &nsINotificationStorage) -> &Self;
}

impl nsINotificationStorageCoerce for nsINotificationStorage {
    #[inline]
    fn coerce_from(v: &nsINotificationStorage) -> &Self {
        v
    }
}

impl nsINotificationStorage {
    #[inline]
    pub fn coerce<T: nsINotificationStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINotificationStorage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINotificationStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINotificationStorage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINotificationStorageVTable {
    pub __base: nsISupportsVTable,

    /* void put (in DOMString origin, in DOMString id, in DOMString title, in DOMString dir, in DOMString lang, in DOMString body, in DOMString tag, in DOMString icon, in DOMString alertName, in DOMString data, in DOMString behavior, in DOMString serviceWorkerRegistrationScope); */
    pub put: unsafe extern "C" fn (this: *const nsINotificationStorage, origin: *const nsAString, id: *const nsAString, title: *const nsAString, dir: *const nsAString, lang: *const nsAString, body: *const nsAString, tag: *const nsAString, icon: *const nsAString, alertName: *const nsAString, data: *const nsAString, behavior: *const nsAString, serviceWorkerRegistrationScope: *const nsAString) -> nsresult,

    /* void get (in DOMString origin, in DOMString tag, in nsINotificationStorageCallback aCallback); */
    pub get: unsafe extern "C" fn (this: *const nsINotificationStorage, origin: *const nsAString, tag: *const nsAString, aCallback: *const nsINotificationStorageCallback) -> nsresult,

    /* void getByID (in DOMString origin, in DOMString id, in nsINotificationStorageCallback aCallback); */
    pub getByID: unsafe extern "C" fn (this: *const nsINotificationStorage, origin: *const nsAString, id: *const nsAString, aCallback: *const nsINotificationStorageCallback) -> nsresult,

    /* void delete (in DOMString origin, in DOMString id); */
    pub delete: unsafe extern "C" fn (this: *const nsINotificationStorage, origin: *const nsAString, id: *const nsAString) -> nsresult,

    /* boolean canPut (in DOMString origin); */
    pub canPut: unsafe extern "C" fn (this: *const nsINotificationStorage, origin: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl nsINotificationStorage {
    /* void put (in DOMString origin, in DOMString id, in DOMString title, in DOMString dir, in DOMString lang, in DOMString body, in DOMString tag, in DOMString icon, in DOMString alertName, in DOMString data, in DOMString behavior, in DOMString serviceWorkerRegistrationScope); */
    #[inline]
    pub unsafe fn put(&self, origin: &[u16], id: &[u16], title: &[u16], dir: &[u16], lang: &[u16], body: &[u16], tag: &[u16], icon: &[u16], alertName: &[u16], data: &[u16], behavior: &[u16], serviceWorkerRegistrationScope: &[u16]) -> Result<(), nsresult> {
        let origin = nsString::from(origin);
        let id = nsString::from(id);
        let title = nsString::from(title);
        let dir = nsString::from(dir);
        let lang = nsString::from(lang);
        let body = nsString::from(body);
        let tag = nsString::from(tag);
        let icon = nsString::from(icon);
        let alertName = nsString::from(alertName);
        let data = nsString::from(data);
        let behavior = nsString::from(behavior);
        let serviceWorkerRegistrationScope = nsString::from(serviceWorkerRegistrationScope);
        match ((*self.vtable).put)(self as *const _, &*origin, &*id, &*title, &*dir, &*lang, &*body, &*tag, &*icon, &*alertName, &*data, &*behavior, &*serviceWorkerRegistrationScope) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void get (in DOMString origin, in DOMString tag, in nsINotificationStorageCallback aCallback); */
    #[inline]
    pub unsafe fn get(&self, origin: &[u16], tag: &[u16], aCallback: Option<&nsINotificationStorageCallback>) -> Result<(), nsresult> {
        let origin = nsString::from(origin);
        let tag = nsString::from(tag);
        match ((*self.vtable).get)(self as *const _, &*origin, &*tag, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getByID (in DOMString origin, in DOMString id, in nsINotificationStorageCallback aCallback); */
    #[inline]
    pub unsafe fn getByID(&self, origin: &[u16], id: &[u16], aCallback: Option<&nsINotificationStorageCallback>) -> Result<(), nsresult> {
        let origin = nsString::from(origin);
        let id = nsString::from(id);
        match ((*self.vtable).getByID)(self as *const _, &*origin, &*id, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void delete (in DOMString origin, in DOMString id); */
    #[inline]
    pub unsafe fn delete(&self, origin: &[u16], id: &[u16]) -> Result<(), nsresult> {
        let origin = nsString::from(origin);
        let id = nsString::from(id);
        match ((*self.vtable).delete)(self as *const _, &*origin, &*id) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean canPut (in DOMString origin); */
    #[inline]
    pub unsafe fn canPut(&self, origin: &[u16]) -> Result<bool, nsresult> {
        let origin = nsString::from(origin);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canPut)(self as *const _, &*origin, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


