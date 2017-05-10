//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIJARChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIJARChannel",
            base: Some("nsIChannel"),
            methods: Some(&[
                    /* [infallible] readonly attribute boolean isUnsafe; */
                    Method {
                        name: "get_isUnsafe",
                        abi: "C",
                        params: &[Param { name: "aIsUnsafe", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile jarFile; */
                    Method {
                        name: "get_jarFile",
                        abi: "C",
                        params: &[Param { name: "aJarFile", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIZipEntry zipEntry; */
                    Method {
                        name: "get_zipEntry",
                        abi: "C",
                        params: &[Param { name: "aZipEntry", ty: "*mut *const nsIZipEntry" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

