//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHstsPrimingCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHstsPrimingCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use,noscript,nostdcall] void onHSTSPrimingSucceeded (in bool aCached); */
                    Method {
                        name: "onHSTSPrimingSucceeded",
                        abi: "C",
                        params: &[Param { name: "aCached", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use,noscript,nostdcall] void onHSTSPrimingFailed (in nsresult aError, in bool aCached); */
                    Method {
                        name: "onHSTSPrimingFailed",
                        abi: "C",
                        params: &[Param { name: "aError", ty: "nsresult" }, Param { name: "aCached", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

