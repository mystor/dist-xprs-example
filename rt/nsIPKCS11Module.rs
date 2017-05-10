//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPKCS11Module.idl
//


#[repr(C)]
pub struct nsIPKCS11Module {
    vtable: *const nsIPKCS11ModuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPKCS11Module {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8a44bdf9, 0xd1a5, 0x4734,
            [0xbd, 0x5a, 0x34, 0xed, 0x7f, 0xe5, 0x64, 0xc2])
    }
}

unsafe impl RefCounted for nsIPKCS11Module {
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
pub trait nsIPKCS11ModuleCoerce {
    fn coerce_from(v: &nsIPKCS11Module) -> &Self;
}

impl nsIPKCS11ModuleCoerce for nsIPKCS11Module {
    #[inline]
    fn coerce_from(v: &nsIPKCS11Module) -> &Self {
        v
    }
}

impl nsIPKCS11Module {
    #[inline]
    pub fn coerce<T: nsIPKCS11ModuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPKCS11Module {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPKCS11ModuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPKCS11Module) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPKCS11ModuleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIPKCS11Module, aName: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String libName; */
    pub get_libName: unsafe extern "C" fn (this: *const nsIPKCS11Module, aLibName: *mut nsACString) -> nsresult,

    /* nsIPKCS11Slot findSlotByName (in AUTF8String name); */
    pub findSlotByName: unsafe extern "C" fn (this: *const nsIPKCS11Module, name: *const nsACString, _retval: *mut *const nsIPKCS11Slot) -> nsresult,

    /* nsISimpleEnumerator listSlots (); */
    pub listSlots: unsafe extern "C" fn (this: *const nsIPKCS11Module, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIPKCS11Module {
    /* readonly attribute AUTF8String name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String libName; */
    #[inline]
    pub unsafe fn get_libName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_libName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* nsISimpleEnumerator listSlots (); */
    #[inline]
    pub unsafe fn listSlots(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).listSlots)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


