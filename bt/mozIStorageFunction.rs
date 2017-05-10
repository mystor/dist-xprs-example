//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageFunction.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageFunction",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIVariant onFunctionCall (in mozIStorageValueArray aFunctionArguments); */
                    Method {
                        name: "onFunctionCall",
                        abi: "C",
                        params: &[Param { name: "aFunctionArguments", ty: "*const mozIStorageValueArray" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

