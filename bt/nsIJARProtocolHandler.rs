//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIJARProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIJARProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Some(&[
                    /* readonly attribute nsIZipReaderCache JARCache; */
                    Method {
                        name: "get_JARCache",
                        abi: "C",
                        params: &[Param { name: "aJARCache", ty: "*mut *const nsIZipReaderCache" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

