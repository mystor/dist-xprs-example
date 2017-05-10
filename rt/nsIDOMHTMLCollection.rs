//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLCollection.idl
//


#[repr(C)]
pub struct nsIDOMHTMLCollection {
    vtable: *const nsIDOMHTMLCollectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLCollection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbb07f567, 0x5b37, 0x4172,
            [0x92, 0xaa, 0x7d, 0x00, 0xce, 0xed, 0x48, 0x09])
    }
}

unsafe impl RefCounted for nsIDOMHTMLCollection {
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
pub trait nsIDOMHTMLCollectionCoerce {
    fn coerce_from(v: &nsIDOMHTMLCollection) -> &Self;
}

impl nsIDOMHTMLCollectionCoerce for nsIDOMHTMLCollection {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLCollection) -> &Self {
        v
    }
}

impl nsIDOMHTMLCollection {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLCollectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLCollection {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLCollectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLCollection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLCollectionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMHTMLCollection, aLength: *mut libc::uint32_t) -> nsresult,

    /* nsIDOMNode item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMHTMLCollection, index: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode namedItem (in DOMString name); */
    pub namedItem: unsafe extern "C" fn (this: *const nsIDOMHTMLCollection, name: *const nsAString, _retval: *mut *const nsIDOMNode) -> nsresult,

}


impl nsIDOMHTMLCollection {
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

    /* nsIDOMNode namedItem (in DOMString name); */
    #[inline]
    pub unsafe fn namedItem(&self, name: &[u16]) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).namedItem)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


