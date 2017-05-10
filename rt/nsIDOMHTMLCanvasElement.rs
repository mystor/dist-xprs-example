//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLCanvasElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLCanvasElement {
    vtable: *const nsIDOMHTMLCanvasElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLCanvasElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4e8f1316, 0xb601, 0x471d,
            [0x8f, 0x44, 0x3c, 0x65, 0x0d, 0x91, 0xee, 0x9b])
    }
}

unsafe impl RefCounted for nsIDOMHTMLCanvasElement {
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
pub trait nsIDOMHTMLCanvasElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLCanvasElement) -> &Self;
}

impl nsIDOMHTMLCanvasElementCoerce for nsIDOMHTMLCanvasElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLCanvasElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLCanvasElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLCanvasElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLCanvasElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLCanvasElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLCanvasElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLCanvasElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLCanvasElement, aWidth: *mut libc::uint32_t) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLCanvasElement, aWidth: libc::uint32_t) -> nsresult,

    /* attribute unsigned long height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMHTMLCanvasElement, aHeight: *mut libc::uint32_t) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIDOMHTMLCanvasElement, aHeight: libc::uint32_t) -> nsresult,

    /* attribute boolean mozOpaque; */
    pub get_mozOpaque: unsafe extern "C" fn (this: *const nsIDOMHTMLCanvasElement, aMozOpaque: *mut bool) -> nsresult,
    pub set_mozOpaque: unsafe extern "C" fn (this: *const nsIDOMHTMLCanvasElement, aMozOpaque: bool) -> nsresult,

}


impl nsIDOMHTMLCanvasElement {
    /* attribute unsigned long width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_width(&self, aWidth: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_width)(self as *const _, aWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_height)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_height(&self, aHeight: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_height)(self as *const _, aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean mozOpaque; */
    #[inline]
    pub unsafe fn get_mozOpaque(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozOpaque)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mozOpaque(&self, aMozOpaque: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_mozOpaque)(self as *const _, aMozOpaque) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


