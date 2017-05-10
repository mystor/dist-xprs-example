//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIconURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMozIconURI",
            base: Some("nsIURI"),
            methods: Some(&[
                    /* attribute nsIURL iconURL; */
                    Method {
                        name: "get_iconURL",
                        abi: "C",
                        params: &[Param { name: "aIconURL", ty: "*mut *const nsIURL" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_iconURL",
                        abi: "C",
                        params: &[Param { name: "aIconURL", ty: "*const nsIURL" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long imageSize; */
                    Method {
                        name: "get_imageSize",
                        abi: "C",
                        params: &[Param { name: "aImageSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_imageSize",
                        abi: "C",
                        params: &[Param { name: "aImageSize", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString stockIcon; */
                    Method {
                        name: "get_stockIcon",
                        abi: "C",
                        params: &[Param { name: "aStockIcon", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString iconSize; */
                    Method {
                        name: "get_iconSize",
                        abi: "C",
                        params: &[Param { name: "aIconSize", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString iconState; */
                    Method {
                        name: "get_iconState",
                        abi: "C",
                        params: &[Param { name: "aIconState", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString contentType; */
                    Method {
                        name: "get_contentType",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentType",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString fileExtension; */
                    Method {
                        name: "get_fileExtension",
                        abi: "C",
                        params: &[Param { name: "aFileExtension", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

