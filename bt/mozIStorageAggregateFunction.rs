//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageAggregateFunction.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageAggregateFunction",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onStep (in mozIStorageValueArray aFunctionArguments); */
                    Method {
                        name: "onStep",
                        abi: "C",
                        params: &[Param { name: "aFunctionArguments", ty: "*const mozIStorageValueArray" }],
                        ret: "nsresult",
                    },

                    /* nsIVariant onFinal (); */
                    Method {
                        name: "onFinal",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

