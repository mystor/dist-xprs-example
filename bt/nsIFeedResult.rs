//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedResult.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean bozo; */
                    Method {
                        name: "get_bozo",
                        abi: "C",
                        params: &[Param { name: "aBozo", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_bozo",
                        abi: "C",
                        params: &[Param { name: "aBozo", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFeedContainer doc; */
                    Method {
                        name: "get_doc",
                        abi: "C",
                        params: &[Param { name: "aDoc", ty: "*mut *const nsIFeedContainer" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_doc",
                        abi: "C",
                        params: &[Param { name: "aDoc", ty: "*const nsIFeedContainer" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURI uri; */
                    Method {
                        name: "get_uri",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_uri",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute AString version; */
                    Method {
                        name: "get_version",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_version",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURI stylesheet; */
                    Method {
                        name: "get_stylesheet",
                        abi: "C",
                        params: &[Param { name: "aStylesheet", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_stylesheet",
                        abi: "C",
                        params: &[Param { name: "aStylesheet", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIProperties headers; */
                    Method {
                        name: "get_headers",
                        abi: "C",
                        params: &[Param { name: "aHeaders", ty: "*mut *const nsIProperties" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_headers",
                        abi: "C",
                        params: &[Param { name: "aHeaders", ty: "*const nsIProperties" }],
                        ret: "nsresult",
                    },

                    /* void registerExtensionPrefix (in AString aNamespace, in AString aPrefix); */
                    Method {
                        name: "registerExtensionPrefix",
                        abi: "C",
                        params: &[Param { name: "aNamespace", ty: "*const nsAString" }, Param { name: "aPrefix", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

