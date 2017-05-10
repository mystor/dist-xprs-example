//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMIMEService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMIMEService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIMIMEInfo getFromTypeAndExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
                    Method {
                        name: "getFromTypeAndExtension",
                        abi: "C",
                        params: &[Param { name: "aMIMEType", ty: "*const nsACString" }, Param { name: "aFileExt", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIMIMEInfo" }],
                        ret: "nsresult",
                    },

                    /* ACString getTypeFromExtension (in AUTF8String aFileExt); */
                    Method {
                        name: "getTypeFromExtension",
                        abi: "C",
                        params: &[Param { name: "aFileExt", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString getTypeFromURI (in nsIURI aURI); */
                    Method {
                        name: "getTypeFromURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString getTypeFromFile (in nsIFile aFile); */
                    Method {
                        name: "getTypeFromFile",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getPrimaryExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
                    Method {
                        name: "getPrimaryExtension",
                        abi: "C",
                        params: &[Param { name: "aMIMEType", ty: "*const nsACString" }, Param { name: "aFileExt", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

