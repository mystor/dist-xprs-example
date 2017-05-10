//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPromptProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthPromptProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getAuthPrompt (in uint32_t aPromptReason, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "getAuthPrompt",
                        abi: "C",
                        params: &[Param { name: "aPromptReason", ty: "uint32_t" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

