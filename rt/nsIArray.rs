//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIArray.idl
//


#[repr(C)]
pub struct nsIArray {
    vtable: *const nsIArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIArray {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x114744d9, 0xc369, 0x456e,
            [0xb5, 0x5a, 0x52, 0xfe, 0x52, 0x88, 0x0d, 0x2d])
    }
}

unsafe impl RefCounted for nsIArray {
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
pub trait nsIArrayCoerce {
    fn coerce_from(v: &nsIArray) -> &Self;
}

impl nsIArrayCoerce for nsIArray {
    #[inline]
    fn coerce_from(v: &nsIArray) -> &Self {
        v
    }
}

impl nsIArray {
    #[inline]
    pub fn coerce<T: nsIArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIArray {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIArray) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIArrayVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIArray, aLength: *mut libc::uint32_t) -> nsresult,

    /* void queryElementAt (in unsigned long index, in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub queryElementAt: unsafe extern "C" fn (this: *const nsIArray, index: libc::uint32_t, uuid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* unsigned long indexOf (in unsigned long startIndex, in nsISupports element); */
    pub indexOf: unsafe extern "C" fn (this: *const nsIArray, startIndex: libc::uint32_t, element: *const nsISupports, _retval: *mut libc::uint32_t) -> nsresult,

    /* nsISimpleEnumerator enumerate (); */
    pub enumerate: unsafe extern "C" fn (this: *const nsIArray, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIArray {
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

    /* void queryElementAt (in unsigned long index, in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn queryElementAt<T: XpCom>(&self, index: libc::uint32_t) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).queryElementAt)(self as *const _, index, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* unsigned long indexOf (in unsigned long startIndex, in nsISupports element); */
    #[inline]
    pub unsafe fn indexOf(&self, startIndex: libc::uint32_t, element: Option<&nsISupports>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).indexOf)(self as *const _, startIndex, element.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISimpleEnumerator enumerate (); */
    #[inline]
    pub unsafe fn enumerate(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerate)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


