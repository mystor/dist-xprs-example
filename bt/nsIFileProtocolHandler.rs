//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFileProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFileProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Some(&[
                    /* nsIURI newFileURI (in nsIFile aFile); */
                    Method {
                        name: "newFileURI",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getURLSpecFromFile (in nsIFile file); */
                    Method {
                        name: "getURLSpecFromFile",
                        abi: "C",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getURLSpecFromActualFile (in nsIFile file); */
                    Method {
                        name: "getURLSpecFromActualFile",
                        abi: "C",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getURLSpecFromDir (in nsIFile file); */
                    Method {
                        name: "getURLSpecFromDir",
                        abi: "C",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsIFile getFileFromURLSpec (in AUTF8String url); */
                    Method {
                        name: "getFileFromURLSpec",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* nsIURI readURLFile (in nsIFile file); */
                    Method {
                        name: "readURLFile",
                        abi: "C",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

