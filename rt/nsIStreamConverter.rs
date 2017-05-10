//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamConverter.idl
//


#[repr(C)]
pub struct nsIStreamConverter {
    vtable: *const nsIStreamConverterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamConverter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0b6e2c69, 0x5cf5, 0x48b0,
            [0x9d, 0xfd, 0xc9, 0x59, 0x50, 0xe2, 0xcc, 0x7b])
    }
}

unsafe impl RefCounted for nsIStreamConverter {
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
pub trait nsIStreamConverterCoerce {
    fn coerce_from(v: &nsIStreamConverter) -> &Self;
}

impl nsIStreamConverterCoerce for nsIStreamConverter {
    #[inline]
    fn coerce_from(v: &nsIStreamConverter) -> &Self {
        v
    }
}

impl nsIStreamConverter {
    #[inline]
    pub fn coerce<T: nsIStreamConverterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamConverter {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIStreamConverterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamConverter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamConverterVTable {
    pub __base: nsIStreamListenerVTable,

    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aCtxt); */
    pub convert: unsafe extern "C" fn (this: *const nsIStreamConverter, aFromStream: *const nsIInputStream, aFromType: *const libc::c_char, aToType: *const libc::c_char, aCtxt: *const nsISupports, _retval: *mut *const nsIInputStream) -> nsresult,

    /* void asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aCtxt); */
    pub asyncConvertData: unsafe extern "C" fn (this: *const nsIStreamConverter, aFromType: *const libc::c_char, aToType: *const libc::c_char, aListener: *const nsIStreamListener, aCtxt: *const nsISupports) -> nsresult,

}


impl nsIStreamConverter {
    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aCtxt); */
    #[inline]
    pub unsafe fn convert(&self, aFromStream: Option<&nsIInputStream>, aFromType: *const libc::c_char, aToType: *const libc::c_char, aCtxt: Option<&nsISupports>) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).convert)(self as *const _, aFromStream.map_or(::std::ptr::null(), |x| x as *const _), aFromType, aToType, aCtxt.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aCtxt); */
    #[inline]
    pub unsafe fn asyncConvertData(&self, aFromType: *const libc::c_char, aToType: *const libc::c_char, aListener: Option<&nsIStreamListener>, aCtxt: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncConvertData)(self as *const _, aFromType, aToType, aListener.map_or(::std::ptr::null(), |x| x as *const _), aCtxt.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


