//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISpeechRecognitionService.idl
//


#[repr(C)]
pub struct nsISpeechGrammarCompilationCallback {
    vtable: *const nsISpeechGrammarCompilationCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISpeechGrammarCompilationCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6fcb6ee8, 0xa6db, 0x49ba,
            [0x9f, 0x06, 0x35, 0x5d, 0x7e, 0xe1, 0x8e, 0xa7])
    }
}

unsafe impl RefCounted for nsISpeechGrammarCompilationCallback {
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
pub trait nsISpeechGrammarCompilationCallbackCoerce {
    fn coerce_from(v: &nsISpeechGrammarCompilationCallback) -> &Self;
}

impl nsISpeechGrammarCompilationCallbackCoerce for nsISpeechGrammarCompilationCallback {
    #[inline]
    fn coerce_from(v: &nsISpeechGrammarCompilationCallback) -> &Self {
        v
    }
}

impl nsISpeechGrammarCompilationCallback {
    #[inline]
    pub fn coerce<T: nsISpeechGrammarCompilationCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISpeechGrammarCompilationCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISpeechGrammarCompilationCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechGrammarCompilationCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISpeechGrammarCompilationCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void grammarCompilationEnd (in SpeechGrammarPtr grammarObject, in boolean success); */
    /// Unable to call function as its signature contains a non-rust type
    pub grammarCompilationEnd: *const ::libc::c_void,

}


impl nsISpeechGrammarCompilationCallback {
    /* void grammarCompilationEnd (in SpeechGrammarPtr grammarObject, in boolean success); */


}


#[repr(C)]
pub struct nsISpeechRecognitionService {
    vtable: *const nsISpeechRecognitionServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISpeechRecognitionService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8e97f287, 0xf322, 0x44e8,
            [0x88, 0x88, 0x83, 0x44, 0xfa, 0x40, 0x8e, 0xf8])
    }
}

unsafe impl RefCounted for nsISpeechRecognitionService {
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
pub trait nsISpeechRecognitionServiceCoerce {
    fn coerce_from(v: &nsISpeechRecognitionService) -> &Self;
}

impl nsISpeechRecognitionServiceCoerce for nsISpeechRecognitionService {
    #[inline]
    fn coerce_from(v: &nsISpeechRecognitionService) -> &Self {
        v
    }
}

impl nsISpeechRecognitionService {
    #[inline]
    pub fn coerce<T: nsISpeechRecognitionServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISpeechRecognitionService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISpeechRecognitionServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechRecognitionService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISpeechRecognitionServiceVTable {
    pub __base: nsISupportsVTable,

    /* void initialize (in SpeechRecognitionWeakPtr aSpeechRecognition); */
    /// Unable to call function as its signature contains a non-rust type
    pub initialize: *const ::libc::c_void,

    /* void processAudioSegment (in AudioSegmentPtr aAudioSegment, in long aSampleRate); */
    /// Unable to call function as its signature contains a non-rust type
    pub processAudioSegment: *const ::libc::c_void,

    /* void validateAndSetGrammarList (in SpeechGrammarPtr aSpeechGrammar, in nsISpeechGrammarCompilationCallback aCallback); */
    /// Unable to call function as its signature contains a non-rust type
    pub validateAndSetGrammarList: *const ::libc::c_void,

    /* void soundEnd (); */
    pub soundEnd: unsafe extern "C" fn (this: *const nsISpeechRecognitionService) -> nsresult,

    /* void abort (); */
    pub abort: unsafe extern "C" fn (this: *const nsISpeechRecognitionService) -> nsresult,

}


impl nsISpeechRecognitionService {
    /* void initialize (in SpeechRecognitionWeakPtr aSpeechRecognition); */


    /* void processAudioSegment (in AudioSegmentPtr aAudioSegment, in long aSampleRate); */


    /* void validateAndSetGrammarList (in SpeechGrammarPtr aSpeechGrammar, in nsISpeechGrammarCompilationCallback aCallback); */


    /* void soundEnd (); */
    #[inline]
    pub unsafe fn soundEnd(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).soundEnd)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void abort (); */
    #[inline]
    pub unsafe fn abort(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).abort)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


