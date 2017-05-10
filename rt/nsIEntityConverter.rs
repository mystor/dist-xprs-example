//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEntityConverter.idl
//


pub mod nsIEntityConverter_consts {
    pub const entityNone: i64 = 0;
    pub const html40Latin1: i64 = 1;
    pub const html40Symbols: i64 = 2;
    pub const html40Special: i64 = 4;
    pub const transliterate: i64 = 8;
    pub const mathml20: i64 = 16;
    pub const html32: i64 = 1;
    pub const html40: i64 = 7;
    pub const entityW3C: i64 = 23;
}


#[repr(C)]
pub struct nsIEntityConverter {
    vtable: *const nsIEntityConverterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEntityConverter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd14c7111, 0x55e0, 0x11d3,
            [0x91, 0xd9, 0x00, 0x10, 0x5a, 0xa3, 0xf7, 0xdc])
    }
}

unsafe impl RefCounted for nsIEntityConverter {
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
pub trait nsIEntityConverterCoerce {
    fn coerce_from(v: &nsIEntityConverter) -> &Self;
}

impl nsIEntityConverterCoerce for nsIEntityConverter {
    #[inline]
    fn coerce_from(v: &nsIEntityConverter) -> &Self {
        v
    }
}

impl nsIEntityConverter {
    #[inline]
    pub fn coerce<T: nsIEntityConverterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEntityConverter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEntityConverterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEntityConverter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEntityConverterVTable {
    pub __base: nsISupportsVTable,

    /* string ConvertUTF32ToEntity (in unsigned long character, in unsigned long entityVersion); */
    pub ConvertUTF32ToEntity: unsafe extern "C" fn (this: *const nsIEntityConverter, character: libc::uint32_t, entityVersion: libc::uint32_t, _retval: *mut *const libc::c_char) -> nsresult,

    /* string ConvertToEntity (in wchar character, in unsigned long entityVersion); */
    pub ConvertToEntity: unsafe extern "C" fn (this: *const nsIEntityConverter, character: libc::int16_t, entityVersion: libc::uint32_t, _retval: *mut *const libc::c_char) -> nsresult,

    /* wstring ConvertToEntities (in wstring inString, in unsigned long entityVersion); */
    pub ConvertToEntities: unsafe extern "C" fn (this: *const nsIEntityConverter, inString: *const libc::int16_t, entityVersion: libc::uint32_t, _retval: *mut *const libc::int16_t) -> nsresult,

}


impl nsIEntityConverter {
    /* string ConvertUTF32ToEntity (in unsigned long character, in unsigned long entityVersion); */
    #[inline]
    pub unsafe fn ConvertUTF32ToEntity(&self, character: libc::uint32_t, entityVersion: libc::uint32_t) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).ConvertUTF32ToEntity)(self as *const _, character, entityVersion, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string ConvertToEntity (in wchar character, in unsigned long entityVersion); */
    #[inline]
    pub unsafe fn ConvertToEntity(&self, character: libc::int16_t, entityVersion: libc::uint32_t) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).ConvertToEntity)(self as *const _, character, entityVersion, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring ConvertToEntities (in wstring inString, in unsigned long entityVersion); */
    #[inline]
    pub unsafe fn ConvertToEntities(&self, inString: *const libc::int16_t, entityVersion: libc::uint32_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).ConvertToEntities)(self as *const _, inString, entityVersion, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


