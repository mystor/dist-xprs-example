//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNodeFilter.idl
//


pub mod nsIDOMNodeFilter_consts {
    pub const FILTER_ACCEPT: i64 = 1;
    pub const FILTER_REJECT: i64 = 2;
    pub const FILTER_SKIP: i64 = 3;
    pub const SHOW_ALL: i64 = 4294967295;
    pub const SHOW_ELEMENT: i64 = 1;
    pub const SHOW_ATTRIBUTE: i64 = 2;
    pub const SHOW_TEXT: i64 = 4;
    pub const SHOW_CDATA_SECTION: i64 = 8;
    pub const SHOW_ENTITY_REFERENCE: i64 = 16;
    pub const SHOW_ENTITY: i64 = 32;
    pub const SHOW_PROCESSING_INSTRUCTION: i64 = 64;
    pub const SHOW_COMMENT: i64 = 128;
    pub const SHOW_DOCUMENT: i64 = 256;
    pub const SHOW_DOCUMENT_TYPE: i64 = 512;
    pub const SHOW_DOCUMENT_FRAGMENT: i64 = 1024;
    pub const SHOW_NOTATION: i64 = 2048;
}


#[repr(C)]
pub struct nsIDOMNodeFilter {
    vtable: *const nsIDOMNodeFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMNodeFilter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe4723748, 0x1dd1, 0x11b2,
            [0x8e, 0xe6, 0x86, 0x6a, 0x53, 0x2a, 0x62, 0x37])
    }
}

unsafe impl RefCounted for nsIDOMNodeFilter {
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
pub trait nsIDOMNodeFilterCoerce {
    fn coerce_from(v: &nsIDOMNodeFilter) -> &Self;
}

impl nsIDOMNodeFilterCoerce for nsIDOMNodeFilter {
    #[inline]
    fn coerce_from(v: &nsIDOMNodeFilter) -> &Self {
        v
    }
}

impl nsIDOMNodeFilter {
    #[inline]
    pub fn coerce<T: nsIDOMNodeFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMNodeFilter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMNodeFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMNodeFilter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMNodeFilterVTable {
    pub __base: nsISupportsVTable,

    /* short acceptNode (in nsIDOMNode n); */
    pub acceptNode: unsafe extern "C" fn (this: *const nsIDOMNodeFilter, n: *const nsIDOMNode, _retval: *mut libc::int16_t) -> nsresult,

}


impl nsIDOMNodeFilter {
    /* short acceptNode (in nsIDOMNode n); */
    #[inline]
    pub unsafe fn acceptNode(&self, n: Option<&nsIDOMNode>) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).acceptNode)(self as *const _, n.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


