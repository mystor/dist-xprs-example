//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFileStreams.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFileInputStream",
            base: Some("nsIInputStream"),
            methods: Some(&[
                    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "ioFlags", ty: "libc::int32_t" }, Param { name: "perm", ty: "libc::int32_t" }, Param { name: "behaviorFlags", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFileOutputStream",
            base: Some("nsIOutputStream"),
            methods: Some(&[
                    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "ioFlags", ty: "libc::int32_t" }, Param { name: "perm", ty: "libc::int32_t" }, Param { name: "behaviorFlags", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void preallocate (in long long length); */
                    Method {
                        name: "preallocate",
                        abi: "C",
                        params: &[Param { name: "length", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFileStream",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "ioFlags", ty: "libc::int32_t" }, Param { name: "perm", ty: "libc::int32_t" }, Param { name: "behaviorFlags", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFileMetadata",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

