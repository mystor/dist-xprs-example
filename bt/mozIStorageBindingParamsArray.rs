//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageBindingParamsArray.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageBindingParamsArray",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* mozIStorageBindingParams newBindingParams (); */
                    Method {
                        name: "newBindingParams",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const mozIStorageBindingParams" }],
                        ret: "nsresult",
                    },

                    /* void addParams (in mozIStorageBindingParams aParameters); */
                    Method {
                        name: "addParams",
                        abi: "C",
                        params: &[Param { name: "aParameters", ty: "*const mozIStorageBindingParams" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

