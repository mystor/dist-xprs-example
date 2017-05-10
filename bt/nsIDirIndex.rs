//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDirIndex.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDirIndex",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute unsigned long type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute string contentType; */
                    Method {
                        name: "get_contentType",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentType",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* attribute string location; */
                    Method {
                        name: "get_location",
                        abi: "C",
                        params: &[Param { name: "aLocation", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_location",
                        abi: "C",
                        params: &[Param { name: "aLocation", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* attribute wstring description; */
                    Method {
                        name: "get_description",
                        abi: "C",
                        params: &[Param { name: "aDescription", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_description",
                        abi: "C",
                        params: &[Param { name: "aDescription", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long long size; */
                    Method {
                        name: "get_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute PRTime lastModified; */
                    Method {
                        name: "get_lastModified",
                        abi: "C",
                        params: &[Param { name: "aLastModified", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_lastModified",
                        abi: "C",
                        params: &[Param { name: "aLastModified", ty: "PRTime" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

