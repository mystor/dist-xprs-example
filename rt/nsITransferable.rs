//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransferable.idl
//


#[repr(C)]
pub struct nsIFlavorDataProvider {
    vtable: *const nsIFlavorDataProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFlavorDataProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7e225e5f, 0x711c, 0x11d7,
            [0x9f, 0xae, 0x00, 0x03, 0x93, 0x63, 0x65, 0x92])
    }
}

unsafe impl RefCounted for nsIFlavorDataProvider {
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
pub trait nsIFlavorDataProviderCoerce {
    fn coerce_from(v: &nsIFlavorDataProvider) -> &Self;
}

impl nsIFlavorDataProviderCoerce for nsIFlavorDataProvider {
    #[inline]
    fn coerce_from(v: &nsIFlavorDataProvider) -> &Self {
        v
    }
}

impl nsIFlavorDataProvider {
    #[inline]
    pub fn coerce<T: nsIFlavorDataProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFlavorDataProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFlavorDataProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFlavorDataProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFlavorDataProviderVTable {
    pub __base: nsISupportsVTable,

    /* void getFlavorData (in nsITransferable aTransferable, in string aFlavor, out nsISupports aData, out unsigned long aDataLen); */
    pub getFlavorData: unsafe extern "C" fn (this: *const nsIFlavorDataProvider, aTransferable: *const nsITransferable, aFlavor: *const libc::c_char, aData: *mut *const nsISupports, aDataLen: *mut libc::uint32_t) -> nsresult,

}


impl nsIFlavorDataProvider {
    /* void getFlavorData (in nsITransferable aTransferable, in string aFlavor, out nsISupports aData, out unsigned long aDataLen); */
    #[inline]
    pub unsafe fn getFlavorData(&self, aTransferable: Option<&nsITransferable>, aFlavor: *const libc::c_char) -> Result<(Option<RefPtr<nsISupports>>, libc::uint32_t), nsresult> {
        let mut aData = GetterAddrefs::new();
        let mut aDataLen: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getFlavorData)(self as *const _, aTransferable.map_or(::std::ptr::null(), |x| x as *const _), aFlavor, aData.ptr(), &mut aDataLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aData.refptr(), aDataLen))
    }

}


pub mod nsITransferable_consts {
    pub const kFlavorHasDataProvider: i64 = 0;
}


#[repr(C)]
pub struct nsITransferable {
    vtable: *const nsITransferableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransferable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x97e0c418, 0x1c1e, 0x4106,
            [0xba, 0xd1, 0x9f, 0xcb, 0x11, 0xdf, 0xf2, 0xfe])
    }
}

unsafe impl RefCounted for nsITransferable {
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
pub trait nsITransferableCoerce {
    fn coerce_from(v: &nsITransferable) -> &Self;
}

impl nsITransferableCoerce for nsITransferable {
    #[inline]
    fn coerce_from(v: &nsITransferable) -> &Self {
        v
    }
}

impl nsITransferable {
    #[inline]
    pub fn coerce<T: nsITransferableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransferable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransferableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransferable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransferableVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsILoadContext aContext); */
    pub init: unsafe extern "C" fn (this: *const nsITransferable, aContext: *const nsILoadContext) -> nsresult,

    /* nsIArray flavorsTransferableCanExport (); */
    pub flavorsTransferableCanExport: unsafe extern "C" fn (this: *const nsITransferable, _retval: *mut *const nsIArray) -> nsresult,

    /* void getTransferData (in string aFlavor, out nsISupports aData, out unsigned long aDataLen); */
    pub getTransferData: unsafe extern "C" fn (this: *const nsITransferable, aFlavor: *const libc::c_char, aData: *mut *const nsISupports, aDataLen: *mut libc::uint32_t) -> nsresult,

    /* void getAnyTransferData (out ACString aFlavor, out nsISupports aData, out unsigned long aDataLen); */
    pub getAnyTransferData: unsafe extern "C" fn (this: *const nsITransferable, aFlavor: *mut nsACString, aData: *mut *const nsISupports, aDataLen: *mut libc::uint32_t) -> nsresult,

    /* boolean isLargeDataSet (); */
    pub isLargeDataSet: unsafe extern "C" fn (this: *const nsITransferable, _retval: *mut bool) -> nsresult,

    /* nsIArray flavorsTransferableCanImport (); */
    pub flavorsTransferableCanImport: unsafe extern "C" fn (this: *const nsITransferable, _retval: *mut *const nsIArray) -> nsresult,

    /* void setTransferData (in string aFlavor, in nsISupports aData, in unsigned long aDataLen); */
    pub setTransferData: unsafe extern "C" fn (this: *const nsITransferable, aFlavor: *const libc::c_char, aData: *const nsISupports, aDataLen: libc::uint32_t) -> nsresult,

    /* void addDataFlavor (in string aDataFlavor); */
    pub addDataFlavor: unsafe extern "C" fn (this: *const nsITransferable, aDataFlavor: *const libc::c_char) -> nsresult,

    /* void removeDataFlavor (in string aDataFlavor); */
    pub removeDataFlavor: unsafe extern "C" fn (this: *const nsITransferable, aDataFlavor: *const libc::c_char) -> nsresult,

