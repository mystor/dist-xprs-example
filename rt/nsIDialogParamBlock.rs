//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDialogParamBlock.idl
//


#[repr(C)]
pub struct nsIDialogParamBlock {
    vtable: *const nsIDialogParamBlockVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDialogParamBlock {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf76c0901, 0x437a, 0x11d3,
            [0xb7, 0xa0, 0xe3, 0x5d, 0xb3, 0x51, 0xb4, 0xbc])
    }
}

unsafe impl RefCounted for nsIDialogParamBlock {
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
pub trait nsIDialogParamBlockCoerce {
    fn coerce_from(v: &nsIDialogParamBlock) -> &Self;
}

impl nsIDialogParamBlockCoerce for nsIDialogParamBlock {
    #[inline]
    fn coerce_from(v: &nsIDialogParamBlock) -> &Self {
        v
    }
}

impl nsIDialogParamBlock {
    #[inline]
    pub fn coerce<T: nsIDialogParamBlockCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDialogParamBlock {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDialogParamBlockCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDialogParamBlock) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDialogParamBlockVTable {
    pub __base: nsISupportsVTable,

    /* int32_t GetInt (in int32_t inIndex); */
    pub GetInt: unsafe extern "C" fn (this: *const nsIDialogParamBlock, inIndex: int32_t, _retval: *mut int32_t) -> nsresult,

    /* void SetInt (in int32_t inIndex, in int32_t inInt); */
    pub SetInt: unsafe extern "C" fn (this: *const nsIDialogParamBlock, inIndex: int32_t, inInt: int32_t) -> nsresult,

    /* void SetNumberStrings (in int32_t inNumStrings); */
    pub SetNumberStrings: unsafe extern "C" fn (this: *const nsIDialogParamBlock, inNumStrings: int32_t) -> nsresult,

    /* wstring GetString (in int32_t inIndex); */
    pub GetString: unsafe extern "C" fn (this: *const nsIDialogParamBlock, inIndex: int32_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* void SetString (in int32_t inIndex, in wstring inString); */
    pub SetString: unsafe extern "C" fn (this: *const nsIDialogParamBlock, inIndex: int32_t, inString: *const libc::int16_t) -> nsresult,

    /* attribute nsIMutableArray objects; */
    pub get_objects: unsafe extern "C" fn (this: *const nsIDialogParamBlock, aObjects: *mut *const nsIMutableArray) -> nsresult,
    pub set_objects: unsafe extern "C" fn (this: *const nsIDialogParamBlock, aObjects: *const nsIMutableArray) -> nsresult,

}


impl nsIDialogParamBlock {
    /* int32_t GetInt (in int32_t inIndex); */
    #[inline]
    pub unsafe fn GetInt(&self, inIndex: int32_t) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetInt)(self as *const _, inIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void SetInt (in int32_t inIndex, in int32_t inInt); */
    #[inline]
    pub unsafe fn SetInt(&self, inIndex: int32_t, inInt: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).SetInt)(self as *const _, inIndex, inInt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void SetNumberStrings (in int32_t inNumStrings); */
    #[inline]
    pub unsafe fn SetNumberStrings(&self, inNumStrings: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).SetNumberStrings)(self as *const _, inNumStrings) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* wstring GetString (in int32_t inIndex); */
    #[inline]
    pub unsafe fn GetString(&self, inIndex: int32_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).GetString)(self as *const _, inIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void SetString (in int32_t inIndex, in wstring inString); */
    #[inline]
    pub unsafe fn SetString(&self, inIndex: int32_t, inString: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).SetString)(self as *const _, inIndex, inString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIMutableArray objects; */
    #[inline]
    pub unsafe fn get_objects(&self, ) -> Result<Option<RefPtr<nsIMutableArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_objects)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_objects(&self, aObjects: Option<&nsIMutableArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_objects)(self as *const _, aObjects.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


