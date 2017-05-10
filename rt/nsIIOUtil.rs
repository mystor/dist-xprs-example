//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIOUtil.idl
//


#[repr(C)]
pub struct nsIIOUtil {
    vtable: *const nsIIOUtilVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIOUtil {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe8152f7f, 0x4209, 0x4c63,
            [0xad, 0x23, 0xc3, 0xd2, 0xaa, 0x0c, 0x5a, 0x49])
    }
}

unsafe impl RefCounted for nsIIOUtil {
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
pub trait nsIIOUtilCoerce {
    fn coerce_from(v: &nsIIOUtil) -> &Self;
}

impl nsIIOUtilCoerce for nsIIOUtil {
    #[inline]
    fn coerce_from(v: &nsIIOUtil) -> &Self {
        v
    }
}

impl nsIIOUtil {
    #[inline]
    pub fn coerce<T: nsIIOUtilCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIOUtil {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIOUtilCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIOUtil) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIOUtilVTable {
    pub __base: nsISupportsVTable,

    /* boolean inputStreamIsBuffered (in nsIInputStream aStream); */
    pub inputStreamIsBuffered: unsafe extern "C" fn (this: *const nsIIOUtil, aStream: *const nsIInputStream, _retval: *mut bool) -> nsresult,

    /* boolean outputStreamIsBuffered (in nsIOutputStream aStream); */
    pub outputStreamIsBuffered: unsafe extern "C" fn (this: *const nsIIOUtil, aStream: *const nsIOutputStream, _retval: *mut bool) -> nsresult,

}


impl nsIIOUtil {
    /* boolean inputStreamIsBuffered (in nsIInputStream aStream); */
    #[inline]
    pub unsafe fn inputStreamIsBuffered(&self, aStream: Option<&nsIInputStream>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).inputStreamIsBuffered)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean outputStreamIsBuffered (in nsIOutputStream aStream); */
    #[inline]
    pub unsafe fn outputStreamIsBuffered(&self, aStream: Option<&nsIOutputStream>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).outputStreamIsBuffered)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


