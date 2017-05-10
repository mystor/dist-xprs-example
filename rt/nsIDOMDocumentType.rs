//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDocumentType.idl
//


#[repr(C)]
pub struct nsIDOMDocumentType {
    vtable: *const nsIDOMDocumentTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDocumentType {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcd7467b9, 0x0f26, 0x4787,
            [0xa3, 0x59, 0x66, 0xe8, 0x0b, 0xa8, 0xdb, 0x92])
    }
}

unsafe impl RefCounted for nsIDOMDocumentType {
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
pub trait nsIDOMDocumentTypeCoerce {
    fn coerce_from(v: &nsIDOMDocumentType) -> &Self;
}

impl nsIDOMDocumentTypeCoerce for nsIDOMDocumentType {
    #[inline]
    fn coerce_from(v: &nsIDOMDocumentType) -> &Self {
        v
    }
}

impl nsIDOMDocumentType {
    #[inline]
    pub fn coerce<T: nsIDOMDocumentTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDocumentType {
    type Target = nsIDOMNode;
    #[inline]
    fn deref(&self) -> &nsIDOMNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMNodeCoerce> nsIDOMDocumentTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDocumentType) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDocumentTypeVTable {
    pub __base: nsIDOMNodeVTable,

    /* readonly attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMDocumentType, aName: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString publicId; */
    pub get_publicId: unsafe extern "C" fn (this: *const nsIDOMDocumentType, aPublicId: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString systemId; */
    pub get_systemId: unsafe extern "C" fn (this: *const nsIDOMDocumentType, aSystemId: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString internalSubset; */
    pub get_internalSubset: unsafe extern "C" fn (this: *const nsIDOMDocumentType, aInternalSubset: *mut nsAString) -> nsresult,

}


impl nsIDOMDocumentType {
    /* readonly attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString publicId; */
    #[inline]
    pub unsafe fn get_publicId(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_publicId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString systemId; */
    #[inline]
    pub unsafe fn get_systemId(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_systemId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString internalSubset; */
    #[inline]
    pub unsafe fn get_internalSubset(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_internalSubset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


