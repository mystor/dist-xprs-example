//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDirIndex.idl
//


pub mod nsIDirIndex_consts {
    pub const TYPE_UNKNOWN: i64 = 0;
    pub const TYPE_DIRECTORY: i64 = 1;
    pub const TYPE_FILE: i64 = 2;
    pub const TYPE_SYMLINK: i64 = 3;
}


#[repr(C)]
pub struct nsIDirIndex {
    vtable: *const nsIDirIndexVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDirIndex {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x23bbabd0, 0x1dd2, 0x11b2,
            [0x86, 0xb7, 0xaa, 0xd6, 0x8a, 0xe7, 0xd7, 0xe0])
    }
}

unsafe impl RefCounted for nsIDirIndex {
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
pub trait nsIDirIndexCoerce {
    fn coerce_from(v: &nsIDirIndex) -> &Self;
}

impl nsIDirIndexCoerce for nsIDirIndex {
    #[inline]
    fn coerce_from(v: &nsIDirIndex) -> &Self {
        v
    }
}

impl nsIDirIndex {
    #[inline]
    pub fn coerce<T: nsIDirIndexCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDirIndex {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDirIndexCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirIndex) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDirIndexVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned long type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDirIndex, aType: *mut libc::uint32_t) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDirIndex, aType: libc::uint32_t) -> nsresult,

    /* attribute string contentType; */
    pub get_contentType: unsafe extern "C" fn (this: *const nsIDirIndex, aContentType: *mut *const libc::c_char) -> nsresult,
    pub set_contentType: unsafe extern "C" fn (this: *const nsIDirIndex, aContentType: *const libc::c_char) -> nsresult,

    /* attribute string location; */
    pub get_location: unsafe extern "C" fn (this: *const nsIDirIndex, aLocation: *mut *const libc::c_char) -> nsresult,
    pub set_location: unsafe extern "C" fn (this: *const nsIDirIndex, aLocation: *const libc::c_char) -> nsresult,

    /* attribute wstring description; */
    pub get_description: unsafe extern "C" fn (this: *const nsIDirIndex, aDescription: *mut *const libc::int16_t) -> nsresult,
    pub set_description: unsafe extern "C" fn (this: *const nsIDirIndex, aDescription: *const libc::int16_t) -> nsresult,

    /* attribute long long size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIDirIndex, aSize: *mut libc::int64_t) -> nsresult,
    pub set_size: unsafe extern "C" fn (this: *const nsIDirIndex, aSize: libc::int64_t) -> nsresult,

    /* attribute PRTime lastModified; */
    pub get_lastModified: unsafe extern "C" fn (this: *const nsIDirIndex, aLastModified: *mut PRTime) -> nsresult,
    pub set_lastModified: unsafe extern "C" fn (this: *const nsIDirIndex, aLastModified: PRTime) -> nsresult,

}


impl nsIDirIndex {
    /* attribute unsigned long type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_type_)(self as *const _, aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute string contentType; */
    #[inline]
    pub unsafe fn get_contentType(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_contentType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentType(&self, aContentType: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_contentType)(self as *const _, aContentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute string location; */
    #[inline]
    pub unsafe fn get_location(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_location)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_location(&self, aLocation: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_location)(self as *const _, aLocation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring description; */
    #[inline]
    pub unsafe fn get_description(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_description)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_description(&self, aDescription: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_description)(self as *const _, aDescription) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long long size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_size)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_size(&self, aSize: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_size)(self as *const _, aSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute PRTime lastModified; */
    #[inline]
    pub unsafe fn get_lastModified(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_lastModified(&self, aLastModified: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).set_lastModified)(self as *const _, aLastModified) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


