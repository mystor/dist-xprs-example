//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/amIAddonPathService.idl
//


#[repr(C)]
pub struct amIAddonPathService {
    vtable: *const amIAddonPathServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for amIAddonPathService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfcd9e270, 0xdfb1, 0x11e3,
            [0x8b, 0x68, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for amIAddonPathService {
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
pub trait amIAddonPathServiceCoerce {
    fn coerce_from(v: &amIAddonPathService) -> &Self;
}

impl amIAddonPathServiceCoerce for amIAddonPathService {
    #[inline]
    fn coerce_from(v: &amIAddonPathService) -> &Self {
        v
    }
}

impl amIAddonPathService {
    #[inline]
    pub fn coerce<T: amIAddonPathServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for amIAddonPathService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> amIAddonPathServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &amIAddonPathService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct amIAddonPathServiceVTable {
    pub __base: nsISupportsVTable,

    /* AString findAddonId (in AString path); */
    pub findAddonId: unsafe extern "C" fn (this: *const amIAddonPathService, path: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void insertPath (in AString path, in AString addonId); */
    pub insertPath: unsafe extern "C" fn (this: *const amIAddonPathService, path: *const nsAString, addonId: *const nsAString) -> nsresult,

    /* AString mapURIToAddonId (in nsIURI aURI); */
    pub mapURIToAddonId: unsafe extern "C" fn (this: *const amIAddonPathService, aURI: *const nsIURI, _retval: *mut nsAString) -> nsresult,

}


impl amIAddonPathService {
    /* AString findAddonId (in AString path); */
    #[inline]
    pub unsafe fn findAddonId(&self, path: &[u16]) -> Result<nsString, nsresult> {
        let path = nsString::from(path);
        let mut _retval = nsString::new();
        match ((*self.vtable).findAddonId)(self as *const _, &*path, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void insertPath (in AString path, in AString addonId); */
    #[inline]
    pub unsafe fn insertPath(&self, path: &[u16], addonId: &[u16]) -> Result<(), nsresult> {
        let path = nsString::from(path);
        let addonId = nsString::from(addonId);
        match ((*self.vtable).insertPath)(self as *const _, &*path, &*addonId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString mapURIToAddonId (in nsIURI aURI); */
    #[inline]
    pub unsafe fn mapURIToAddonId(&self, aURI: Option<&nsIURI>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).mapURIToAddonId)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


