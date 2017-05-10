//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibilityService.idl
//


#[repr(C)]
pub struct nsIAccessibilityService {
    vtable: *const nsIAccessibilityServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibilityService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9a6f80fe, 0x25cc, 0x405c,
            [0x9f, 0x8f, 0x25, 0x86, 0x9b, 0xc9, 0xf9, 0x4e])
    }
}

unsafe impl RefCounted for nsIAccessibilityService {
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
pub trait nsIAccessibilityServiceCoerce {
    fn coerce_from(v: &nsIAccessibilityService) -> &Self;
}

impl nsIAccessibilityServiceCoerce for nsIAccessibilityService {
    #[inline]
    fn coerce_from(v: &nsIAccessibilityService) -> &Self {
        v
    }
}

impl nsIAccessibilityService {
    #[inline]
    pub fn coerce<T: nsIAccessibilityServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibilityService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibilityServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibilityService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibilityServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIAccessible getApplicationAccessible (); */
    pub getApplicationAccessible: unsafe extern "C" fn (this: *const nsIAccessibilityService, _retval: *mut *const nsIAccessible) -> nsresult,

    /* nsIAccessible getAccessibleFor (in nsIDOMNode aNode); */
    pub getAccessibleFor: unsafe extern "C" fn (this: *const nsIAccessibilityService, aNode: *const nsIDOMNode, _retval: *mut *const nsIAccessible) -> nsresult,

    /* AString getStringRole (in unsigned long aRole); */
    pub getStringRole: unsafe extern "C" fn (this: *const nsIAccessibilityService, aRole: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* nsISupports getStringStates (in unsigned long aStates, in unsigned long aExtraStates); */
    pub getStringStates: unsafe extern "C" fn (this: *const nsIAccessibilityService, aStates: libc::uint32_t, aExtraStates: libc::uint32_t, _retval: *mut *const nsISupports) -> nsresult,

    /* AString getStringEventType (in unsigned long aEventType); */
    pub getStringEventType: unsafe extern "C" fn (this: *const nsIAccessibilityService, aEventType: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getStringRelationType (in unsigned long aRelationType); */
    pub getStringRelationType: unsafe extern "C" fn (this: *const nsIAccessibilityService, aRelationType: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* nsIAccessible getAccessibleFromCache (in nsIDOMNode aNode); */
    pub getAccessibleFromCache: unsafe extern "C" fn (this: *const nsIAccessibilityService, aNode: *const nsIDOMNode, _retval: *mut *const nsIAccessible) -> nsresult,

    /* nsIAccessiblePivot createAccessiblePivot (in nsIAccessible aRoot); */
    pub createAccessiblePivot: unsafe extern "C" fn (this: *const nsIAccessibilityService, aRoot: *const nsIAccessible, _retval: *mut *const nsIAccessiblePivot) -> nsresult,

    /* void setLogging (in ACString aModules); */
    pub setLogging: unsafe extern "C" fn (this: *const nsIAccessibilityService, aModules: *const nsACString) -> nsresult,

    /* boolean isLogged (in AString aModule); */
    pub isLogged: unsafe extern "C" fn (this: *const nsIAccessibilityService, aModule: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl nsIAccessibilityService {
    /* nsIAccessible getApplicationAccessible (); */
    #[inline]
    pub unsafe fn getApplicationAccessible(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getApplicationAccessible)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessible getAccessibleFor (in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn getAccessibleFor(&self, aNode: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAccessibleFor)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AString getStringRole (in unsigned long aRole); */
    #[inline]
    pub unsafe fn getStringRole(&self, aRole: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getStringRole)(self as *const _, aRole, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports getStringStates (in unsigned long aStates, in unsigned long aExtraStates); */
    #[inline]
    pub unsafe fn getStringStates(&self, aStates: libc::uint32_t, aExtraStates: libc::uint32_t) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getStringStates)(self as *const _, aStates, aExtraStates, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AString getStringEventType (in unsigned long aEventType); */
    #[inline]
    pub unsafe fn getStringEventType(&self, aEventType: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getStringEventType)(self as *const _, aEventType, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getStringRelationType (in unsigned long aRelationType); */
    #[inline]
    pub unsafe fn getStringRelationType(&self, aRelationType: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getStringRelationType)(self as *const _, aRelationType, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIAccessible getAccessibleFromCache (in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn getAccessibleFromCache(&self, aNode: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAccessibleFromCache)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessiblePivot createAccessiblePivot (in nsIAccessible aRoot); */
    #[inline]
    pub unsafe fn createAccessiblePivot(&self, aRoot: Option<&nsIAccessible>) -> Result<Option<RefPtr<nsIAccessiblePivot>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createAccessiblePivot)(self as *const _, aRoot.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setLogging (in ACString aModules); */
    #[inline]
    pub unsafe fn setLogging(&self, aModules: &[u8]) -> Result<(), nsresult> {
        let aModules = nsCString::from(aModules);
        match ((*self.vtable).setLogging)(self as *const _, &*aModules) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isLogged (in AString aModule); */
    #[inline]
    pub unsafe fn isLogged(&self, aModule: &[u16]) -> Result<bool, nsresult> {
        let aModule = nsString::from(aModule);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isLogged)(self as *const _, &*aModule, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIAccessibleRetrieval {
    vtable: *const nsIAccessibleRetrievalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleRetrieval {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd85e0cbe, 0x47ce, 0x490c,
            [0x84, 0x88, 0xf8, 0x21, 0xdd, 0x2b, 0xe0, 0xc2])
    }
}

unsafe impl RefCounted for nsIAccessibleRetrieval {
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
pub trait nsIAccessibleRetrievalCoerce {
    fn coerce_from(v: &nsIAccessibleRetrieval) -> &Self;
}

impl nsIAccessibleRetrievalCoerce for nsIAccessibleRetrieval {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRetrieval) -> &Self {
        v
    }
}

impl nsIAccessibleRetrieval {
    #[inline]
    pub fn coerce<T: nsIAccessibleRetrievalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleRetrieval {
    type Target = nsIAccessibilityService;
    #[inline]
    fn deref(&self) -> &nsIAccessibilityService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAccessibilityServiceCoerce> nsIAccessibleRetrievalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRetrieval) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleRetrievalVTable {
    pub __base: nsIAccessibilityServiceVTable,

}


impl nsIAccessibleRetrieval {
}


