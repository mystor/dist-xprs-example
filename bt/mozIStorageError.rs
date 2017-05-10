//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageError.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageError",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long result; */
                    Method {
                        name: "get_result",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String message; */
                    Method {
                        name: "get_message",
                        abi: "C",
                        params: &[Param { name: "aMessage", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

