//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamingProtocolService.idl
//


#[repr(C)]
pub struct nsIStreamingProtocolControllerService {
    vtable: *const nsIStreamingProtocolControllerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamingProtocolControllerService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x94838530, 0x8627, 0x11e2,
            [0x9e, 0x96, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIStreamingProtocolControllerService {
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
pub trait nsIStreamingProtocolControllerServiceCoerce {
    fn coerce_from(v: &nsIStreamingProtocolControllerService) -> &Self;
}

impl nsIStreamingProtocolControllerServiceCoerce for nsIStreamingProtocolControllerService {
    #[inline]
    fn coerce_from(v: &nsIStreamingProtocolControllerService) -> &Self {
        v
    }
}

impl nsIStreamingProtocolControllerService {
    #[inline]
    pub fn coerce<T: nsIStreamingProtocolControllerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamingProtocolControllerService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStreamingProtocolControllerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamingProtocolControllerService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamingProtocolControllerServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIStreamingProtocolController create (in nsIChannel channel); */
    pub create: unsafe extern "C" fn (this: *const nsIStreamingProtocolControllerService, channel: *const nsIChannel, _retval: *mut *const nsIStreamingProtocolController) -> nsresult,

}


impl nsIStreamingProtocolControllerService {
    /* nsIStreamingProtocolController create (in nsIChannel channel); */
    #[inline]
    pub unsafe fn create(&self, channel: Option<&nsIChannel>) -> Result<Option<RefPtr<nsIStreamingProtocolController>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).create)(self as *const _, channel.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


