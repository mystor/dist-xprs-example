//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDomainPolicy.idl
//


#[repr(C)]
pub struct nsIDomainPolicy {
    vtable: *const nsIDomainPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDomainPolicy {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x82b24a20, 0x6701, 0x4d40,
            [0xa0, 0xf9, 0xf5, 0xdc, 0x73, 0x21, 0xb5, 0x55])
    }
}

unsafe impl RefCounted for nsIDomainPolicy {
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
pub trait nsIDomainPolicyCoerce {
    fn coerce_from(v: &nsIDomainPolicy) -> &Self;
}

impl nsIDomainPolicyCoerce for nsIDomainPolicy {
    #[inline]
    fn coerce_from(v: &nsIDomainPolicy) -> &Self {
        v
    }
}

impl nsIDomainPolicy {
    #[inline]
    pub fn coerce<T: nsIDomainPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDomainPolicy {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDomainPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDomainPolicy) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDomainPolicyVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDomainSet blacklist; */
    pub get_blacklist: unsafe extern "C" fn (this: *const nsIDomainPolicy, aBlacklist: *mut *const nsIDomainSet) -> nsresult,

    /* readonly attribute nsIDomainSet superBlacklist; */
    pub get_superBlacklist: unsafe extern "C" fn (this: *const nsIDomainPolicy, aSuperBlacklist: *mut *const nsIDomainSet) -> nsresult,

    /* readonly attribute nsIDomainSet whitelist; */
    pub get_whitelist: unsafe extern "C" fn (this: *const nsIDomainPolicy, aWhitelist: *mut *const nsIDomainSet) -> nsresult,

    /* readonly attribute nsIDomainSet superWhitelist; */
    pub get_superWhitelist: unsafe extern "C" fn (this: *const nsIDomainPolicy, aSuperWhitelist: *mut *const nsIDomainSet) -> nsresult,

    /* void deactivate (); */
    pub deactivate: unsafe extern "C" fn (this: *const nsIDomainPolicy) -> nsresult,

    /* [noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone); */
    /// Unable to call function as its signature contains a non-rust type
    pub cloneDomainPolicy: *const ::libc::c_void,

    /* [noscript,notxpcom] void applyClone (in DomainPolicyCloneConstPtr aClone); */
    /// Unable to call function as its signature contains a non-rust type
    pub applyClone: *const ::libc::c_void,

}


impl nsIDomainPolicy {
    /* readonly attribute nsIDomainSet blacklist; */
    #[inline]
    pub unsafe fn get_blacklist(&self, ) -> Result<Option<RefPtr<nsIDomainSet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_blacklist)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDomainSet superBlacklist; */
    #[inline]
    pub unsafe fn get_superBlacklist(&self, ) -> Result<Option<RefPtr<nsIDomainSet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_superBlacklist)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDomainSet whitelist; */
    #[inline]
    pub unsafe fn get_whitelist(&self, ) -> Result<Option<RefPtr<nsIDomainSet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_whitelist)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDomainSet superWhitelist; */
    #[inline]
    pub unsafe fn get_superWhitelist(&self, ) -> Result<Option<RefPtr<nsIDomainSet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_superWhitelist)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void deactivate (); */
    #[inline]
    pub unsafe fn deactivate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).deactivate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone); */


    /* [noscript,notxpcom] void applyClone (in DomainPolicyCloneConstPtr aClone); */


}


#[repr(C)]
pub struct nsIDomainSet {
    vtable: *const nsIDomainSetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDomainSet {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x665c981b, 0x0a0f, 0x4229,
            [0xac, 0x06, 0xa8, 0x26, 0xe0, 0x2d, 0x4f, 0x69])
    }
}

unsafe impl RefCounted for nsIDomainSet {
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
pub trait nsIDomainSetCoerce {
    fn coerce_from(v: &nsIDomainSet) -> &Self;
}

impl nsIDomainSetCoerce for nsIDomainSet {
    #[inline]
    fn coerce_from(v: &nsIDomainSet) -> &Self {
        v
    }
}

impl nsIDomainSet {
    #[inline]
    pub fn coerce<T: nsIDomainSetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDomainSet {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDomainSetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDomainSet) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDomainSetVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] readonly attribute uint32_t type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDomainSet, aType: *mut uint32_t) -> nsresult,

    /* void add (in nsIURI aDomain); */
    pub add: unsafe extern "C" fn (this: *const nsIDomainSet, aDomain: *const nsIURI) -> nsresult,

    /* void remove (in nsIURI aDomain); */
    pub remove: unsafe extern "C" fn (this: *const nsIDomainSet, aDomain: *const nsIURI) -> nsresult,

    /* void clear (); */
    pub clear: unsafe extern "C" fn (this: *const nsIDomainSet) -> nsresult,

    /* bool contains (in nsIURI aDomain); */
    pub contains: unsafe extern "C" fn (this: *const nsIDomainSet, aDomain: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* bool containsSuperDomain (in nsIURI aDomain); */
    pub containsSuperDomain: unsafe extern "C" fn (this: *const nsIDomainSet, aDomain: *const nsIURI, _retval: *mut bool) -> nsresult,

}


impl nsIDomainSet {
    /* [noscript] readonly attribute uint32_t type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void add (in nsIURI aDomain); */
    #[inline]
    pub unsafe fn add(&self, aDomain: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).add)(self as *const _, aDomain.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void remove (in nsIURI aDomain); */
    #[inline]
    pub unsafe fn remove(&self, aDomain: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).remove)(self as *const _, aDomain.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clear (); */
    #[inline]
    pub unsafe fn clear(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clear)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool contains (in nsIURI aDomain); */
    #[inline]
    pub unsafe fn contains(&self, aDomain: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).contains)(self as *const _, aDomain.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool containsSuperDomain (in nsIURI aDomain); */
    #[inline]
    pub unsafe fn containsSuperDomain(&self, aDomain: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).containsSuperDomain)(self as *const _, aDomain.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


