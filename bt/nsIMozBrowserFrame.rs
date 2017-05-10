//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMozBrowserFrame.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMozBrowserFrame",
            base: Some("nsIDOMMozBrowserFrame"),
            methods: Some(&[
                    /* [infallible] readonly attribute boolean reallyIsBrowser; */
                    Method {
                        name: "get_reallyIsBrowser",
                        abi: "C",
                        params: &[Param { name: "aReallyIsBrowser", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [infallible] readonly attribute boolean isolated; */
                    Method {
                        name: "get_isolated",
                        abi: "C",
                        params: &[Param { name: "aIsolated", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void disallowCreateFrameLoader (); */
                    Method {
                        name: "disallowCreateFrameLoader",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void allowCreateFrameLoader (); */
                    Method {
                        name: "allowCreateFrameLoader",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void createRemoteFrameLoader (in nsITabParent aTabParent); */
                    Method {
                        name: "createRemoteFrameLoader",
                        abi: "C",
                        params: &[Param { name: "aTabParent", ty: "*const nsITabParent" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void initializeBrowserAPI (); */
                    Method {
                        name: "initializeBrowserAPI",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [noscript] void destroyBrowserFrameScripts (); */
                    Method {
                        name: "destroyBrowserFrameScripts",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