    /* attribute nsIFormatConverter converter; */
    pub get_converter: unsafe extern "C" fn (this: *const nsITransferable, aConverter: *mut *const nsIFormatConverter) -> nsresult,
    pub set_converter: unsafe extern "C" fn (this: *const nsITransferable, aConverter: *const nsIFormatConverter) -> nsresult,

    /* [noscript] attribute boolean isPrivateData; */
    pub get_isPrivateData: unsafe extern "C" fn (this: *const nsITransferable, aIsPrivateData: *mut bool) -> nsresult,
    pub set_isPrivateData: unsafe extern "C" fn (this: *const nsITransferable, aIsPrivateData: bool) -> nsresult,

    /* [noscript] attribute nsIPrincipal requestingPrincipal; */
    pub get_requestingPrincipal: unsafe extern "C" fn (this: *const nsITransferable, aRequestingPrincipal: *mut *const nsIPrincipal) -> nsresult,
    pub set_requestingPrincipal: unsafe extern "C" fn (this: *const nsITransferable, aRequestingPrincipal: *const nsIPrincipal) -> nsresult,

    /* [noscript] attribute nsContentPolicyType contentPolicyType; */
    pub get_contentPolicyType: unsafe extern "C" fn (this: *const nsITransferable, aContentPolicyType: *mut nsContentPolicyType) -> nsresult,
    pub set_contentPolicyType: unsafe extern "C" fn (this: *const nsITransferable, aContentPolicyType: nsContentPolicyType) -> nsresult,

}


impl nsITransferable {
    /* void init (in nsILoadContext aContext); */
    #[inline]
    pub unsafe fn init(&self, aContext: Option<&nsILoadContext>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIArray flavorsTransferableCanExport (); */
    #[inline]
    pub unsafe fn flavorsTransferableCanExport(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).flavorsTransferableCanExport)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getTransferData (in string aFlavor, out nsISupports aData, out unsigned long aDataLen); */
    #[inline]
    pub unsafe fn getTransferData(&self, aFlavor: *const libc::c_char) -> Result<(Option<RefPtr<nsISupports>>, libc::uint32_t), nsresult> {
        let mut aData = GetterAddrefs::new();
        let mut aDataLen: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getTransferData)(self as *const _, aFlavor, aData.ptr(), &mut aDataLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aData.refptr(), aDataLen))
    }

    /* void getAnyTransferData (out ACString aFlavor, out nsISupports aData, out unsigned long aDataLen); */
    #[inline]
    pub unsafe fn getAnyTransferData(&self, ) -> Result<(nsCString, Option<RefPtr<nsISupports>>, libc::uint32_t), nsresult> {
        let mut aFlavor = nsCString::new();
        let mut aData = GetterAddrefs::new();
        let mut aDataLen: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getAnyTransferData)(self as *const _, &mut *aFlavor, aData.ptr(), &mut aDataLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFlavor, aData.refptr(), aDataLen))
    }

    /* boolean isLargeDataSet (); */
    #[inline]
    pub unsafe fn isLargeDataSet(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isLargeDataSet)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIArray flavorsTransferableCanImport (); */
    #[inline]
    pub unsafe fn flavorsTransferableCanImport(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).flavorsTransferableCanImport)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setTransferData (in string aFlavor, in nsISupports aData, in unsigned long aDataLen); */
    #[inline]
    pub unsafe fn setTransferData(&self, aFlavor: *const libc::c_char, aData: Option<&nsISupports>, aDataLen: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setTransferData)(self as *const _, aFlavor, aData.map_or(::std::ptr::null(), |x| x as *const _), aDataLen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addDataFlavor (in string aDataFlavor); */
    #[inline]
    pub unsafe fn addDataFlavor(&self, aDataFlavor: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).addDataFlavor)(self as *const _, aDataFlavor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDataFlavor (in string aDataFlavor); */
    #[inline]
    pub unsafe fn removeDataFlavor(&self, aDataFlavor: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).removeDataFlavor)(self as *const _, aDataFlavor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFormatConverter converter; */
    #[inline]
    pub unsafe fn get_converter(&self, ) -> Result<Option<RefPtr<nsIFormatConverter>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_converter)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_converter(&self, aConverter: Option<&nsIFormatConverter>) -> Result<(), nsresult> {

        match ((*self.vtable).set_converter)(self as *const _, aConverter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute boolean isPrivateData; */
    #[inline]
    pub unsafe fn get_isPrivateData(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPrivateData)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isPrivateData(&self, aIsPrivateData: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isPrivateData)(self as *const _, aIsPrivateData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute nsIPrincipal requestingPrincipal; */
    #[inline]
    pub unsafe fn get_requestingPrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_requestingPrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_requestingPrincipal(&self, aRequestingPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).set_requestingPrincipal)(self as *const _, aRequestingPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute nsContentPolicyType contentPolicyType; */
    #[inline]
    pub unsafe fn get_contentPolicyType(&self, ) -> Result<nsContentPolicyType, nsresult> {
        let mut _retval: nsContentPolicyType = ::std::mem::zeroed();
        match ((*self.vtable).get_contentPolicyType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentPolicyType(&self, aContentPolicyType: nsContentPolicyType) -> Result<(), nsresult> {

        match ((*self.vtable).set_contentPolicyType)(self as *const _, aContentPolicyType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


