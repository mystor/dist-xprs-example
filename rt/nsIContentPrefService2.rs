//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPrefService2.idl
//


#[repr(C)]
pub struct nsIContentPrefService2 {
    vtable: *const nsIContentPrefService2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPrefService2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbed98666, 0xd995, 0x470f,
            [0xbe, 0xbd, 0x62, 0x47, 0x6d, 0x31, 0x85, 0x76])
    }
}

unsafe impl RefCounted for nsIContentPrefService2 {
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
pub trait nsIContentPrefService2Coerce {
    fn coerce_from(v: &nsIContentPrefService2) -> &Self;
}

impl nsIContentPrefService2Coerce for nsIContentPrefService2 {
    #[inline]
    fn coerce_from(v: &nsIContentPrefService2) -> &Self {
        v
    }
}

impl nsIContentPrefService2 {
    #[inline]
    pub fn coerce<T: nsIContentPrefService2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPrefService2 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPrefService2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPrefService2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPrefService2VTable {
    pub __base: nsISupportsVTable,

    /* void getByName (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    pub getByName: unsafe extern "C" fn (this: *const nsIContentPrefService2, name: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void getByDomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    pub getByDomainAndName: unsafe extern "C" fn (this: *const nsIContentPrefService2, domain: *const nsAString, name: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void getBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    pub getBySubdomainAndName: unsafe extern "C" fn (this: *const nsIContentPrefService2, domain: *const nsAString, name: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void getGlobal (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    pub getGlobal: unsafe extern "C" fn (this: *const nsIContentPrefService2, name: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* nsIContentPref getCachedByDomainAndName (in AString domain, in AString name, in nsILoadContext context); */
    pub getCachedByDomainAndName: unsafe extern "C" fn (this: *const nsIContentPrefService2, domain: *const nsAString, name: *const nsAString, context: *const nsILoadContext, _retval: *mut *const nsIContentPref) -> nsresult,

    /* void getCachedBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] out unsigned long len, [array, size_is (len), retval] out nsIContentPref prefs); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCachedBySubdomainAndName: *const ::libc::c_void,

    /* nsIContentPref getCachedGlobal (in AString name, in nsILoadContext context); */
    pub getCachedGlobal: unsafe extern "C" fn (this: *const nsIContentPrefService2, name: *const nsAString, context: *const nsILoadContext, _retval: *mut *const nsIContentPref) -> nsresult,

    /* void set (in AString domain, in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub set: unsafe extern "C" fn (this: *const nsIContentPrefService2, domain: *const nsAString, name: *const nsAString, value: *const nsIVariant, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void setGlobal (in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub setGlobal: unsafe extern "C" fn (this: *const nsIContentPrefService2, name: *const nsAString, value: *const nsIVariant, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeByDomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeByDomainAndName: unsafe extern "C" fn (this: *const nsIContentPrefService2, domain: *const nsAString, name: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeBySubdomainAndName: unsafe extern "C" fn (this: *const nsIContentPrefService2, domain: *const nsAString, name: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeGlobal (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeGlobal: unsafe extern "C" fn (this: *const nsIContentPrefService2, name: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeByDomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeByDomain: unsafe extern "C" fn (this: *const nsIContentPrefService2, domain: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeBySubdomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeBySubdomain: unsafe extern "C" fn (this: *const nsIContentPrefService2, domain: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeByName (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeByName: unsafe extern "C" fn (this: *const nsIContentPrefService2, name: *const nsAString, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeAllDomains (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeAllDomains: unsafe extern "C" fn (this: *const nsIContentPrefService2, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeAllDomainsSince (in unsigned long long since, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeAllDomainsSince: unsafe extern "C" fn (this: *const nsIContentPrefService2, since: libc::uint64_t, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void removeAllGlobals (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    pub removeAllGlobals: unsafe extern "C" fn (this: *const nsIContentPrefService2, context: *const nsILoadContext, callback: *const nsIContentPrefCallback2) -> nsresult,

    /* void addObserverForName (in AString name, in nsIContentPrefObserver observer); */
    pub addObserverForName: unsafe extern "C" fn (this: *const nsIContentPrefService2, name: *const nsAString, observer: *const nsIContentPrefObserver) -> nsresult,

    /* void removeObserverForName (in AString name, in nsIContentPrefObserver observer); */
    pub removeObserverForName: unsafe extern "C" fn (this: *const nsIContentPrefService2, name: *const nsAString, observer: *const nsIContentPrefObserver) -> nsresult,

    /* AString extractDomain (in AString str); */
    pub extractDomain: unsafe extern "C" fn (this: *const nsIContentPrefService2, str: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsIContentPrefService2 {
    /* void getByName (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn getByName(&self, name: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).getByName)(self as *const _, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getByDomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn getByDomainAndName(&self, domain: &[u16], name: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let domain = nsString::from(domain);
        let name = nsString::from(name);
        match ((*self.vtable).getByDomainAndName)(self as *const _, &*domain, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn getBySubdomainAndName(&self, domain: &[u16], name: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let domain = nsString::from(domain);
        let name = nsString::from(name);
        match ((*self.vtable).getBySubdomainAndName)(self as *const _, &*domain, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getGlobal (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn getGlobal(&self, name: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).getGlobal)(self as *const _, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIContentPref getCachedByDomainAndName (in AString domain, in AString name, in nsILoadContext context); */
    #[inline]
    pub unsafe fn getCachedByDomainAndName(&self, domain: &[u16], name: &[u16], context: Option<&nsILoadContext>) -> Result<Option<RefPtr<nsIContentPref>>, nsresult> {
        let domain = nsString::from(domain);
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCachedByDomainAndName)(self as *const _, &*domain, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getCachedBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] out unsigned long len, [array, size_is (len), retval] out nsIContentPref prefs); */


    /* nsIContentPref getCachedGlobal (in AString name, in nsILoadContext context); */
    #[inline]
    pub unsafe fn getCachedGlobal(&self, name: &[u16], context: Option<&nsILoadContext>) -> Result<Option<RefPtr<nsIContentPref>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCachedGlobal)(self as *const _, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void set (in AString domain, in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn set(&self, domain: &[u16], name: &[u16], value: Option<&nsIVariant>, context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let domain = nsString::from(domain);
        let name = nsString::from(name);
        match ((*self.vtable).set)(self as *const _, &*domain, &*name, value.map_or(::std::ptr::null(), |x| x as *const _), context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setGlobal (in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn setGlobal(&self, name: &[u16], value: Option<&nsIVariant>, context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).setGlobal)(self as *const _, &*name, value.map_or(::std::ptr::null(), |x| x as *const _), context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeByDomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeByDomainAndName(&self, domain: &[u16], name: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let domain = nsString::from(domain);
        let name = nsString::from(name);
        match ((*self.vtable).removeByDomainAndName)(self as *const _, &*domain, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeBySubdomainAndName(&self, domain: &[u16], name: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let domain = nsString::from(domain);
        let name = nsString::from(name);
        match ((*self.vtable).removeBySubdomainAndName)(self as *const _, &*domain, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeGlobal (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeGlobal(&self, name: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).removeGlobal)(self as *const _, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeByDomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeByDomain(&self, domain: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let domain = nsString::from(domain);
        match ((*self.vtable).removeByDomain)(self as *const _, &*domain, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeBySubdomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeBySubdomain(&self, domain: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let domain = nsString::from(domain);
        match ((*self.vtable).removeBySubdomain)(self as *const _, &*domain, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeByName (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeByName(&self, name: &[u16], context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).removeByName)(self as *const _, &*name, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAllDomains (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeAllDomains(&self, context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllDomains)(self as *const _, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAllDomainsSince (in unsigned long long since, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeAllDomainsSince(&self, since: libc::uint64_t, context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllDomainsSince)(self as *const _, since, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAllGlobals (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
    #[inline]
    pub unsafe fn removeAllGlobals(&self, context: Option<&nsILoadContext>, callback: Option<&nsIContentPrefCallback2>) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllGlobals)(self as *const _, context.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addObserverForName (in AString name, in nsIContentPrefObserver observer); */
    #[inline]
    pub unsafe fn addObserverForName(&self, name: &[u16], observer: Option<&nsIContentPrefObserver>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).addObserverForName)(self as *const _, &*name, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserverForName (in AString name, in nsIContentPrefObserver observer); */
    #[inline]
    pub unsafe fn removeObserverForName(&self, name: &[u16], observer: Option<&nsIContentPrefObserver>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).removeObserverForName)(self as *const _, &*name, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString extractDomain (in AString str); */
    #[inline]
    pub unsafe fn extractDomain(&self, str: &[u16]) -> Result<nsString, nsresult> {
        let str = nsString::from(str);
        let mut _retval = nsString::new();
        match ((*self.vtable).extractDomain)(self as *const _, &*str, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIContentPrefCallback2_consts {
    pub const COMPLETE_OK: i64 = 0;
    pub const COMPLETE_ERROR: i64 = 1;
}


#[repr(C)]
pub struct nsIContentPrefCallback2 {
    vtable: *const nsIContentPrefCallback2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPrefCallback2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1a12cf41, 0x79e8, 0x4d0f,
            [0x98, 0x99, 0x2f, 0x7b, 0x27, 0xc5, 0xd9, 0xa1])
    }
}

unsafe impl RefCounted for nsIContentPrefCallback2 {
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
pub trait nsIContentPrefCallback2Coerce {
    fn coerce_from(v: &nsIContentPrefCallback2) -> &Self;
}

impl nsIContentPrefCallback2Coerce for nsIContentPrefCallback2 {
    #[inline]
    fn coerce_from(v: &nsIContentPrefCallback2) -> &Self {
        v
    }
}

impl nsIContentPrefCallback2 {
    #[inline]
    pub fn coerce<T: nsIContentPrefCallback2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPrefCallback2 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPrefCallback2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPrefCallback2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPrefCallback2VTable {
    pub __base: nsISupportsVTable,

    /* void handleResult (in nsIContentPref pref); */
    pub handleResult: unsafe extern "C" fn (this: *const nsIContentPrefCallback2, pref: *const nsIContentPref) -> nsresult,

    /* void handleError (in nsresult error); */
    pub handleError: unsafe extern "C" fn (this: *const nsIContentPrefCallback2, error: nsresult) -> nsresult,

    /* void handleCompletion (in unsigned short reason); */
    pub handleCompletion: unsafe extern "C" fn (this: *const nsIContentPrefCallback2, reason: libc::uint16_t) -> nsresult,

}


impl nsIContentPrefCallback2 {
    /* void handleResult (in nsIContentPref pref); */
    #[inline]
    pub unsafe fn handleResult(&self, pref: Option<&nsIContentPref>) -> Result<(), nsresult> {

        match ((*self.vtable).handleResult)(self as *const _, pref.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleError (in nsresult error); */
    #[inline]
    pub unsafe fn handleError(&self, error: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).handleError)(self as *const _, error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleCompletion (in unsigned short reason); */
    #[inline]
    pub unsafe fn handleCompletion(&self, reason: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).handleCompletion)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIContentPref {
    vtable: *const nsIContentPrefVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPref {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9f24948d, 0x24b5, 0x4b1b,
            [0xb5, 0x54, 0x7d, 0xbd, 0x58, 0xc1, 0xd7, 0x92])
    }
}

unsafe impl RefCounted for nsIContentPref {
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
pub trait nsIContentPrefCoerce {
    fn coerce_from(v: &nsIContentPref) -> &Self;
}

impl nsIContentPrefCoerce for nsIContentPref {
    #[inline]
    fn coerce_from(v: &nsIContentPref) -> &Self {
        v
    }
}

impl nsIContentPref {
    #[inline]
    pub fn coerce<T: nsIContentPrefCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPref {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPrefCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPref) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPrefVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString domain; */
    pub get_domain: unsafe extern "C" fn (this: *const nsIContentPref, aDomain: *mut nsAString) -> nsresult,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIContentPref, aName: *mut nsAString) -> nsresult,

    /* readonly attribute nsIVariant value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIContentPref, aValue: *mut *const nsIVariant) -> nsresult,

}


impl nsIContentPref {
    /* readonly attribute AString domain; */
    #[inline]
    pub unsafe fn get_domain(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_domain)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIVariant value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_value)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


