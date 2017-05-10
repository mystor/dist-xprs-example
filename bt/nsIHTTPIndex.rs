//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTTPIndex.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTTPIndex",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute string BaseURL; */
                    Method {
                        name: "get_BaseURL",
                        abi: "C",
                        params: &[Param { name: "aBaseURL", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIRDFDataSource DataSource; */
                    Method {
                        name: "get_DataSource",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*mut *const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },

                    /* attribute string encoding; */
                    Method {
                        name: "get_encoding",
                        abi: "C",
                        params: &[Param { name: "aEncoding", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_encoding",
                        abi: "C",
                        params: &[Param { name: "aEncoding", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

