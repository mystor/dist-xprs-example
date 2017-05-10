//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPrefService.idl
//


#[repr(C)]
pub struct nsIContentPrefObserver {
    vtable: *const nsIContentPrefObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPrefObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x43635c53, 0xb445, 0x4c4e,
            [0x8c, 0xc5, 0x56, 0x26, 0x97, 0x29, 0x9b, 0x55])
    }
}

unsafe impl RefCounted for nsIContentPrefObserver {
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
pub trait nsIContentPrefObserverCoerce {
    fn coerce_from(v: &nsIContentPrefObserver) -> &Self;
}

impl nsIContentPrefObserverCoerce for nsIContentPrefObserver {
    #[inline]
    fn coerce_from(v: &nsIContentPrefObserver) -> &Self {
        v
    }
}

impl nsIContentPrefObserver {
    #[inline]
    pub fn coerce<T: nsIContentPrefObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPrefObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPrefObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPrefObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPrefObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onContentPrefSet (in AString aGroup, in AString aName, in nsIVariant aValue, [optional] in boolean aIsPrivate); */
    pub onContentPrefSet: unsafe extern "C" fn (this: *const nsIContentPrefObserver, aGroup: *const nsAString, aName: *const nsAString, aValue: *const nsIVariant, aIsPrivate: bool) -> nsresult,

