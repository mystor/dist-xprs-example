//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPushNotifier.idl
//


#[repr(C)]
pub struct nsIPushNotifier {
    vtable: *const nsIPushNotifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushNotifier {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb00dfdeb, 0x14e5, 0x425b,
            [0xad, 0xc7, 0xb5, 0x31, 0x44, 0x2e, 0x32, 0x16])
    }
}

unsafe impl RefCounted for nsIPushNotifier {
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
pub trait nsIPushNotifierCoerce {
    fn coerce_from(v: &nsIPushNotifier) -> &Self;
}

impl nsIPushNotifierCoerce for nsIPushNotifier {
    #[inline]
    fn coerce_from(v: &nsIPushNotifier) -> &Self {
        v
    }
}

impl nsIPushNotifier {
    #[inline]
    pub fn coerce<T: nsIPushNotifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushNotifier {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushNotifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushNotifier) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushNotifierVTable {
    pub __base: nsISupportsVTable,

    /* void notifyPush (in ACString scope, in nsIPrincipal principal, in DOMString messageId); */
    pub notifyPush: unsafe extern "C" fn (this: *const nsIPushNotifier, scope: *const nsACString, principal: *const nsIPrincipal, messageId: *const nsAString) -> nsresult,

    /* void notifyPushWithData (in ACString scope, in nsIPrincipal principal, in DOMString messageId, [optional] in uint32_t dataLen, [array, size_is (dataLen)] in uint8_t data); */
    /// Unable to call function as its signature contains a non-rust type
    pub notifyPushWithData: *const ::libc::c_void,

    /* void notifySubscriptionChange (in ACString scope, in nsIPrincipal principal); */
    pub notifySubscriptionChange: unsafe extern "C" fn (this: *const nsIPushNotifier, scope: *const nsACString, principal: *const nsIPrincipal) -> nsresult,

    /* void notifySubscriptionModified (in ACString scope, in nsIPrincipal principal); */
    pub notifySubscriptionModified: unsafe extern "C" fn (this: *const nsIPushNotifier, scope: *const nsACString, principal: *const nsIPrincipal) -> nsresult,

    /* void notifyError (in ACString scope, in nsIPrincipal principal, in DOMString message, in uint32_t flags); */
    pub notifyError: unsafe extern "C" fn (this: *const nsIPushNotifier, scope: *const nsACString, principal: *const nsIPrincipal, message: *const nsAString, flags: uint32_t) -> nsresult,

}


impl nsIPushNotifier {
    /* void notifyPush (in ACString scope, in nsIPrincipal principal, in DOMString messageId); */
    #[inline]
    pub unsafe fn notifyPush(&self, scope: &[u8], principal: Option<&nsIPrincipal>, messageId: &[u16]) -> Result<(), nsresult> {
        let scope = nsCString::from(scope);
        let messageId = nsString::from(messageId);
        match ((*self.vtable).notifyPush)(self as *const _, &*scope, principal.map_or(::std::ptr::null(), |x| x as *const _), &*messageId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyPushWithData (in ACString scope, in nsIPrincipal principal, in DOMString messageId, [optional] in uint32_t dataLen, [array, size_is (dataLen)] in uint8_t data); */


    /* void notifySubscriptionChange (in ACString scope, in nsIPrincipal principal); */
    #[inline]
    pub unsafe fn notifySubscriptionChange(&self, scope: &[u8], principal: Option<&nsIPrincipal>) -> Result<(), nsresult> {
        let scope = nsCString::from(scope);
        match ((*self.vtable).notifySubscriptionChange)(self as *const _, &*scope, principal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifySubscriptionModified (in ACString scope, in nsIPrincipal principal); */
    #[inline]
    pub unsafe fn notifySubscriptionModified(&self, scope: &[u8], principal: Option<&nsIPrincipal>) -> Result<(), nsresult> {
        let scope = nsCString::from(scope);
        match ((*self.vtable).notifySubscriptionModified)(self as *const _, &*scope, principal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyError (in ACString scope, in nsIPrincipal principal, in DOMString message, in uint32_t flags); */
    #[inline]
    pub unsafe fn notifyError(&self, scope: &[u8], principal: Option<&nsIPrincipal>, message: &[u16], flags: uint32_t) -> Result<(), nsresult> {
        let scope = nsCString::from(scope);
        let message = nsString::from(message);
        match ((*self.vtable).notifyError)(self as *const _, &*scope, principal.map_or(::std::ptr::null(), |x| x as *const _), &*message, flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPushData {
    vtable: *const nsIPushDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushData {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdfc4f151, 0xcead, 0x40df,
            [0x8e, 0xb7, 0x7a, 0x7a, 0x67, 0xc5, 0x4b, 0x16])
    }
}

unsafe impl RefCounted for nsIPushData {
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
pub trait nsIPushDataCoerce {
    fn coerce_from(v: &nsIPushData) -> &Self;
}

impl nsIPushDataCoerce for nsIPushData {
    #[inline]
    fn coerce_from(v: &nsIPushData) -> &Self {
        v
    }
}

impl nsIPushData {
    #[inline]
    pub fn coerce<T: nsIPushDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushData {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushData) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushDataVTable {
    pub __base: nsISupportsVTable,

    /* DOMString text (); */
    pub text: unsafe extern "C" fn (this: *const nsIPushData, _retval: *mut nsAString) -> nsresult,

    /* [implicit_jscontext] jsval json (); */
    /// Unable to call function as its signature contains a non-rust type
    pub json: *const ::libc::c_void,

    /* void binary ([optional] out uint32_t dataLen, [array, size_is (dataLen), retval] out uint8_t data); */
    /// Unable to call function as its signature contains a non-rust type
    pub binary: *const ::libc::c_void,

}


impl nsIPushData {
    /* DOMString text (); */
    #[inline]
    pub unsafe fn text(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).text)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval json (); */


    /* void binary ([optional] out uint32_t dataLen, [array, size_is (dataLen), retval] out uint8_t data); */


}


#[repr(C)]
pub struct nsIPushMessage {
    vtable: *const nsIPushMessageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPushMessage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb9d063ca, 0x0e3f, 0x4fee,
            [0xbe, 0x4b, 0xea, 0x91, 0x03, 0x26, 0x34, 0x33])
    }
}

unsafe impl RefCounted for nsIPushMessage {
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
pub trait nsIPushMessageCoerce {
    fn coerce_from(v: &nsIPushMessage) -> &Self;
}

impl nsIPushMessageCoerce for nsIPushMessage {
    #[inline]
    fn coerce_from(v: &nsIPushMessage) -> &Self {
        v
    }
}

impl nsIPushMessage {
    #[inline]
    pub fn coerce<T: nsIPushMessageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPushMessage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPushMessageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushMessage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPushMessageVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIPushMessage, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* readonly attribute nsIPushData data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIPushMessage, aData: *mut *const nsIPushData) -> nsresult,

}


impl nsIPushMessage {
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

    /* readonly attribute nsIPushData data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<Option<RefPtr<nsIPushData>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_data)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


