//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIClassifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIClassifierCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onClassifyComplete (in nsresult aErrorCode, in ACString aList, in ACString aProvider, in ACString aPrefix); */
                    Method {
                        name: "onClassifyComplete",
                        abi: "C",
                        params: &[Param { name: "aErrorCode", ty: "nsresult" }, Param { name: "aList", ty: "*const nsACString" }, Param { name: "aProvider", ty: "*const nsACString" }, Param { name: "aPrefix", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIURIClassifier",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

