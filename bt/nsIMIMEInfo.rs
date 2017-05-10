//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMIMEInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHandlerInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString description; */
                    Method {
                        name: "get_description",
                        abi: "C",
                        params: &[Param { name: "aDescription", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_description",
                        abi: "C",
                        params: &[Param { name: "aDescription", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIHandlerApp preferredApplicationHandler; */
                    Method {
                        name: "get_preferredApplicationHandler",
                        abi: "C",
                        params: &[Param { name: "aPreferredApplicationHandler", ty: "*mut *const nsIHandlerApp" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_preferredApplicationHandler",
                        abi: "C",
                        params: &[Param { name: "aPreferredApplicationHandler", ty: "*const nsIHandlerApp" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIMutableArray possibleApplicationHandlers; */
                    Method {
                        name: "get_possibleApplicationHandlers",
                        abi: "C",
                        params: &[Param { name: "aPossibleApplicationHandlers", ty: "*mut *const nsIMutableArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean hasDefaultHandler; */
                    Method {
                        name: "get_hasDefaultHandler",
                        abi: "C",
                        params: &[Param { name: "aHasDefaultHandler", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString defaultDescription; */
                    Method {
                        name: "get_defaultDescription",
                        abi: "C",
                        params: &[Param { name: "aDefaultDescription", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void launchWithURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "launchWithURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* attribute nsHandlerInfoAction preferredAction; */
                    Method {
                        name: "get_preferredAction",
                        abi: "C",
                        params: &[Param { name: "aPreferredAction", ty: "*mut nsHandlerInfoAction" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_preferredAction",
                        abi: "C",
                        params: &[Param { name: "aPreferredAction", ty: "nsHandlerInfoAction" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean alwaysAskBeforeHandling; */
                    Method {
                        name: "get_alwaysAskBeforeHandling",
                        abi: "C",
                        params: &[Param { name: "aAlwaysAskBeforeHandling", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_alwaysAskBeforeHandling",
                        abi: "C",
                        params: &[Param { name: "aAlwaysAskBeforeHandling", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIMIMEInfo",
            base: Some("nsIHandlerInfo"),
            methods: Some(&[
                    /* nsIUTF8StringEnumerator getFileExtensions (); */
                    Method {
                        name: "getFileExtensions",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIUTF8StringEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void setFileExtensions (in AUTF8String aExtensions); */
                    Method {
                        name: "setFileExtensions",
                        abi: "C",
                        params: &[Param { name: "aExtensions", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean extensionExists (in AUTF8String aExtension); */
                    Method {
                        name: "extensionExists",
                        abi: "C",
                        params: &[Param { name: "aExtension", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void appendExtension (in AUTF8String aExtension); */
                    Method {
                        name: "appendExtension",
                        abi: "C",
                        params: &[Param { name: "aExtension", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String primaryExtension; */
                    Method {
                        name: "get_primaryExtension",
                        abi: "C",
                        params: &[Param { name: "aPrimaryExtension", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_primaryExtension",
                        abi: "C",
                        params: &[Param { name: "aPrimaryExtension", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString MIMEType; */
                    Method {
                        name: "get_MIMEType",
                        abi: "C",
                        params: &[Param { name: "aMIMEType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean equals (in nsIMIMEInfo aMIMEInfo); */
                    Method {
                        name: "equals",
                        abi: "C",
                        params: &[Param { name: "aMIMEInfo", ty: "*const nsIMIMEInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray possibleLocalHandlers; */
                    Method {
                        name: "get_possibleLocalHandlers",
                        abi: "C",
                        params: &[Param { name: "aPossibleLocalHandlers", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void launchWithFile (in nsIFile aFile); */
                    Method {
                        name: "launchWithFile",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHandlerApp",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString detailedDescription; */
                    Method {
                        name: "get_detailedDescription",
                        abi: "C",
                        params: &[Param { name: "aDetailedDescription", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_detailedDescription",
                        abi: "C",
                        params: &[Param { name: "aDetailedDescription", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean equals (in nsIHandlerApp aHandlerApp); */
                    Method {
                        name: "equals",
                        abi: "C",
                        params: &[Param { name: "aHandlerApp", ty: "*const nsIHandlerApp" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void launchWithURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "launchWithURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsILocalHandlerApp",
            base: Some("nsIHandlerApp"),
            methods: Some(&[
                    /* attribute nsIFile executable; */
                    Method {
                        name: "get_executable",
                        abi: "C",
                        params: &[Param { name: "aExecutable", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_executable",
                        abi: "C",
                        params: &[Param { name: "aExecutable", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long parameterCount; */
                    Method {
                        name: "get_parameterCount",
                        abi: "C",
                        params: &[Param { name: "aParameterCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void clearParameters (); */
                    Method {
                        name: "clearParameters",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void appendParameter (in AString param); */
                    Method {
                        name: "appendParameter",
                        abi: "C",
                        params: &[Param { name: "param", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getParameter (in unsigned long parameterIndex); */
                    Method {
                        name: "getParameter",
                        abi: "C",
                        params: &[Param { name: "parameterIndex", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean parameterExists (in AString param); */
                    Method {
                        name: "parameterExists",
                        abi: "C",
                        params: &[Param { name: "param", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebHandlerApp",
            base: Some("nsIHandlerApp"),
            methods: Some(&[
                    /* attribute AUTF8String uriTemplate; */
                    Method {
                        name: "get_uriTemplate",
                        abi: "C",
                        params: &[Param { name: "aUriTemplate", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_uriTemplate",
                        abi: "C",
                        params: &[Param { name: "aUriTemplate", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDBusHandlerApp",
            base: Some("nsIHandlerApp"),
            methods: Some(&[
                    /* attribute AUTF8String service; */
                    Method {
                        name: "get_service",
                        abi: "C",
                        params: &[Param { name: "aService", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_service",
                        abi: "C",
                        params: &[Param { name: "aService", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String objectPath; */
                    Method {
                        name: "get_objectPath",
                        abi: "C",
                        params: &[Param { name: "aObjectPath", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_objectPath",
                        abi: "C",
                        params: &[Param { name: "aObjectPath", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String dBusInterface; */
                    Method {
                        name: "get_dBusInterface",
                        abi: "C",
                        params: &[Param { name: "aDBusInterface", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_dBusInterface",
                        abi: "C",
                        params: &[Param { name: "aDBusInterface", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String method; */
                    Method {
                        name: "get_method",
                        abi: "C",
                        params: &[Param { name: "aMethod", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_method",
                        abi: "C",
                        params: &[Param { name: "aMethod", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

