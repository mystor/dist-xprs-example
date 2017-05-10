//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGZFileWriter.idl
//


#[repr(C)]
pub struct nsIGZFileWriter {
    vtable: *const nsIGZFileWriterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGZFileWriter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6bd5642c, 0x1b90, 0x4499,
            [0xba, 0x4b, 0x19, 0x9f, 0x27, 0xef, 0xab, 0xa5])
    }
}

unsafe impl RefCounted for nsIGZFileWriter {
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
pub trait nsIGZFileWriterCoerce {
    fn coerce_from(v: &nsIGZFileWriter) -> &Self;
}

impl nsIGZFileWriterCoerce for nsIGZFileWriter {
    #[inline]
    fn coerce_from(v: &nsIGZFileWriter) -> &Self {
        v
    }
}

impl nsIGZFileWriter {
    #[inline]
    pub fn coerce<T: nsIGZFileWriterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGZFileWriter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGZFileWriterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGZFileWriter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGZFileWriterVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void init (in nsIFile file); */
    pub init: unsafe extern "C" fn (this: *const nsIGZFileWriter, file: *const nsIFile) -> nsresult,

    /* [must_use,noscript] void initANSIFileDesc (in FILE file); */
    /// Unable to call function as its signature contains a non-rust type
    pub initANSIFileDesc: *const ::libc::c_void,

    /* [must_use] void write (in AUTF8String str); */
    pub write: unsafe extern "C" fn (this: *const nsIGZFileWriter, str: *const nsACString) -> nsresult,

    /* void finish (); */
    pub finish: unsafe extern "C" fn (this: *const nsIGZFileWriter) -> nsresult,

}


impl nsIGZFileWriter {
    /* [must_use] void init (in nsIFile file); */
    #[inline]
    pub unsafe fn init(&self, file: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, file.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use,noscript] void initANSIFileDesc (in FILE file); */


    /* [must_use] void write (in AUTF8String str); */
    #[inline]
    pub unsafe fn write(&self, str: &[u8]) -> Result<(), nsresult> {
        let str = nsCString::from(str);
        match ((*self.vtable).write)(self as *const _, &*str) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finish (); */
    #[inline]
    pub unsafe fn finish(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finish)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


