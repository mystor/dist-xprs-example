//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMClientRectList.idl
//


#[repr(C)]
pub struct nsIDOMClientRectList {
    vtable: *const nsIDOMClientRectListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMClientRectList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf474c567, 0xcbcb, 0x458f,
            [0xab, 0xad, 0xae, 0x42, 0x36, 0x3d, 0xa2, 0x87])
    }
}

unsafe impl RefCounted for nsIDOMClientRectList {
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
pub trait nsIDOMClientRectListCoerce {
    fn coerce_from(v: &nsIDOMClientRectList) -> &Self;
}

impl nsIDOMClientRectListCoerce for nsIDOMClientRectList {
    #[inline]
    fn coerce_from(v: &nsIDOMClientRectList) -> &Self {
        v
    }
}

impl nsIDOMClientRectList {
    #[inline]
    pub fn coerce<T: nsIDOMClientRectListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMClientRectList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMClientRectListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMClientRectList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMClientRectListVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMClientRectList, aLength: *mut libc::uint32_t) -> nsresult,

    /* nsIDOMClientRect item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMClientRectList, index: libc::uint32_t, _retval: *mut *const nsIDOMClientRect) -> nsresult,

}


impl nsIDOMClientRectList {
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

    /* nsIDOMClientRect item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMClientRect>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


