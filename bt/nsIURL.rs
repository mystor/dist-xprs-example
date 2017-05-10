//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURL.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURL",
            base: Some("nsIURI"),
            methods: Some(&[
                    /* attribute AUTF8String directory; */
                    Method {
                        name: "get_directory",
                        abi: "C",
                        params: &[Param { name: "aDirectory", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_directory",
                        abi: "C",
                        params: &[Param { name: "aDirectory", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String fileName; */
                    Method {
                        name: "get_fileName",
                        abi: "C",
                        params: &[Param { name: "aFileName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_fileName",
                        abi: "C",
                        params: &[Param { name: "aFileName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String fileBaseName; */
                    Method {
                        name: "get_fileBaseName",
                        abi: "C",
                        params: &[Param { name: "aFileBaseName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_fileBaseName",
                        abi: "C",
                        params: &[Param { name: "aFileBaseName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String fileExtension; */
                    Method {
                        name: "get_fileExtension",
                        abi: "C",
                        params: &[Param { name: "aFileExtension", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_fileExtension",
                        abi: "C",
                        params: &[Param { name: "aFileExtension", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getCommonBaseSpec (in nsIURI aURIToCompare); */
                    Method {
                        name: "getCommonBaseSpec",
                        abi: "C",
                        params: &[Param { name: "aURIToCompare", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getRelativeSpec (in nsIURI aURIToCompare); */
                    Method {
                        name: "getRelativeSpec",
                        abi: "C",
                        params: &[Param { name: "aURIToCompare", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

