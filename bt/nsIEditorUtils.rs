//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditorBlobListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onResult (in ACString aResult); */
                    Method {
                        name: "onResult",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onError (in AString aErrorName); */
                    Method {
                        name: "onError",
                        abi: "C",
                        params: &[Param { name: "aErrorName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIEditorUtils",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void slurpBlob (in nsIDOMBlob aBlob, in mozIDOMWindowProxy aScope, in nsIEditorBlobListener aListener); */
                    Method {
                        name: "slurpBlob",
                        abi: "C",
                        params: &[Param { name: "aBlob", ty: "*const nsIDOMBlob" }, Param { name: "aScope", ty: "*const mozIDOMWindowProxy" }, Param { name: "aListener", ty: "*const nsIEditorBlobListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

