//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrinterEnumerator.idl
//


#[repr(C)]
pub struct nsIPrinterEnumerator {
    vtable: *const nsIPrinterEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrinterEnumerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5e738fff, 0x404c, 0x4c94,
            [0x91, 0x89, 0xe8, 0xf2, 0xcc, 0xe9, 0x3e, 0x94])
    }
}

unsafe impl RefCounted for nsIPrinterEnumerator {
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
pub trait nsIPrinterEnumeratorCoerce {
    fn coerce_from(v: &nsIPrinterEnumerator) -> &Self;
}

impl nsIPrinterEnumeratorCoerce for nsIPrinterEnumerator {
    #[inline]
    fn coerce_from(v: &nsIPrinterEnumerator) -> &Self {
        v
    }
}

impl nsIPrinterEnumerator {
    #[inline]
    pub fn coerce<T: nsIPrinterEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrinterEnumerator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrinterEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrinterEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrinterEnumeratorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute wstring defaultPrinterName; */
    pub get_defaultPrinterName: unsafe extern "C" fn (this: *const nsIPrinterEnumerator, aDefaultPrinterName: *mut *const libc::int16_t) -> nsresult,

    /* void initPrintSettingsFromPrinter (in wstring aPrinterName, in nsIPrintSettings aPrintSettings); */
    pub initPrintSettingsFromPrinter: unsafe extern "C" fn (this: *const nsIPrinterEnumerator, aPrinterName: *const libc::int16_t, aPrintSettings: *const nsIPrintSettings) -> nsresult,

    /* readonly attribute nsIStringEnumerator printerNameList; */
    pub get_printerNameList: unsafe extern "C" fn (this: *const nsIPrinterEnumerator, aPrinterNameList: *mut *const nsIStringEnumerator) -> nsresult,

}


impl nsIPrinterEnumerator {
    /* readonly attribute wstring defaultPrinterName; */
    #[inline]
    pub unsafe fn get_defaultPrinterName(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultPrinterName)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void initPrintSettingsFromPrinter (in wstring aPrinterName, in nsIPrintSettings aPrintSettings); */
    #[inline]
    pub unsafe fn initPrintSettingsFromPrinter(&self, aPrinterName: *const libc::int16_t, aPrintSettings: Option<&nsIPrintSettings>) -> Result<(), nsresult> {

        match ((*self.vtable).initPrintSettingsFromPrinter)(self as *const _, aPrinterName, aPrintSettings.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIStringEnumerator printerNameList; */
    #[inline]
    pub unsafe fn get_printerNameList(&self, ) -> Result<Option<RefPtr<nsIStringEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_printerNameList)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


