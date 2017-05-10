//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMutableArray.idl
//


#[repr(C)]
pub struct nsIMutableArray {
    vtable: *const nsIMutableArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMutableArray {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaf059da0, 0xc85b, 0x40ec,
            [0xaf, 0x07, 0xae, 0x4b, 0xfd, 0xc1, 0x92, 0xcc])
    }
}

unsafe impl RefCounted for nsIMutableArray {
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
pub trait nsIMutableArrayCoerce {
    fn coerce_from(v: &nsIMutableArray) -> &Self;
}

impl nsIMutableArrayCoerce for nsIMutableArray {
    #[inline]
    fn coerce_from(v: &nsIMutableArray) -> &Self {
        v
    }
}

impl nsIMutableArray {
    #[inline]
    pub fn coerce<T: nsIMutableArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMutableArray {
    type Target = nsIArrayExtensions;
    #[inline]
    fn deref(&self) -> &nsIArrayExtensions {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIArrayExtensionsCoerce> nsIMutableArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMutableArray) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMutableArrayVTable {
    pub __base: nsIArrayExtensionsVTable,

    /* void appendElement (in nsISupports element, [optional] in boolean weak); */
    pub appendElement: unsafe extern "C" fn (this: *const nsIMutableArray, element: *const nsISupports, weak: bool) -> nsresult,

    /* void removeElementAt (in unsigned long index); */
    pub removeElementAt: unsafe extern "C" fn (this: *const nsIMutableArray, index: libc::uint32_t) -> nsresult,

    /* void insertElementAt (in nsISupports element, in unsigned long index, in boolean weak); */
    pub insertElementAt: unsafe extern "C" fn (this: *const nsIMutableArray, element: *const nsISupports, index: libc::uint32_t, weak: bool) -> nsresult,

    /* void replaceElementAt (in nsISupports element, in unsigned long index, in boolean weak); */
    pub replaceElementAt: unsafe extern "C" fn (this: *const nsIMutableArray, element: *const nsISupports, index: libc::uint32_t, weak: bool) -> nsresult,

    /* void clear (); */
    pub clear: unsafe extern "C" fn (this: *const nsIMutableArray) -> nsresult,

}


impl nsIMutableArray {
    /* void appendElement (in nsISupports element, [optional] in boolean weak); */
    #[inline]
    pub unsafe fn appendElement(&self, element: Option<&nsISupports>, weak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).appendElement)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), weak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeElementAt (in unsigned long index); */
    #[inline]
    pub unsafe fn removeElementAt(&self, index: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeElementAt)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertElementAt (in nsISupports element, in unsigned long index, in boolean weak); */
    #[inline]
    pub unsafe fn insertElementAt(&self, element: Option<&nsISupports>, index: libc::uint32_t, weak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).insertElementAt)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), index, weak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void replaceElementAt (in nsISupports element, in unsigned long index, in boolean weak); */
    #[inline]
    pub unsafe fn replaceElementAt(&self, element: Option<&nsISupports>, index: libc::uint32_t, weak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).replaceElementAt)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), index, weak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clear (); */
    #[inline]
    pub unsafe fn clear(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clear)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


