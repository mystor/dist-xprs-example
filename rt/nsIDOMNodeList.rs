//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNodeList.idl
//


#[repr(C)]
pub struct nsIDOMNodeList {
    vtable: *const nsIDOMNodeListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMNodeList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x450cf0ba, 0xde90, 0x4f86,
            [0x85, 0xbf, 0xe1, 0x0c, 0xc8, 0xb8, 0x71, 0x3f])
    }
}

unsafe impl RefCounted for nsIDOMNodeList {
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
pub trait nsIDOMNodeListCoerce {
    fn coerce_from(v: &nsIDOMNodeList) -> &Self;
}

impl nsIDOMNodeListCoerce for nsIDOMNodeList {
    #[inline]
    fn coerce_from(v: &nsIDOMNodeList) -> &Self {
        v
    }
}

impl nsIDOMNodeList {
    #[inline]
    pub fn coerce<T: nsIDOMNodeListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMNodeList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMNodeListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMNodeList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMNodeListVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMNode item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMNodeList, index: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMNodeList, aLength: *mut libc::uint32_t) -> nsresult,

}


impl nsIDOMNodeList {
    /* nsIDOMNode item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
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


