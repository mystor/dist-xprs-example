//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMediaList.idl
//


#[repr(C)]
pub struct nsIDOMMediaList {
    vtable: *const nsIDOMMediaListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMMediaList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9b0c2ed7, 0x111c, 0x4824,
            [0xad, 0xf9, 0xef, 0x0d, 0xa6, 0xda, 0xd3, 0x71])
    }
}

unsafe impl RefCounted for nsIDOMMediaList {
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
pub trait nsIDOMMediaListCoerce {
    fn coerce_from(v: &nsIDOMMediaList) -> &Self;
}

impl nsIDOMMediaListCoerce for nsIDOMMediaList {
    #[inline]
    fn coerce_from(v: &nsIDOMMediaList) -> &Self {
        v
    }
}

impl nsIDOMMediaList {
    #[inline]
    pub fn coerce<T: nsIDOMMediaListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMMediaList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMMediaListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMediaList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMMediaListVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString mediaText; */
    pub get_mediaText: unsafe extern "C" fn (this: *const nsIDOMMediaList, aMediaText: *mut nsAString) -> nsresult,
    pub set_mediaText: unsafe extern "C" fn (this: *const nsIDOMMediaList, aMediaText: *const nsAString) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMMediaList, aLength: *mut libc::uint32_t) -> nsresult,

    /* DOMString item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMMediaList, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* void deleteMedium (in DOMString oldMedium) raises (DOMException); */
    pub deleteMedium: unsafe extern "C" fn (this: *const nsIDOMMediaList, oldMedium: *const nsAString) -> nsresult,

    /* void appendMedium (in DOMString newMedium) raises (DOMException); */
    pub appendMedium: unsafe extern "C" fn (this: *const nsIDOMMediaList, newMedium: *const nsAString) -> nsresult,

}


impl nsIDOMMediaList {
    /* attribute DOMString mediaText; */
    #[inline]
    pub unsafe fn get_mediaText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_mediaText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mediaText(&self, aMediaText: &[u16]) -> Result<(), nsresult> {
        let aMediaText = nsString::from(aMediaText);
        match ((*self.vtable).set_mediaText)(self as *const _, &*aMediaText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* DOMString item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).item)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void deleteMedium (in DOMString oldMedium) raises (DOMException); */
    #[inline]
    pub unsafe fn deleteMedium(&self, oldMedium: &[u16]) -> Result<(), nsresult> {
        let oldMedium = nsString::from(oldMedium);
        match ((*self.vtable).deleteMedium)(self as *const _, &*oldMedium) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendMedium (in DOMString newMedium) raises (DOMException); */
    #[inline]
    pub unsafe fn appendMedium(&self, newMedium: &[u16]) -> Result<(), nsresult> {
        let newMedium = nsString::from(newMedium);
        match ((*self.vtable).appendMedium)(self as *const _, &*newMedium) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


