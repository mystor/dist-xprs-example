//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINativeOSFileInternals.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINativeOSFileResult",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsINativeOSFileSuccessCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void complete (in nsINativeOSFileResult result); */
                    Method {
                        name: "complete",
                        abi: "C",
                        params: &[Param { name: "result", ty: "*const nsINativeOSFileResult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINativeOSFileErrorCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void complete (in ACString operation, in long OSstatus); */
                    Method {
                        name: "complete",
                        abi: "C",
                        params: &[Param { name: "operation", ty: "*const nsACString" }, Param { name: "OSstatus", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINativeOSFileInternalsService",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

