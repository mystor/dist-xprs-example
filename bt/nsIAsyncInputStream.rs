//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncInputStream",
            base: Some("nsIInputStream"),
            methods: Some(&[
                    /* void closeWithStatus (in nsresult aStatus); */
                    Method {
                        name: "closeWithStatus",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void asyncWait (in nsIInputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
                    Method {
                        name: "asyncWait",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIInputStreamCallback" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aRequestedCount", ty: "libc::uint32_t" }, Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIInputStreamCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onInputStreamReady (in nsIAsyncInputStream aStream); */
                    Method {
                        name: "onInputStreamReady",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIAsyncInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

