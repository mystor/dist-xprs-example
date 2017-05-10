//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChromeRegistry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIChromeRegistry",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIURI convertChromeURL (in nsIURI aChromeURL); */
                    Method {
                        name: "convertChromeURL",
                        abi: "C",
                        params: &[Param { name: "aChromeURL", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void checkForNewChrome (); */
                    Method {
                        name: "checkForNewChrome",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [notxpcom] boolean wrappersEnabled (in nsIURI aURI); */
                    Method {
                        name: "wrappersEnabled",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }],
                        ret: "bool",
                    },

                    ]),
        },


        Interface {
            name: "nsIXULChromeRegistry",
            base: Some("nsIChromeRegistry"),
            methods: Some(&[
                    /* void reloadChrome (); */
                    Method {
                        name: "reloadChrome",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* ACString getSelectedLocale (in ACString packageName, [optional] in boolean asBCP47); */
                    Method {
                        name: "getSelectedLocale",
                        abi: "C",
                        params: &[Param { name: "packageName", ty: "*const nsACString" }, Param { name: "asBCP47", ty: "bool" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean isLocaleRTL (in ACString package); */
                    Method {
                        name: "isLocaleRTL",
                        abi: "C",
                        params: &[Param { name: "package", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void refreshSkins (); */
                    Method {
                        name: "refreshSkins",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean allowScriptsForPackage (in nsIURI url); */
                    Method {
                        name: "allowScriptsForPackage",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean allowContentToAccess (in nsIURI url); */
                    Method {
                        name: "allowContentToAccess",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canLoadURLRemotely (in nsIURI url); */
                    Method {
                        name: "canLoadURLRemotely",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean mustLoadURLRemotely (in nsIURI url); */
                    Method {
                        name: "mustLoadURLRemotely",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

