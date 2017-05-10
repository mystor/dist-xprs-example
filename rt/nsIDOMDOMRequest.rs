//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDOMRequest.idl
//


#[repr(C)]
pub struct nsIDOMDOMRequest {
    vtable: *const nsIDOMDOMRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDOMRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe39da69e, 0x2232, 0x4e49,
            [0x98, 0x56, 0xb8, 0xa4, 0xa6, 0x21, 0x03, 0x36])
    }
}

unsafe impl RefCounted for nsIDOMDOMRequest {
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
pub trait nsIDOMDOMRequestCoerce {
    fn coerce_from(v: &nsIDOMDOMRequest) -> &Self;
}

impl nsIDOMDOMRequestCoerce for nsIDOMDOMRequest {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMRequest) -> &Self {
        v
    }
}

impl nsIDOMDOMRequest {
    #[inline]
    pub fn coerce<T: nsIDOMDOMRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDOMRequest {
    type Target = nsIDOMEventTarget;
    #[inline]
    fn deref(&self) -> &nsIDOMEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMEventTargetCoerce> nsIDOMDOMRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDOMRequestVTable {
    pub __base: nsIDOMEventTargetVTable,

    /* readonly attribute DOMString readyState; */
    pub get_readyState: unsafe extern "C" fn (this: *const nsIDOMDOMRequest, aReadyState: *mut nsAString) -> nsresult,

    /* readonly attribute jsval result; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_result: *const ::libc::c_void,

    /* readonly attribute nsISupports error; */
    pub get_error: unsafe extern "C" fn (this: *const nsIDOMDOMRequest, aError: *mut *const nsISupports) -> nsresult,

    /* [implicit_jscontext] attribute jsval onsuccess; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onsuccess: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onsuccess: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onerror; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onerror: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onerror: *const ::libc::c_void,

}


impl nsIDOMDOMRequest {
    /* readonly attribute DOMString readyState; */
    #[inline]
    pub unsafe fn get_readyState(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_readyState)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute jsval result; */


    /* readonly attribute nsISupports error; */
    #[inline]
    pub unsafe fn get_error(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_error)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] attribute jsval onsuccess; */



    /* [implicit_jscontext] attribute jsval onerror; */



}


#[repr(C)]
pub struct nsIDOMRequestService {
    vtable: *const nsIDOMRequestServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMRequestService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9a57e5de, 0xce93, 0x45fa,
            [0x81, 0x45, 0x75, 0x57, 0x22, 0x83, 0x4f, 0x7c])
    }
}

unsafe impl RefCounted for nsIDOMRequestService {
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
pub trait nsIDOMRequestServiceCoerce {
    fn coerce_from(v: &nsIDOMRequestService) -> &Self;
}

impl nsIDOMRequestServiceCoerce for nsIDOMRequestService {
    #[inline]
    fn coerce_from(v: &nsIDOMRequestService) -> &Self {
        v
    }
}

impl nsIDOMRequestService {
    #[inline]
    pub fn coerce<T: nsIDOMRequestServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMRequestService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMRequestServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMRequestService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMRequestServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMDOMRequest createRequest (in mozIDOMWindow window); */
    pub createRequest: unsafe extern "C" fn (this: *const nsIDOMRequestService, window: *const mozIDOMWindow, _retval: *mut *const nsIDOMDOMRequest) -> nsresult,

    /* nsIDOMDOMCursor createCursor (in mozIDOMWindow window, in nsICursorContinueCallback aCallback); */
    pub createCursor: unsafe extern "C" fn (this: *const nsIDOMRequestService, window: *const mozIDOMWindow, aCallback: *const nsICursorContinueCallback, _retval: *mut *const nsIDOMDOMCursor) -> nsresult,

    /* void fireSuccess (in nsIDOMDOMRequest request, in jsval result); */
    /// Unable to call function as its signature contains a non-rust type
    pub fireSuccess: *const ::libc::c_void,

    /* void fireError (in nsIDOMDOMRequest request, in DOMString error); */
    pub fireError: unsafe extern "C" fn (this: *const nsIDOMRequestService, request: *const nsIDOMDOMRequest, error: *const nsAString) -> nsresult,

    /* void fireDetailedError (in nsIDOMDOMRequest request, in nsISupports error); */
    pub fireDetailedError: unsafe extern "C" fn (this: *const nsIDOMRequestService, request: *const nsIDOMDOMRequest, error: *const nsISupports) -> nsresult,

    /* void fireSuccessAsync (in nsIDOMDOMRequest request, in jsval result); */
    /// Unable to call function as its signature contains a non-rust type
    pub fireSuccessAsync: *const ::libc::c_void,

    /* void fireErrorAsync (in nsIDOMDOMRequest request, in DOMString error); */
    pub fireErrorAsync: unsafe extern "C" fn (this: *const nsIDOMRequestService, request: *const nsIDOMDOMRequest, error: *const nsAString) -> nsresult,

    /* void fireDone (in nsIDOMDOMCursor cursor); */
    pub fireDone: unsafe extern "C" fn (this: *const nsIDOMRequestService, cursor: *const nsIDOMDOMCursor) -> nsresult,

}


impl nsIDOMRequestService {
    /* nsIDOMDOMRequest createRequest (in mozIDOMWindow window); */
    #[inline]
    pub unsafe fn createRequest(&self, window: Option<&mozIDOMWindow>) -> Result<Option<RefPtr<nsIDOMDOMRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createRequest)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDOMCursor createCursor (in mozIDOMWindow window, in nsICursorContinueCallback aCallback); */
    #[inline]
    pub unsafe fn createCursor(&self, window: Option<&mozIDOMWindow>, aCallback: Option<&nsICursorContinueCallback>) -> Result<Option<RefPtr<nsIDOMDOMCursor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createCursor)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void fireSuccess (in nsIDOMDOMRequest request, in jsval result); */


    /* void fireError (in nsIDOMDOMRequest request, in DOMString error); */
    #[inline]
    pub unsafe fn fireError(&self, request: Option<&nsIDOMDOMRequest>, error: &[u16]) -> Result<(), nsresult> {
        let error = nsString::from(error);
        match ((*self.vtable).fireError)(self as *const _, request.map_or(::std::ptr::null(), |x| x as *const _), &*error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void fireDetailedError (in nsIDOMDOMRequest request, in nsISupports error); */
    #[inline]
    pub unsafe fn fireDetailedError(&self, request: Option<&nsIDOMDOMRequest>, error: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).fireDetailedError)(self as *const _, request.map_or(::std::ptr::null(), |x| x as *const _), error.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void fireSuccessAsync (in nsIDOMDOMRequest request, in jsval result); */


    /* void fireErrorAsync (in nsIDOMDOMRequest request, in DOMString error); */
    #[inline]
    pub unsafe fn fireErrorAsync(&self, request: Option<&nsIDOMDOMRequest>, error: &[u16]) -> Result<(), nsresult> {
        let error = nsString::from(error);
        match ((*self.vtable).fireErrorAsync)(self as *const _, request.map_or(::std::ptr::null(), |x| x as *const _), &*error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void fireDone (in nsIDOMDOMCursor cursor); */
    #[inline]
    pub unsafe fn fireDone(&self, cursor: Option<&nsIDOMDOMCursor>) -> Result<(), nsresult> {

        match ((*self.vtable).fireDone)(self as *const _, cursor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


