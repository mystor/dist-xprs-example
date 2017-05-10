//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIShellService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIShellService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean isDefaultBrowser (in boolean aStartupCheck, [optional] in boolean aForAllTypes); */
                    Method {
                        name: "isDefaultBrowser",
                        abi: "C",
                        params: &[Param { name: "aStartupCheck", ty: "bool" }, Param { name: "aForAllTypes", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setDefaultBrowser (in boolean aClaimAllTypes, in boolean aForAllUsers); */
                    Method {
                        name: "setDefaultBrowser",
                        abi: "C",
                        params: &[Param { name: "aClaimAllTypes", ty: "bool" }, Param { name: "aForAllUsers", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setDesktopBackground (in nsIDOMElement aElement, in long aPosition); */
                    Method {
                        name: "setDesktopBackground",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "aPosition", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void openApplication (in long aApplication); */
                    Method {
                        name: "openApplication",
                        abi: "C",
                        params: &[Param { name: "aApplication", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long desktopBackgroundColor; */
                    Method {
                        name: "get_desktopBackgroundColor",
                        abi: "C",
                        params: &[Param { name: "aDesktopBackgroundColor", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_desktopBackgroundColor",
                        abi: "C",
                        params: &[Param { name: "aDesktopBackgroundColor", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void openApplicationWithURI (in nsIFile aApplication, in ACString aURI); */
                    Method {
                        name: "openApplicationWithURI",
                        abi: "C",
                        params: &[Param { name: "aApplication", ty: "*const nsIFile" }, Param { name: "aURI", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile defaultFeedReader; */
                    Method {
                        name: "get_defaultFeedReader",
                        abi: "C",
                        params: &[Param { name: "aDefaultFeedReader", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

