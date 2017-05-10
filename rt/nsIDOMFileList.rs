//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMFileList.idl
//


#[repr(C)]
pub struct nsIDOMFileList {
    vtable: *const nsIDOMFileListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMFileList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x57128a85, 0x34de, 0x42db,
            [0xa2, 0x52, 0x84, 0xdd, 0x57, 0x72, 0x4a, 0x59])
    }
}

unsafe impl RefCounted for nsIDOMFileList {
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
pub trait nsIDOMFileListCoerce {
    fn coerce_from(v: &nsIDOMFileList) -> &Self;
}

impl nsIDOMFileListCoerce for nsIDOMFileList {
    #[inline]
    fn coerce_from(v: &nsIDOMFileList) -> &Self {
        v
    }
}

impl nsIDOMFileList {
    #[inline]
    pub fn coerce<T: nsIDOMFileListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMFileList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMFileListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMFileList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMFileListVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMFileList, aLength: *mut libc::uint32_t) -> nsresult,

    /* nsISupports item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMFileList, index: libc::uint32_t, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIDOMFileList {
    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


