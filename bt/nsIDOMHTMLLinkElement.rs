//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLLinkElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLLinkElement",
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

                    /* attribute DOMString charset; */
                    Method {
                        name: "get_charset",
                        abi: "C",
                        params: &[Param { name: "aCharset", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_charset",
                        abi: "C",
                        params: &[Param { name: "aCharset", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString href; */
                    Method {
                        name: "get_href",
                        abi: "C",
                        params: &[Param { name: "aHref", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_href",
                        abi: "C",
                        params: &[Param { name: "aHref", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString hreflang; */
                    Method {
                        name: "get_hreflang",
                        abi: "C",
                        params: &[Param { name: "aHreflang", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hreflang",
                        abi: "C",
                        params: &[Param { name: "aHreflang", ty: "*const nsAString" }],
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

                    /* attribute DOMString rel; */
                    Method {
                        name: "get_rel",
                        abi: "C",
                        params: &[Param { name: "aRel", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rel",
                        abi: "C",
                        params: &[Param { name: "aRel", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString rev; */
                    Method {
                        name: "get_rev",
                        abi: "C",
                        params: &[Param { name: "aRev", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rev",
                        abi: "C",
                        params: &[Param { name: "aRev", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString target; */
                    Method {
                        name: "get_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*const nsAString" }],
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

                    ]),
        },


        ]; D}

