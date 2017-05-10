//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMTimeRanges.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMTimeRanges",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* double start (in unsigned long index); */
                    Method {
                        name: "start",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* double end (in unsigned long index); */
                    Method {
                        name: "end",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

