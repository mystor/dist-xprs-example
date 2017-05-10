//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefLocalizedString.idl
//


#[repr(C)]
pub struct nsIPrefLocalizedString {
    vtable: *const nsIPrefLocalizedStringVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrefLocalizedString {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xae419e24, 0x1dd1, 0x11b2,
            [0xb3, 0x9a, 0xd3, 0xe5, 0xe7, 0x07, 0x38, 0x02])
    }
}

unsafe impl RefCounted for nsIPrefLocalizedString {
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
pub trait nsIPrefLocalizedStringCoerce {
    fn coerce_from(v: &nsIPrefLocalizedString) -> &Self;
}

impl nsIPrefLocalizedStringCoerce for nsIPrefLocalizedString {
    #[inline]
    fn coerce_from(v: &nsIPrefLocalizedString) -> &Self {
        v
    }
}

impl nsIPrefLocalizedString {
    #[inline]
    pub fn coerce<T: nsIPrefLocalizedStringCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrefLocalizedString {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrefLocalizedStringCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefLocalizedString) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrefLocalizedStringVTable {
    pub __base: nsISupportsVTable,

    /* attribute wstring data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIPrefLocalizedString, aData: *mut *const libc::int16_t) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsIPrefLocalizedString, aData: *const libc::int16_t) -> nsresult,

    /* wstring toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIPrefLocalizedString, _retval: *mut *const libc::int16_t) -> nsresult,

    /* void setDataWithLength (in unsigned long length, [size_is (length)] in wstring data); */
    pub setDataWithLength: unsafe extern "C" fn (this: *const nsIPrefLocalizedString, length: libc::uint32_t, data: *const libc::int16_t) -> nsresult,

}


impl nsIPrefLocalizedString {
    /* attribute wstring data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* wstring toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setDataWithLength (in unsigned long length, [size_is (length)] in wstring data); */
    #[inline]
    pub unsafe fn setDataWithLength(&self, length: libc::uint32_t, data: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDataWithLength)(self as *const _, length, data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


