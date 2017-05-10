//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableInputStream",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void close (); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void init (in nsIInputStream aInputStream); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aInputStream", ty: "*const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* unsigned long long available (); */
                    Method {
                        name: "available",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* string read (in unsigned long aCount); */
                    Method {
                        name: "read",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* ACString readBytes (in unsigned long aCount); */
                    Method {
                        name: "readBytes",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

