//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMultiplexInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMultiplexInputStream",
            base: Some("nsIInputStream"),
            methods: Some(&[
                    /* readonly attribute unsigned long count; */
                    Method {
                        name: "get_count",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void appendStream (in nsIInputStream stream); */
                    Method {
                        name: "appendStream",
                        abi: "C",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* void insertStream (in nsIInputStream stream, in unsigned long index); */
                    Method {
                        name: "insertStream",
                        abi: "C",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }, Param { name: "index", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void removeStream (in unsigned long index); */
                    Method {
                        name: "removeStream",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream getStream (in unsigned long index); */
                    Method {
                        name: "getStream",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

