//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalProtocolService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExternalProtocolService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean externalProtocolHandlerExists (in string aProtocolScheme); */
                    Method {
                        name: "externalProtocolHandlerExists",
                        abi: "C",
                        params: &[Param { name: "aProtocolScheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isExposedProtocol (in string aProtocolScheme); */
                    Method {
                        name: "isExposedProtocol",
                        abi: "C",
                        params: &[Param { name: "aProtocolScheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIHandlerInfo getProtocolHandlerInfo (in ACString aProtocolScheme); */
                    Method {
                        name: "getProtocolHandlerInfo",
                        abi: "C",
                        params: &[Param { name: "aProtocolScheme", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIHandlerInfo" }],
                        ret: "nsresult",
                    },

                    /* nsIHandlerInfo getProtocolHandlerInfoFromOS (in ACString aProtocolScheme, out boolean aFound); */
                    Method {
                        name: "getProtocolHandlerInfoFromOS",
                        abi: "C",
                        params: &[Param { name: "aProtocolScheme", ty: "*const nsACString" }, Param { name: "aFound", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const nsIHandlerInfo" }],
                        ret: "nsresult",
                    },

                    /* void setProtocolHandlerDefaults (in nsIHandlerInfo aHandlerInfo, in boolean aOSHandlerExists); */
                    Method {
                        name: "setProtocolHandlerDefaults",
                        abi: "C",
                        params: &[Param { name: "aHandlerInfo", ty: "*const nsIHandlerInfo" }, Param { name: "aOSHandlerExists", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [deprecated] void loadUrl (in nsIURI aURL); */
                    Method {
                        name: "loadUrl",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void loadURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "loadURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* AString getApplicationDescription (in AUTF8String aScheme); */
                    Method {
                        name: "getApplicationDescription",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

