//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncOutputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncOutputStream",
            base: Some("nsIOutputStream"),
            methods: Some(&[
                    /* void closeWithStatus (in nsresult reason); */
                    Method {
                        name: "closeWithStatus",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void asyncWait (in nsIOutputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
                    Method {
                        name: "asyncWait",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIOutputStreamCallback" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aRequestedCount", ty: "libc::uint32_t" }, Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIOutputStreamCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onOutputStreamReady (in nsIAsyncOutputStream aStream); */
                    Method {
                        name: "onOutputStreamReady",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIAsyncOutputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

