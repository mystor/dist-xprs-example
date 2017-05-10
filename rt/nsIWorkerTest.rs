//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWorkerTest.idl
//


#[repr(C)]
pub struct nsIWorkerTestCallback {
    vtable: *const nsIWorkerTestCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWorkerTestCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x10f8ebdf, 0x1373, 0x4640,
            [0x9c, 0x34, 0x53, 0xde, 0xe9, 0x9f, 0x52, 0x6f])
    }
}

unsafe impl RefCounted for nsIWorkerTestCallback {
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
pub trait nsIWorkerTestCallbackCoerce {
    fn coerce_from(v: &nsIWorkerTestCallback) -> &Self;
}

impl nsIWorkerTestCallbackCoerce for nsIWorkerTestCallback {
    #[inline]
    fn coerce_from(v: &nsIWorkerTestCallback) -> &Self {
        v
    }
}

impl nsIWorkerTestCallback {
    #[inline]
    pub fn coerce<T: nsIWorkerTestCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWorkerTestCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWorkerTestCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerTestCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWorkerTestCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onmessage (in DOMString data); */
    pub onmessage: unsafe extern "C" fn (this: *const nsIWorkerTestCallback, data: *const nsAString) -> nsresult,

    /* void onerror (in DOMString data); */
    pub onerror: unsafe extern "C" fn (this: *const nsIWorkerTestCallback, data: *const nsAString) -> nsresult,

}


impl nsIWorkerTestCallback {
    /* void onmessage (in DOMString data); */
    #[inline]
    pub unsafe fn onmessage(&self, data: &[u16]) -> Result<(), nsresult> {
        let data = nsString::from(data);
        match ((*self.vtable).onmessage)(self as *const _, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onerror (in DOMString data); */
    #[inline]
    pub unsafe fn onerror(&self, data: &[u16]) -> Result<(), nsresult> {
        let data = nsString::from(data);
        match ((*self.vtable).onerror)(self as *const _, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIWorkerTest {
    vtable: *const nsIWorkerTestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWorkerTest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x887a0614, 0xa0f0, 0x4c0e,
            [0x80, 0xe0, 0xcf, 0x31, 0xe6, 0xd4, 0xe2, 0x86])
    }
}

unsafe impl RefCounted for nsIWorkerTest {
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
pub trait nsIWorkerTestCoerce {
    fn coerce_from(v: &nsIWorkerTest) -> &Self;
}

impl nsIWorkerTestCoerce for nsIWorkerTest {
    #[inline]
    fn coerce_from(v: &nsIWorkerTest) -> &Self {
        v
    }
}

impl nsIWorkerTest {
    #[inline]
    pub fn coerce<T: nsIWorkerTestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWorkerTest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWorkerTestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerTest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWorkerTestVTable {
    pub __base: nsISupportsVTable,

    /* void postMessage (in DOMString data); */
    pub postMessage: unsafe extern "C" fn (this: *const nsIWorkerTest, data: *const nsAString) -> nsresult,

    /* void terminate (); */
    pub terminate: unsafe extern "C" fn (this: *const nsIWorkerTest) -> nsresult,

    /* attribute nsIWorkerTestCallback callback; */
    pub get_callback: unsafe extern "C" fn (this: *const nsIWorkerTest, aCallback: *mut *const nsIWorkerTestCallback) -> nsresult,
    pub set_callback: unsafe extern "C" fn (this: *const nsIWorkerTest, aCallback: *const nsIWorkerTestCallback) -> nsresult,

}


impl nsIWorkerTest {
    /* void postMessage (in DOMString data); */
    #[inline]
    pub unsafe fn postMessage(&self, data: &[u16]) -> Result<(), nsresult> {
        let data = nsString::from(data);
        match ((*self.vtable).postMessage)(self as *const _, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void terminate (); */
    #[inline]
    pub unsafe fn terminate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).terminate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIWorkerTestCallback callback; */
    #[inline]
    pub unsafe fn get_callback(&self, ) -> Result<Option<RefPtr<nsIWorkerTestCallback>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_callback)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_callback(&self, aCallback: Option<&nsIWorkerTestCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).set_callback)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


