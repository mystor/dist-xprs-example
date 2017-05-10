//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFLiteral.idl
//


#[repr(C)]
pub struct nsIRDFLiteral {
    vtable: *const nsIRDFLiteralVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFLiteral {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe0c493d2, 0x9542, 0x11d2,
            [0x8e, 0xb8, 0x00, 0x80, 0x5f, 0x29, 0xf3, 0x70])
    }
}

unsafe impl RefCounted for nsIRDFLiteral {
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
pub trait nsIRDFLiteralCoerce {
    fn coerce_from(v: &nsIRDFLiteral) -> &Self;
}

impl nsIRDFLiteralCoerce for nsIRDFLiteral {
    #[inline]
    fn coerce_from(v: &nsIRDFLiteral) -> &Self {
        v
    }
}

impl nsIRDFLiteral {
    #[inline]
    pub fn coerce<T: nsIRDFLiteralCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFLiteral {
    type Target = nsIRDFNode;
    #[inline]
    fn deref(&self) -> &nsIRDFNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRDFNodeCoerce> nsIRDFLiteralCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFLiteral) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFLiteralVTable {
    pub __base: nsIRDFNodeVTable,

    /* readonly attribute wstring Value; */
    pub get_Value: unsafe extern "C" fn (this: *const nsIRDFLiteral, aValue: *mut *const libc::int16_t) -> nsresult,

    /* [noscript] void GetValueConst ([shared] out wstring aConstValue); */
    pub GetValueConst: unsafe extern "C" fn (this: *const nsIRDFLiteral, aConstValue: *mut *const libc::int16_t) -> nsresult,

}


impl nsIRDFLiteral {
    /* readonly attribute wstring Value; */
    #[inline]
    pub unsafe fn get_Value(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_Value)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void GetValueConst ([shared] out wstring aConstValue); */
    #[inline]
    pub unsafe fn GetValueConst(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut aConstValue: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).GetValueConst)(self as *const _, &mut aConstValue as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aConstValue)
    }

}


#[repr(C)]
pub struct nsIRDFDate {
    vtable: *const nsIRDFDateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFDate {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe13a24e1, 0xc77a, 0x11d2,
            [0x80, 0xbe, 0x00, 0x60, 0x97, 0xb7, 0x6b, 0x8e])
    }
}

unsafe impl RefCounted for nsIRDFDate {
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
pub trait nsIRDFDateCoerce {
    fn coerce_from(v: &nsIRDFDate) -> &Self;
}

impl nsIRDFDateCoerce for nsIRDFDate {
    #[inline]
    fn coerce_from(v: &nsIRDFDate) -> &Self {
        v
    }
}

impl nsIRDFDate {
    #[inline]
    pub fn coerce<T: nsIRDFDateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFDate {
    type Target = nsIRDFNode;
    #[inline]
    fn deref(&self) -> &nsIRDFNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRDFNodeCoerce> nsIRDFDateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFDate) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFDateVTable {
    pub __base: nsIRDFNodeVTable,

    /* readonly attribute PRTime Value; */
    pub get_Value: unsafe extern "C" fn (this: *const nsIRDFDate, aValue: *mut PRTime) -> nsresult,

}


impl nsIRDFDate {
    /* readonly attribute PRTime Value; */
    #[inline]
    pub unsafe fn get_Value(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_Value)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIRDFInt {
    vtable: *const nsIRDFIntVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFInt {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe13a24e3, 0xc77a, 0x11d2,
            [0x80, 0xbe, 0x00, 0x60, 0x97, 0xb7, 0x6b, 0x8e])
    }
}

unsafe impl RefCounted for nsIRDFInt {
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
pub trait nsIRDFIntCoerce {
    fn coerce_from(v: &nsIRDFInt) -> &Self;
}

impl nsIRDFIntCoerce for nsIRDFInt {
    #[inline]
    fn coerce_from(v: &nsIRDFInt) -> &Self {
        v
    }
}

impl nsIRDFInt {
    #[inline]
    pub fn coerce<T: nsIRDFIntCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFInt {
    type Target = nsIRDFNode;
    #[inline]
    fn deref(&self) -> &nsIRDFNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRDFNodeCoerce> nsIRDFIntCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFInt) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFIntVTable {
    pub __base: nsIRDFNodeVTable,

    /* readonly attribute long Value; */
    pub get_Value: unsafe extern "C" fn (this: *const nsIRDFInt, aValue: *mut libc::int32_t) -> nsresult,

}


impl nsIRDFInt {
    /* readonly attribute long Value; */
    #[inline]
    pub unsafe fn get_Value(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_Value)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIRDFBlob {
    vtable: *const nsIRDFBlobVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFBlob {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x237f85a2, 0x1dd2, 0x11b2,
            [0x94, 0xaf, 0x81, 0x22, 0x58, 0x2f, 0xc4, 0x5e])
    }
}

unsafe impl RefCounted for nsIRDFBlob {
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
pub trait nsIRDFBlobCoerce {
    fn coerce_from(v: &nsIRDFBlob) -> &Self;
}

impl nsIRDFBlobCoerce for nsIRDFBlob {
    #[inline]
    fn coerce_from(v: &nsIRDFBlob) -> &Self {
        v
    }
}

impl nsIRDFBlob {
    #[inline]
    pub fn coerce<T: nsIRDFBlobCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFBlob {
    type Target = nsIRDFNode;
    #[inline]
    fn deref(&self) -> &nsIRDFNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRDFNodeCoerce> nsIRDFBlobCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFBlob) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFBlobVTable {
    pub __base: nsIRDFNodeVTable,

    /* [noscript] readonly attribute const_octet_ptr value; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_value: *const ::libc::c_void,

    /* readonly attribute long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIRDFBlob, aLength: *mut libc::int32_t) -> nsresult,

}


impl nsIRDFBlob {
    /* [noscript] readonly attribute const_octet_ptr value; */


    /* readonly attribute long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


