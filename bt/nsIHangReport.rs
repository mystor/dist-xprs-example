//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHangReport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHangReport",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long hangType; */
                    Method {
                        name: "get_hangType",
                        abi: "C",
                        params: &[Param { name: "aHangType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement scriptBrowser; */
                    Method {
                        name: "get_scriptBrowser",
                        abi: "C",
                        params: &[Param { name: "aScriptBrowser", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString scriptFileName; */
                    Method {
                        name: "get_scriptFileName",
                        abi: "C",
                        params: &[Param { name: "aScriptFileName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString pluginName; */
                    Method {
                        name: "get_pluginName",
                        abi: "C",
                        params: &[Param { name: "aPluginName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* void userCanceled (); */
                    Method {
                        name: "userCanceled",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void terminateScript (); */
                    Method {
                        name: "terminateScript",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void terminatePlugin (); */
                    Method {
                        name: "terminatePlugin",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void beginStartingDebugger (); */
                    Method {
                        name: "beginStartingDebugger",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void endStartingDebugger (); */
                    Method {
                        name: "endStartingDebugger",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* bool isReportForBrowser (in nsIFrameLoader aFrameLoader); */
                    Method {
                        name: "isReportForBrowser",
                        abi: "C",
                        params: &[Param { name: "aFrameLoader", ty: "*const nsIFrameLoader" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

