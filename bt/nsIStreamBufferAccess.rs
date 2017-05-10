//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamBufferAccess.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamBufferAccess",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [noscript,notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
                    Method {
                        name: "getBuffer",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "uint32_t" }, Param { name: "aAlignMask", ty: "uint32_t" }],
                        ret: "*const u8",
                    },

                    /* [noscript,notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
                    Method {
                        name: "putBuffer",
                        abi: "C",
                        params: &[Param { name: "aBuffer", ty: "*const u8" }, Param { name: "aLength", ty: "uint32_t" }],
                        ret: "libc::c_void",
                    },

                    /* void disableBuffering (); */
                    Method {
                        name: "disableBuffering",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void enableBuffering (); */
                    Method {
                        name: "enableBuffering",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISupports unbufferedStream; */
                    Method {
                        name: "get_unbufferedStream",
                        abi: "C",
                        params: &[Param { name: "aUnbufferedStream", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

