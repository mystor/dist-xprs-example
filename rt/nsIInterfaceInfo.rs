//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInterfaceInfo.idl
//


#[repr(C)]
pub struct nsIInterfaceInfo {
    vtable: *const nsIInterfaceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInterfaceInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3820e663, 0x8e22, 0x4789,
            [0xb4, 0x70, 0x56, 0xbc, 0xf7, 0x08, 0x3f, 0x2b])
    }
}

unsafe impl RefCounted for nsIInterfaceInfo {
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
pub trait nsIInterfaceInfoCoerce {
    fn coerce_from(v: &nsIInterfaceInfo) -> &Self;
}

impl nsIInterfaceInfoCoerce for nsIInterfaceInfo {
    #[inline]
    fn coerce_from(v: &nsIInterfaceInfo) -> &Self {
        v
    }
}

impl nsIInterfaceInfo {
    #[inline]
    pub fn coerce<T: nsIInterfaceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInterfaceInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInterfaceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInterfaceInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInterfaceInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIInterfaceInfo, aName: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute nsIIDPtr InterfaceIID; */
    pub get_InterfaceIID: unsafe extern "C" fn (this: *const nsIInterfaceInfo, aInterfaceIID: *mut *const nsIID) -> nsresult,

    /* boolean isScriptable (); */
    pub isScriptable: unsafe extern "C" fn (this: *const nsIInterfaceInfo, _retval: *mut bool) -> nsresult,

    /* boolean isBuiltinClass (); */
    pub isBuiltinClass: unsafe extern "C" fn (this: *const nsIInterfaceInfo, _retval: *mut bool) -> nsresult,

    /* readonly attribute nsIInterfaceInfo parent; */
    pub get_parent: unsafe extern "C" fn (this: *const nsIInterfaceInfo, aParent: *mut *const nsIInterfaceInfo) -> nsresult,

    /* readonly attribute uint16_t methodCount; */
    pub get_methodCount: unsafe extern "C" fn (this: *const nsIInterfaceInfo, aMethodCount: *mut uint16_t) -> nsresult,

    /* readonly attribute uint16_t constantCount; */
    pub get_constantCount: unsafe extern "C" fn (this: *const nsIInterfaceInfo, aConstantCount: *mut uint16_t) -> nsresult,

    /* void getMethodInfo (in uint16_t index, [shared, retval] out nsXPTMethodInfoPtr info); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMethodInfo: *const ::libc::c_void,

    /* void getMethodInfoForName (in string methodName, out uint16_t index, [shared, retval] out nsXPTMethodInfoPtr info); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMethodInfoForName: *const ::libc::c_void,

    /* void getConstant (in uint16_t index, out jsval constant, out string name); */
    /// Unable to call function as its signature contains a non-rust type
    pub getConstant: *const ::libc::c_void,

    /* nsIInterfaceInfo getInfoForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param); */
    /// Unable to call function as its signature contains a non-rust type
    pub getInfoForParam: *const ::libc::c_void,

    /* nsIIDPtr getIIDForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param); */
    /// Unable to call function as its signature contains a non-rust type
    pub getIIDForParam: *const ::libc::c_void,

    /* nsXPTType getTypeForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param, in uint16_t dimension); */
    /// Unable to call function as its signature contains a non-rust type
    pub getTypeForParam: *const ::libc::c_void,

    /* uint8_t getSizeIsArgNumberForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param, in uint16_t dimension); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSizeIsArgNumberForParam: *const ::libc::c_void,

    /* uint8_t getInterfaceIsArgNumberForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param); */
    /// Unable to call function as its signature contains a non-rust type
    pub getInterfaceIsArgNumberForParam: *const ::libc::c_void,

    /* boolean isIID (in nsIIDPtr IID); */
    pub isIID: unsafe extern "C" fn (this: *const nsIInterfaceInfo, IID: *const nsIID, _retval: *mut bool) -> nsresult,

    /* void getNameShared ([shared, retval] out string name); */
    pub getNameShared: unsafe extern "C" fn (this: *const nsIInterfaceInfo, name: *mut *const libc::c_char) -> nsresult,

