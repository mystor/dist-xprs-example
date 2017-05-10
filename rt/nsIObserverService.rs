//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIObserverService.idl
//


#[repr(C)]
pub struct nsIObserverService {
    vtable: *const nsIObserverServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIObserverService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd07f5192, 0xe3d1, 0x11d2,
            [0x8a, 0xcd, 0x00, 0x10, 0x5a, 0x1b, 0x88, 0x60])
    }
}

unsafe impl RefCounted for nsIObserverService {
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
pub trait nsIObserverServiceCoerce {
    fn coerce_from(v: &nsIObserverService) -> &Self;
}

impl nsIObserverServiceCoerce for nsIObserverService {
    #[inline]
    fn coerce_from(v: &nsIObserverService) -> &Self {
        v
    }
}

impl nsIObserverService {
    #[inline]
    pub fn coerce<T: nsIObserverServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIObserverService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIObserverServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObserverService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIObserverServiceVTable {
    pub __base: nsISupportsVTable,

    /* void addObserver (in nsIObserver anObserver, in string aTopic, [optional] in boolean ownsWeak); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIObserverService, anObserver: *const nsIObserver, aTopic: *const libc::c_char, ownsWeak: bool) -> nsresult,

    /* void removeObserver (in nsIObserver anObserver, in string aTopic); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIObserverService, anObserver: *const nsIObserver, aTopic: *const libc::c_char) -> nsresult,

    /* void notifyObservers (in nsISupports aSubject, in string aTopic, [optional] in wstring someData); */
    pub notifyObservers: unsafe extern "C" fn (this: *const nsIObserverService, aSubject: *const nsISupports, aTopic: *const libc::c_char, someData: *const libc::int16_t) -> nsresult,

    /* nsISimpleEnumerator enumerateObservers (in string aTopic); */
    pub enumerateObservers: unsafe extern "C" fn (this: *const nsIObserverService, aTopic: *const libc::c_char, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIObserverService {
    /* void addObserver (in nsIObserver anObserver, in string aTopic, [optional] in boolean ownsWeak); */
    #[inline]
    pub unsafe fn addObserver(&self, anObserver: Option<&nsIObserver>, aTopic: *const libc::c_char, ownsWeak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, anObserver.map_or(::std::ptr::null(), |x| x as *const _), aTopic, ownsWeak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsIObserver anObserver, in string aTopic); */
    #[inline]
    pub unsafe fn removeObserver(&self, anObserver: Option<&nsIObserver>, aTopic: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, anObserver.map_or(::std::ptr::null(), |x| x as *const _), aTopic) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyObservers (in nsISupports aSubject, in string aTopic, [optional] in wstring someData); */
    #[inline]
    pub unsafe fn notifyObservers(&self, aSubject: Option<&nsISupports>, aTopic: *const libc::c_char, someData: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).notifyObservers)(self as *const _, aSubject.map_or(::std::ptr::null(), |x| x as *const _), aTopic, someData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator enumerateObservers (in string aTopic); */
    #[inline]
    pub unsafe fn enumerateObservers(&self, aTopic: *const libc::c_char) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerateObservers)(self as *const _, aTopic, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


