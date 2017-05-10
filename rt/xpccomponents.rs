//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpccomponents.idl
//


#[repr(C)]
pub struct nsIXPCComponents_InterfacesByID {
    vtable: *const nsIXPCComponents_InterfacesByIDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_InterfacesByID {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf235ef76, 0x9919, 0x478b,
            [0xaa, 0x0f, 0x28, 0x2d, 0x99, 0x4d, 0xdf, 0x76])
    }
}

unsafe impl RefCounted for nsIXPCComponents_InterfacesByID {
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
pub trait nsIXPCComponents_InterfacesByIDCoerce {
    fn coerce_from(v: &nsIXPCComponents_InterfacesByID) -> &Self;
}

impl nsIXPCComponents_InterfacesByIDCoerce for nsIXPCComponents_InterfacesByID {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_InterfacesByID) -> &Self {
        v
    }
}

impl nsIXPCComponents_InterfacesByID {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_InterfacesByIDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_InterfacesByID {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_InterfacesByIDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_InterfacesByID) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_InterfacesByIDVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_InterfacesByID {
}


#[repr(C)]
pub struct nsIXPCComponents_Interfaces {
    vtable: *const nsIXPCComponents_InterfacesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_Interfaces {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb8c31bba, 0x79db, 0x4a1d,
            [0x93, 0x0d, 0x4c, 0xdd, 0x68, 0x71, 0x3f, 0x9e])
    }
}

unsafe impl RefCounted for nsIXPCComponents_Interfaces {
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
pub trait nsIXPCComponents_InterfacesCoerce {
    fn coerce_from(v: &nsIXPCComponents_Interfaces) -> &Self;
}

impl nsIXPCComponents_InterfacesCoerce for nsIXPCComponents_Interfaces {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Interfaces) -> &Self {
        v
    }
}

