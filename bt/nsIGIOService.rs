//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGIOService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGIOMimeApp",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String command; */
                    Method {
                        name: "get_command",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long expectsURIs; */
                    Method {
                        name: "get_expectsURIs",
                        abi: "C",
                        params: &[Param { name: "aExpectsURIs", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIUTF8StringEnumerator supportedURISchemes; */
                    Method {
                        name: "get_supportedURISchemes",
                        abi: "C",
                        params: &[Param { name: "aSupportedURISchemes", ty: "*mut *const nsIUTF8StringEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void launch (in AUTF8String uri); */
                    Method {
                        name: "launch",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setAsDefaultForMimeType (in AUTF8String mimeType); */
                    Method {
                        name: "setAsDefaultForMimeType",
                        abi: "C",
                        params: &[Param { name: "mimeType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setAsDefaultForFileExtensions (in AUTF8String extensions); */
                    Method {
                        name: "setAsDefaultForFileExtensions",
                        abi: "C",
                        params: &[Param { name: "extensions", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setAsDefaultForURIScheme (in AUTF8String uriScheme); */
                    Method {
                        name: "setAsDefaultForURIScheme",
                        abi: "C",
                        params: &[Param { name: "uriScheme", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIGIOService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AUTF8String getMimeTypeFromExtension (in AUTF8String extension); */
                    Method {
                        name: "getMimeTypeFromExtension",
                        abi: "C",
                        params: &[Param { name: "extension", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsIGIOMimeApp getAppForURIScheme (in AUTF8String aURIScheme); */
                    Method {
                        name: "getAppForURIScheme",
                        abi: "C",
                        params: &[Param { name: "aURIScheme", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIGIOMimeApp" }],
                        ret: "nsresult",
                    },

                    /* nsIGIOMimeApp getAppForMimeType (in AUTF8String mimeType); */
                    Method {
                        name: "getAppForMimeType",
                        abi: "C",
                        params: &[Param { name: "mimeType", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIGIOMimeApp" }],
                        ret: "nsresult",
                    },

                    /* nsIGIOMimeApp createAppFromCommand (in AUTF8String cmd, in AUTF8String appName); */
                    Method {
                        name: "createAppFromCommand",
                        abi: "C",
                        params: &[Param { name: "cmd", ty: "*const nsACString" }, Param { name: "appName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIGIOMimeApp" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getDescriptionForMimeType (in AUTF8String mimeType); */
                    Method {
                        name: "getDescriptionForMimeType",
                        abi: "C",
                        params: &[Param { name: "mimeType", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* void showURI (in nsIURI uri); */
                    Method {
                        name: "showURI",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void showURIForInput (in ACString uri); */
                    Method {
                        name: "showURIForInput",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void orgFreedesktopFileManager1ShowItems (in ACString path); */
                    Method {
                        name: "orgFreedesktopFileManager1ShowItems",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

