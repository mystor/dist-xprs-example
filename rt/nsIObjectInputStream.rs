//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIObjectInputStream.idl
//


#[repr(C)]
pub struct nsIObjectInputStream {
    vtable: *const nsIObjectInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIObjectInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6c248606, 0x4eae, 0x46fa,
            [0x9d, 0xf0, 0xba, 0x58, 0x50, 0x23, 0x68, 0xeb])
    }
}

unsafe impl RefCounted for nsIObjectInputStream {
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
pub trait nsIObjectInputStreamCoerce {
    fn coerce_from(v: &nsIObjectInputStream) -> &Self;
}

impl nsIObjectInputStreamCoerce for nsIObjectInputStream {
    #[inline]
    fn coerce_from(v: &nsIObjectInputStream) -> &Self {
        v
    }
}

impl nsIObjectInputStream {
    #[inline]
    pub fn coerce<T: nsIObjectInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIObjectInputStream {
    type Target = nsIBinaryInputStream;
    #[inline]
    fn deref(&self) -> &nsIBinaryInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIBinaryInputStreamCoerce> nsIObjectInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObjectInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIObjectInputStreamVTable {
    pub __base: nsIBinaryInputStreamVTable,

    /* nsISupports readObject (in boolean aIsStrongRef); */
    pub readObject: unsafe extern "C" fn (this: *const nsIObjectInputStream, aIsStrongRef: bool, _retval: *mut *const nsISupports) -> nsresult,

    /* [notxpcom] nsresult readID (out nsID aID); */
    /// Unable to call function as its signature contains a non-rust type
    pub readID: *const ::libc::c_void,

    /* [notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
    pub getBuffer: unsafe extern "C" fn (this: *const nsIObjectInputStream, aLength: uint32_t, aAlignMask: uint32_t) -> *const u8,

    /* [notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
    pub putBuffer: unsafe extern "C" fn (this: *const nsIObjectInputStream, aBuffer: *const u8, aLength: uint32_t) -> libc::c_void,

}


impl nsIObjectInputStream {
    /* nsISupports readObject (in boolean aIsStrongRef); */
    #[inline]
    pub unsafe fn readObject(&self, aIsStrongRef: bool) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).readObject)(self as *const _, aIsStrongRef, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [notxpcom] nsresult readID (out nsID aID); */


    /* [notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
    #[inline]
    pub unsafe fn getBuffer(&self, aLength: uint32_t, aAlignMask: uint32_t) -> *const u8 {

        let _retval = ((*self.vtable).getBuffer)(self as *const _, aLength, aAlignMask);
        _retval
    }

    /* [notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
    #[inline]
    pub unsafe fn putBuffer(&self, aBuffer: *const u8, aLength: uint32_t) -> () {

        let _retval = ((*self.vtable).putBuffer)(self as *const _, aBuffer, aLength);
        ()
    }

}


