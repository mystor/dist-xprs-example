//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleRelation.idl
//


pub mod nsIAccessibleRelation_consts {
    pub const RELATION_LABELLED_BY: i64 = 0;
    pub const RELATION_LABEL_FOR: i64 = 1;
    pub const RELATION_DESCRIBED_BY: i64 = 2;
    pub const RELATION_DESCRIPTION_FOR: i64 = 3;
    pub const RELATION_NODE_CHILD_OF: i64 = 4;
    pub const RELATION_NODE_PARENT_OF: i64 = 5;
    pub const RELATION_CONTROLLED_BY: i64 = 6;
    pub const RELATION_CONTROLLER_FOR: i64 = 7;
    pub const RELATION_FLOWS_TO: i64 = 8;
    pub const RELATION_FLOWS_FROM: i64 = 9;
    pub const RELATION_MEMBER_OF: i64 = 10;
    pub const RELATION_SUBWINDOW_OF: i64 = 11;
    pub const RELATION_EMBEDS: i64 = 12;
    pub const RELATION_EMBEDDED_BY: i64 = 13;
    pub const RELATION_POPUP_FOR: i64 = 14;
    pub const RELATION_PARENT_WINDOW_OF: i64 = 15;
    pub const RELATION_DEFAULT_BUTTON: i64 = 16;
    pub const RELATION_CONTAINING_DOCUMENT: i64 = 17;
    pub const RELATION_CONTAINING_TAB_PANE: i64 = 18;
    pub const RELATION_CONTAINING_APPLICATION: i64 = 20;
    pub const RELATION_DETAILS: i64 = 21;
    pub const RELATION_DETAILS_FOR: i64 = 22;
    pub const RELATION_ERRORMSG: i64 = 23;
    pub const RELATION_ERRORMSG_FOR: i64 = 24;
}


#[repr(C)]
pub struct nsIAccessibleRelation {
    vtable: *const nsIAccessibleRelationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleRelation {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x55b308c4, 0x2ae4, 0x46bc,
            [0xb4, 0xcd, 0x4d, 0x43, 0x70, 0xe0, 0xa6, 0x60])
    }
}

unsafe impl RefCounted for nsIAccessibleRelation {
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
pub trait nsIAccessibleRelationCoerce {
    fn coerce_from(v: &nsIAccessibleRelation) -> &Self;
}

impl nsIAccessibleRelationCoerce for nsIAccessibleRelation {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRelation) -> &Self {
        v
    }
}

impl nsIAccessibleRelation {
    #[inline]
    pub fn coerce<T: nsIAccessibleRelationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleRelation {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleRelationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRelation) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleRelationVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long relationType; */
    pub get_relationType: unsafe extern "C" fn (this: *const nsIAccessibleRelation, aRelationType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long targetsCount; */
    pub get_targetsCount: unsafe extern "C" fn (this: *const nsIAccessibleRelation, aTargetsCount: *mut libc::uint32_t) -> nsresult,

    /* nsIAccessible getTarget (in unsigned long index); */
    pub getTarget: unsafe extern "C" fn (this: *const nsIAccessibleRelation, index: libc::uint32_t, _retval: *mut *const nsIAccessible) -> nsresult,

    /* nsIArray getTargets (); */
    pub getTargets: unsafe extern "C" fn (this: *const nsIAccessibleRelation, _retval: *mut *const nsIArray) -> nsresult,

}


impl nsIAccessibleRelation {
    /* readonly attribute unsigned long relationType; */
    #[inline]
    pub unsafe fn get_relationType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_relationType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long targetsCount; */
    #[inline]
    pub unsafe fn get_targetsCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_targetsCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIAccessible getTarget (in unsigned long index); */
    #[inline]
    pub unsafe fn getTarget(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getTarget)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIArray getTargets (); */
    #[inline]
    pub unsafe fn getTargets(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getTargets)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


