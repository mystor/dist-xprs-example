//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPromptCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthPromptCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onAuthAvailable (in nsISupports aContext, in nsIAuthInformation aAuthInfo); */
                    Method {
                        name: "onAuthAvailable",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aAuthInfo", ty: "*const nsIAuthInformation" }],
                        ret: "nsresult",
                    },

                    /* void onAuthCancelled (in nsISupports aContext, in boolean userCancel); */
                    Method {
                        name: "onAuthCancelled",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "userCancel", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

