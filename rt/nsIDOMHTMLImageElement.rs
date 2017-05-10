//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLImageElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLImageElement {
    vtable: *const nsIDOMHTMLImageElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLImageElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xec18e71c, 0x4f5c, 0x4cc3,
            [0xaa, 0x36, 0x52, 0x73, 0x16, 0x86, 0x44, 0xdc])
    }
}

unsafe impl RefCounted for nsIDOMHTMLImageElement {
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
pub trait nsIDOMHTMLImageElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLImageElement) -> &Self;
}

impl nsIDOMHTMLImageElementCoerce for nsIDOMHTMLImageElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLImageElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLImageElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLImageElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLImageElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLImageElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLImageElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLImageElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString alt; */
    pub get_alt: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aAlt: *mut nsAString) -> nsresult,
    pub set_alt: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aAlt: *const nsAString) -> nsresult,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aSrc: *const nsAString) -> nsresult,

    /* attribute DOMString srcset; */
    pub get_srcset: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aSrcset: *mut nsAString) -> nsresult,
    pub set_srcset: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aSrcset: *const nsAString) -> nsresult,

    /* attribute DOMString sizes; */
    pub get_sizes: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aSizes: *mut nsAString) -> nsresult,
    pub set_sizes: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aSizes: *const nsAString) -> nsresult,

    /* attribute DOMString useMap; */
    pub get_useMap: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aUseMap: *mut nsAString) -> nsresult,
    pub set_useMap: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aUseMap: *const nsAString) -> nsresult,

    /* attribute boolean isMap; */
    pub get_isMap: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aIsMap: *mut bool) -> nsresult,
    pub set_isMap: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aIsMap: bool) -> nsresult,

    /* attribute unsigned long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aWidth: *mut libc::uint32_t) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aWidth: libc::uint32_t) -> nsresult,

    /* attribute unsigned long height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aHeight: *mut libc::uint32_t) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aHeight: libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long naturalWidth; */
    pub get_naturalWidth: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aNaturalWidth: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long naturalHeight; */
    pub get_naturalHeight: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aNaturalHeight: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean complete; */
    pub get_complete: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aComplete: *mut bool) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aAlign: *const nsAString) -> nsresult,

    /* attribute DOMString border; */
    pub get_border: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aBorder: *mut nsAString) -> nsresult,
    pub set_border: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aBorder: *const nsAString) -> nsresult,

    /* attribute long hspace; */
    pub get_hspace: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aHspace: *mut libc::int32_t) -> nsresult,
    pub set_hspace: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aHspace: libc::int32_t) -> nsresult,

    /* attribute DOMString longDesc; */
    pub get_longDesc: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aLongDesc: *mut nsAString) -> nsresult,
    pub set_longDesc: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aLongDesc: *const nsAString) -> nsresult,

    /* attribute long vspace; */
    pub get_vspace: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aVspace: *mut libc::int32_t) -> nsresult,
    pub set_vspace: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aVspace: libc::int32_t) -> nsresult,

    /* attribute DOMString lowsrc; */
    pub get_lowsrc: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aLowsrc: *mut nsAString) -> nsresult,
    pub set_lowsrc: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aLowsrc: *const nsAString) -> nsresult,

    /* readonly attribute DOMString currentSrc; */
    pub get_currentSrc: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aCurrentSrc: *mut nsAString) -> nsresult,

    /* readonly attribute long x; */
    pub get_x: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long y; */
    pub get_y: unsafe extern "C" fn (this: *const nsIDOMHTMLImageElement, aY: *mut libc::int32_t) -> nsresult,

}


impl nsIDOMHTMLImageElement {
    /* attribute DOMString alt; */
    #[inline]
    pub unsafe fn get_alt(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_alt)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_alt(&self, aAlt: &[u16]) -> Result<(), nsresult> {
        let aAlt = nsString::from(aAlt);
        match ((*self.vtable).set_alt)(self as *const _, &*aAlt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString src; */
    #[inline]
    pub unsafe fn get_src(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_src)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_src(&self, aSrc: &[u16]) -> Result<(), nsresult> {
        let aSrc = nsString::from(aSrc);
        match ((*self.vtable).set_src)(self as *const _, &*aSrc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString srcset; */
    #[inline]
    pub unsafe fn get_srcset(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_srcset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_srcset(&self, aSrcset: &[u16]) -> Result<(), nsresult> {
        let aSrcset = nsString::from(aSrcset);
        match ((*self.vtable).set_srcset)(self as *const _, &*aSrcset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString sizes; */
    #[inline]
    pub unsafe fn get_sizes(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_sizes)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sizes(&self, aSizes: &[u16]) -> Result<(), nsresult> {
        let aSizes = nsString::from(aSizes);
        match ((*self.vtable).set_sizes)(self as *const _, &*aSizes) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString useMap; */
    #[inline]
    pub unsafe fn get_useMap(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_useMap)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_useMap(&self, aUseMap: &[u16]) -> Result<(), nsresult> {
        let aUseMap = nsString::from(aUseMap);
        match ((*self.vtable).set_useMap)(self as *const _, &*aUseMap) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isMap; */
    #[inline]
    pub unsafe fn get_isMap(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isMap)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isMap(&self, aIsMap: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isMap)(self as *const _, aIsMap) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* readonly attribute unsigned long naturalWidth; */
    #[inline]
    pub unsafe fn get_naturalWidth(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_naturalWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long naturalHeight; */
    #[inline]
    pub unsafe fn get_naturalHeight(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_naturalHeight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean complete; */
    #[inline]
    pub unsafe fn get_complete(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_complete)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString align; */
    #[inline]
    pub unsafe fn get_align(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_align)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_align(&self, aAlign: &[u16]) -> Result<(), nsresult> {
        let aAlign = nsString::from(aAlign);
        match ((*self.vtable).set_align)(self as *const _, &*aAlign) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString border; */
    #[inline]
    pub unsafe fn get_border(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_border)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_border(&self, aBorder: &[u16]) -> Result<(), nsresult> {
        let aBorder = nsString::from(aBorder);
        match ((*self.vtable).set_border)(self as *const _, &*aBorder) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long hspace; */
    #[inline]
    pub unsafe fn get_hspace(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_hspace)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hspace(&self, aHspace: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_hspace)(self as *const _, aHspace) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString longDesc; */
    #[inline]
    pub unsafe fn get_longDesc(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_longDesc)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_longDesc(&self, aLongDesc: &[u16]) -> Result<(), nsresult> {
        let aLongDesc = nsString::from(aLongDesc);
        match ((*self.vtable).set_longDesc)(self as *const _, &*aLongDesc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long vspace; */
    #[inline]
    pub unsafe fn get_vspace(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_vspace)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_vspace(&self, aVspace: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_vspace)(self as *const _, aVspace) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString lowsrc; */
    #[inline]
    pub unsafe fn get_lowsrc(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_lowsrc)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_lowsrc(&self, aLowsrc: &[u16]) -> Result<(), nsresult> {
        let aLowsrc = nsString::from(aLowsrc);
        match ((*self.vtable).set_lowsrc)(self as *const _, &*aLowsrc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute DOMString currentSrc; */
    #[inline]
    pub unsafe fn get_currentSrc(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_currentSrc)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long x; */
    #[inline]
    pub unsafe fn get_x(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_x)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long y; */
    #[inline]
    pub unsafe fn get_y(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_y)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


