//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWorkerDebugger.idl
//


#[repr(C)]
pub struct nsIWorkerDebuggerListener {
    vtable: *const nsIWorkerDebuggerListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWorkerDebuggerListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9cf3b48e, 0x361d, 0x486a,
            [0x89, 0x17, 0x55, 0xcf, 0x8d, 0x00, 0xbb, 0x41])
    }
}

unsafe impl RefCounted for nsIWorkerDebuggerListener {
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
pub trait nsIWorkerDebuggerListenerCoerce {
    fn coerce_from(v: &nsIWorkerDebuggerListener) -> &Self;
}

impl nsIWorkerDebuggerListenerCoerce for nsIWorkerDebuggerListener {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerListener) -> &Self {
        v
    }
}

impl nsIWorkerDebuggerListener {
    #[inline]
    pub fn coerce<T: nsIWorkerDebuggerListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWorkerDebuggerListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWorkerDebuggerListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWorkerDebuggerListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onClose (); */
    pub onClose: unsafe extern "C" fn (this: *const nsIWorkerDebuggerListener) -> nsresult,

    /* void onError (in DOMString filename, in unsigned long lineno, in DOMString message); */
    pub onError: unsafe extern "C" fn (this: *const nsIWorkerDebuggerListener, filename: *const nsAString, lineno: libc::uint32_t, message: *const nsAString) -> nsresult,

    /* void onMessage (in DOMString message); */
    pub onMessage: unsafe extern "C" fn (this: *const nsIWorkerDebuggerListener, message: *const nsAString) -> nsresult,

}


impl nsIWorkerDebuggerListener {
    /* void onClose (); */
    #[inline]
    pub unsafe fn onClose(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onClose)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onError (in DOMString filename, in unsigned long lineno, in DOMString message); */
    #[inline]
    pub unsafe fn onError(&self, filename: &[u16], lineno: libc::uint32_t, message: &[u16]) -> Result<(), nsresult> {
        let filename = nsString::from(filename);
        let message = nsString::from(message);
        match ((*self.vtable).onError)(self as *const _, &*filename, lineno, &*message) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onMessage (in DOMString message); */
    #[inline]
    pub unsafe fn onMessage(&self, message: &[u16]) -> Result<(), nsresult> {
        let message = nsString::from(message);
        match ((*self.vtable).onMessage)(self as *const _, &*message) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIWorkerDebugger_consts {
    pub const TYPE_DEDICATED: i64 = 0;
    pub const TYPE_SHARED: i64 = 1;
    pub const TYPE_SERVICE: i64 = 2;
}


#[repr(C)]
pub struct nsIWorkerDebugger {
    vtable: *const nsIWorkerDebuggerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWorkerDebugger {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x22f93aa3, 0x8a05, 0x46be,
            [0x87, 0xe0, 0xfa, 0x93, 0xbf, 0x8a, 0x8e, 0xff])
    }
}

unsafe impl RefCounted for nsIWorkerDebugger {
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
pub trait nsIWorkerDebuggerCoerce {
    fn coerce_from(v: &nsIWorkerDebugger) -> &Self;
}

impl nsIWorkerDebuggerCoerce for nsIWorkerDebugger {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebugger) -> &Self {
        v
    }
}

impl nsIWorkerDebugger {
    #[inline]
    pub fn coerce<T: nsIWorkerDebuggerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWorkerDebugger {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWorkerDebuggerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebugger) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWorkerDebuggerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute bool isClosed; */
    pub get_isClosed: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aIsClosed: *mut bool) -> nsresult,

    /* readonly attribute bool isChrome; */
    pub get_isChrome: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aIsChrome: *mut bool) -> nsresult,

    /* readonly attribute bool isInitialized; */
    pub get_isInitialized: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aIsInitialized: *mut bool) -> nsresult,

    /* readonly attribute nsIWorkerDebugger parent; */
    pub get_parent: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aParent: *mut *const nsIWorkerDebugger) -> nsresult,

    /* readonly attribute unsigned long type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute DOMString url; */
    pub get_url: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aUrl: *mut nsAString) -> nsresult,

    /* readonly attribute mozIDOMWindow window; */
    pub get_window: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aWindow: *mut *const mozIDOMWindow) -> nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* readonly attribute unsigned long serviceWorkerID; */
    pub get_serviceWorkerID: unsafe extern "C" fn (this: *const nsIWorkerDebugger, aServiceWorkerID: *mut libc::uint32_t) -> nsresult,

    /* void initialize (in DOMString url); */
    pub initialize: unsafe extern "C" fn (this: *const nsIWorkerDebugger, url: *const nsAString) -> nsresult,

    /* [binaryname(PostMessageMoz)] void postMessage (in DOMString message); */
    pub PostMessageMoz: unsafe extern "C" fn (this: *const nsIWorkerDebugger, message: *const nsAString) -> nsresult,

    /* void addListener (in nsIWorkerDebuggerListener listener); */
    pub addListener: unsafe extern "C" fn (this: *const nsIWorkerDebugger, listener: *const nsIWorkerDebuggerListener) -> nsresult,

    /* void removeListener (in nsIWorkerDebuggerListener listener); */
    pub removeListener: unsafe extern "C" fn (this: *const nsIWorkerDebugger, listener: *const nsIWorkerDebuggerListener) -> nsresult,

}


impl nsIWorkerDebugger {
    /* readonly attribute bool isClosed; */
    #[inline]
    pub unsafe fn get_isClosed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isClosed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool isChrome; */
    #[inline]
    pub unsafe fn get_isChrome(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isChrome)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool isInitialized; */
    #[inline]
    pub unsafe fn get_isInitialized(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInitialized)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIWorkerDebugger parent; */
    #[inline]
    pub unsafe fn get_parent(&self, ) -> Result<Option<RefPtr<nsIWorkerDebugger>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString url; */
    #[inline]
    pub unsafe fn get_url(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_url)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* readonly attribute unsigned long serviceWorkerID; */
    #[inline]
    pub unsafe fn get_serviceWorkerID(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_serviceWorkerID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void initialize (in DOMString url); */
    #[inline]
    pub unsafe fn initialize(&self, url: &[u16]) -> Result<(), nsresult> {
        let url = nsString::from(url);
        match ((*self.vtable).initialize)(self as *const _, &*url) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(PostMessageMoz)] void postMessage (in DOMString message); */
    #[inline]
    pub unsafe fn PostMessageMoz(&self, message: &[u16]) -> Result<(), nsresult> {
        let message = nsString::from(message);
        match ((*self.vtable).PostMessageMoz)(self as *const _, &*message) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addListener (in nsIWorkerDebuggerListener listener); */
    #[inline]
    pub unsafe fn addListener(&self, listener: Option<&nsIWorkerDebuggerListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListener (in nsIWorkerDebuggerListener listener); */
    #[inline]
    pub unsafe fn removeListener(&self, listener: Option<&nsIWorkerDebuggerListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


