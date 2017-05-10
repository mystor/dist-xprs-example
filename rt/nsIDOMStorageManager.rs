//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMStorageManager.idl
//


#[repr(C)]
pub struct nsIDOMStorageManager {
    vtable: *const nsIDOMStorageManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMStorageManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa20c742e, 0x3ed1, 0x44fb,
            [0xb8, 0x97, 0x40, 0x80, 0xa7, 0x5b, 0x16, 0x62])
    }
}

unsafe impl RefCounted for nsIDOMStorageManager {
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
pub trait nsIDOMStorageManagerCoerce {
    fn coerce_from(v: &nsIDOMStorageManager) -> &Self;
}

impl nsIDOMStorageManagerCoerce for nsIDOMStorageManager {
    #[inline]
    fn coerce_from(v: &nsIDOMStorageManager) -> &Self {
        v
    }
}

impl nsIDOMStorageManager {
    #[inline]
    pub fn coerce<T: nsIDOMStorageManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMStorageManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMStorageManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMStorageManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMStorageManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMStorage precacheStorage (in nsIPrincipal aPrincipal); */
    pub precacheStorage: unsafe extern "C" fn (this: *const nsIDOMStorageManager, aPrincipal: *const nsIPrincipal, _retval: *mut *const nsIDOMStorage) -> nsresult,

    /* nsIDOMStorage createStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in DOMString aDocumentURI, [optional] in bool aPrivate); */
    pub createStorage: unsafe extern "C" fn (this: *const nsIDOMStorageManager, aWindow: *const mozIDOMWindow, aPrincipal: *const nsIPrincipal, aDocumentURI: *const nsAString, aPrivate: bool, _retval: *mut *const nsIDOMStorage) -> nsresult,

    /* nsIDOMStorage getStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, [optional] in bool aPrivate); */
    pub getStorage: unsafe extern "C" fn (this: *const nsIDOMStorageManager, aWindow: *const mozIDOMWindow, aPrincipal: *const nsIPrincipal, aPrivate: bool, _retval: *mut *const nsIDOMStorage) -> nsresult,

    /* void cloneStorage (in nsIDOMStorage aStorageToCloneFrom); */
    pub cloneStorage: unsafe extern "C" fn (this: *const nsIDOMStorageManager, aStorageToCloneFrom: *const nsIDOMStorage) -> nsresult,

    /* bool checkStorage (in nsIPrincipal aPrincipal, in nsIDOMStorage aStorage); */
    pub checkStorage: unsafe extern "C" fn (this: *const nsIDOMStorageManager, aPrincipal: *const nsIPrincipal, aStorage: *const nsIDOMStorage, _retval: *mut bool) -> nsresult,

    /* nsIDOMStorage getLocalStorageForPrincipal (in nsIPrincipal aPrincipal, in DOMString aDocumentURI, [optional] in bool aPrivate); */
    pub getLocalStorageForPrincipal: unsafe extern "C" fn (this: *const nsIDOMStorageManager, aPrincipal: *const nsIPrincipal, aDocumentURI: *const nsAString, aPrivate: bool, _retval: *mut *const nsIDOMStorage) -> nsresult,

}


impl nsIDOMStorageManager {
    /* nsIDOMStorage precacheStorage (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn precacheStorage(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<Option<RefPtr<nsIDOMStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).precacheStorage)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMStorage createStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in DOMString aDocumentURI, [optional] in bool aPrivate); */
    #[inline]
    pub unsafe fn createStorage(&self, aWindow: Option<&mozIDOMWindow>, aPrincipal: Option<&nsIPrincipal>, aDocumentURI: &[u16], aPrivate: bool) -> Result<Option<RefPtr<nsIDOMStorage>>, nsresult> {
        let aDocumentURI = nsString::from(aDocumentURI);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createStorage)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &*aDocumentURI, aPrivate, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMStorage getStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, [optional] in bool aPrivate); */
    #[inline]
    pub unsafe fn getStorage(&self, aWindow: Option<&mozIDOMWindow>, aPrincipal: Option<&nsIPrincipal>, aPrivate: bool) -> Result<Option<RefPtr<nsIDOMStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getStorage)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aPrivate, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void cloneStorage (in nsIDOMStorage aStorageToCloneFrom); */
    #[inline]
    pub unsafe fn cloneStorage(&self, aStorageToCloneFrom: Option<&nsIDOMStorage>) -> Result<(), nsresult> {

        match ((*self.vtable).cloneStorage)(self as *const _, aStorageToCloneFrom.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool checkStorage (in nsIPrincipal aPrincipal, in nsIDOMStorage aStorage); */
    #[inline]
    pub unsafe fn checkStorage(&self, aPrincipal: Option<&nsIPrincipal>, aStorage: Option<&nsIDOMStorage>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).checkStorage)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aStorage.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMStorage getLocalStorageForPrincipal (in nsIPrincipal aPrincipal, in DOMString aDocumentURI, [optional] in bool aPrivate); */
    #[inline]
    pub unsafe fn getLocalStorageForPrincipal(&self, aPrincipal: Option<&nsIPrincipal>, aDocumentURI: &[u16], aPrivate: bool) -> Result<Option<RefPtr<nsIDOMStorage>>, nsresult> {
        let aDocumentURI = nsString::from(aDocumentURI);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getLocalStorageForPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &*aDocumentURI, aPrivate, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