    /* void getIIDShared ([shared, retval] out nsIIDPtrShared iid); */
    pub getIIDShared: unsafe extern "C" fn (this: *const nsIInterfaceInfo, iid: *mut *const nsIID) -> nsresult,

    /* boolean isFunction (); */
    pub isFunction: unsafe extern "C" fn (this: *const nsIInterfaceInfo, _retval: *mut bool) -> nsresult,

    /* boolean hasAncestor (in nsIIDPtr iid); */
    pub hasAncestor: unsafe extern "C" fn (this: *const nsIInterfaceInfo, iid: *const nsIID, _retval: *mut bool) -> nsresult,

    /* [notxpcom] nsresult getIIDForParamNoAlloc (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param, out nsIID iid); */
    /// Unable to call function as its signature contains a non-rust type
    pub getIIDForParamNoAlloc: *const ::libc::c_void,

    /* boolean isMainProcessScriptableOnly (); */
    pub isMainProcessScriptableOnly: unsafe extern "C" fn (this: *const nsIInterfaceInfo, _retval: *mut bool) -> nsresult,

}


impl nsIInterfaceInfo {
    /* readonly attribute string name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_name)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIIDPtr InterfaceIID; */
    #[inline]
    pub unsafe fn get_InterfaceIID(&self, ) -> Result<*const nsIID, nsresult> {
        let mut _retval: *const nsIID = ::std::ptr::null();
        match ((*self.vtable).get_InterfaceIID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isScriptable (); */
    #[inline]
    pub unsafe fn isScriptable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isScriptable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isBuiltinClass (); */
    #[inline]
    pub unsafe fn isBuiltinClass(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isBuiltinClass)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIInterfaceInfo parent; */
    #[inline]
    pub unsafe fn get_parent(&self, ) -> Result<Option<RefPtr<nsIInterfaceInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute uint16_t methodCount; */
    #[inline]
    pub unsafe fn get_methodCount(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_methodCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint16_t constantCount; */
    #[inline]
    pub unsafe fn get_constantCount(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_constantCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getMethodInfo (in uint16_t index, [shared, retval] out nsXPTMethodInfoPtr info); */


    /* void getMethodInfoForName (in string methodName, out uint16_t index, [shared, retval] out nsXPTMethodInfoPtr info); */


    /* void getConstant (in uint16_t index, out jsval constant, out string name); */


    /* nsIInterfaceInfo getInfoForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param); */


    /* nsIIDPtr getIIDForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param); */


    /* nsXPTType getTypeForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param, in uint16_t dimension); */


    /* uint8_t getSizeIsArgNumberForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param, in uint16_t dimension); */


    /* uint8_t getInterfaceIsArgNumberForParam (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param); */


    /* boolean isIID (in nsIIDPtr IID); */
    #[inline]
    pub unsafe fn isIID(&self, IID: *const nsIID) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isIID)(self as *const _, IID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getNameShared ([shared, retval] out string name); */
    #[inline]
    pub unsafe fn getNameShared(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut name: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getNameShared)(self as *const _, &mut name as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(name)
    }

    /* void getIIDShared ([shared, retval] out nsIIDPtrShared iid); */
    #[inline]
    pub unsafe fn getIIDShared(&self, ) -> Result<*const nsIID, nsresult> {
        let mut iid: *const nsIID = ::std::ptr::null();
        match ((*self.vtable).getIIDShared)(self as *const _, &mut iid as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(iid)
    }

    /* boolean isFunction (); */
    #[inline]
    pub unsafe fn isFunction(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isFunction)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean hasAncestor (in nsIIDPtr iid); */
    #[inline]
    pub unsafe fn hasAncestor(&self, iid: *const nsIID) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasAncestor)(self as *const _, iid, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [notxpcom] nsresult getIIDForParamNoAlloc (in uint16_t methodIndex, [const] in nsXPTParamInfoPtr param, out nsIID iid); */


    /* boolean isMainProcessScriptableOnly (); */
    #[inline]
    pub unsafe fn isMainProcessScriptableOnly(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isMainProcessScriptableOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


