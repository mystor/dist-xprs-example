//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPKCS11ModuleDB.idl
//


#[repr(C)]
pub struct nsIPKCS11ModuleDB {
    vtable: *const nsIPKCS11ModuleDBVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPKCS11ModuleDB {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xff9fbcd7, 0x9517, 0x4334,
            [0xb9, 0x7a, 0xce, 0xed, 0x78, 0x90, 0x99, 0x74])
    }
}

unsafe impl RefCounted for nsIPKCS11ModuleDB {
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
pub trait nsIPKCS11ModuleDBCoerce {
    fn coerce_from(v: &nsIPKCS11ModuleDB) -> &Self;
}

impl nsIPKCS11ModuleDBCoerce for nsIPKCS11ModuleDB {
    #[inline]
    fn coerce_from(v: &nsIPKCS11ModuleDB) -> &Self {
        v
    }
}

impl nsIPKCS11ModuleDB {
    #[inline]
    pub fn coerce<T: nsIPKCS11ModuleDBCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPKCS11ModuleDB {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPKCS11ModuleDBCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPKCS11ModuleDB) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPKCS11ModuleDBVTable {
    pub __base: nsISupportsVTable,

    /* nsIPKCS11Module getInternal (); */
    pub getInternal: unsafe extern "C" fn (this: *const nsIPKCS11ModuleDB, _retval: *mut *const nsIPKCS11Module) -> nsresult,

    /* nsIPKCS11Module getInternalFIPS (); */
    pub getInternalFIPS: unsafe extern "C" fn (this: *const nsIPKCS11ModuleDB, _retval: *mut *const nsIPKCS11Module) -> nsresult,

    /* nsIPKCS11Module findModuleByName (in AUTF8String name); */
    pub findModuleByName: unsafe extern "C" fn (this: *const nsIPKCS11ModuleDB, name: *const nsACString, _retval: *mut *const nsIPKCS11Module) -> nsresult,

    /* nsIPKCS11Slot findSlotByName (in AUTF8String name); */
    pub findSlotByName: unsafe extern "C" fn (this: *const nsIPKCS11ModuleDB, name: *const nsACString, _retval: *mut *const nsIPKCS11Slot) -> nsresult,

    /* nsISimpleEnumerator listModules (); */
    pub listModules: unsafe extern "C" fn (this: *const nsIPKCS11ModuleDB, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* readonly attribute boolean canToggleFIPS; */
    pub get_canToggleFIPS: unsafe extern "C" fn (this: *const nsIPKCS11ModuleDB, aCanToggleFIPS: *mut bool) -> nsresult,

    /* void toggleFIPSMode (); */
    pub toggleFIPSMode: unsafe extern "C" fn (this: *const nsIPKCS11ModuleDB) -> nsresult,

    /* readonly attribute boolean isFIPSEnabled; */
    pub get_isFIPSEnabled: unsafe extern "C" fn (this: *const nsIPKCS11ModuleDB, aIsFIPSEnabled: *mut bool) -> nsresult,

}


impl nsIPKCS11ModuleDB {
    /* nsIPKCS11Module getInternal (); */
    #[inline]
    pub unsafe fn getInternal(&self, ) -> Result<Option<RefPtr<nsIPKCS11Module>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInternal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPKCS11Module getInternalFIPS (); */
    #[inline]
    pub unsafe fn getInternalFIPS(&self, ) -> Result<Option<RefPtr<nsIPKCS11Module>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInternalFIPS)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPKCS11Module findModuleByName (in AUTF8String name); */
    #[inline]
    pub unsafe fn findModuleByName(&self, name: &[u8]) -> Result<Option<RefPtr<nsIPKCS11Module>>, nsresult> {
        let name = nsCString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findModuleByName)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPKCS11Slot findSlotByName (in AUTF8String name); */
    #[inline]
    pub unsafe fn findSlotByName(&self, name: &[u8]) -> Result<Option<RefPtr<nsIPKCS11Slot>>, nsresult> {
        let name = nsCString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findSlotByName)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator listModules (); */
    #[inline]
    pub unsafe fn listModules(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).listModules)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean canToggleFIPS; */
    #[inline]
    pub unsafe fn get_canToggleFIPS(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canToggleFIPS)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void toggleFIPSMode (); */
    #[inline]
    pub unsafe fn toggleFIPSMode(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).toggleFIPSMode)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isFIPSEnabled; */
    #[inline]
    pub unsafe fn get_isFIPSEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isFIPSEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


