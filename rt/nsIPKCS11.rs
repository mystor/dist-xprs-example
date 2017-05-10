//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPKCS11.idl
//


#[repr(C)]
pub struct nsIPKCS11 {
    vtable: *const nsIPKCS11VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPKCS11 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5743f870, 0x958e, 0x4f02,
            [0xae, 0xf2, 0xc0, 0xaf, 0xee, 0xf6, 0x7f, 0x05])
    }
}

unsafe impl RefCounted for nsIPKCS11 {
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
pub trait nsIPKCS11Coerce {
    fn coerce_from(v: &nsIPKCS11) -> &Self;
}

impl nsIPKCS11Coerce for nsIPKCS11 {
    #[inline]
    fn coerce_from(v: &nsIPKCS11) -> &Self {
        v
    }
}

impl nsIPKCS11 {
    #[inline]
    pub fn coerce<T: nsIPKCS11Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPKCS11 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPKCS11Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIPKCS11) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPKCS11VTable {
    pub __base: nsISupportsVTable,

    /* void deleteModule (in AString moduleName); */
    pub deleteModule: unsafe extern "C" fn (this: *const nsIPKCS11, moduleName: *const nsAString) -> nsresult,

    /* void addModule (in AString moduleName, in AString libraryFullPath, in long cryptoMechanismFlags, in long cipherFlags); */
    pub addModule: unsafe extern "C" fn (this: *const nsIPKCS11, moduleName: *const nsAString, libraryFullPath: *const nsAString, cryptoMechanismFlags: libc::int32_t, cipherFlags: libc::int32_t) -> nsresult,

}


impl nsIPKCS11 {
    /* void deleteModule (in AString moduleName); */
    #[inline]
    pub unsafe fn deleteModule(&self, moduleName: &[u16]) -> Result<(), nsresult> {
        let moduleName = nsString::from(moduleName);
        match ((*self.vtable).deleteModule)(self as *const _, &*moduleName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addModule (in AString moduleName, in AString libraryFullPath, in long cryptoMechanismFlags, in long cipherFlags); */
    #[inline]
    pub unsafe fn addModule(&self, moduleName: &[u16], libraryFullPath: &[u16], cryptoMechanismFlags: libc::int32_t, cipherFlags: libc::int32_t) -> Result<(), nsresult> {
        let moduleName = nsString::from(moduleName);
        let libraryFullPath = nsString::from(libraryFullPath);
        match ((*self.vtable).addModule)(self as *const _, &*moduleName, &*libraryFullPath, cryptoMechanismFlags, cipherFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


