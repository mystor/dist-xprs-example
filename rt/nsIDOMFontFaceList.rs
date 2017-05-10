//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMFontFaceList.idl
//


#[repr(C)]
pub struct nsIDOMFontFaceList {
    vtable: *const nsIDOMFontFaceListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMFontFaceList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2538579c, 0x9472, 0x4fd9,
            [0x8d, 0xc1, 0xd4, 0x4c, 0xe4, 0xc1, 0xb7, 0xba])
    }
}

unsafe impl RefCounted for nsIDOMFontFaceList {
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
pub trait nsIDOMFontFaceListCoerce {
    fn coerce_from(v: &nsIDOMFontFaceList) -> &Self;
}

impl nsIDOMFontFaceListCoerce for nsIDOMFontFaceList {
    #[inline]
    fn coerce_from(v: &nsIDOMFontFaceList) -> &Self {
        v
    }
}

impl nsIDOMFontFaceList {
    #[inline]
    pub fn coerce<T: nsIDOMFontFaceListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMFontFaceList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMFontFaceListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMFontFaceList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMFontFaceListVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMFontFace item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMFontFaceList, index: libc::uint32_t, _retval: *mut *const nsIDOMFontFace) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMFontFaceList, aLength: *mut libc::uint32_t) -> nsresult,

}


impl nsIDOMFontFaceList {
    /* nsIDOMFontFace item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMFontFace>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

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

}


