//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLStyleElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLStyleElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [binaryname(MozDisabled)] attribute boolean disabled; */
                    Method {
                        name: "get_MozDisabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_MozDisabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString media; */
                    Method {
                        name: "get_media",
                        abi: "C",
                        params: &[Param { name: "aMedia", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_media",
                        abi: "C",
                        params: &[Param { name: "aMedia", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean scoped; */
                    Method {
                        name: "get_scoped",
                        abi: "C",
                        params: &[Param { name: "aScoped", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_scoped",
                        abi: "C",
                        params: &[Param { name: "aScoped", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

