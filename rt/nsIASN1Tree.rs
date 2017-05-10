//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIASN1Tree.idl
//


#[repr(C)]
pub struct nsIASN1Tree {
    vtable: *const nsIASN1TreeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIASN1Tree {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xde142307, 0x7b88, 0x4e0a,
            [0xb2, 0x32, 0x25, 0x0f, 0x31, 0x0e, 0x25, 0xd8])
    }
}

unsafe impl RefCounted for nsIASN1Tree {
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
pub trait nsIASN1TreeCoerce {
    fn coerce_from(v: &nsIASN1Tree) -> &Self;
}

impl nsIASN1TreeCoerce for nsIASN1Tree {
    #[inline]
    fn coerce_from(v: &nsIASN1Tree) -> &Self {
        v
    }
}

impl nsIASN1Tree {
    #[inline]
    pub fn coerce<T: nsIASN1TreeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIASN1Tree {
    type Target = nsITreeView;
    #[inline]
    fn deref(&self) -> &nsITreeView {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsITreeViewCoerce> nsIASN1TreeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIASN1Tree) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIASN1TreeVTable {
    pub __base: nsITreeViewVTable,

    /* void loadASN1Structure (in nsIASN1Object asn1Object); */
    pub loadASN1Structure: unsafe extern "C" fn (this: *const nsIASN1Tree, asn1Object: *const nsIASN1Object) -> nsresult,

    /* AString getDisplayData (in unsigned long index); */
    pub getDisplayData: unsafe extern "C" fn (this: *const nsIASN1Tree, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

}


impl nsIASN1Tree {
    /* void loadASN1Structure (in nsIASN1Object asn1Object); */
    #[inline]
    pub unsafe fn loadASN1Structure(&self, asn1Object: Option<&nsIASN1Object>) -> Result<(), nsresult> {

        match ((*self.vtable).loadASN1Structure)(self as *const _, asn1Object.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getDisplayData (in unsigned long index); */
    #[inline]
    pub unsafe fn getDisplayData(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getDisplayData)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


