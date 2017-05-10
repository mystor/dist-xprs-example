//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIObjectOutputStream.idl
//


#[repr(C)]
pub struct nsIObjectOutputStream {
    vtable: *const nsIObjectOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIObjectOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x92c898ac, 0x5fde, 0x4b99,
            [0x87, 0xb3, 0x5d, 0x48, 0x64, 0x22, 0x09, 0x4b])
    }
}

unsafe impl RefCounted for nsIObjectOutputStream {
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
pub trait nsIObjectOutputStreamCoerce {
    fn coerce_from(v: &nsIObjectOutputStream) -> &Self;
}

impl nsIObjectOutputStreamCoerce for nsIObjectOutputStream {
    #[inline]
    fn coerce_from(v: &nsIObjectOutputStream) -> &Self {
        v
    }
}

impl nsIObjectOutputStream {
    #[inline]
    pub fn coerce<T: nsIObjectOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIObjectOutputStream {
    type Target = nsIBinaryOutputStream;
    #[inline]
    fn deref(&self) -> &nsIBinaryOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIBinaryOutputStreamCoerce> nsIObjectOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObjectOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIObjectOutputStreamVTable {
    pub __base: nsIBinaryOutputStreamVTable,

    /* void writeObject (in nsISupports aObject, in boolean aIsStrongRef); */
    pub writeObject: unsafe extern "C" fn (this: *const nsIObjectOutputStream, aObject: *const nsISupports, aIsStrongRef: bool) -> nsresult,

    /* void writeSingleRefObject (in nsISupports aObject); */
    pub writeSingleRefObject: unsafe extern "C" fn (this: *const nsIObjectOutputStream, aObject: *const nsISupports) -> nsresult,

    /* void writeCompoundObject (in nsISupports aObject, in nsIIDRef aIID, in boolean aIsStrongRef); */
    pub writeCompoundObject: unsafe extern "C" fn (this: *const nsIObjectOutputStream, aObject: *const nsISupports, aIID: *const nsIID, aIsStrongRef: bool) -> nsresult,

    /* void writeID (in nsIDRef aID); */
    pub writeID: unsafe extern "C" fn (this: *const nsIObjectOutputStream, aID: *const nsID) -> nsresult,

    /* [notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
    pub getBuffer: unsafe extern "C" fn (this: *const nsIObjectOutputStream, aLength: uint32_t, aAlignMask: uint32_t) -> *const u8,

    /* [notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
    pub putBuffer: unsafe extern "C" fn (this: *const nsIObjectOutputStream, aBuffer: *const u8, aLength: uint32_t) -> libc::c_void,

}


impl nsIObjectOutputStream {
    /* void writeObject (in nsISupports aObject, in boolean aIsStrongRef); */
    #[inline]
    pub unsafe fn writeObject(&self, aObject: Option<&nsISupports>, aIsStrongRef: bool) -> Result<(), nsresult> {

        match ((*self.vtable).writeObject)(self as *const _, aObject.map_or(::std::ptr::null(), |x| x as *const _), aIsStrongRef) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeSingleRefObject (in nsISupports aObject); */
    #[inline]
    pub unsafe fn writeSingleRefObject(&self, aObject: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).writeSingleRefObject)(self as *const _, aObject.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeCompoundObject (in nsISupports aObject, in nsIIDRef aIID, in boolean aIsStrongRef); */
    #[inline]
    pub unsafe fn writeCompoundObject(&self, aObject: Option<&nsISupports>, aIID: *const nsIID, aIsStrongRef: bool) -> Result<(), nsresult> {

        match ((*self.vtable).writeCompoundObject)(self as *const _, aObject.map_or(::std::ptr::null(), |x| x as *const _), aIID, aIsStrongRef) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeID (in nsIDRef aID); */
    #[inline]
    pub unsafe fn writeID(&self, aID: *const nsID) -> Result<(), nsresult> {

        match ((*self.vtable).writeID)(self as *const _, aID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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


