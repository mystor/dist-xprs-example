//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIPromptService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsPIPromptService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void doDialog (in nsIDOMWindow aParent, in nsIDialogParamBlock aParamBlock, in string aChromeURL); */
                    Method {
                        name: "doDialog",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const nsIDOMWindow" }, Param { name: "aParamBlock", ty: "*const nsIDialogParamBlock" }, Param { name: "aChromeURL", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

