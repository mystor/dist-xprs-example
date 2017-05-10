//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMIMEInfo.idl
//


pub type nsHandlerInfoAction = libc::int32_t;


pub mod nsIHandlerInfo_consts {
    pub const saveToDisk: i64 = 0;
    pub const alwaysAsk: i64 = 1;
    pub const useHelperApp: i64 = 2;
    pub const handleInternally: i64 = 3;
    pub const useSystemDefault: i64 = 4;
}


#[repr(C)]
pub struct nsIHandlerInfo {
    vtable: *const nsIHandlerInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHandlerInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x325e56a7, 0x3762, 0x4312,
            [0xae, 0xc7, 0xf1, 0xfc, 0xf8, 0x4b, 0x41, 0x45])
    }
}

unsafe impl RefCounted for nsIHandlerInfo {
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
pub trait nsIHandlerInfoCoerce {
    fn coerce_from(v: &nsIHandlerInfo) -> &Self;
}

impl nsIHandlerInfoCoerce for nsIHandlerInfo {
    #[inline]
    fn coerce_from(v: &nsIHandlerInfo) -> &Self {
        v
    }
}

impl nsIHandlerInfo {
    #[inline]
    pub fn coerce<T: nsIHandlerInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHandlerInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHandlerInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHandlerInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHandlerInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIHandlerInfo, aType: *mut nsACString) -> nsresult,

    /* attribute AString description; */
    pub get_description: unsafe extern "C" fn (this: *const nsIHandlerInfo, aDescription: *mut nsAString) -> nsresult,
    pub set_description: unsafe extern "C" fn (this: *const nsIHandlerInfo, aDescription: *const nsAString) -> nsresult,

    /* attribute nsIHandlerApp preferredApplicationHandler; */
    pub get_preferredApplicationHandler: unsafe extern "C" fn (this: *const nsIHandlerInfo, aPreferredApplicationHandler: *mut *const nsIHandlerApp) -> nsresult,
    pub set_preferredApplicationHandler: unsafe extern "C" fn (this: *const nsIHandlerInfo, aPreferredApplicationHandler: *const nsIHandlerApp) -> nsresult,

    /* readonly attribute nsIMutableArray possibleApplicationHandlers; */
    pub get_possibleApplicationHandlers: unsafe extern "C" fn (this: *const nsIHandlerInfo, aPossibleApplicationHandlers: *mut *const nsIMutableArray) -> nsresult,

    /* readonly attribute boolean hasDefaultHandler; */
    pub get_hasDefaultHandler: unsafe extern "C" fn (this: *const nsIHandlerInfo, aHasDefaultHandler: *mut bool) -> nsresult,

    /* readonly attribute AString defaultDescription; */
    pub get_defaultDescription: unsafe extern "C" fn (this: *const nsIHandlerInfo, aDefaultDescription: *mut nsAString) -> nsresult,

    /* void launchWithURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
    pub launchWithURI: unsafe extern "C" fn (this: *const nsIHandlerInfo, aURI: *const nsIURI, aWindowContext: *const nsIInterfaceRequestor) -> nsresult,

    /* attribute nsHandlerInfoAction preferredAction; */
    pub get_preferredAction: unsafe extern "C" fn (this: *const nsIHandlerInfo, aPreferredAction: *mut nsHandlerInfoAction) -> nsresult,
    pub set_preferredAction: unsafe extern "C" fn (this: *const nsIHandlerInfo, aPreferredAction: nsHandlerInfoAction) -> nsresult,

    /* attribute boolean alwaysAskBeforeHandling; */
    pub get_alwaysAskBeforeHandling: unsafe extern "C" fn (this: *const nsIHandlerInfo, aAlwaysAskBeforeHandling: *mut bool) -> nsresult,
    pub set_alwaysAskBeforeHandling: unsafe extern "C" fn (this: *const nsIHandlerInfo, aAlwaysAskBeforeHandling: bool) -> nsresult,

}


