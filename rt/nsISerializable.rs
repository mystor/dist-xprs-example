//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISerializable.idl
//


#[repr(C)]
pub struct nsISerializable {
    vtable: *const nsISerializableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISerializable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x91cca981, 0xc26d, 0x44a8,
            [0xbe, 0xbe, 0xd9, 0xed, 0x48, 0x91, 0x50, 0x3a])
    }
}

unsafe impl RefCounted for nsISerializable {
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
pub trait nsISerializableCoerce {
    fn coerce_from(v: &nsISerializable) -> &Self;
}

impl nsISerializableCoerce for nsISerializable {
    #[inline]
    fn coerce_from(v: &nsISerializable) -> &Self {
        v
    }
}

impl nsISerializable {
    #[inline]
    pub fn coerce<T: nsISerializableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISerializable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISerializableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISerializable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISerializableVTable {
    pub __base: nsISupportsVTable,

    /* void read (in nsIObjectInputStream aInputStream); */
    pub read: unsafe extern "C" fn (this: *const nsISerializable, aInputStream: *const nsIObjectInputStream) -> nsresult,

    /* void write (in nsIObjectOutputStream aOutputStream); */
    pub write: unsafe extern "C" fn (this: *const nsISerializable, aOutputStream: *const nsIObjectOutputStream) -> nsresult,

}


impl nsISerializable {
    /* void read (in nsIObjectInputStream aInputStream); */
    #[inline]
    pub unsafe fn read(&self, aInputStream: Option<&nsIObjectInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).read)(self as *const _, aInputStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void write (in nsIObjectOutputStream aOutputStream); */
    #[inline]
    pub unsafe fn write(&self, aOutputStream: Option<&nsIObjectOutputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).write)(self as *const _, aOutputStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


