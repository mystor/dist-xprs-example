//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTMLAbsPosEditor.idl
//


#[repr(C)]
pub struct nsIHTMLAbsPosEditor {
    vtable: *const nsIHTMLAbsPosEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHTMLAbsPosEditor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x91375f52, 0x20e6, 0x4757,
            [0x98, 0x35, 0xeb, 0x04, 0xfa, 0xbe, 0x54, 0x98])
    }
}

unsafe impl RefCounted for nsIHTMLAbsPosEditor {
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
pub trait nsIHTMLAbsPosEditorCoerce {
    fn coerce_from(v: &nsIHTMLAbsPosEditor) -> &Self;
}

impl nsIHTMLAbsPosEditorCoerce for nsIHTMLAbsPosEditor {
    #[inline]
    fn coerce_from(v: &nsIHTMLAbsPosEditor) -> &Self {
        v
    }
}

impl nsIHTMLAbsPosEditor {
    #[inline]
    pub fn coerce<T: nsIHTMLAbsPosEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHTMLAbsPosEditor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHTMLAbsPosEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTMLAbsPosEditor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHTMLAbsPosEditorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean selectionContainerAbsolutelyPositioned; */
    pub get_selectionContainerAbsolutelyPositioned: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aSelectionContainerAbsolutelyPositioned: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMElement positionedElement; */
    pub get_positionedElement: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aPositionedElement: *mut *const nsIDOMElement) -> nsresult,

    /* attribute boolean absolutePositioningEnabled; */
    pub get_absolutePositioningEnabled: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aAbsolutePositioningEnabled: *mut bool) -> nsresult,
    pub set_absolutePositioningEnabled: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aAbsolutePositioningEnabled: bool) -> nsresult,

    /* attribute boolean snapToGridEnabled; */
    pub get_snapToGridEnabled: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aSnapToGridEnabled: *mut bool) -> nsresult,
    pub set_snapToGridEnabled: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aSnapToGridEnabled: bool) -> nsresult,

    /* attribute unsigned long gridSize; */
    pub get_gridSize: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aGridSize: *mut libc::uint32_t) -> nsresult,
    pub set_gridSize: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aGridSize: libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMElement absolutelyPositionedSelectionContainer; */
    pub get_absolutelyPositionedSelectionContainer: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aAbsolutelyPositionedSelectionContainer: *mut *const nsIDOMElement) -> nsresult,

    /* void absolutePositionSelection (in boolean aEnabled); */
    pub absolutePositionSelection: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aEnabled: bool) -> nsresult,

    /* void relativeChangeZIndex (in long aChange); */
    pub relativeChangeZIndex: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aChange: libc::int32_t) -> nsresult,

    /* void absolutelyPositionElement (in nsIDOMElement aElement, in boolean aEnabled); */
    pub absolutelyPositionElement: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aElement: *const nsIDOMElement, aEnabled: bool) -> nsresult,

    /* void setElementPosition (in nsIDOMElement aElement, in long aX, in long aY); */
    pub setElementPosition: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aElement: *const nsIDOMElement, aX: libc::int32_t, aY: libc::int32_t) -> nsresult,

    /* long getElementZIndex (in nsIDOMElement aElement); */
    pub getElementZIndex: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aElement: *const nsIDOMElement, _retval: *mut libc::int32_t) -> nsresult,

    /* void setElementZIndex (in nsIDOMElement aElement, in long aZorder); */
    pub setElementZIndex: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aElement: *const nsIDOMElement, aZorder: libc::int32_t) -> nsresult,

    /* long relativeChangeElementZIndex (in nsIDOMElement aElement, in long aChange); */
    pub relativeChangeElementZIndex: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aElement: *const nsIDOMElement, aChange: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* void showGrabberOnElement (in nsIDOMElement aElement); */
    pub showGrabberOnElement: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor, aElement: *const nsIDOMElement) -> nsresult,

    /* void hideGrabber (); */
    pub hideGrabber: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor) -> nsresult,

    /* void refreshGrabber (); */
    pub refreshGrabber: unsafe extern "C" fn (this: *const nsIHTMLAbsPosEditor) -> nsresult,

}


impl nsIHTMLAbsPosEditor {
    /* readonly attribute boolean selectionContainerAbsolutelyPositioned; */
    #[inline]
    pub unsafe fn get_selectionContainerAbsolutelyPositioned(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_selectionContainerAbsolutelyPositioned)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMElement positionedElement; */
    #[inline]
    pub unsafe fn get_positionedElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_positionedElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean absolutePositioningEnabled; */
    #[inline]
    pub unsafe fn get_absolutePositioningEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_absolutePositioningEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_absolutePositioningEnabled(&self, aAbsolutePositioningEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_absolutePositioningEnabled)(self as *const _, aAbsolutePositioningEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean snapToGridEnabled; */
    #[inline]
    pub unsafe fn get_snapToGridEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_snapToGridEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_snapToGridEnabled(&self, aSnapToGridEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_snapToGridEnabled)(self as *const _, aSnapToGridEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long gridSize; */
    #[inline]
    pub unsafe fn get_gridSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_gridSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_gridSize(&self, aGridSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_gridSize)(self as *const _, aGridSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMElement absolutelyPositionedSelectionContainer; */
    #[inline]
    pub unsafe fn get_absolutelyPositionedSelectionContainer(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_absolutelyPositionedSelectionContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void absolutePositionSelection (in boolean aEnabled); */
    #[inline]
    pub unsafe fn absolutePositionSelection(&self, aEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).absolutePositionSelection)(self as *const _, aEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void relativeChangeZIndex (in long aChange); */
    #[inline]
    pub unsafe fn relativeChangeZIndex(&self, aChange: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).relativeChangeZIndex)(self as *const _, aChange) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void absolutelyPositionElement (in nsIDOMElement aElement, in boolean aEnabled); */
    #[inline]
    pub unsafe fn absolutelyPositionElement(&self, aElement: Option<&nsIDOMElement>, aEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).absolutelyPositionElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setElementPosition (in nsIDOMElement aElement, in long aX, in long aY); */
    #[inline]
    pub unsafe fn setElementPosition(&self, aElement: Option<&nsIDOMElement>, aX: libc::int32_t, aY: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setElementPosition)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aX, aY) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long getElementZIndex (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn getElementZIndex(&self, aElement: Option<&nsIDOMElement>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getElementZIndex)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setElementZIndex (in nsIDOMElement aElement, in long aZorder); */
    #[inline]
    pub unsafe fn setElementZIndex(&self, aElement: Option<&nsIDOMElement>, aZorder: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setElementZIndex)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aZorder) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long relativeChangeElementZIndex (in nsIDOMElement aElement, in long aChange); */
    #[inline]
    pub unsafe fn relativeChangeElementZIndex(&self, aElement: Option<&nsIDOMElement>, aChange: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).relativeChangeElementZIndex)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aChange, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void showGrabberOnElement (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn showGrabberOnElement(&self, aElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).showGrabberOnElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void hideGrabber (); */
    #[inline]
    pub unsafe fn hideGrabber(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).hideGrabber)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void refreshGrabber (); */
    #[inline]
    pub unsafe fn refreshGrabber(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).refreshGrabber)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


