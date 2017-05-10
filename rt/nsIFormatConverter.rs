//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormatConverter.idl
//


#[repr(C)]
pub struct nsIFormatConverter {
    vtable: *const nsIFormatConverterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFormatConverter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x948a0023, 0xe3a7, 0x11d2,
            [0x96, 0xcf, 0x00, 0x60, 0xb0, 0xfb, 0x99, 0x56])
    }
}

unsafe impl RefCounted for nsIFormatConverter {
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
pub trait nsIFormatConverterCoerce {
    fn coerce_from(v: &nsIFormatConverter) -> &Self;
}

impl nsIFormatConverterCoerce for nsIFormatConverter {
    #[inline]
    fn coerce_from(v: &nsIFormatConverter) -> &Self {
        v
    }
}

impl nsIFormatConverter {
    #[inline]
    pub fn coerce<T: nsIFormatConverterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFormatConverter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFormatConverterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormatConverter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFormatConverterVTable {
    pub __base: nsISupportsVTable,

    /* nsIArray getInputDataFlavors (); */
    pub getInputDataFlavors: unsafe extern "C" fn (this: *const nsIFormatConverter, _retval: *mut *const nsIArray) -> nsresult,

    /* nsIArray getOutputDataFlavors (); */
    pub getOutputDataFlavors: unsafe extern "C" fn (this: *const nsIFormatConverter, _retval: *mut *const nsIArray) -> nsresult,

    /* boolean canConvert (in string aFromDataFlavor, in string aToDataFlavor); */
    pub canConvert: unsafe extern "C" fn (this: *const nsIFormatConverter, aFromDataFlavor: *const libc::c_char, aToDataFlavor: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* void convert (in string aFromDataFlavor, in nsISupports aFromData, in unsigned long aDataLen, in string aToDataFlavor, out nsISupports aToData, out unsigned long aDataToLen); */
    pub convert: unsafe extern "C" fn (this: *const nsIFormatConverter, aFromDataFlavor: *const libc::c_char, aFromData: *const nsISupports, aDataLen: libc::uint32_t, aToDataFlavor: *const libc::c_char, aToData: *mut *const nsISupports, aDataToLen: *mut libc::uint32_t) -> nsresult,

}


impl nsIFormatConverter {
    /* nsIArray getInputDataFlavors (); */
    #[inline]
    pub unsafe fn getInputDataFlavors(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInputDataFlavors)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIArray getOutputDataFlavors (); */
    #[inline]
    pub unsafe fn getOutputDataFlavors(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getOutputDataFlavors)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean canConvert (in string aFromDataFlavor, in string aToDataFlavor); */
    #[inline]
    pub unsafe fn canConvert(&self, aFromDataFlavor: *const libc::c_char, aToDataFlavor: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canConvert)(self as *const _, aFromDataFlavor, aToDataFlavor, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void convert (in string aFromDataFlavor, in nsISupports aFromData, in unsigned long aDataLen, in string aToDataFlavor, out nsISupports aToData, out unsigned long aDataToLen); */
    #[inline]
    pub unsafe fn convert(&self, aFromDataFlavor: *const libc::c_char, aFromData: Option<&nsISupports>, aDataLen: libc::uint32_t, aToDataFlavor: *const libc::c_char) -> Result<(Option<RefPtr<nsISupports>>, libc::uint32_t), nsresult> {
        let mut aToData = GetterAddrefs::new();
        let mut aDataToLen: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).convert)(self as *const _, aFromDataFlavor, aFromData.map_or(::std::ptr::null(), |x| x as *const _), aDataLen, aToDataFlavor, aToData.ptr(), &mut aDataToLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aToData.refptr(), aDataToLen))
    }

}


