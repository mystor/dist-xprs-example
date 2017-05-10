//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMProcessingInstruction.idl
//


#[repr(C)]
pub struct nsIDOMProcessingInstruction {
    vtable: *const nsIDOMProcessingInstructionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMProcessingInstruction {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5a139df7, 0x04d0, 0x438d,
            [0xbd, 0x18, 0xd8, 0x12, 0x25, 0x64, 0x25, 0x8f])
    }
}

unsafe impl RefCounted for nsIDOMProcessingInstruction {
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
pub trait nsIDOMProcessingInstructionCoerce {
    fn coerce_from(v: &nsIDOMProcessingInstruction) -> &Self;
}

impl nsIDOMProcessingInstructionCoerce for nsIDOMProcessingInstruction {
    #[inline]
    fn coerce_from(v: &nsIDOMProcessingInstruction) -> &Self {
        v
    }
}

impl nsIDOMProcessingInstruction {
    #[inline]
    pub fn coerce<T: nsIDOMProcessingInstructionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMProcessingInstruction {
    type Target = nsIDOMCharacterData;
    #[inline]
    fn deref(&self) -> &nsIDOMCharacterData {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCharacterDataCoerce> nsIDOMProcessingInstructionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMProcessingInstruction) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMProcessingInstructionVTable {
    pub __base: nsIDOMCharacterDataVTable,

    /* readonly attribute DOMString target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDOMProcessingInstruction, aTarget: *mut nsAString) -> nsresult,

}


impl nsIDOMProcessingInstruction {
    /* readonly attribute DOMString target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_target)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


