//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPermissionPrompt.idl
//


#[repr(C)]
pub struct nsIContentPermissionType {
    vtable: *const nsIContentPermissionTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPermissionType {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xef4db3b8, 0xca9c, 0x4b1d,
            [0x8f, 0x81, 0xfd, 0x88, 0xec, 0x32, 0xaf, 0x13])
    }
}

unsafe impl RefCounted for nsIContentPermissionType {
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
pub trait nsIContentPermissionTypeCoerce {
    fn coerce_from(v: &nsIContentPermissionType) -> &Self;
}

impl nsIContentPermissionTypeCoerce for nsIContentPermissionType {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionType) -> &Self {
        v
    }
}

impl nsIContentPermissionType {
    #[inline]
    pub fn coerce<T: nsIContentPermissionTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPermissionType {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPermissionTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionType) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPermissionTypeVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIContentPermissionType, aType: *mut nsACString) -> nsresult,

    /* readonly attribute ACString access; */
    pub get_access: unsafe extern "C" fn (this: *const nsIContentPermissionType, aAccess: *mut nsACString) -> nsresult,

    /* readonly attribute nsIArray options; */
    pub get_options: unsafe extern "C" fn (this: *const nsIContentPermissionType, aOptions: *mut *const nsIArray) -> nsresult,

}


impl nsIContentPermissionType {
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

