//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIASN1PrintableItem.idl
//


#[repr(C)]
pub struct nsIASN1PrintableItem {
    vtable: *const nsIASN1PrintableItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIASN1PrintableItem {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x114e1142, 0x1dd2, 0x11b2,
            [0xac, 0x26, 0xb6, 0xdb, 0x19, 0xd9, 0x18, 0x4a])
    }
}

unsafe impl RefCounted for nsIASN1PrintableItem {
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
pub trait nsIASN1PrintableItemCoerce {
    fn coerce_from(v: &nsIASN1PrintableItem) -> &Self;
}

impl nsIASN1PrintableItemCoerce for nsIASN1PrintableItem {
    #[inline]
    fn coerce_from(v: &nsIASN1PrintableItem) -> &Self {
        v
    }
}

impl nsIASN1PrintableItem {
    #[inline]
    pub fn coerce<T: nsIASN1PrintableItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIASN1PrintableItem {
    type Target = nsIASN1Object;
    #[inline]
    fn deref(&self) -> &nsIASN1Object {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIASN1ObjectCoerce> nsIASN1PrintableItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIASN1PrintableItem) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIASN1PrintableItemVTable {
    pub __base: nsIASN1ObjectVTable,

    /* [noscript] void setData (in charPtr data, in unsigned long len); */
    pub setData: unsafe extern "C" fn (this: *const nsIASN1PrintableItem, data: *const u8, len: libc::uint32_t) -> nsresult,

    /* [noscript] void getData (out charPtr data, out unsigned long len); */
    pub getData: unsafe extern "C" fn (this: *const nsIASN1PrintableItem, data: *mut *const u8, len: *mut libc::uint32_t) -> nsresult,

}


impl nsIASN1PrintableItem {
    /* [noscript] void setData (in charPtr data, in unsigned long len); */
    #[inline]
    pub unsafe fn setData(&self, data: *const u8, len: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setData)(self as *const _, data, len) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void getData (out charPtr data, out unsigned long len); */
    #[inline]
    pub unsafe fn getData(&self, ) -> Result<(*const u8, libc::uint32_t), nsresult> {
        let mut data: *const u8 = ::std::ptr::null();
        let mut len: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getData)(self as *const _, &mut data as *mut _, &mut len as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((data, len))
    }

}


