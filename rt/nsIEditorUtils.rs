//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorUtils.idl
//


#[repr(C)]
pub struct nsIEditorBlobListener {
    vtable: *const nsIEditorBlobListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditorBlobListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeb8b8ad9, 0x5d8f, 0x43bd,
            [0x8c, 0xe5, 0x5b, 0x94, 0x3c, 0x18, 0x0d, 0x56])
    }
}

unsafe impl RefCounted for nsIEditorBlobListener {
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
pub trait nsIEditorBlobListenerCoerce {
    fn coerce_from(v: &nsIEditorBlobListener) -> &Self;
}

impl nsIEditorBlobListenerCoerce for nsIEditorBlobListener {
    #[inline]
    fn coerce_from(v: &nsIEditorBlobListener) -> &Self {
        v
    }
}

impl nsIEditorBlobListener {
    #[inline]
    pub fn coerce<T: nsIEditorBlobListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditorBlobListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditorBlobListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorBlobListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorBlobListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onResult (in ACString aResult); */
    pub onResult: unsafe extern "C" fn (this: *const nsIEditorBlobListener, aResult: *const nsACString) -> nsresult,

    /* void onError (in AString aErrorName); */
    pub onError: unsafe extern "C" fn (this: *const nsIEditorBlobListener, aErrorName: *const nsAString) -> nsresult,

}


impl nsIEditorBlobListener {
    /* void onResult (in ACString aResult); */
    #[inline]
    pub unsafe fn onResult(&self, aResult: &[u8]) -> Result<(), nsresult> {
        let aResult = nsCString::from(aResult);
        match ((*self.vtable).onResult)(self as *const _, &*aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onError (in AString aErrorName); */
    #[inline]
    pub unsafe fn onError(&self, aErrorName: &[u16]) -> Result<(), nsresult> {
        let aErrorName = nsString::from(aErrorName);
        match ((*self.vtable).onError)(self as *const _, &*aErrorName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIEditorUtils {
    vtable: *const nsIEditorUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditorUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4bf94928, 0x575e, 0x4bd1,
            [0x83, 0x21, 0xa2, 0xc4, 0xb3, 0xd0, 0x11, 0x9e])
    }
}

unsafe impl RefCounted for nsIEditorUtils {
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
pub trait nsIEditorUtilsCoerce {
    fn coerce_from(v: &nsIEditorUtils) -> &Self;
}

impl nsIEditorUtilsCoerce for nsIEditorUtils {
    #[inline]
    fn coerce_from(v: &nsIEditorUtils) -> &Self {
        v
    }
}

impl nsIEditorUtils {
    #[inline]
    pub fn coerce<T: nsIEditorUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditorUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditorUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorUtilsVTable {
    pub __base: nsISupportsVTable,

    /* void slurpBlob (in nsIDOMBlob aBlob, in mozIDOMWindowProxy aScope, in nsIEditorBlobListener aListener); */
    pub slurpBlob: unsafe extern "C" fn (this: *const nsIEditorUtils, aBlob: *const nsIDOMBlob, aScope: *const mozIDOMWindowProxy, aListener: *const nsIEditorBlobListener) -> nsresult,

}


impl nsIEditorUtils {
    /* void slurpBlob (in nsIDOMBlob aBlob, in mozIDOMWindowProxy aScope, in nsIEditorBlobListener aListener); */
    #[inline]
    pub unsafe fn slurpBlob(&self, aBlob: Option<&nsIDOMBlob>, aScope: Option<&mozIDOMWindowProxy>, aListener: Option<&nsIEditorBlobListener>) -> Result<(), nsresult> {

        match ((*self.vtable).slurpBlob)(self as *const _, aBlob.map_or(::std::ptr::null(), |x| x as *const _), aScope.map_or(::std::ptr::null(), |x| x as *const _), aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


