//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamConverterService.idl
//


#[repr(C)]
pub struct nsIStreamConverterService {
    vtable: *const nsIStreamConverterServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamConverterService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf2b1ab53, 0xf0bd, 0x4adb,
            [0x93, 0x65, 0xe5, 0x9b, 0x17, 0x01, 0xa2, 0x58])
    }
}

unsafe impl RefCounted for nsIStreamConverterService {
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
pub trait nsIStreamConverterServiceCoerce {
    fn coerce_from(v: &nsIStreamConverterService) -> &Self;
}

impl nsIStreamConverterServiceCoerce for nsIStreamConverterService {
    #[inline]
    fn coerce_from(v: &nsIStreamConverterService) -> &Self {
        v
    }
}

impl nsIStreamConverterService {
    #[inline]
    pub fn coerce<T: nsIStreamConverterServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamConverterService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStreamConverterServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamConverterService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamConverterServiceVTable {
    pub __base: nsISupportsVTable,

    /* boolean canConvert (in string aFromType, in string aToType); */
    pub canConvert: unsafe extern "C" fn (this: *const nsIStreamConverterService, aFromType: *const libc::c_char, aToType: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aContext); */
    pub convert: unsafe extern "C" fn (this: *const nsIStreamConverterService, aFromStream: *const nsIInputStream, aFromType: *const libc::c_char, aToType: *const libc::c_char, aContext: *const nsISupports, _retval: *mut *const nsIInputStream) -> nsresult,

    /* nsIStreamListener asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aContext); */
    pub asyncConvertData: unsafe extern "C" fn (this: *const nsIStreamConverterService, aFromType: *const libc::c_char, aToType: *const libc::c_char, aListener: *const nsIStreamListener, aContext: *const nsISupports, _retval: *mut *const nsIStreamListener) -> nsresult,

}


impl nsIStreamConverterService {
    /* boolean canConvert (in string aFromType, in string aToType); */
    #[inline]
    pub unsafe fn canConvert(&self, aFromType: *const libc::c_char, aToType: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canConvert)(self as *const _, aFromType, aToType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aContext); */
    #[inline]
    pub unsafe fn convert(&self, aFromStream: Option<&nsIInputStream>, aFromType: *const libc::c_char, aToType: *const libc::c_char, aContext: Option<&nsISupports>) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).convert)(self as *const _, aFromStream.map_or(::std::ptr::null(), |x| x as *const _), aFromType, aToType, aContext.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIStreamListener asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aContext); */
    #[inline]
    pub unsafe fn asyncConvertData(&self, aFromType: *const libc::c_char, aToType: *const libc::c_char, aListener: Option<&nsIStreamListener>, aContext: Option<&nsISupports>) -> Result<Option<RefPtr<nsIStreamListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).asyncConvertData)(self as *const _, aFromType, aToType, aListener.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


