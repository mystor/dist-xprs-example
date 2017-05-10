//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCharacterData.idl
//


#[repr(C)]
pub struct nsIDOMCharacterData {
    vtable: *const nsIDOMCharacterDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCharacterData {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4109a2d2, 0xe7af, 0x445d,
            [0xbb, 0x72, 0xc7, 0xc9, 0xb8, 0x75, 0xf3, 0x5e])
    }
}

unsafe impl RefCounted for nsIDOMCharacterData {
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
pub trait nsIDOMCharacterDataCoerce {
    fn coerce_from(v: &nsIDOMCharacterData) -> &Self;
}

impl nsIDOMCharacterDataCoerce for nsIDOMCharacterData {
    #[inline]
    fn coerce_from(v: &nsIDOMCharacterData) -> &Self {
        v
    }
}

impl nsIDOMCharacterData {
    #[inline]
    pub fn coerce<T: nsIDOMCharacterDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCharacterData {
    type Target = nsIDOMNode;
    #[inline]
    fn deref(&self) -> &nsIDOMNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMNodeCoerce> nsIDOMCharacterDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCharacterData) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCharacterDataVTable {
    pub __base: nsIDOMNodeVTable,

    /* attribute DOMString data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIDOMCharacterData, aData: *mut nsAString) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsIDOMCharacterData, aData: *const nsAString) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMCharacterData, aLength: *mut libc::uint32_t) -> nsresult,

    /* DOMString substringData (in unsigned long offset, in unsigned long count) raises (DOMException); */
    pub substringData: unsafe extern "C" fn (this: *const nsIDOMCharacterData, offset: libc::uint32_t, count: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* void appendData (in DOMString arg) raises (DOMException); */
    pub appendData: unsafe extern "C" fn (this: *const nsIDOMCharacterData, arg: *const nsAString) -> nsresult,

    /* void insertData (in unsigned long offset, in DOMString arg) raises (DOMException); */
    pub insertData: unsafe extern "C" fn (this: *const nsIDOMCharacterData, offset: libc::uint32_t, arg: *const nsAString) -> nsresult,

    /* void deleteData (in unsigned long offset, in unsigned long count) raises (DOMException); */
    pub deleteData: unsafe extern "C" fn (this: *const nsIDOMCharacterData, offset: libc::uint32_t, count: libc::uint32_t) -> nsresult,

    /* void replaceData (in unsigned long offset, in unsigned long count, in DOMString arg) raises (DOMException); */
    pub replaceData: unsafe extern "C" fn (this: *const nsIDOMCharacterData, offset: libc::uint32_t, count: libc::uint32_t, arg: *const nsAString) -> nsresult,

}


impl nsIDOMCharacterData {
    /* attribute DOMString data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_data)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: &[u16]) -> Result<(), nsresult> {
        let aData = nsString::from(aData);
        match ((*self.vtable).set_data)(self as *const _, &*aData) {
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

    /* DOMString substringData (in unsigned long offset, in unsigned long count) raises (DOMException); */
    #[inline]
    pub unsafe fn substringData(&self, offset: libc::uint32_t, count: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).substringData)(self as *const _, offset, count, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void appendData (in DOMString arg) raises (DOMException); */
    #[inline]
    pub unsafe fn appendData(&self, arg: &[u16]) -> Result<(), nsresult> {
        let arg = nsString::from(arg);
        match ((*self.vtable).appendData)(self as *const _, &*arg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertData (in unsigned long offset, in DOMString arg) raises (DOMException); */
    #[inline]
    pub unsafe fn insertData(&self, offset: libc::uint32_t, arg: &[u16]) -> Result<(), nsresult> {
        let arg = nsString::from(arg);
        match ((*self.vtable).insertData)(self as *const _, offset, &*arg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteData (in unsigned long offset, in unsigned long count) raises (DOMException); */
    #[inline]
    pub unsafe fn deleteData(&self, offset: libc::uint32_t, count: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).deleteData)(self as *const _, offset, count) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void replaceData (in unsigned long offset, in unsigned long count, in DOMString arg) raises (DOMException); */
    #[inline]
    pub unsafe fn replaceData(&self, offset: libc::uint32_t, count: libc::uint32_t, arg: &[u16]) -> Result<(), nsresult> {
        let arg = nsString::from(arg);
        match ((*self.vtable).replaceData)(self as *const _, offset, count, &*arg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


