//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDragService.idl
//


pub mod nsIDragService_consts {
    pub const DRAGDROP_ACTION_NONE: i64 = 0;
    pub const DRAGDROP_ACTION_COPY: i64 = 1;
    pub const DRAGDROP_ACTION_MOVE: i64 = 2;
    pub const DRAGDROP_ACTION_LINK: i64 = 4;
    pub const DRAGDROP_ACTION_UNINITIALIZED: i64 = 64;
}


#[repr(C)]
pub struct nsIDragService {
    vtable: *const nsIDragServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDragService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xebd6b3a2, 0xaf16, 0x43af,
            [0xa6, 0x98, 0x30, 0x91, 0xa0, 0x87, 0xdd, 0x62])
    }
}

unsafe impl RefCounted for nsIDragService {
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
pub trait nsIDragServiceCoerce {
    fn coerce_from(v: &nsIDragService) -> &Self;
}

impl nsIDragServiceCoerce for nsIDragService {
    #[inline]
    fn coerce_from(v: &nsIDragService) -> &Self {
        v
    }
}

impl nsIDragService {
    #[inline]
    pub fn coerce<T: nsIDragServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDragService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDragServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDragService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDragServiceVTable {
    pub __base: nsISupportsVTable,

    /* void invokeDragSession (in nsIDOMNode aDOMNode, in nsIArray aTransferables, in nsIScriptableRegion aRegion, in unsigned long aActionType, [optional] in nsContentPolicyType aContentPolicyType); */
    pub invokeDragSession: unsafe extern "C" fn (this: *const nsIDragService, aDOMNode: *const nsIDOMNode, aTransferables: *const nsIArray, aRegion: *const nsIScriptableRegion, aActionType: libc::uint32_t, aContentPolicyType: nsContentPolicyType) -> nsresult,

    /* void invokeDragSessionWithImage (in nsIDOMNode aDOMNode, in nsIArray aTransferableArray, in nsIScriptableRegion aRegion, in unsigned long aActionType, in nsIDOMNode aImage, in long aImageX, in long aImageY, in nsIDOMDragEvent aDragEvent, in nsIDOMDataTransfer aDataTransfer); */
    pub invokeDragSessionWithImage: unsafe extern "C" fn (this: *const nsIDragService, aDOMNode: *const nsIDOMNode, aTransferableArray: *const nsIArray, aRegion: *const nsIScriptableRegion, aActionType: libc::uint32_t, aImage: *const nsIDOMNode, aImageX: libc::int32_t, aImageY: libc::int32_t, aDragEvent: *const nsIDOMDragEvent, aDataTransfer: *const nsIDOMDataTransfer) -> nsresult,

    /* void invokeDragSessionWithSelection (in nsISelection aSelection, in nsIArray aTransferableArray, in unsigned long aActionType, in nsIDOMDragEvent aDragEvent, in nsIDOMDataTransfer aDataTransfer); */
    pub invokeDragSessionWithSelection: unsafe extern "C" fn (this: *const nsIDragService, aSelection: *const nsISelection, aTransferableArray: *const nsIArray, aActionType: libc::uint32_t, aDragEvent: *const nsIDOMDragEvent, aDataTransfer: *const nsIDOMDataTransfer) -> nsresult,

    /* nsIDragSession getCurrentSession (); */
    pub getCurrentSession: unsafe extern "C" fn (this: *const nsIDragService, _retval: *mut *const nsIDragSession) -> nsresult,

    /* void startDragSession (); */
    pub startDragSession: unsafe extern "C" fn (this: *const nsIDragService) -> nsresult,

    /* void endDragSession (in boolean aDoneDrag, [optional] in unsigned long aKeyModifiers); */
    pub endDragSession: unsafe extern "C" fn (this: *const nsIDragService, aDoneDrag: bool, aKeyModifiers: libc::uint32_t) -> nsresult,

    /* [noscript] void fireDragEventAtSource (in EventMessage aEventMessage, in unsigned long aKeyModifiers); */
    /// Unable to call function as its signature contains a non-rust type
    pub fireDragEventAtSource: *const ::libc::c_void,

    /* void suppress (); */
    pub suppress: unsafe extern "C" fn (this: *const nsIDragService) -> nsresult,

    /* void unsuppress (); */
    pub unsuppress: unsafe extern "C" fn (this: *const nsIDragService) -> nsresult,

    /* [noscript] void dragMoved (in long aX, in long aY); */
    pub dragMoved: unsafe extern "C" fn (this: *const nsIDragService, aX: libc::int32_t, aY: libc::int32_t) -> nsresult,

    /* [nostdcall,notxpcom] boolean maybeAddChildProcess (in ContentParentPtr aChild); */
    /// Unable to call function as its signature contains a non-rust type
    pub maybeAddChildProcess: *const ::libc::c_void,

}


impl nsIDragService {
    /* void invokeDragSession (in nsIDOMNode aDOMNode, in nsIArray aTransferables, in nsIScriptableRegion aRegion, in unsigned long aActionType, [optional] in nsContentPolicyType aContentPolicyType); */
    #[inline]
    pub unsafe fn invokeDragSession(&self, aDOMNode: Option<&nsIDOMNode>, aTransferables: Option<&nsIArray>, aRegion: Option<&nsIScriptableRegion>, aActionType: libc::uint32_t, aContentPolicyType: nsContentPolicyType) -> Result<(), nsresult> {

        match ((*self.vtable).invokeDragSession)(self as *const _, aDOMNode.map_or(::std::ptr::null(), |x| x as *const _), aTransferables.map_or(::std::ptr::null(), |x| x as *const _), aRegion.map_or(::std::ptr::null(), |x| x as *const _), aActionType, aContentPolicyType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invokeDragSessionWithImage (in nsIDOMNode aDOMNode, in nsIArray aTransferableArray, in nsIScriptableRegion aRegion, in unsigned long aActionType, in nsIDOMNode aImage, in long aImageX, in long aImageY, in nsIDOMDragEvent aDragEvent, in nsIDOMDataTransfer aDataTransfer); */
    #[inline]
    pub unsafe fn invokeDragSessionWithImage(&self, aDOMNode: Option<&nsIDOMNode>, aTransferableArray: Option<&nsIArray>, aRegion: Option<&nsIScriptableRegion>, aActionType: libc::uint32_t, aImage: Option<&nsIDOMNode>, aImageX: libc::int32_t, aImageY: libc::int32_t, aDragEvent: Option<&nsIDOMDragEvent>, aDataTransfer: Option<&nsIDOMDataTransfer>) -> Result<(), nsresult> {

        match ((*self.vtable).invokeDragSessionWithImage)(self as *const _, aDOMNode.map_or(::std::ptr::null(), |x| x as *const _), aTransferableArray.map_or(::std::ptr::null(), |x| x as *const _), aRegion.map_or(::std::ptr::null(), |x| x as *const _), aActionType, aImage.map_or(::std::ptr::null(), |x| x as *const _), aImageX, aImageY, aDragEvent.map_or(::std::ptr::null(), |x| x as *const _), aDataTransfer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invokeDragSessionWithSelection (in nsISelection aSelection, in nsIArray aTransferableArray, in unsigned long aActionType, in nsIDOMDragEvent aDragEvent, in nsIDOMDataTransfer aDataTransfer); */
    #[inline]
    pub unsafe fn invokeDragSessionWithSelection(&self, aSelection: Option<&nsISelection>, aTransferableArray: Option<&nsIArray>, aActionType: libc::uint32_t, aDragEvent: Option<&nsIDOMDragEvent>, aDataTransfer: Option<&nsIDOMDataTransfer>) -> Result<(), nsresult> {

        match ((*self.vtable).invokeDragSessionWithSelection)(self as *const _, aSelection.map_or(::std::ptr::null(), |x| x as *const _), aTransferableArray.map_or(::std::ptr::null(), |x| x as *const _), aActionType, aDragEvent.map_or(::std::ptr::null(), |x| x as *const _), aDataTransfer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDragSession getCurrentSession (); */
    #[inline]
    pub unsafe fn getCurrentSession(&self, ) -> Result<Option<RefPtr<nsIDragSession>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCurrentSession)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void startDragSession (); */
    #[inline]
    pub unsafe fn startDragSession(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).startDragSession)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endDragSession (in boolean aDoneDrag, [optional] in unsigned long aKeyModifiers); */
    #[inline]
    pub unsafe fn endDragSession(&self, aDoneDrag: bool, aKeyModifiers: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).endDragSession)(self as *const _, aDoneDrag, aKeyModifiers) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void fireDragEventAtSource (in EventMessage aEventMessage, in unsigned long aKeyModifiers); */


    /* void suppress (); */
    #[inline]
    pub unsafe fn suppress(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).suppress)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unsuppress (); */
    #[inline]
    pub unsafe fn unsuppress(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unsuppress)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void dragMoved (in long aX, in long aY); */
    #[inline]
    pub unsafe fn dragMoved(&self, aX: libc::int32_t, aY: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).dragMoved)(self as *const _, aX, aY) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [nostdcall,notxpcom] boolean maybeAddChildProcess (in ContentParentPtr aChild); */


}


