//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleImage.idl
//


#[repr(C)]
pub struct nsIAccessibleImage {
    vtable: *const nsIAccessibleImageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleImage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x09086623, 0x0f09, 0x4310,
            [0xac, 0x56, 0xc2, 0xcd, 0xa7, 0xc2, 0x96, 0x48])
    }
}

unsafe impl RefCounted for nsIAccessibleImage {
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
pub trait nsIAccessibleImageCoerce {
    fn coerce_from(v: &nsIAccessibleImage) -> &Self;
}

impl nsIAccessibleImageCoerce for nsIAccessibleImage {
    #[inline]
    fn coerce_from(v: &nsIAccessibleImage) -> &Self {
        v
    }
}

impl nsIAccessibleImage {
    #[inline]
    pub fn coerce<T: nsIAccessibleImageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleImage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleImageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleImage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleImageVTable {
    pub __base: nsISupportsVTable,

    /* void getImagePosition (in unsigned long coordType, out long x, out long y); */
    pub getImagePosition: unsafe extern "C" fn (this: *const nsIAccessibleImage, coordType: libc::uint32_t, x: *mut libc::int32_t, y: *mut libc::int32_t) -> nsresult,

    /* void getImageSize (out long width, out long height); */
    pub getImageSize: unsafe extern "C" fn (this: *const nsIAccessibleImage, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

}


impl nsIAccessibleImage {
    /* void getImagePosition (in unsigned long coordType, out long x, out long y); */
    #[inline]
    pub unsafe fn getImagePosition(&self, coordType: libc::uint32_t) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getImagePosition)(self as *const _, coordType, &mut x as *mut _, &mut y as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y))
    }

    /* void getImageSize (out long width, out long height); */
    #[inline]
    pub unsafe fn getImageSize(&self, ) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getImageSize)(self as *const _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((width, height))
    }

}


