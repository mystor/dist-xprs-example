//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCanvasRenderingContext2D.idl
//


pub mod nsIDOMCanvasRenderingContext2D_consts {
    pub const DRAWWINDOW_DRAW_CARET: i64 = 1;
    pub const DRAWWINDOW_DO_NOT_FLUSH: i64 = 2;
    pub const DRAWWINDOW_DRAW_VIEW: i64 = 4;
    pub const DRAWWINDOW_USE_WIDGET_LAYERS: i64 = 8;
    pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: i64 = 16;
}


#[repr(C)]
pub struct nsIDOMCanvasRenderingContext2D {
    vtable: *const nsIDOMCanvasRenderingContext2DVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCanvasRenderingContext2D {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4417cab7, 0xc7eb, 0x4e0c,
            [0xb0, 0x0a, 0xc4, 0x38, 0x42, 0xf0, 0xcb, 0xa8])
    }
}

unsafe impl RefCounted for nsIDOMCanvasRenderingContext2D {
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
pub trait nsIDOMCanvasRenderingContext2DCoerce {
    fn coerce_from(v: &nsIDOMCanvasRenderingContext2D) -> &Self;
}

impl nsIDOMCanvasRenderingContext2DCoerce for nsIDOMCanvasRenderingContext2D {
    #[inline]
    fn coerce_from(v: &nsIDOMCanvasRenderingContext2D) -> &Self {
        v
    }
}

impl nsIDOMCanvasRenderingContext2D {
    #[inline]
    pub fn coerce<T: nsIDOMCanvasRenderingContext2DCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCanvasRenderingContext2D {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCanvasRenderingContext2DCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCanvasRenderingContext2D) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCanvasRenderingContext2DVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMCanvasRenderingContext2D {
}


