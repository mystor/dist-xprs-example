//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPrompt2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthPrompt2",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean promptAuth (in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo); */
                    Method {
                        name: "promptAuth",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "level", ty: "uint32_t" }, Param { name: "authInfo", ty: "*const nsIAuthInformation" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsICancelable asyncPromptAuth (in nsIChannel aChannel, in nsIAuthPromptCallback aCallback, in nsISupports aContext, in uint32_t level, in nsIAuthInformation authInfo); */
                    Method {
                        name: "asyncPromptAuth",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aCallback", ty: "*const nsIAuthPromptCallback" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "level", ty: "uint32_t" }, Param { name: "authInfo", ty: "*const nsIAuthInformation" }, Param { name: "_retval", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