    /* readonly attribute ACString access; */
    #[inline]
    pub unsafe fn get_access(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_access)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIArray options; */
    #[inline]
    pub unsafe fn get_options(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_options)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIContentPermissionRequestCallback {
    vtable: *const nsIContentPermissionRequestCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPermissionRequestCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5fb5bb60, 0x7069, 0x11e4,
            [0x98, 0x03, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIContentPermissionRequestCallback {
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
pub trait nsIContentPermissionRequestCallbackCoerce {
    fn coerce_from(v: &nsIContentPermissionRequestCallback) -> &Self;
}

impl nsIContentPermissionRequestCallbackCoerce for nsIContentPermissionRequestCallback {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionRequestCallback) -> &Self {
        v
    }
}

impl nsIContentPermissionRequestCallback {
    #[inline]
    pub fn coerce<T: nsIContentPermissionRequestCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPermissionRequestCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPermissionRequestCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionRequestCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPermissionRequestCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void notifyVisibility (in boolean isVisible); */
    pub notifyVisibility: unsafe extern "C" fn (this: *const nsIContentPermissionRequestCallback, isVisible: bool) -> nsresult,

}


impl nsIContentPermissionRequestCallback {
    /* void notifyVisibility (in boolean isVisible); */
    #[inline]
    pub unsafe fn notifyVisibility(&self, isVisible: bool) -> Result<(), nsresult> {

        match ((*self.vtable).notifyVisibility)(self as *const _, isVisible) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIContentPermissionRequester {
    vtable: *const nsIContentPermissionRequesterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPermissionRequester {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf8577124, 0x6a5f, 0x486f,
            [0xae, 0x04, 0xc5, 0xbc, 0xae, 0x91, 0x1e, 0xb5])
    }
}

unsafe impl RefCounted for nsIContentPermissionRequester {
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
pub trait nsIContentPermissionRequesterCoerce {
    fn coerce_from(v: &nsIContentPermissionRequester) -> &Self;
}

impl nsIContentPermissionRequesterCoerce for nsIContentPermissionRequester {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionRequester) -> &Self {
        v
    }
}

impl nsIContentPermissionRequester {
    #[inline]
    pub fn coerce<T: nsIContentPermissionRequesterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPermissionRequester {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPermissionRequesterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionRequester) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPermissionRequesterVTable {
    pub __base: nsISupportsVTable,

    /* void getVisibility (in nsIContentPermissionRequestCallback callback); */
    pub getVisibility: unsafe extern "C" fn (this: *const nsIContentPermissionRequester, callback: *const nsIContentPermissionRequestCallback) -> nsresult,

    /* attribute nsIContentPermissionRequestCallback onVisibilityChange; */
    pub get_onVisibilityChange: unsafe extern "C" fn (this: *const nsIContentPermissionRequester, aOnVisibilityChange: *mut *const nsIContentPermissionRequestCallback) -> nsresult,
    pub set_onVisibilityChange: unsafe extern "C" fn (this: *const nsIContentPermissionRequester, aOnVisibilityChange: *const nsIContentPermissionRequestCallback) -> nsresult,

}


impl nsIContentPermissionRequester {
    /* void getVisibility (in nsIContentPermissionRequestCallback callback); */
    #[inline]
    pub unsafe fn getVisibility(&self, callback: Option<&nsIContentPermissionRequestCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).getVisibility)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIContentPermissionRequestCallback onVisibilityChange; */
    #[inline]
    pub unsafe fn get_onVisibilityChange(&self, ) -> Result<Option<RefPtr<nsIContentPermissionRequestCallback>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_onVisibilityChange)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_onVisibilityChange(&self, aOnVisibilityChange: Option<&nsIContentPermissionRequestCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).set_onVisibilityChange)(self as *const _, aOnVisibilityChange.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIContentPermissionRequest {
    vtable: *const nsIContentPermissionRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPermissionRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x875733da, 0x0ac0, 0x4a26,
            [0x8c, 0x76, 0x70, 0xa3, 0x08, 0x76, 0xbe, 0x46])
    }
}

unsafe impl RefCounted for nsIContentPermissionRequest {
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
pub trait nsIContentPermissionRequestCoerce {
    fn coerce_from(v: &nsIContentPermissionRequest) -> &Self;
}

impl nsIContentPermissionRequestCoerce for nsIContentPermissionRequest {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionRequest) -> &Self {
        v
    }
}

impl nsIContentPermissionRequest {
    #[inline]
    pub fn coerce<T: nsIContentPermissionRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPermissionRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPermissionRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPermissionRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIArray types; */
    pub get_types: unsafe extern "C" fn (this: *const nsIContentPermissionRequest, aTypes: *mut *const nsIArray) -> nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIContentPermissionRequest, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* readonly attribute mozIDOMWindow window; */
    pub get_window: unsafe extern "C" fn (this: *const nsIContentPermissionRequest, aWindow: *mut *const mozIDOMWindow) -> nsresult,

    /* readonly attribute nsIDOMElement element; */
    pub get_element: unsafe extern "C" fn (this: *const nsIContentPermissionRequest, aElement: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsIContentPermissionRequester requester; */
    pub get_requester: unsafe extern "C" fn (this: *const nsIContentPermissionRequest, aRequester: *mut *const nsIContentPermissionRequester) -> nsresult,

    /* void cancel (); */
    pub cancel: unsafe extern "C" fn (this: *const nsIContentPermissionRequest) -> nsresult,

    /* void allow ([optional] in jsval choices); */
    /// Unable to call function as its signature contains a non-rust type
    pub allow: *const ::libc::c_void,

}


impl nsIContentPermissionRequest {
    /* readonly attribute nsIArray types; */
    #[inline]
    pub unsafe fn get_types(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_types)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIPrincipal principal; */
    #[inline]
    pub unsafe fn get_principal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIDOMWindow window; */
    #[inline]
    pub unsafe fn get_window(&self, ) -> Result<Option<RefPtr<mozIDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_window)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement element; */
    #[inline]
    pub unsafe fn get_element(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_element)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIContentPermissionRequester requester; */
    #[inline]
    pub unsafe fn get_requester(&self, ) -> Result<Option<RefPtr<nsIContentPermissionRequester>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_requester)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void cancel (); */
    #[inline]
    pub unsafe fn cancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void allow ([optional] in jsval choices); */


}


#[repr(C)]
pub struct nsIContentPermissionPrompt {
    vtable: *const nsIContentPermissionPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPermissionPrompt {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf72de90d, 0xe954, 0x4e69,
            [0x9a, 0x61, 0x91, 0x73, 0x03, 0x02, 0x93, 0x01])
    }
}

unsafe impl RefCounted for nsIContentPermissionPrompt {
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
pub trait nsIContentPermissionPromptCoerce {
    fn coerce_from(v: &nsIContentPermissionPrompt) -> &Self;
}

impl nsIContentPermissionPromptCoerce for nsIContentPermissionPrompt {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionPrompt) -> &Self {
        v
    }
}

impl nsIContentPermissionPrompt {
    #[inline]
    pub fn coerce<T: nsIContentPermissionPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPermissionPrompt {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPermissionPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionPrompt) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPermissionPromptVTable {
    pub __base: nsISupportsVTable,

    /* void prompt (in nsIContentPermissionRequest request); */
    pub prompt: unsafe extern "C" fn (this: *const nsIContentPermissionPrompt, request: *const nsIContentPermissionRequest) -> nsresult,

}


impl nsIContentPermissionPrompt {
    /* void prompt (in nsIContentPermissionRequest request); */
    #[inline]
    pub unsafe fn prompt(&self, request: Option<&nsIContentPermissionRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).prompt)(self as *const _, request.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