    /* void onContentPrefRemoved (in AString aGroup, in AString aName, [optional] in boolean aIsPrivate); */
    pub onContentPrefRemoved: unsafe extern "C" fn (this: *const nsIContentPrefObserver, aGroup: *const nsAString, aName: *const nsAString, aIsPrivate: bool) -> nsresult,

}


impl nsIContentPrefObserver {
    /* void onContentPrefSet (in AString aGroup, in AString aName, in nsIVariant aValue, [optional] in boolean aIsPrivate); */
    #[inline]
    pub unsafe fn onContentPrefSet(&self, aGroup: &[u16], aName: &[u16], aValue: Option<&nsIVariant>, aIsPrivate: bool) -> Result<(), nsresult> {
        let aGroup = nsString::from(aGroup);
        let aName = nsString::from(aName);
        match ((*self.vtable).onContentPrefSet)(self as *const _, &*aGroup, &*aName, aValue.map_or(::std::ptr::null(), |x| x as *const _), aIsPrivate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onContentPrefRemoved (in AString aGroup, in AString aName, [optional] in boolean aIsPrivate); */
    #[inline]
    pub unsafe fn onContentPrefRemoved(&self, aGroup: &[u16], aName: &[u16], aIsPrivate: bool) -> Result<(), nsresult> {
        let aGroup = nsString::from(aGroup);
        let aName = nsString::from(aName);
        match ((*self.vtable).onContentPrefRemoved)(self as *const _, &*aGroup, &*aName, aIsPrivate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIContentPrefCallback {
    vtable: *const nsIContentPrefCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPrefCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc1b3d6df, 0x5373, 0x4606,
            [0x84, 0x94, 0x8b, 0xcf, 0x14, 0xa7, 0xfc, 0x62])
    }
}

unsafe impl RefCounted for nsIContentPrefCallback {
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
pub trait nsIContentPrefCallbackCoerce {
    fn coerce_from(v: &nsIContentPrefCallback) -> &Self;
}

impl nsIContentPrefCallbackCoerce for nsIContentPrefCallback {
    #[inline]
    fn coerce_from(v: &nsIContentPrefCallback) -> &Self {
        v
    }
}

impl nsIContentPrefCallback {
    #[inline]
    pub fn coerce<T: nsIContentPrefCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPrefCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPrefCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPrefCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPrefCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onResult (in nsIVariant aResult); */
    pub onResult: unsafe extern "C" fn (this: *const nsIContentPrefCallback, aResult: *const nsIVariant) -> nsresult,

}


impl nsIContentPrefCallback {
    /* void onResult (in nsIVariant aResult); */
    #[inline]
    pub unsafe fn onResult(&self, aResult: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).onResult)(self as *const _, aResult.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIContentPrefService {
    vtable: *const nsIContentPrefServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPrefService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe3f772f3, 0x023f, 0x4b32,
            [0xb0, 0x74, 0x36, 0xcf, 0x0f, 0xd5, 0xd4, 0x14])
    }
}

unsafe impl RefCounted for nsIContentPrefService {
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
pub trait nsIContentPrefServiceCoerce {
    fn coerce_from(v: &nsIContentPrefService) -> &Self;
}

impl nsIContentPrefServiceCoerce for nsIContentPrefService {
    #[inline]
    fn coerce_from(v: &nsIContentPrefService) -> &Self {
        v
    }
}

impl nsIContentPrefService {
    #[inline]
    pub fn coerce<T: nsIContentPrefServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPrefService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPrefServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPrefService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPrefServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIVariant getPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aPrivacyContext, [optional] in nsIContentPrefCallback aCallback); */
    pub getPref: unsafe extern "C" fn (this: *const nsIContentPrefService, aGroup: *const nsIVariant, aName: *const nsAString, aPrivacyContext: *const nsILoadContext, aCallback: *const nsIContentPrefCallback, _retval: *mut *const nsIVariant) -> nsresult,

    /* void setPref (in nsIVariant aGroup, in AString aName, in nsIVariant aValue, in nsILoadContext aPrivacyContext); */
    pub setPref: unsafe extern "C" fn (this: *const nsIContentPrefService, aGroup: *const nsIVariant, aName: *const nsAString, aValue: *const nsIVariant, aPrivacyContext: *const nsILoadContext) -> nsresult,

    /* boolean hasPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
    pub hasPref: unsafe extern "C" fn (this: *const nsIContentPrefService, aGroup: *const nsIVariant, aName: *const nsAString, aContext: *const nsILoadContext, _retval: *mut bool) -> nsresult,

    /* boolean hasCachedPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
    pub hasCachedPref: unsafe extern "C" fn (this: *const nsIContentPrefService, aGroup: *const nsIVariant, aName: *const nsAString, aContext: *const nsILoadContext, _retval: *mut bool) -> nsresult,

    /* void removePref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
    pub removePref: unsafe extern "C" fn (this: *const nsIContentPrefService, aGroup: *const nsIVariant, aName: *const nsAString, aContext: *const nsILoadContext) -> nsresult,

    /* void removeGroupedPrefs (in nsILoadContext aContext); */
    pub removeGroupedPrefs: unsafe extern "C" fn (this: *const nsIContentPrefService, aContext: *const nsILoadContext) -> nsresult,

    /* void removePrefsByName (in AString aName, in nsILoadContext aContext); */
    pub removePrefsByName: unsafe extern "C" fn (this: *const nsIContentPrefService, aName: *const nsAString, aContext: *const nsILoadContext) -> nsresult,

    /* nsIPropertyBag2 getPrefs (in nsIVariant aGroup, in nsILoadContext aContext); */
    pub getPrefs: unsafe extern "C" fn (this: *const nsIContentPrefService, aGroup: *const nsIVariant, aContext: *const nsILoadContext, _retval: *mut *const nsIPropertyBag2) -> nsresult,

    /* nsIPropertyBag2 getPrefsByName (in AString aName, in nsILoadContext aContext); */
    pub getPrefsByName: unsafe extern "C" fn (this: *const nsIContentPrefService, aName: *const nsAString, aContext: *const nsILoadContext, _retval: *mut *const nsIPropertyBag2) -> nsresult,

    /* void addObserver (in AString aName, in nsIContentPrefObserver aObserver); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIContentPrefService, aName: *const nsAString, aObserver: *const nsIContentPrefObserver) -> nsresult,

    /* void removeObserver (in AString aName, in nsIContentPrefObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIContentPrefService, aName: *const nsAString, aObserver: *const nsIContentPrefObserver) -> nsresult,

    /* readonly attribute nsIContentURIGrouper grouper; */
    pub get_grouper: unsafe extern "C" fn (this: *const nsIContentPrefService, aGrouper: *mut *const nsIContentURIGrouper) -> nsresult,

    /* readonly attribute mozIStorageConnection DBConnection; */
    pub get_DBConnection: unsafe extern "C" fn (this: *const nsIContentPrefService, aDBConnection: *mut *const mozIStorageConnection) -> nsresult,

}


impl nsIContentPrefService {
    /* nsIVariant getPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aPrivacyContext, [optional] in nsIContentPrefCallback aCallback); */
    #[inline]
    pub unsafe fn getPref(&self, aGroup: Option<&nsIVariant>, aName: &[u16], aPrivacyContext: Option<&nsILoadContext>, aCallback: Option<&nsIContentPrefCallback>) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPref)(self as *const _, aGroup.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aPrivacyContext.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setPref (in nsIVariant aGroup, in AString aName, in nsIVariant aValue, in nsILoadContext aPrivacyContext); */
    #[inline]
    pub unsafe fn setPref(&self, aGroup: Option<&nsIVariant>, aName: &[u16], aValue: Option<&nsIVariant>, aPrivacyContext: Option<&nsILoadContext>) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).setPref)(self as *const _, aGroup.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aValue.map_or(::std::ptr::null(), |x| x as *const _), aPrivacyContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
    #[inline]
    pub unsafe fn hasPref(&self, aGroup: Option<&nsIVariant>, aName: &[u16], aContext: Option<&nsILoadContext>) -> Result<bool, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasPref)(self as *const _, aGroup.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aContext.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean hasCachedPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
    #[inline]
    pub unsafe fn hasCachedPref(&self, aGroup: Option<&nsIVariant>, aName: &[u16], aContext: Option<&nsILoadContext>) -> Result<bool, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasCachedPref)(self as *const _, aGroup.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aContext.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void removePref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
    #[inline]
    pub unsafe fn removePref(&self, aGroup: Option<&nsIVariant>, aName: &[u16], aContext: Option<&nsILoadContext>) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).removePref)(self as *const _, aGroup.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeGroupedPrefs (in nsILoadContext aContext); */
    #[inline]
    pub unsafe fn removeGroupedPrefs(&self, aContext: Option<&nsILoadContext>) -> Result<(), nsresult> {

        match ((*self.vtable).removeGroupedPrefs)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removePrefsByName (in AString aName, in nsILoadContext aContext); */
    #[inline]
    pub unsafe fn removePrefsByName(&self, aName: &[u16], aContext: Option<&nsILoadContext>) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).removePrefsByName)(self as *const _, &*aName, aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIPropertyBag2 getPrefs (in nsIVariant aGroup, in nsILoadContext aContext); */
    #[inline]
    pub unsafe fn getPrefs(&self, aGroup: Option<&nsIVariant>, aContext: Option<&nsILoadContext>) -> Result<Option<RefPtr<nsIPropertyBag2>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPrefs)(self as *const _, aGroup.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPropertyBag2 getPrefsByName (in AString aName, in nsILoadContext aContext); */
    #[inline]
    pub unsafe fn getPrefsByName(&self, aName: &[u16], aContext: Option<&nsILoadContext>) -> Result<Option<RefPtr<nsIPropertyBag2>>, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPrefsByName)(self as *const _, &*aName, aContext.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addObserver (in AString aName, in nsIContentPrefObserver aObserver); */
    #[inline]
    pub unsafe fn addObserver(&self, aName: &[u16], aObserver: Option<&nsIContentPrefObserver>) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).addObserver)(self as *const _, &*aName, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in AString aName, in nsIContentPrefObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aName: &[u16], aObserver: Option<&nsIContentPrefObserver>) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).removeObserver)(self as *const _, &*aName, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIContentURIGrouper grouper; */
    #[inline]
    pub unsafe fn get_grouper(&self, ) -> Result<Option<RefPtr<nsIContentURIGrouper>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_grouper)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIStorageConnection DBConnection; */
    #[inline]
    pub unsafe fn get_DBConnection(&self, ) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DBConnection)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


