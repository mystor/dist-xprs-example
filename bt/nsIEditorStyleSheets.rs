//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorStyleSheets.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditorStyleSheets",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void replaceStyleSheet (in AString aURL); */
                    Method {
                        name: "replaceStyleSheet",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void addStyleSheet (in AString aURL); */
                    Method {
                        name: "addStyleSheet",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void replaceOverrideStyleSheet (in AString aURL); */
                    Method {
                        name: "replaceOverrideStyleSheet",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void addOverrideStyleSheet (in AString aURL); */
                    Method {
                        name: "addOverrideStyleSheet",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void removeStyleSheet (in AString aURL); */
                    Method {
                        name: "removeStyleSheet",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void removeOverrideStyleSheet (in AString aURL); */
                    Method {
                        name: "removeOverrideStyleSheet",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void enableStyleSheet (in AString aURL, in boolean aEnable); */
                    Method {
                        name: "enableStyleSheet",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsAString" }, Param { name: "aEnable", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

