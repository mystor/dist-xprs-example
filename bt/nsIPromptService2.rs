//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPromptService2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPromptService2",
            base: Some("nsIPromptService"),
            methods: Some(&[
                    /* boolean promptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
                    Method {
                        name: "promptAuth",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "level", ty: "uint32_t" }, Param { name: "authInfo", ty: "*const nsIAuthInformation" }, Param { name: "checkboxLabel", ty: "*const libc::int16_t" }, Param { name: "checkValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsICancelable asyncPromptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in nsIAuthPromptCallback aCallback, in nsISupports aContext, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
                    Method {
                        name: "asyncPromptAuth",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aCallback", ty: "*const nsIAuthPromptCallback" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "level", ty: "uint32_t" }, Param { name: "authInfo", ty: "*const nsIAuthInformation" }, Param { name: "checkboxLabel", ty: "*const libc::int16_t" }, Param { name: "checkValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

