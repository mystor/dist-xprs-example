//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLTableCellElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLTableCellElement {
    vtable: *const nsIDOMHTMLTableCellElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLTableCellElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3203c36f, 0x33fd, 0x4628,
            [0x8c, 0x88, 0x77, 0xe8, 0x2d, 0x38, 0xdf, 0x1e])
    }
}

unsafe impl RefCounted for nsIDOMHTMLTableCellElement {
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
pub trait nsIDOMHTMLTableCellElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLTableCellElement) -> &Self;
}

impl nsIDOMHTMLTableCellElementCoerce for nsIDOMHTMLTableCellElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLTableCellElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLTableCellElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLTableCellElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLTableCellElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLTableCellElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLTableCellElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLTableCellElementVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long cellIndex; */
    pub get_cellIndex: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aCellIndex: *mut libc::int32_t) -> nsresult,

    /* attribute DOMString abbr; */
    pub get_abbr: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aAbbr: *mut nsAString) -> nsresult,
    pub set_abbr: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aAbbr: *const nsAString) -> nsresult,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aAlign: *const nsAString) -> nsresult,

    /* attribute DOMString axis; */
    pub get_axis: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aAxis: *mut nsAString) -> nsresult,
    pub set_axis: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aAxis: *const nsAString) -> nsresult,

    /* attribute DOMString bgColor; */
    pub get_bgColor: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aBgColor: *mut nsAString) -> nsresult,
    pub set_bgColor: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aBgColor: *const nsAString) -> nsresult,

    /* attribute DOMString ch; */
    pub get_ch: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aCh: *mut nsAString) -> nsresult,
    pub set_ch: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aCh: *const nsAString) -> nsresult,

    /* attribute DOMString chOff; */
    pub get_chOff: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aChOff: *mut nsAString) -> nsresult,
    pub set_chOff: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aChOff: *const nsAString) -> nsresult,

    /* attribute long colSpan; */
    pub get_colSpan: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aColSpan: *mut libc::int32_t) -> nsresult,
    pub set_colSpan: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aColSpan: libc::int32_t) -> nsresult,

    /* attribute DOMString headers; */
    pub get_headers: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aHeaders: *mut nsAString) -> nsresult,
    pub set_headers: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aHeaders: *const nsAString) -> nsresult,

    /* attribute DOMString height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aHeight: *mut nsAString) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aHeight: *const nsAString) -> nsresult,

    /* attribute boolean noWrap; */
    pub get_noWrap: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aNoWrap: *mut bool) -> nsresult,
    pub set_noWrap: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aNoWrap: bool) -> nsresult,

    /* attribute long rowSpan; */
    pub get_rowSpan: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aRowSpan: *mut libc::int32_t) -> nsresult,
    pub set_rowSpan: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aRowSpan: libc::int32_t) -> nsresult,

    /* attribute DOMString scope; */
    pub get_scope: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aScope: *mut nsAString) -> nsresult,
    pub set_scope: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aScope: *const nsAString) -> nsresult,

    /* attribute DOMString vAlign; */
    pub get_vAlign: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aVAlign: *mut nsAString) -> nsresult,
    pub set_vAlign: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aVAlign: *const nsAString) -> nsresult,

    /* attribute DOMString width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aWidth: *mut nsAString) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLTableCellElement, aWidth: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLTableCellElement {
    /* readonly attribute long cellIndex; */
    #[inline]
    pub unsafe fn get_cellIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_cellIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString abbr; */
    #[inline]
    pub unsafe fn get_abbr(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_abbr)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_abbr(&self, aAbbr: &[u16]) -> Result<(), nsresult> {
        let aAbbr = nsString::from(aAbbr);
        match ((*self.vtable).set_abbr)(self as *const _, &*aAbbr) {
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

    /* attribute DOMString axis; */
    #[inline]
    pub unsafe fn get_axis(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_axis)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_axis(&self, aAxis: &[u16]) -> Result<(), nsresult> {
        let aAxis = nsString::from(aAxis);
        match ((*self.vtable).set_axis)(self as *const _, &*aAxis) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString bgColor; */
    #[inline]
    pub unsafe fn get_bgColor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_bgColor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_bgColor(&self, aBgColor: &[u16]) -> Result<(), nsresult> {
        let aBgColor = nsString::from(aBgColor);
        match ((*self.vtable).set_bgColor)(self as *const _, &*aBgColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString ch; */
    #[inline]
    pub unsafe fn get_ch(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_ch)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_ch(&self, aCh: &[u16]) -> Result<(), nsresult> {
        let aCh = nsString::from(aCh);
        match ((*self.vtable).set_ch)(self as *const _, &*aCh) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString chOff; */
    #[inline]
    pub unsafe fn get_chOff(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_chOff)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_chOff(&self, aChOff: &[u16]) -> Result<(), nsresult> {
        let aChOff = nsString::from(aChOff);
        match ((*self.vtable).set_chOff)(self as *const _, &*aChOff) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long colSpan; */
    #[inline]
    pub unsafe fn get_colSpan(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_colSpan)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_colSpan(&self, aColSpan: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_colSpan)(self as *const _, aColSpan) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString headers; */
    #[inline]
    pub unsafe fn get_headers(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_headers)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_headers(&self, aHeaders: &[u16]) -> Result<(), nsresult> {
        let aHeaders = nsString::from(aHeaders);
        match ((*self.vtable).set_headers)(self as *const _, &*aHeaders) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_height)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_height(&self, aHeight: &[u16]) -> Result<(), nsresult> {
        let aHeight = nsString::from(aHeight);
        match ((*self.vtable).set_height)(self as *const _, &*aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean noWrap; */
    #[inline]
    pub unsafe fn get_noWrap(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_noWrap)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_noWrap(&self, aNoWrap: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_noWrap)(self as *const _, aNoWrap) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long rowSpan; */
    #[inline]
    pub unsafe fn get_rowSpan(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rowSpan)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_rowSpan(&self, aRowSpan: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_rowSpan)(self as *const _, aRowSpan) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString scope; */
    #[inline]
    pub unsafe fn get_scope(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_scope)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_scope(&self, aScope: &[u16]) -> Result<(), nsresult> {
        let aScope = nsString::from(aScope);
        match ((*self.vtable).set_scope)(self as *const _, &*aScope) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString vAlign; */
    #[inline]
    pub unsafe fn get_vAlign(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_vAlign)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_vAlign(&self, aVAlign: &[u16]) -> Result<(), nsresult> {
        let aVAlign = nsString::from(aVAlign);
        match ((*self.vtable).set_vAlign)(self as *const _, &*aVAlign) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_width)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_width(&self, aWidth: &[u16]) -> Result<(), nsresult> {
        let aWidth = nsString::from(aWidth);
        match ((*self.vtable).set_width)(self as *const _, &*aWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


