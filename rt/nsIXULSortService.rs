//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULSortService.idl
//


pub mod nsIXULSortService_consts {
    pub const SORT_COMPARECASE: i64 = 1;
    pub const SORT_INTEGER: i64 = 256;
}


#[repr(C)]
pub struct nsIXULSortService {
    vtable: *const nsIXULSortServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULSortService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf29270c8, 0x3be5, 0x4046,
            [0x9b, 0x57, 0x94, 0x5a, 0x84, 0xdf, 0xf1, 0x32])
    }
}

unsafe impl RefCounted for nsIXULSortService {
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
pub trait nsIXULSortServiceCoerce {
    fn coerce_from(v: &nsIXULSortService) -> &Self;
}

impl nsIXULSortServiceCoerce for nsIXULSortService {
    #[inline]
    fn coerce_from(v: &nsIXULSortService) -> &Self {
        v
    }
}

impl nsIXULSortService {
    #[inline]
    pub fn coerce<T: nsIXULSortServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULSortService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULSortServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULSortService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULSortServiceVTable {
    pub __base: nsISupportsVTable,

    /* void sort (in nsIDOMNode aNode, in AString aSortKey, in AString aSortHints); */
    pub sort: unsafe extern "C" fn (this: *const nsIXULSortService, aNode: *const nsIDOMNode, aSortKey: *const nsAString, aSortHints: *const nsAString) -> nsresult,

}


impl nsIXULSortService {
    /* void sort (in nsIDOMNode aNode, in AString aSortKey, in AString aSortHints); */
    #[inline]
    pub unsafe fn sort(&self, aNode: Option<&nsIDOMNode>, aSortKey: &[u16], aSortHints: &[u16]) -> Result<(), nsresult> {
        let aSortKey = nsString::from(aSortKey);
        let aSortHints = nsString::from(aSortHints);
        match ((*self.vtable).sort)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &*aSortKey, &*aSortHints) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


