//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPipe.idl
//


#[repr(C)]
pub struct nsIPipe {
    vtable: *const nsIPipeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPipe {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x25d0de93, 0x685e, 0x4ea4,
            [0x95, 0xd3, 0xd8, 0x84, 0xe3, 0x1d, 0xf6, 0x3c])
    }
}

unsafe impl RefCounted for nsIPipe {
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
pub trait nsIPipeCoerce {
    fn coerce_from(v: &nsIPipe) -> &Self;
}

impl nsIPipeCoerce for nsIPipe {
    #[inline]
    fn coerce_from(v: &nsIPipe) -> &Self {
        v
    }
}

impl nsIPipe {
    #[inline]
    pub fn coerce<T: nsIPipeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPipe {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPipeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPipe) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPipeVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void init (in boolean nonBlockingInput, in boolean nonBlockingOutput, in unsigned long segmentSize, in unsigned long segmentCount); */
    pub init: unsafe extern "C" fn (this: *const nsIPipe, nonBlockingInput: bool, nonBlockingOutput: bool, segmentSize: libc::uint32_t, segmentCount: libc::uint32_t) -> nsresult,

    /* [must_use] readonly attribute nsIAsyncInputStream inputStream; */
    pub get_inputStream: unsafe extern "C" fn (this: *const nsIPipe, aInputStream: *mut *const nsIAsyncInputStream) -> nsresult,

    /* [must_use] readonly attribute nsIAsyncOutputStream outputStream; */
    pub get_outputStream: unsafe extern "C" fn (this: *const nsIPipe, aOutputStream: *mut *const nsIAsyncOutputStream) -> nsresult,

}


impl nsIPipe {
    /* [must_use] void init (in boolean nonBlockingInput, in boolean nonBlockingOutput, in unsigned long segmentSize, in unsigned long segmentCount); */
    #[inline]
    pub unsafe fn init(&self, nonBlockingInput: bool, nonBlockingOutput: bool, segmentSize: libc::uint32_t, segmentCount: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, nonBlockingInput, nonBlockingOutput, segmentSize, segmentCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute nsIAsyncInputStream inputStream; */
    #[inline]
    pub unsafe fn get_inputStream(&self, ) -> Result<Option<RefPtr<nsIAsyncInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_inputStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] readonly attribute nsIAsyncOutputStream outputStream; */
    #[inline]
    pub unsafe fn get_outputStream(&self, ) -> Result<Option<RefPtr<nsIAsyncOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_outputStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsISearchableInputStream {
    vtable: *const nsISearchableInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISearchableInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8c39ef62, 0xf7c9, 0x11d4,
            [0x98, 0xf5, 0x00, 0x10, 0x83, 0x01, 0x0e, 0x9b])
    }
}

unsafe impl RefCounted for nsISearchableInputStream {
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
pub trait nsISearchableInputStreamCoerce {
    fn coerce_from(v: &nsISearchableInputStream) -> &Self;
}

impl nsISearchableInputStreamCoerce for nsISearchableInputStream {
    #[inline]
    fn coerce_from(v: &nsISearchableInputStream) -> &Self {
        v
    }
}

impl nsISearchableInputStream {
    #[inline]
    pub fn coerce<T: nsISearchableInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISearchableInputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISearchableInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchableInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISearchableInputStreamVTable {
    pub __base: nsISupportsVTable,

    /* void search (in string forString, in boolean ignoreCase, out boolean found, out unsigned long offsetSearchedTo); */
    pub search: unsafe extern "C" fn (this: *const nsISearchableInputStream, forString: *const libc::c_char, ignoreCase: bool, found: *mut bool, offsetSearchedTo: *mut libc::uint32_t) -> nsresult,

}


impl nsISearchableInputStream {
    /* void search (in string forString, in boolean ignoreCase, out boolean found, out unsigned long offsetSearchedTo); */
    #[inline]
    pub unsafe fn search(&self, forString: *const libc::c_char, ignoreCase: bool) -> Result<(bool, libc::uint32_t), nsresult> {
        let mut found: bool = ::std::mem::zeroed();
        let mut offsetSearchedTo: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).search)(self as *const _, forString, ignoreCase, &mut found as *mut _, &mut offsetSearchedTo as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((found, offsetSearchedTo))
    }

}