impl nsIXPCComponents_Interfaces {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_InterfacesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_Interfaces {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_InterfacesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Interfaces) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_InterfacesVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_Interfaces {
}


#[repr(C)]
pub struct nsIXPCComponents_Classes {
    vtable: *const nsIXPCComponents_ClassesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_Classes {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x978ff520, 0xd26c, 0x11d2,
            [0x98, 0x42, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsIXPCComponents_Classes {
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
pub trait nsIXPCComponents_ClassesCoerce {
    fn coerce_from(v: &nsIXPCComponents_Classes) -> &Self;
}

impl nsIXPCComponents_ClassesCoerce for nsIXPCComponents_Classes {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Classes) -> &Self {
        v
    }
}

impl nsIXPCComponents_Classes {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ClassesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_Classes {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_ClassesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Classes) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_ClassesVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_Classes {
}


#[repr(C)]
pub struct nsIXPCComponents_ClassesByID {
    vtable: *const nsIXPCComponents_ClassesByIDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_ClassesByID {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x336a9590, 0x4d19, 0x11d3,
            [0x98, 0x93, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsIXPCComponents_ClassesByID {
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
pub trait nsIXPCComponents_ClassesByIDCoerce {
    fn coerce_from(v: &nsIXPCComponents_ClassesByID) -> &Self;
}

impl nsIXPCComponents_ClassesByIDCoerce for nsIXPCComponents_ClassesByID {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_ClassesByID) -> &Self {
        v
    }
}

impl nsIXPCComponents_ClassesByID {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ClassesByIDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_ClassesByID {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_ClassesByIDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_ClassesByID) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_ClassesByIDVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_ClassesByID {
}


#[repr(C)]
pub struct nsIXPCComponents_Results {
    vtable: *const nsIXPCComponents_ResultsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_Results {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2fc229a0, 0x5860, 0x11d3,
            [0x98, 0x99, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsIXPCComponents_Results {
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
pub trait nsIXPCComponents_ResultsCoerce {
    fn coerce_from(v: &nsIXPCComponents_Results) -> &Self;
}

impl nsIXPCComponents_ResultsCoerce for nsIXPCComponents_Results {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Results) -> &Self {
        v
    }
}

impl nsIXPCComponents_Results {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ResultsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_Results {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_ResultsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Results) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_ResultsVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_Results {
}


#[repr(C)]
pub struct nsIXPCComponents_ID {
    vtable: *const nsIXPCComponents_IDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_ID {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7994a6e0, 0xe028, 0x11d3,
            [0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIXPCComponents_ID {
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
pub trait nsIXPCComponents_IDCoerce {
    fn coerce_from(v: &nsIXPCComponents_ID) -> &Self;
}

impl nsIXPCComponents_IDCoerce for nsIXPCComponents_ID {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_ID) -> &Self {
        v
    }
}

impl nsIXPCComponents_ID {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_IDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_ID {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_IDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_ID) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_IDVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_ID {
}


#[repr(C)]
pub struct nsIXPCComponents_Exception {
    vtable: *const nsIXPCComponents_ExceptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_Exception {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5bf039c0, 0xe028, 0x11d3,
            [0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIXPCComponents_Exception {
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
pub trait nsIXPCComponents_ExceptionCoerce {
    fn coerce_from(v: &nsIXPCComponents_Exception) -> &Self;
}

impl nsIXPCComponents_ExceptionCoerce for nsIXPCComponents_Exception {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Exception) -> &Self {
        v
    }
}

impl nsIXPCComponents_Exception {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ExceptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_Exception {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_ExceptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Exception) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_ExceptionVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_Exception {
}


#[repr(C)]
pub struct nsIXPCComponents_Constructor {
    vtable: *const nsIXPCComponents_ConstructorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_Constructor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x88655640, 0xe028, 0x11d3,
            [0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIXPCComponents_Constructor {
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
pub trait nsIXPCComponents_ConstructorCoerce {
    fn coerce_from(v: &nsIXPCComponents_Constructor) -> &Self;
}

impl nsIXPCComponents_ConstructorCoerce for nsIXPCComponents_Constructor {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Constructor) -> &Self {
        v
    }
}

impl nsIXPCComponents_Constructor {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_ConstructorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_Constructor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_ConstructorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Constructor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_ConstructorVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_Constructor {
}


#[repr(C)]
pub struct nsIXPCConstructor {
    vtable: *const nsIXPCConstructorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCConstructor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc814ca20, 0xe0dc, 0x11d3,
            [0x8f, 0x5f, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIXPCConstructor {
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
pub trait nsIXPCConstructorCoerce {
    fn coerce_from(v: &nsIXPCConstructor) -> &Self;
}

impl nsIXPCConstructorCoerce for nsIXPCConstructor {
    #[inline]
    fn coerce_from(v: &nsIXPCConstructor) -> &Self {
        v
    }
}

impl nsIXPCConstructor {
    #[inline]
    pub fn coerce<T: nsIXPCConstructorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCConstructor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCConstructorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCConstructor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCConstructorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIJSCID classID; */
    pub get_classID: unsafe extern "C" fn (this: *const nsIXPCConstructor, aClassID: *mut *const nsIJSCID) -> nsresult,

    /* readonly attribute nsIJSIID interfaceID; */
    pub get_interfaceID: unsafe extern "C" fn (this: *const nsIXPCConstructor, aInterfaceID: *mut *const nsIJSIID) -> nsresult,

    /* readonly attribute string initializer; */
    pub get_initializer: unsafe extern "C" fn (this: *const nsIXPCConstructor, aInitializer: *mut *const libc::c_char) -> nsresult,

}


impl nsIXPCConstructor {
    /* readonly attribute nsIJSCID classID; */
    #[inline]
    pub unsafe fn get_classID(&self, ) -> Result<Option<RefPtr<nsIJSCID>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_classID)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIJSIID interfaceID; */
    #[inline]
    pub unsafe fn get_interfaceID(&self, ) -> Result<Option<RefPtr<nsIJSIID>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_interfaceID)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute string initializer; */
    #[inline]
    pub unsafe fn get_initializer(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_initializer)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIXPCComponents_utils_Sandbox {
    vtable: *const nsIXPCComponents_utils_SandboxVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_utils_Sandbox {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4f8ae0dc, 0xd266, 0x4a32,
            [0x87, 0x5b, 0x6a, 0x9d, 0xe7, 0x1a, 0x8c, 0xe9])
    }
}

unsafe impl RefCounted for nsIXPCComponents_utils_Sandbox {
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
pub trait nsIXPCComponents_utils_SandboxCoerce {
    fn coerce_from(v: &nsIXPCComponents_utils_Sandbox) -> &Self;
}

impl nsIXPCComponents_utils_SandboxCoerce for nsIXPCComponents_utils_Sandbox {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_utils_Sandbox) -> &Self {
        v
    }
}

impl nsIXPCComponents_utils_Sandbox {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_utils_SandboxCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_utils_Sandbox {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_utils_SandboxCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_utils_Sandbox) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_utils_SandboxVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXPCComponents_utils_Sandbox {
}


#[repr(C)]
pub struct ScheduledGCCallback {
    vtable: *const ScheduledGCCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for ScheduledGCCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x71000535, 0xb0fd, 0x44d1,
            [0x8c, 0xe0, 0x90, 0x97, 0x60, 0xe3, 0x95, 0x3c])
    }
}

unsafe impl RefCounted for ScheduledGCCallback {
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
pub trait ScheduledGCCallbackCoerce {
    fn coerce_from(v: &ScheduledGCCallback) -> &Self;
}

impl ScheduledGCCallbackCoerce for ScheduledGCCallback {
    #[inline]
    fn coerce_from(v: &ScheduledGCCallback) -> &Self {
        v
    }
}

impl ScheduledGCCallback {
    #[inline]
    pub fn coerce<T: ScheduledGCCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for ScheduledGCCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> ScheduledGCCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &ScheduledGCCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct ScheduledGCCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void callback (); */
    pub callback: unsafe extern "C" fn (this: *const ScheduledGCCallback) -> nsresult,

}


impl ScheduledGCCallback {
    /* void callback (); */
    #[inline]
    pub unsafe fn callback(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).callback)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIXPCComponents_Utils {
    vtable: *const nsIXPCComponents_UtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents_Utils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x86003fe3, 0xee9a, 0x4620,
            [0x91, 0xdc, 0xee, 0xf8, 0xb1, 0xe5, 0x88, 0x15])
    }
}

unsafe impl RefCounted for nsIXPCComponents_Utils {
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
pub trait nsIXPCComponents_UtilsCoerce {
    fn coerce_from(v: &nsIXPCComponents_Utils) -> &Self;
}

impl nsIXPCComponents_UtilsCoerce for nsIXPCComponents_Utils {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Utils) -> &Self {
        v
    }
}

impl nsIXPCComponents_Utils {
    #[inline]
    pub fn coerce<T: nsIXPCComponents_UtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents_Utils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponents_UtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents_Utils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponents_UtilsVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void reportError (in jsval error); */
    /// Unable to call function as its signature contains a non-rust type
    pub reportError: *const ::libc::c_void,

    /* readonly attribute nsIXPCComponents_utils_Sandbox Sandbox; */
    pub get_Sandbox: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, aSandbox: *mut *const nsIXPCComponents_utils_Sandbox) -> nsresult,

    /* [implicit_jscontext,optional_argc] jsval evalInSandbox (in AString source, in jsval sandbox, [optional] in jsval version, [optional] in AUTF8String filename, [optional] in long lineNo); */
    /// Unable to call function as its signature contains a non-rust type
    pub evalInSandbox: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getSandboxAddonId (in jsval sandbox); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSandboxAddonId: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getSandboxMetadata (in jsval sandbox); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSandboxMetadata: *const ::libc::c_void,

    /* [implicit_jscontext] void setSandboxMetadata (in jsval sandbox, in jsval metadata); */
    /// Unable to call function as its signature contains a non-rust type
    pub setSandboxMetadata: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] jsval import (in AUTF8String aResourceURI, [optional] in jsval targetObj); */
    /// Unable to call function as its signature contains a non-rust type
    pub import: *const ::libc::c_void,

    /* boolean isModuleLoaded (in AUTF8String aResourceURI); */
    pub isModuleLoaded: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, aResourceURI: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void unload (in AUTF8String registryLocation); */
    pub unload: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, registryLocation: *const nsACString) -> nsresult,

    /* [implicit_jscontext] void importGlobalProperties (in jsval aPropertyList); */
    /// Unable to call function as its signature contains a non-rust type
    pub importGlobalProperties: *const ::libc::c_void,

    /* [implicit_jscontext] xpcIJSWeakReference getWeakReference (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub getWeakReference: *const ::libc::c_void,

    /* void forceGC (); */
    pub forceGC: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils) -> nsresult,

    /* void forceCC ([optional] in nsICycleCollectorListener aListener); */
    pub forceCC: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, aListener: *const nsICycleCollectorListener) -> nsresult,

    /* void finishCC (); */
    pub finishCC: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils) -> nsresult,

    /* void ccSlice (in long long budget); */
    pub ccSlice: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, budget: libc::int64_t) -> nsresult,

    /* long getMaxCCSliceTimeSinceClear (); */
    pub getMaxCCSliceTimeSinceClear: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, _retval: *mut libc::int32_t) -> nsresult,

    /* void clearMaxCCTime (); */
    pub clearMaxCCTime: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils) -> nsresult,

    /* void forceShrinkingGC (); */
    pub forceShrinkingGC: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils) -> nsresult,

    /* void schedulePreciseGC (in ScheduledGCCallback callback); */
    pub schedulePreciseGC: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, callback: *const ScheduledGCCallback) -> nsresult,

    /* void schedulePreciseShrinkingGC (in ScheduledGCCallback callback); */
    pub schedulePreciseShrinkingGC: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, callback: *const ScheduledGCCallback) -> nsresult,

    /* void unlinkGhostWindows (); */
    pub unlinkGhostWindows: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils) -> nsresult,

    /* [implicit_jscontext] jsval getJSTestingFunctions (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getJSTestingFunctions: *const ::libc::c_void,

    /* [implicit_jscontext] jsval callFunctionWithAsyncStack (in jsval function, in nsIStackFrame stack, in AString asyncCause); */
    /// Unable to call function as its signature contains a non-rust type
    pub callFunctionWithAsyncStack: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getGlobalForObject (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub getGlobalForObject: *const ::libc::c_void,

    /* [implicit_jscontext] boolean isProxy (in jsval vobject); */
    /// Unable to call function as its signature contains a non-rust type
    pub isProxy: *const ::libc::c_void,

    /* [implicit_jscontext] jsval exportFunction (in jsval vfunction, in jsval vscope, [optional] in jsval voptions); */
    /// Unable to call function as its signature contains a non-rust type
    pub exportFunction: *const ::libc::c_void,

    /* [implicit_jscontext] jsval createObjectIn (in jsval vobj, [optional] in jsval voptions); */
    /// Unable to call function as its signature contains a non-rust type
    pub createObjectIn: *const ::libc::c_void,

    /* [implicit_jscontext] void makeObjectPropsNormal (in jsval vobj); */
    /// Unable to call function as its signature contains a non-rust type
    pub makeObjectPropsNormal: *const ::libc::c_void,

    /* bool isDeadWrapper (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub isDeadWrapper: *const ::libc::c_void,

    /* bool isCrossProcessWrapper (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub isCrossProcessWrapper: *const ::libc::c_void,

    /* ACString getCrossProcessWrapperTag (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCrossProcessWrapperTag: *const ::libc::c_void,

    /* void permitCPOWsInScope (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub permitCPOWsInScope: *const ::libc::c_void,

    /* [implicit_jscontext] void recomputeWrappers ([optional] in jsval vobj); */
    /// Unable to call function as its signature contains a non-rust type
    pub recomputeWrappers: *const ::libc::c_void,

    /* [implicit_jscontext] void setWantXrays (in jsval vscope); */
    /// Unable to call function as its signature contains a non-rust type
    pub setWantXrays: *const ::libc::c_void,

    /* [implicit_jscontext] void forcePermissiveCOWs (); */
    /// Unable to call function as its signature contains a non-rust type
    pub forcePermissiveCOWs: *const ::libc::c_void,

    /* [implicit_jscontext] void forcePrivilegedComponentsForScope (in jsval vscope); */
    /// Unable to call function as its signature contains a non-rust type
    pub forcePrivilegedComponentsForScope: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getComponentsForScope (in jsval vscope); */
    /// Unable to call function as its signature contains a non-rust type
    pub getComponentsForScope: *const ::libc::c_void,

    /* [implicit_jscontext] void dispatch (in jsval runnable, [optional] in jsval scope); */
    /// Unable to call function as its signature contains a non-rust type
    pub dispatch: *const ::libc::c_void,

    /* [implicit_jscontext] attribute boolean strict; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_strict: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_strict: *const ::libc::c_void,

    /* [implicit_jscontext] attribute boolean werror; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_werror: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_werror: *const ::libc::c_void,

    /* [implicit_jscontext] attribute boolean strict_mode; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_strict_mode: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_strict_mode: *const ::libc::c_void,

    /* [implicit_jscontext] attribute boolean ion; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_ion: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_ion: *const ::libc::c_void,

    /* [implicit_jscontext] void setGCZeal (in long zeal); */
    /// Unable to call function as its signature contains a non-rust type
    pub setGCZeal: *const ::libc::c_void,

    /* [implicit_jscontext] void nukeSandbox (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub nukeSandbox: *const ::libc::c_void,

    /* [implicit_jscontext] void blockScriptForGlobal (in jsval global); */
    /// Unable to call function as its signature contains a non-rust type
    pub blockScriptForGlobal: *const ::libc::c_void,

    /* [implicit_jscontext] void unblockScriptForGlobal (in jsval global); */
    /// Unable to call function as its signature contains a non-rust type
    pub unblockScriptForGlobal: *const ::libc::c_void,

    /* bool isXrayWrapper (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub isXrayWrapper: *const ::libc::c_void,

    /* [implicit_jscontext] jsval waiveXrays (in jsval aVal); */
    /// Unable to call function as its signature contains a non-rust type
    pub waiveXrays: *const ::libc::c_void,

    /* [implicit_jscontext] jsval unwaiveXrays (in jsval aVal); */
    /// Unable to call function as its signature contains a non-rust type
    pub unwaiveXrays: *const ::libc::c_void,

    /* [implicit_jscontext] string getClassName (in jsval aObj, in bool aUnwrap); */
    /// Unable to call function as its signature contains a non-rust type
    pub getClassName: *const ::libc::c_void,

    /* nsIClassInfo getDOMClassInfo (in AString aClassName); */
    pub getDOMClassInfo: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, aClassName: *const nsAString, _retval: *mut *const nsIClassInfo) -> nsresult,

    /* [implicit_jscontext] jsval getIncumbentGlobal ([optional] in jsval callback); */
    /// Unable to call function as its signature contains a non-rust type
    pub getIncumbentGlobal: *const ::libc::c_void,

    /* [implicit_jscontext] nsISupports generateXPCWrappedJS (in jsval obj, [optional] in jsval scope); */
    /// Unable to call function as its signature contains a non-rust type
    pub generateXPCWrappedJS: *const ::libc::c_void,

    /* PRTime getWatchdogTimestamp (in AString aCategory); */
    pub getWatchdogTimestamp: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, aCategory: *const nsAString, _retval: *mut PRTime) -> nsresult,

    /* [implicit_jscontext] jsval getJSEngineTelemetryValue (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getJSEngineTelemetryValue: *const ::libc::c_void,

    /* [implicit_jscontext] jsval cloneInto (in jsval value, in jsval scope, [optional] in jsval options); */
    /// Unable to call function as its signature contains a non-rust type
    pub cloneInto: *const ::libc::c_void,

    /* nsIPrincipal getWebIDLCallerPrincipal (); */
    pub getWebIDLCallerPrincipal: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* [implicit_jscontext] nsIPrincipal getObjectPrincipal (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub getObjectPrincipal: *const ::libc::c_void,

    /* [implicit_jscontext] ACString getCompartmentLocation (in jsval obj); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCompartmentLocation: *const ::libc::c_void,

    /* [implicit_jscontext] void setAddonInterposition (in ACString addonId, in nsIAddonInterposition interposition); */
    /// Unable to call function as its signature contains a non-rust type
    pub setAddonInterposition: *const ::libc::c_void,

    /* [implicit_jscontext] void setAddonCallInterposition (in jsval target); */
    /// Unable to call function as its signature contains a non-rust type
    pub setAddonCallInterposition: *const ::libc::c_void,

    /* [implicit_jscontext] void allowCPOWsInAddon (in ACString addonId, in bool allow); */
    /// Unable to call function as its signature contains a non-rust type
    pub allowCPOWsInAddon: *const ::libc::c_void,

    /* double now (); */
    pub now: unsafe extern "C" fn (this: *const nsIXPCComponents_Utils, _retval: *mut libc::c_double) -> nsresult,

}


impl nsIXPCComponents_Utils {
    /* [implicit_jscontext] void reportError (in jsval error); */


    /* readonly attribute nsIXPCComponents_utils_Sandbox Sandbox; */
    #[inline]
    pub unsafe fn get_Sandbox(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_utils_Sandbox>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_Sandbox)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext,optional_argc] jsval evalInSandbox (in AString source, in jsval sandbox, [optional] in jsval version, [optional] in AUTF8String filename, [optional] in long lineNo); */


    /* [implicit_jscontext] jsval getSandboxAddonId (in jsval sandbox); */


    /* [implicit_jscontext] jsval getSandboxMetadata (in jsval sandbox); */


    /* [implicit_jscontext] void setSandboxMetadata (in jsval sandbox, in jsval metadata); */


    /* [implicit_jscontext,optional_argc] jsval import (in AUTF8String aResourceURI, [optional] in jsval targetObj); */


    /* boolean isModuleLoaded (in AUTF8String aResourceURI); */
    #[inline]
    pub unsafe fn isModuleLoaded(&self, aResourceURI: &[u8]) -> Result<bool, nsresult> {
        let aResourceURI = nsCString::from(aResourceURI);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isModuleLoaded)(self as *const _, &*aResourceURI, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void unload (in AUTF8String registryLocation); */
    #[inline]
    pub unsafe fn unload(&self, registryLocation: &[u8]) -> Result<(), nsresult> {
        let registryLocation = nsCString::from(registryLocation);
        match ((*self.vtable).unload)(self as *const _, &*registryLocation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] void importGlobalProperties (in jsval aPropertyList); */


    /* [implicit_jscontext] xpcIJSWeakReference getWeakReference (in jsval obj); */


    /* void forceGC (); */
    #[inline]
    pub unsafe fn forceGC(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).forceGC)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceCC ([optional] in nsICycleCollectorListener aListener); */
    #[inline]
    pub unsafe fn forceCC(&self, aListener: Option<&nsICycleCollectorListener>) -> Result<(), nsresult> {

        match ((*self.vtable).forceCC)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finishCC (); */
    #[inline]
    pub unsafe fn finishCC(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finishCC)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ccSlice (in long long budget); */
    #[inline]
    pub unsafe fn ccSlice(&self, budget: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).ccSlice)(self as *const _, budget) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long getMaxCCSliceTimeSinceClear (); */
    #[inline]
    pub unsafe fn getMaxCCSliceTimeSinceClear(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getMaxCCSliceTimeSinceClear)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void clearMaxCCTime (); */
    #[inline]
    pub unsafe fn clearMaxCCTime(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearMaxCCTime)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceShrinkingGC (); */
    #[inline]
    pub unsafe fn forceShrinkingGC(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).forceShrinkingGC)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void schedulePreciseGC (in ScheduledGCCallback callback); */
    #[inline]
    pub unsafe fn schedulePreciseGC(&self, callback: Option<&ScheduledGCCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).schedulePreciseGC)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void schedulePreciseShrinkingGC (in ScheduledGCCallback callback); */
    #[inline]
    pub unsafe fn schedulePreciseShrinkingGC(&self, callback: Option<&ScheduledGCCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).schedulePreciseShrinkingGC)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unlinkGhostWindows (); */
    #[inline]
    pub unsafe fn unlinkGhostWindows(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unlinkGhostWindows)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval getJSTestingFunctions (); */


    /* [implicit_jscontext] jsval callFunctionWithAsyncStack (in jsval function, in nsIStackFrame stack, in AString asyncCause); */


    /* [implicit_jscontext] jsval getGlobalForObject (in jsval obj); */


    /* [implicit_jscontext] boolean isProxy (in jsval vobject); */


    /* [implicit_jscontext] jsval exportFunction (in jsval vfunction, in jsval vscope, [optional] in jsval voptions); */


    /* [implicit_jscontext] jsval createObjectIn (in jsval vobj, [optional] in jsval voptions); */


    /* [implicit_jscontext] void makeObjectPropsNormal (in jsval vobj); */


    /* bool isDeadWrapper (in jsval obj); */


    /* bool isCrossProcessWrapper (in jsval obj); */


    /* ACString getCrossProcessWrapperTag (in jsval obj); */


    /* void permitCPOWsInScope (in jsval obj); */


    /* [implicit_jscontext] void recomputeWrappers ([optional] in jsval vobj); */


    /* [implicit_jscontext] void setWantXrays (in jsval vscope); */


    /* [implicit_jscontext] void forcePermissiveCOWs (); */


    /* [implicit_jscontext] void forcePrivilegedComponentsForScope (in jsval vscope); */


    /* [implicit_jscontext] jsval getComponentsForScope (in jsval vscope); */


    /* [implicit_jscontext] void dispatch (in jsval runnable, [optional] in jsval scope); */


    /* [implicit_jscontext] attribute boolean strict; */



    /* [implicit_jscontext] attribute boolean werror; */



    /* [implicit_jscontext] attribute boolean strict_mode; */



    /* [implicit_jscontext] attribute boolean ion; */



    /* [implicit_jscontext] void setGCZeal (in long zeal); */


    /* [implicit_jscontext] void nukeSandbox (in jsval obj); */


    /* [implicit_jscontext] void blockScriptForGlobal (in jsval global); */


    /* [implicit_jscontext] void unblockScriptForGlobal (in jsval global); */


    /* bool isXrayWrapper (in jsval obj); */


    /* [implicit_jscontext] jsval waiveXrays (in jsval aVal); */


    /* [implicit_jscontext] jsval unwaiveXrays (in jsval aVal); */


    /* [implicit_jscontext] string getClassName (in jsval aObj, in bool aUnwrap); */


    /* nsIClassInfo getDOMClassInfo (in AString aClassName); */
    #[inline]
    pub unsafe fn getDOMClassInfo(&self, aClassName: &[u16]) -> Result<Option<RefPtr<nsIClassInfo>>, nsresult> {
        let aClassName = nsString::from(aClassName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getDOMClassInfo)(self as *const _, &*aClassName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] jsval getIncumbentGlobal ([optional] in jsval callback); */


    /* [implicit_jscontext] nsISupports generateXPCWrappedJS (in jsval obj, [optional] in jsval scope); */


    /* PRTime getWatchdogTimestamp (in AString aCategory); */
    #[inline]
    pub unsafe fn getWatchdogTimestamp(&self, aCategory: &[u16]) -> Result<PRTime, nsresult> {
        let aCategory = nsString::from(aCategory);
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).getWatchdogTimestamp)(self as *const _, &*aCategory, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval getJSEngineTelemetryValue (); */


    /* [implicit_jscontext] jsval cloneInto (in jsval value, in jsval scope, [optional] in jsval options); */


    /* nsIPrincipal getWebIDLCallerPrincipal (); */
    #[inline]
    pub unsafe fn getWebIDLCallerPrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getWebIDLCallerPrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] nsIPrincipal getObjectPrincipal (in jsval obj); */


    /* [implicit_jscontext] ACString getCompartmentLocation (in jsval obj); */


    /* [implicit_jscontext] void setAddonInterposition (in ACString addonId, in nsIAddonInterposition interposition); */


    /* [implicit_jscontext] void setAddonCallInterposition (in jsval target); */


    /* [implicit_jscontext] void allowCPOWsInAddon (in ACString addonId, in bool allow); */


    /* double now (); */
    #[inline]
    pub unsafe fn now(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).now)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIXPCComponentsBase {
    vtable: *const nsIXPCComponentsBaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponentsBase {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeeeada2f, 0x86c0, 0x4609,
            [0xb2, 0xbf, 0x4b, 0xf2, 0x35, 0x1b, 0x1c, 0xe6])
    }
}

unsafe impl RefCounted for nsIXPCComponentsBase {
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
pub trait nsIXPCComponentsBaseCoerce {
    fn coerce_from(v: &nsIXPCComponentsBase) -> &Self;
}

impl nsIXPCComponentsBaseCoerce for nsIXPCComponentsBase {
    #[inline]
    fn coerce_from(v: &nsIXPCComponentsBase) -> &Self {
        v
    }
}

impl nsIXPCComponentsBase {
    #[inline]
    pub fn coerce<T: nsIXPCComponentsBaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponentsBase {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCComponentsBaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponentsBase) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponentsBaseVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIXPCComponents_Interfaces interfaces; */
    pub get_interfaces: unsafe extern "C" fn (this: *const nsIXPCComponentsBase, aInterfaces: *mut *const nsIXPCComponents_Interfaces) -> nsresult,

    /* readonly attribute nsIXPCComponents_InterfacesByID interfacesByID; */
    pub get_interfacesByID: unsafe extern "C" fn (this: *const nsIXPCComponentsBase, aInterfacesByID: *mut *const nsIXPCComponents_InterfacesByID) -> nsresult,

    /* readonly attribute nsIXPCComponents_Results results; */
    pub get_results: unsafe extern "C" fn (this: *const nsIXPCComponentsBase, aResults: *mut *const nsIXPCComponents_Results) -> nsresult,

    /* boolean isSuccessCode (in nsresult result); */
    pub isSuccessCode: unsafe extern "C" fn (this: *const nsIXPCComponentsBase, result: nsresult, _retval: *mut bool) -> nsresult,

}


impl nsIXPCComponentsBase {
    /* readonly attribute nsIXPCComponents_Interfaces interfaces; */
    #[inline]
    pub unsafe fn get_interfaces(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_Interfaces>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_interfaces)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXPCComponents_InterfacesByID interfacesByID; */
    #[inline]
    pub unsafe fn get_interfacesByID(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_InterfacesByID>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_interfacesByID)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXPCComponents_Results results; */
    #[inline]
    pub unsafe fn get_results(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_Results>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_results)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isSuccessCode (in nsresult result); */
    #[inline]
    pub unsafe fn isSuccessCode(&self, result: nsresult) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSuccessCode)(self as *const _, result, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIXPCComponents {
    vtable: *const nsIXPCComponentsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCComponents {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaa28aaf6, 0x70ce, 0x4b03,
            [0x95, 0x14, 0xaf, 0xe4, 0x3c, 0x7d, 0xfd, 0xa8])
    }
}

unsafe impl RefCounted for nsIXPCComponents {
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
pub trait nsIXPCComponentsCoerce {
    fn coerce_from(v: &nsIXPCComponents) -> &Self;
}

impl nsIXPCComponentsCoerce for nsIXPCComponents {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents) -> &Self {
        v
    }
}

impl nsIXPCComponents {
    #[inline]
    pub fn coerce<T: nsIXPCComponentsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCComponents {
    type Target = nsIXPCComponentsBase;
    #[inline]
    fn deref(&self) -> &nsIXPCComponentsBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIXPCComponentsBaseCoerce> nsIXPCComponentsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCComponents) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCComponentsVTable {
    pub __base: nsIXPCComponentsBaseVTable,

    /* readonly attribute nsIXPCComponents_Classes classes; */
    pub get_classes: unsafe extern "C" fn (this: *const nsIXPCComponents, aClasses: *mut *const nsIXPCComponents_Classes) -> nsresult,

    /* readonly attribute nsIXPCComponents_ClassesByID classesByID; */
    pub get_classesByID: unsafe extern "C" fn (this: *const nsIXPCComponents, aClassesByID: *mut *const nsIXPCComponents_ClassesByID) -> nsresult,

    /* readonly attribute nsIStackFrame stack; */
    pub get_stack: unsafe extern "C" fn (this: *const nsIXPCComponents, aStack: *mut *const nsIStackFrame) -> nsresult,

    /* readonly attribute nsIComponentManager manager; */
    pub get_manager: unsafe extern "C" fn (this: *const nsIXPCComponents, aManager: *mut *const nsIComponentManager) -> nsresult,

    /* readonly attribute nsIXPCComponents_Utils utils; */
    pub get_utils: unsafe extern "C" fn (this: *const nsIXPCComponents, aUtils: *mut *const nsIXPCComponents_Utils) -> nsresult,

    /* readonly attribute nsIXPCComponents_ID ID; */
    pub get_ID: unsafe extern "C" fn (this: *const nsIXPCComponents, aID: *mut *const nsIXPCComponents_ID) -> nsresult,

    /* readonly attribute nsIXPCComponents_Exception Exception; */
    pub get_Exception: unsafe extern "C" fn (this: *const nsIXPCComponents, aException: *mut *const nsIXPCComponents_Exception) -> nsresult,

    /* readonly attribute nsIXPCComponents_Constructor Constructor; */
    pub get_Constructor: unsafe extern "C" fn (this: *const nsIXPCComponents, aConstructor: *mut *const nsIXPCComponents_Constructor) -> nsresult,

    /* [implicit_jscontext] attribute jsval returnCode; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_returnCode: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_returnCode: *const ::libc::c_void,

    /* [deprecated,implicit_jscontext] void reportError (in jsval error); */
    /// Unable to call function as its signature contains a non-rust type
    pub reportError: *const ::libc::c_void,

}


impl nsIXPCComponents {
    /* readonly attribute nsIXPCComponents_Classes classes; */
    #[inline]
    pub unsafe fn get_classes(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_Classes>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_classes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXPCComponents_ClassesByID classesByID; */
    #[inline]
    pub unsafe fn get_classesByID(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_ClassesByID>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_classesByID)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIStackFrame stack; */
    #[inline]
    pub unsafe fn get_stack(&self, ) -> Result<Option<RefPtr<nsIStackFrame>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_stack)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIComponentManager manager; */
    #[inline]
    pub unsafe fn get_manager(&self, ) -> Result<Option<RefPtr<nsIComponentManager>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_manager)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXPCComponents_Utils utils; */
    #[inline]
    pub unsafe fn get_utils(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_Utils>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_utils)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXPCComponents_ID ID; */
    #[inline]
    pub unsafe fn get_ID(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_ID>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ID)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXPCComponents_Exception Exception; */
    #[inline]
    pub unsafe fn get_Exception(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_Exception>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_Exception)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXPCComponents_Constructor Constructor; */
    #[inline]
    pub unsafe fn get_Constructor(&self, ) -> Result<Option<RefPtr<nsIXPCComponents_Constructor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_Constructor)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] attribute jsval returnCode; */



    /* [deprecated,implicit_jscontext] void reportError (in jsval error); */


}


