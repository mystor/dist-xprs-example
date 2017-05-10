//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageProgressHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageProgressHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean onProgress (in mozIStorageConnection aConnection); */
                    Method {
                        name: "onProgress",
                        abi: "C",
                        params: &[Param { name: "aConnection", ty: "*const mozIStorageConnection" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

