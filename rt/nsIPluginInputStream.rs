//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPluginInputStream.idl
//


#[repr(C)]
pub struct nsIPluginInputStream {
    vtable: *const nsIPluginInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPluginInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaf160530, 0x542a, 0x11d2,
            [0x81, 0x64, 0x00, 0x60, 0x08, 0x11, 0x9d, 0x7a])
    }
}

unsafe impl RefCounted for nsIPluginInputStream {
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
pub trait nsIPluginInputStreamCoerce {
    fn coerce_from(v: &nsIPluginInputStream) -> &Self;
}

impl nsIPluginInputStreamCoerce for nsIPluginInputStream {
    #[inline]
    fn coerce_from(v: &nsIPluginInputStream) -> &Self {
        v
    }
}

impl nsIPluginInputStream {
    #[inline]
    pub fn coerce<T: nsIPluginInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPluginInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIPluginInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPluginInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* void getLastModified (out unsigned long aResult); */
    pub getLastModified: unsafe extern "C" fn (this: *const nsIPluginInputStream, aResult: *mut libc::uint32_t) -> nsresult,

    /* void requestRead (out NPByteRange aRangeList); */
    /// Unable to call function as its signature contains a non-rust type
    pub requestRead: *const ::libc::c_void,

}


impl nsIPluginInputStream {
    /* void getLastModified (out unsigned long aResult); */
    #[inline]
    pub unsafe fn getLastModified(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut aResult: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getLastModified)(self as *const _, &mut aResult as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aResult)
    }

    /* void requestRead (out NPByteRange aRangeList); */


}


