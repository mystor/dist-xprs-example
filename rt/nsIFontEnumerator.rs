//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFontEnumerator.idl
//


#[repr(C)]
pub struct nsIFontEnumerator {
    vtable: *const nsIFontEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFontEnumerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x924d98d9, 0x3518, 0x4cb4,
            [0x87, 0x08, 0xc7, 0x4f, 0xe8, 0xe3, 0xec, 0x3c])
    }
}

unsafe impl RefCounted for nsIFontEnumerator {
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
pub trait nsIFontEnumeratorCoerce {
    fn coerce_from(v: &nsIFontEnumerator) -> &Self;
}

impl nsIFontEnumeratorCoerce for nsIFontEnumerator {
    #[inline]
    fn coerce_from(v: &nsIFontEnumerator) -> &Self {
        v
    }
}

impl nsIFontEnumerator {
    #[inline]
    pub fn coerce<T: nsIFontEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFontEnumerator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFontEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFontEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFontEnumeratorVTable {
    pub __base: nsISupportsVTable,

    /* void EnumerateAllFonts (out uint32_t aCount, [array, size_is (aCount), retval] out wstring aResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub EnumerateAllFonts: *const ::libc::c_void,

    /* void EnumerateFonts (in string aLangGroup, in string aGeneric, out uint32_t aCount, [array, size_is (aCount), retval] out wstring aResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub EnumerateFonts: *const ::libc::c_void,

    /* void HaveFontFor (in string aLangGroup, [retval] out boolean aResult); */
    pub HaveFontFor: unsafe extern "C" fn (this: *const nsIFontEnumerator, aLangGroup: *const libc::c_char, aResult: *mut bool) -> nsresult,

    /* wstring getDefaultFont (in string aLangGroup, in string aGeneric); */
    pub getDefaultFont: unsafe extern "C" fn (this: *const nsIFontEnumerator, aLangGroup: *const libc::c_char, aGeneric: *const libc::c_char, _retval: *mut *const libc::int16_t) -> nsresult,

    /* boolean updateFontList (); */
    pub updateFontList: unsafe extern "C" fn (this: *const nsIFontEnumerator, _retval: *mut bool) -> nsresult,

    /* wstring getStandardFamilyName (in wstring aName); */
    pub getStandardFamilyName: unsafe extern "C" fn (this: *const nsIFontEnumerator, aName: *const libc::int16_t, _retval: *mut *const libc::int16_t) -> nsresult,

}


impl nsIFontEnumerator {
    /* void EnumerateAllFonts (out uint32_t aCount, [array, size_is (aCount), retval] out wstring aResult); */


    /* void EnumerateFonts (in string aLangGroup, in string aGeneric, out uint32_t aCount, [array, size_is (aCount), retval] out wstring aResult); */


    /* void HaveFontFor (in string aLangGroup, [retval] out boolean aResult); */
    #[inline]
    pub unsafe fn HaveFontFor(&self, aLangGroup: *const libc::c_char) -> Result<bool, nsresult> {
        let mut aResult: bool = ::std::mem::zeroed();
        match ((*self.vtable).HaveFontFor)(self as *const _, aLangGroup, &mut aResult as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aResult)
    }

    /* wstring getDefaultFont (in string aLangGroup, in string aGeneric); */
    #[inline]
    pub unsafe fn getDefaultFont(&self, aLangGroup: *const libc::c_char, aGeneric: *const libc::c_char) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getDefaultFont)(self as *const _, aLangGroup, aGeneric, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean updateFontList (); */
    #[inline]
    pub unsafe fn updateFontList(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).updateFontList)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring getStandardFamilyName (in wstring aName); */
    #[inline]
    pub unsafe fn getStandardFamilyName(&self, aName: *const libc::int16_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getStandardFamilyName)(self as *const _, aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


