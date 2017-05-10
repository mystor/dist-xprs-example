//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintProgressParams.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrintProgressParams",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute wstring docTitle; */
                    Method {
                        name: "get_docTitle",
                        abi: "C",
                        params: &[Param { name: "aDocTitle", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_docTitle",
                        abi: "C",
                        params: &[Param { name: "aDocTitle", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute wstring docURL; */
                    Method {
                        name: "get_docURL",
                        abi: "C",
                        params: &[Param { name: "aDocURL", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_docURL",
                        abi: "C",
                        params: &[Param { name: "aDocURL", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

