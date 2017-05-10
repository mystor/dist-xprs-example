//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStorageStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStorageStream",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in uint32_t segmentSize, in uint32_t maxSize); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "segmentSize", ty: "uint32_t" }, Param { name: "maxSize", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIOutputStream getOutputStream (in int32_t startPosition); */
                    Method {
                        name: "getOutputStream",
                        abi: "C",
                        params: &[Param { name: "startPosition", ty: "int32_t" }, Param { name: "_retval", ty: "*mut *const nsIOutputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream newInputStream (in int32_t startPosition); */
                    Method {
                        name: "newInputStream",
                        abi: "C",
                        params: &[Param { name: "startPosition", ty: "int32_t" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* attribute uint32_t length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean writeInProgress; */
                    Method {
                        name: "get_writeInProgress",
                        abi: "C",
                        params: &[Param { name: "aWriteInProgress", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

