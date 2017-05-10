//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboard.idl
//


pub mod nsIClipboard_consts {
    pub const kSelectionClipboard: i64 = 0;
    pub const kGlobalClipboard: i64 = 1;
    pub const kFindClipboard: i64 = 2;
    pub const kSelectionCache: i64 = 3;
}


#[repr(C)]
pub struct nsIClipboard {
    vtable: *const nsIClipboardVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClipboard {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xceaa0047, 0x647f, 0x4b8e,
            [0xad, 0x1c, 0xaf, 0xf9, 0xfa, 0x62, 0xaa, 0x51])
    }
}

unsafe impl RefCounted for nsIClipboard {
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
pub trait nsIClipboardCoerce {
    fn coerce_from(v: &nsIClipboard) -> &Self;
}

impl nsIClipboardCoerce for nsIClipboard {
    #[inline]
    fn coerce_from(v: &nsIClipboard) -> &Self {
        v
    }
}

impl nsIClipboard {
    #[inline]
    pub fn coerce<T: nsIClipboardCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClipboard {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClipboardCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClipboard) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClipboardVTable {
    pub __base: nsISupportsVTable,

    /* void setData (in nsITransferable aTransferable, in nsIClipboardOwner anOwner, in long aWhichClipboard); */
    pub setData: unsafe extern "C" fn (this: *const nsIClipboard, aTransferable: *const nsITransferable, anOwner: *const nsIClipboardOwner, aWhichClipboard: libc::int32_t) -> nsresult,

    /* void getData (in nsITransferable aTransferable, in long aWhichClipboard); */
    pub getData: unsafe extern "C" fn (this: *const nsIClipboard, aTransferable: *const nsITransferable, aWhichClipboard: libc::int32_t) -> nsresult,

    /* void emptyClipboard (in long aWhichClipboard); */
    pub emptyClipboard: unsafe extern "C" fn (this: *const nsIClipboard, aWhichClipboard: libc::int32_t) -> nsresult,

    /* boolean hasDataMatchingFlavors ([array, size_is (aLength)] in string aFlavorList, in unsigned long aLength, in long aWhichClipboard); */
    /// Unable to call function as its signature contains a non-rust type
    pub hasDataMatchingFlavors: *const ::libc::c_void,

    /* boolean supportsSelectionClipboard (); */
    pub supportsSelectionClipboard: unsafe extern "C" fn (this: *const nsIClipboard, _retval: *mut bool) -> nsresult,

    /* boolean supportsFindClipboard (); */
    pub supportsFindClipboard: unsafe extern "C" fn (this: *const nsIClipboard, _retval: *mut bool) -> nsresult,

}


impl nsIClipboard {
    /* void setData (in nsITransferable aTransferable, in nsIClipboardOwner anOwner, in long aWhichClipboard); */
    #[inline]
    pub unsafe fn setData(&self, aTransferable: Option<&nsITransferable>, anOwner: Option<&nsIClipboardOwner>, aWhichClipboard: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setData)(self as *const _, aTransferable.map_or(::std::ptr::null(), |x| x as *const _), anOwner.map_or(::std::ptr::null(), |x| x as *const _), aWhichClipboard) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getData (in nsITransferable aTransferable, in long aWhichClipboard); */
    #[inline]
    pub unsafe fn getData(&self, aTransferable: Option<&nsITransferable>, aWhichClipboard: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).getData)(self as *const _, aTransferable.map_or(::std::ptr::null(), |x| x as *const _), aWhichClipboard) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void emptyClipboard (in long aWhichClipboard); */
    #[inline]
    pub unsafe fn emptyClipboard(&self, aWhichClipboard: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).emptyClipboard)(self as *const _, aWhichClipboard) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasDataMatchingFlavors ([array, size_is (aLength)] in string aFlavorList, in unsigned long aLength, in long aWhichClipboard); */


    /* boolean supportsSelectionClipboard (); */
    #[inline]
    pub unsafe fn supportsSelectionClipboard(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).supportsSelectionClipboard)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean supportsFindClipboard (); */
    #[inline]
    pub unsafe fn supportsFindClipboard(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).supportsFindClipboard)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


