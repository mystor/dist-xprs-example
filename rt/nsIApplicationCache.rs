//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationCache.idl
//


pub mod nsIApplicationCacheNamespace_consts {
    pub const NAMESPACE_BYPASS: i64 = 1;
    pub const NAMESPACE_FALLBACK: i64 = 2;
    pub const NAMESPACE_OPPORTUNISTIC: i64 = 4;
}


#[repr(C)]
pub struct nsIApplicationCacheNamespace {
    vtable: *const nsIApplicationCacheNamespaceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationCacheNamespace {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x96e4c264, 0x2065, 0x4ce9,
            [0x93, 0xbb, 0x43, 0x73, 0x4c, 0x62, 0xc4, 0xeb])
    }
}

unsafe impl RefCounted for nsIApplicationCacheNamespace {
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
pub trait nsIApplicationCacheNamespaceCoerce {
    fn coerce_from(v: &nsIApplicationCacheNamespace) -> &Self;
}

impl nsIApplicationCacheNamespaceCoerce for nsIApplicationCacheNamespace {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheNamespace) -> &Self {
        v
    }
}

impl nsIApplicationCacheNamespace {
    #[inline]
    pub fn coerce<T: nsIApplicationCacheNamespaceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationCacheNamespace {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIApplicationCacheNamespaceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheNamespace) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationCacheNamespaceVTable {
    pub __base: nsISupportsVTable,

    /* void init (in unsigned long itemType, in ACString namespaceSpec, in ACString data); */
    pub init: unsafe extern "C" fn (this: *const nsIApplicationCacheNamespace, itemType: libc::uint32_t, namespaceSpec: *const nsACString, data: *const nsACString) -> nsresult,

    /* readonly attribute unsigned long itemType; */
    pub get_itemType: unsafe extern "C" fn (this: *const nsIApplicationCacheNamespace, aItemType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute ACString namespaceSpec; */
    pub get_namespaceSpec: unsafe extern "C" fn (this: *const nsIApplicationCacheNamespace, aNamespaceSpec: *mut nsACString) -> nsresult,

    /* readonly attribute ACString data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIApplicationCacheNamespace, aData: *mut nsACString) -> nsresult,

}


impl nsIApplicationCacheNamespace {
    /* void init (in unsigned long itemType, in ACString namespaceSpec, in ACString data); */
    #[inline]
    pub unsafe fn init(&self, itemType: libc::uint32_t, namespaceSpec: &[u8], data: &[u8]) -> Result<(), nsresult> {
        let namespaceSpec = nsCString::from(namespaceSpec);
        let data = nsCString::from(data);
        match ((*self.vtable).init)(self as *const _, itemType, &*namespaceSpec, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long itemType; */
    #[inline]
    pub unsafe fn get_itemType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_itemType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString namespaceSpec; */
    #[inline]
    pub unsafe fn get_namespaceSpec(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_namespaceSpec)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_data)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIApplicationCache_consts {
    pub const ITEM_MANIFEST: i64 = 1;
    pub const ITEM_EXPLICIT: i64 = 2;
    pub const ITEM_IMPLICIT: i64 = 4;
    pub const ITEM_DYNAMIC: i64 = 8;
    pub const ITEM_FOREIGN: i64 = 16;
    pub const ITEM_FALLBACK: i64 = 32;
    pub const ITEM_OPPORTUNISTIC: i64 = 64;
}


#[repr(C)]
pub struct nsIApplicationCache {
    vtable: *const nsIApplicationCacheVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationCache {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x06568dae, 0xc374, 0x4383,
            [0xa1, 0x22, 0x0c, 0xc9, 0x6c, 0x71, 0x77, 0xf2])
    }
}

unsafe impl RefCounted for nsIApplicationCache {
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
pub trait nsIApplicationCacheCoerce {
    fn coerce_from(v: &nsIApplicationCache) -> &Self;
}

impl nsIApplicationCacheCoerce for nsIApplicationCache {
    #[inline]
    fn coerce_from(v: &nsIApplicationCache) -> &Self {
        v
    }
}

impl nsIApplicationCache {
    #[inline]
    pub fn coerce<T: nsIApplicationCacheCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationCache {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIApplicationCacheCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCache) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationCacheVTable {
    pub __base: nsISupportsVTable,

    /* void initAsHandle (in ACString groupId, in ACString clientId); */
    pub initAsHandle: unsafe extern "C" fn (this: *const nsIApplicationCache, groupId: *const nsACString, clientId: *const nsACString) -> nsresult,

    /* readonly attribute nsIURI manifestURI; */
    pub get_manifestURI: unsafe extern "C" fn (this: *const nsIApplicationCache, aManifestURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute ACString groupID; */
    pub get_groupID: unsafe extern "C" fn (this: *const nsIApplicationCache, aGroupID: *mut nsACString) -> nsresult,

    /* readonly attribute ACString clientID; */
    pub get_clientID: unsafe extern "C" fn (this: *const nsIApplicationCache, aClientID: *mut nsACString) -> nsresult,

    /* readonly attribute boolean active; */
    pub get_active: unsafe extern "C" fn (this: *const nsIApplicationCache, aActive: *mut bool) -> nsresult,

    /* readonly attribute unsigned long usage; */
    pub get_usage: unsafe extern "C" fn (this: *const nsIApplicationCache, aUsage: *mut libc::uint32_t) -> nsresult,

    /* void activate (); */
    pub activate: unsafe extern "C" fn (this: *const nsIApplicationCache) -> nsresult,

    /* void discard (); */
    pub discard: unsafe extern "C" fn (this: *const nsIApplicationCache) -> nsresult,

    /* void markEntry (in ACString key, in unsigned long typeBits); */
    pub markEntry: unsafe extern "C" fn (this: *const nsIApplicationCache, key: *const nsACString, typeBits: libc::uint32_t) -> nsresult,

    /* void unmarkEntry (in ACString key, in unsigned long typeBits); */
    pub unmarkEntry: unsafe extern "C" fn (this: *const nsIApplicationCache, key: *const nsACString, typeBits: libc::uint32_t) -> nsresult,

    /* unsigned long getTypes (in ACString key); */
    pub getTypes: unsafe extern "C" fn (this: *const nsIApplicationCache, key: *const nsACString, _retval: *mut libc::uint32_t) -> nsresult,

    /* void gatherEntries (in uint32_t typeBits, out unsigned long count, [array, size_is (count)] out string keys); */
    /// Unable to call function as its signature contains a non-rust type
    pub gatherEntries: *const ::libc::c_void,

    /* void addNamespaces (in nsIArray namespaces); */
    pub addNamespaces: unsafe extern "C" fn (this: *const nsIApplicationCache, namespaces: *const nsIArray) -> nsresult,

    /* nsIApplicationCacheNamespace getMatchingNamespace (in ACString key); */
    pub getMatchingNamespace: unsafe extern "C" fn (this: *const nsIApplicationCache, key: *const nsACString, _retval: *mut *const nsIApplicationCacheNamespace) -> nsresult,

    /* readonly attribute nsIFile profileDirectory; */
    pub get_profileDirectory: unsafe extern "C" fn (this: *const nsIApplicationCache, aProfileDirectory: *mut *const nsIFile) -> nsresult,

}


impl nsIApplicationCache {
    /* void initAsHandle (in ACString groupId, in ACString clientId); */
    #[inline]
    pub unsafe fn initAsHandle(&self, groupId: &[u8], clientId: &[u8]) -> Result<(), nsresult> {
        let groupId = nsCString::from(groupId);
        let clientId = nsCString::from(clientId);
        match ((*self.vtable).initAsHandle)(self as *const _, &*groupId, &*clientId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIURI manifestURI; */
    #[inline]
    pub unsafe fn get_manifestURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_manifestURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute ACString groupID; */
    #[inline]
    pub unsafe fn get_groupID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_groupID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString clientID; */
    #[inline]
    pub unsafe fn get_clientID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_clientID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean active; */
    #[inline]
    pub unsafe fn get_active(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_active)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long usage; */
    #[inline]
    pub unsafe fn get_usage(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_usage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void activate (); */
    #[inline]
    pub unsafe fn activate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).activate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void discard (); */
    #[inline]
    pub unsafe fn discard(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).discard)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void markEntry (in ACString key, in unsigned long typeBits); */
    #[inline]
    pub unsafe fn markEntry(&self, key: &[u8], typeBits: libc::uint32_t) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).markEntry)(self as *const _, &*key, typeBits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unmarkEntry (in ACString key, in unsigned long typeBits); */
    #[inline]
    pub unsafe fn unmarkEntry(&self, key: &[u8], typeBits: libc::uint32_t) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).unmarkEntry)(self as *const _, &*key, typeBits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long getTypes (in ACString key); */
    #[inline]
    pub unsafe fn getTypes(&self, key: &[u8]) -> Result<libc::uint32_t, nsresult> {
        let key = nsCString::from(key);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getTypes)(self as *const _, &*key, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void gatherEntries (in uint32_t typeBits, out unsigned long count, [array, size_is (count)] out string keys); */


    /* void addNamespaces (in nsIArray namespaces); */
    #[inline]
    pub unsafe fn addNamespaces(&self, namespaces: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).addNamespaces)(self as *const _, namespaces.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIApplicationCacheNamespace getMatchingNamespace (in ACString key); */
    #[inline]
    pub unsafe fn getMatchingNamespace(&self, key: &[u8]) -> Result<Option<RefPtr<nsIApplicationCacheNamespace>>, nsresult> {
        let key = nsCString::from(key);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getMatchingNamespace)(self as *const _, &*key, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIFile profileDirectory; */
    #[inline]
    pub unsafe fn get_profileDirectory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_profileDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


