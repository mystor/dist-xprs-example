//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPromptFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPromptFactory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getPrompt (in mozIDOMWindowProxy aParent, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "getPrompt",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