impl nsIHandlerInfo {
    /* readonly attribute ACString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString description; */
    #[inline]
    pub unsafe fn get_description(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_description)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_description(&self, aDescription: &[u16]) -> Result<(), nsresult> {
        let aDescription = nsString::from(aDescription);
        match ((*self.vtable).set_description)(self as *const _, &*aDescription) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIHandlerApp preferredApplicationHandler; */
    #[inline]
    pub unsafe fn get_preferredApplicationHandler(&self, ) -> Result<Option<RefPtr<nsIHandlerApp>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_preferredApplicationHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_preferredApplicationHandler(&self, aPreferredApplicationHandler: Option<&nsIHandlerApp>) -> Result<(), nsresult> {

        match ((*self.vtable).set_preferredApplicationHandler)(self as *const _, aPreferredApplicationHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIMutableArray possibleApplicationHandlers; */
    #[inline]
    pub unsafe fn get_possibleApplicationHandlers(&self, ) -> Result<Option<RefPtr<nsIMutableArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_possibleApplicationHandlers)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean hasDefaultHandler; */
    #[inline]
    pub unsafe fn get_hasDefaultHandler(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasDefaultHandler)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString defaultDescription; */
    #[inline]
    pub unsafe fn get_defaultDescription(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_defaultDescription)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void launchWithURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
    #[inline]
    pub unsafe fn launchWithURI(&self, aURI: Option<&nsIURI>, aWindowContext: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).launchWithURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aWindowContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsHandlerInfoAction preferredAction; */
    #[inline]
    pub unsafe fn get_preferredAction(&self, ) -> Result<nsHandlerInfoAction, nsresult> {
        let mut _retval: nsHandlerInfoAction = ::std::mem::zeroed();
        match ((*self.vtable).get_preferredAction)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_preferredAction(&self, aPreferredAction: nsHandlerInfoAction) -> Result<(), nsresult> {

        match ((*self.vtable).set_preferredAction)(self as *const _, aPreferredAction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean alwaysAskBeforeHandling; */
    #[inline]
    pub unsafe fn get_alwaysAskBeforeHandling(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_alwaysAskBeforeHandling)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_alwaysAskBeforeHandling(&self, aAlwaysAskBeforeHandling: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_alwaysAskBeforeHandling)(self as *const _, aAlwaysAskBeforeHandling) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIMIMEInfo {
    vtable: *const nsIMIMEInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMIMEInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1c21acef, 0xc7a1, 0x40c6,
            [0x9d, 0x40, 0xa2, 0x04, 0x80, 0xee, 0x53, 0xa1])
    }
}

unsafe impl RefCounted for nsIMIMEInfo {
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
pub trait nsIMIMEInfoCoerce {
    fn coerce_from(v: &nsIMIMEInfo) -> &Self;
}

impl nsIMIMEInfoCoerce for nsIMIMEInfo {
    #[inline]
    fn coerce_from(v: &nsIMIMEInfo) -> &Self {
        v
    }
}

impl nsIMIMEInfo {
    #[inline]
    pub fn coerce<T: nsIMIMEInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMIMEInfo {
    type Target = nsIHandlerInfo;
    #[inline]
    fn deref(&self) -> &nsIHandlerInfo {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIHandlerInfoCoerce> nsIMIMEInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMIMEInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMIMEInfoVTable {
    pub __base: nsIHandlerInfoVTable,

    /* nsIUTF8StringEnumerator getFileExtensions (); */
    pub getFileExtensions: unsafe extern "C" fn (this: *const nsIMIMEInfo, _retval: *mut *const nsIUTF8StringEnumerator) -> nsresult,

    /* void setFileExtensions (in AUTF8String aExtensions); */
    pub setFileExtensions: unsafe extern "C" fn (this: *const nsIMIMEInfo, aExtensions: *const nsACString) -> nsresult,

    /* boolean extensionExists (in AUTF8String aExtension); */
    pub extensionExists: unsafe extern "C" fn (this: *const nsIMIMEInfo, aExtension: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void appendExtension (in AUTF8String aExtension); */
    pub appendExtension: unsafe extern "C" fn (this: *const nsIMIMEInfo, aExtension: *const nsACString) -> nsresult,

    /* attribute AUTF8String primaryExtension; */
    pub get_primaryExtension: unsafe extern "C" fn (this: *const nsIMIMEInfo, aPrimaryExtension: *mut nsACString) -> nsresult,
    pub set_primaryExtension: unsafe extern "C" fn (this: *const nsIMIMEInfo, aPrimaryExtension: *const nsACString) -> nsresult,

    /* readonly attribute ACString MIMEType; */
    pub get_MIMEType: unsafe extern "C" fn (this: *const nsIMIMEInfo, aMIMEType: *mut nsACString) -> nsresult,

    /* boolean equals (in nsIMIMEInfo aMIMEInfo); */
    pub equals: unsafe extern "C" fn (this: *const nsIMIMEInfo, aMIMEInfo: *const nsIMIMEInfo, _retval: *mut bool) -> nsresult,

    /* readonly attribute nsIArray possibleLocalHandlers; */
    pub get_possibleLocalHandlers: unsafe extern "C" fn (this: *const nsIMIMEInfo, aPossibleLocalHandlers: *mut *const nsIArray) -> nsresult,

    /* void launchWithFile (in nsIFile aFile); */
    pub launchWithFile: unsafe extern "C" fn (this: *const nsIMIMEInfo, aFile: *const nsIFile) -> nsresult,

}


impl nsIMIMEInfo {
    /* nsIUTF8StringEnumerator getFileExtensions (); */
    #[inline]
    pub unsafe fn getFileExtensions(&self, ) -> Result<Option<RefPtr<nsIUTF8StringEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFileExtensions)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setFileExtensions (in AUTF8String aExtensions); */
    #[inline]
    pub unsafe fn setFileExtensions(&self, aExtensions: &[u8]) -> Result<(), nsresult> {
        let aExtensions = nsCString::from(aExtensions);
        match ((*self.vtable).setFileExtensions)(self as *const _, &*aExtensions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean extensionExists (in AUTF8String aExtension); */
    #[inline]
    pub unsafe fn extensionExists(&self, aExtension: &[u8]) -> Result<bool, nsresult> {
        let aExtension = nsCString::from(aExtension);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).extensionExists)(self as *const _, &*aExtension, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void appendExtension (in AUTF8String aExtension); */
    #[inline]
    pub unsafe fn appendExtension(&self, aExtension: &[u8]) -> Result<(), nsresult> {
        let aExtension = nsCString::from(aExtension);
        match ((*self.vtable).appendExtension)(self as *const _, &*aExtension) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String primaryExtension; */
    #[inline]
    pub unsafe fn get_primaryExtension(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_primaryExtension)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_primaryExtension(&self, aPrimaryExtension: &[u8]) -> Result<(), nsresult> {
        let aPrimaryExtension = nsCString::from(aPrimaryExtension);
        match ((*self.vtable).set_primaryExtension)(self as *const _, &*aPrimaryExtension) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute ACString MIMEType; */
    #[inline]
    pub unsafe fn get_MIMEType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_MIMEType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean equals (in nsIMIMEInfo aMIMEInfo); */
    #[inline]
    pub unsafe fn equals(&self, aMIMEInfo: Option<&nsIMIMEInfo>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, aMIMEInfo.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIArray possibleLocalHandlers; */
    #[inline]
    pub unsafe fn get_possibleLocalHandlers(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_possibleLocalHandlers)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void launchWithFile (in nsIFile aFile); */
    #[inline]
    pub unsafe fn launchWithFile(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).launchWithFile)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIHandlerApp {
    vtable: *const nsIHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHandlerApp {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8bdf20a4, 0x9170, 0x4548,
            [0xaf, 0x52, 0x78, 0x31, 0x1a, 0x44, 0xf9, 0x20])
    }
}

unsafe impl RefCounted for nsIHandlerApp {
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
pub trait nsIHandlerAppCoerce {
    fn coerce_from(v: &nsIHandlerApp) -> &Self;
}

impl nsIHandlerAppCoerce for nsIHandlerApp {
    #[inline]
    fn coerce_from(v: &nsIHandlerApp) -> &Self {
        v
    }
}

impl nsIHandlerApp {
    #[inline]
    pub fn coerce<T: nsIHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHandlerApp {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHandlerAppVTable {
    pub __base: nsISupportsVTable,

    /* attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIHandlerApp, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIHandlerApp, aName: *const nsAString) -> nsresult,

    /* attribute AString detailedDescription; */
    pub get_detailedDescription: unsafe extern "C" fn (this: *const nsIHandlerApp, aDetailedDescription: *mut nsAString) -> nsresult,
    pub set_detailedDescription: unsafe extern "C" fn (this: *const nsIHandlerApp, aDetailedDescription: *const nsAString) -> nsresult,

    /* boolean equals (in nsIHandlerApp aHandlerApp); */
    pub equals: unsafe extern "C" fn (this: *const nsIHandlerApp, aHandlerApp: *const nsIHandlerApp, _retval: *mut bool) -> nsresult,

    /* void launchWithURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
    pub launchWithURI: unsafe extern "C" fn (this: *const nsIHandlerApp, aURI: *const nsIURI, aWindowContext: *const nsIInterfaceRequestor) -> nsresult,

}


impl nsIHandlerApp {
    /* attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString detailedDescription; */
    #[inline]
    pub unsafe fn get_detailedDescription(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_detailedDescription)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_detailedDescription(&self, aDetailedDescription: &[u16]) -> Result<(), nsresult> {
        let aDetailedDescription = nsString::from(aDetailedDescription);
        match ((*self.vtable).set_detailedDescription)(self as *const _, &*aDetailedDescription) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean equals (in nsIHandlerApp aHandlerApp); */
    #[inline]
    pub unsafe fn equals(&self, aHandlerApp: Option<&nsIHandlerApp>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, aHandlerApp.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void launchWithURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
    #[inline]
    pub unsafe fn launchWithURI(&self, aURI: Option<&nsIURI>, aWindowContext: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).launchWithURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aWindowContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsILocalHandlerApp {
    vtable: *const nsILocalHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocalHandlerApp {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd36b6329, 0x52ae, 0x4f45,
            [0x80, 0xf4, 0xb2, 0x53, 0x6a, 0xe5, 0xf8, 0xb2])
    }
}

unsafe impl RefCounted for nsILocalHandlerApp {
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
pub trait nsILocalHandlerAppCoerce {
    fn coerce_from(v: &nsILocalHandlerApp) -> &Self;
}

impl nsILocalHandlerAppCoerce for nsILocalHandlerApp {
    #[inline]
    fn coerce_from(v: &nsILocalHandlerApp) -> &Self {
        v
    }
}

impl nsILocalHandlerApp {
    #[inline]
    pub fn coerce<T: nsILocalHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocalHandlerApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIHandlerAppCoerce> nsILocalHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocalHandlerAppVTable {
    pub __base: nsIHandlerAppVTable,

    /* attribute nsIFile executable; */
    pub get_executable: unsafe extern "C" fn (this: *const nsILocalHandlerApp, aExecutable: *mut *const nsIFile) -> nsresult,
    pub set_executable: unsafe extern "C" fn (this: *const nsILocalHandlerApp, aExecutable: *const nsIFile) -> nsresult,

    /* readonly attribute unsigned long parameterCount; */
    pub get_parameterCount: unsafe extern "C" fn (this: *const nsILocalHandlerApp, aParameterCount: *mut libc::uint32_t) -> nsresult,

    /* void clearParameters (); */
    pub clearParameters: unsafe extern "C" fn (this: *const nsILocalHandlerApp) -> nsresult,

    /* void appendParameter (in AString param); */
    pub appendParameter: unsafe extern "C" fn (this: *const nsILocalHandlerApp, param: *const nsAString) -> nsresult,

    /* AString getParameter (in unsigned long parameterIndex); */
    pub getParameter: unsafe extern "C" fn (this: *const nsILocalHandlerApp, parameterIndex: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* boolean parameterExists (in AString param); */
    pub parameterExists: unsafe extern "C" fn (this: *const nsILocalHandlerApp, param: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl nsILocalHandlerApp {
    /* attribute nsIFile executable; */
    #[inline]
    pub unsafe fn get_executable(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_executable)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_executable(&self, aExecutable: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).set_executable)(self as *const _, aExecutable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long parameterCount; */
    #[inline]
    pub unsafe fn get_parameterCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_parameterCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void clearParameters (); */
    #[inline]
    pub unsafe fn clearParameters(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearParameters)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendParameter (in AString param); */
    #[inline]
    pub unsafe fn appendParameter(&self, param: &[u16]) -> Result<(), nsresult> {
        let param = nsString::from(param);
        match ((*self.vtable).appendParameter)(self as *const _, &*param) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getParameter (in unsigned long parameterIndex); */
    #[inline]
    pub unsafe fn getParameter(&self, parameterIndex: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getParameter)(self as *const _, parameterIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean parameterExists (in AString param); */
    #[inline]
    pub unsafe fn parameterExists(&self, param: &[u16]) -> Result<bool, nsresult> {
        let param = nsString::from(param);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).parameterExists)(self as *const _, &*param, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIWebHandlerApp {
    vtable: *const nsIWebHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebHandlerApp {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7521a093, 0xc498, 0x45ce,
            [0xb4, 0x62, 0xdf, 0x7b, 0xa0, 0xd8, 0x82, 0xf6])
    }
}

unsafe impl RefCounted for nsIWebHandlerApp {
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
pub trait nsIWebHandlerAppCoerce {
    fn coerce_from(v: &nsIWebHandlerApp) -> &Self;
}

impl nsIWebHandlerAppCoerce for nsIWebHandlerApp {
    #[inline]
    fn coerce_from(v: &nsIWebHandlerApp) -> &Self {
        v
    }
}

impl nsIWebHandlerApp {
    #[inline]
    pub fn coerce<T: nsIWebHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebHandlerApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIHandlerAppCoerce> nsIWebHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebHandlerAppVTable {
    pub __base: nsIHandlerAppVTable,

    /* attribute AUTF8String uriTemplate; */
    pub get_uriTemplate: unsafe extern "C" fn (this: *const nsIWebHandlerApp, aUriTemplate: *mut nsACString) -> nsresult,
    pub set_uriTemplate: unsafe extern "C" fn (this: *const nsIWebHandlerApp, aUriTemplate: *const nsACString) -> nsresult,

}


impl nsIWebHandlerApp {
    /* attribute AUTF8String uriTemplate; */
    #[inline]
    pub unsafe fn get_uriTemplate(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_uriTemplate)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_uriTemplate(&self, aUriTemplate: &[u8]) -> Result<(), nsresult> {
        let aUriTemplate = nsCString::from(aUriTemplate);
        match ((*self.vtable).set_uriTemplate)(self as *const _, &*aUriTemplate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDBusHandlerApp {
    vtable: *const nsIDBusHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDBusHandlerApp {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1ffc274b, 0x4cbf, 0x4bb5,
            [0xa6, 0x35, 0x05, 0xad, 0x2c, 0xbb, 0x65, 0x34])
    }
}

unsafe impl RefCounted for nsIDBusHandlerApp {
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
pub trait nsIDBusHandlerAppCoerce {
    fn coerce_from(v: &nsIDBusHandlerApp) -> &Self;
}

impl nsIDBusHandlerAppCoerce for nsIDBusHandlerApp {
    #[inline]
    fn coerce_from(v: &nsIDBusHandlerApp) -> &Self {
        v
    }
}

impl nsIDBusHandlerApp {
    #[inline]
    pub fn coerce<T: nsIDBusHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDBusHandlerApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIHandlerAppCoerce> nsIDBusHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDBusHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDBusHandlerAppVTable {
    pub __base: nsIHandlerAppVTable,

    /* attribute AUTF8String service; */
    pub get_service: unsafe extern "C" fn (this: *const nsIDBusHandlerApp, aService: *mut nsACString) -> nsresult,
    pub set_service: unsafe extern "C" fn (this: *const nsIDBusHandlerApp, aService: *const nsACString) -> nsresult,

    /* attribute AUTF8String objectPath; */
    pub get_objectPath: unsafe extern "C" fn (this: *const nsIDBusHandlerApp, aObjectPath: *mut nsACString) -> nsresult,
    pub set_objectPath: unsafe extern "C" fn (this: *const nsIDBusHandlerApp, aObjectPath: *const nsACString) -> nsresult,

    /* attribute AUTF8String dBusInterface; */
    pub get_dBusInterface: unsafe extern "C" fn (this: *const nsIDBusHandlerApp, aDBusInterface: *mut nsACString) -> nsresult,
    pub set_dBusInterface: unsafe extern "C" fn (this: *const nsIDBusHandlerApp, aDBusInterface: *const nsACString) -> nsresult,

    /* attribute AUTF8String method; */
    pub get_method: unsafe extern "C" fn (this: *const nsIDBusHandlerApp, aMethod: *mut nsACString) -> nsresult,
    pub set_method: unsafe extern "C" fn (this: *const nsIDBusHandlerApp, aMethod: *const nsACString) -> nsresult,

}


impl nsIDBusHandlerApp {
    /* attribute AUTF8String service; */
    #[inline]
    pub unsafe fn get_service(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_service)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_service(&self, aService: &[u8]) -> Result<(), nsresult> {
        let aService = nsCString::from(aService);
        match ((*self.vtable).set_service)(self as *const _, &*aService) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String objectPath; */
    #[inline]
    pub unsafe fn get_objectPath(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_objectPath)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_objectPath(&self, aObjectPath: &[u8]) -> Result<(), nsresult> {
        let aObjectPath = nsCString::from(aObjectPath);
        match ((*self.vtable).set_objectPath)(self as *const _, &*aObjectPath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String dBusInterface; */
    #[inline]
    pub unsafe fn get_dBusInterface(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_dBusInterface)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_dBusInterface(&self, aDBusInterface: &[u8]) -> Result<(), nsresult> {
        let aDBusInterface = nsCString::from(aDBusInterface);
        match ((*self.vtable).set_dBusInterface)(self as *const _, &*aDBusInterface) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String method; */
    #[inline]
    pub unsafe fn get_method(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_method)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_method(&self, aMethod: &[u8]) -> Result<(), nsresult> {
        let aMethod = nsCString::from(aMethod);
        match ((*self.vtable).set_method)(self as *const _, &*aMethod) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


