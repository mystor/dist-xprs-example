//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISeekableStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISeekableStream",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void seek (in long whence, in long long offset); */
                    Method {
                        name: "seek",
                        abi: "C",
                        params: &[Param { name: "whence", ty: "libc::int32_t" }, Param { name: "offset", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* long long tell (); */
                    Method {
                        name: "tell",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* void setEOF (); */
                    Method {
                        name: "setEOF",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

