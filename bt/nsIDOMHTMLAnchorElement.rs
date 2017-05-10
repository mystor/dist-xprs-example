//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLAnchorElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLAnchorElement",
            base: Some("nsISupports"),
            methods: Some(&[
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

                    /* attribute DOMString ping; */
                    Method {
                        name: "get_ping",
                        abi: "C",
                        params: &[Param { name: "aPing", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_ping",
                        abi: "C",
                        params: &[Param { name: "aPing", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString download; */
                    Method {
                        name: "get_download",
                        abi: "C",
                        params: &[Param { name: "aDownload", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_download",
                        abi: "C",
                        params: &[Param { name: "aDownload", ty: "*const nsAString" }],
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

                    /* [Null(Stringify)] attribute DOMString text; */
                    Method {
                        name: "get_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString protocol; */
                    Method {
                        name: "get_protocol",
                        abi: "C",
                        params: &[Param { name: "aProtocol", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_protocol",
                        abi: "C",
                        params: &[Param { name: "aProtocol", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString host; */
                    Method {
                        name: "get_host",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_host",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString hostname; */
                    Method {
                        name: "get_hostname",
                        abi: "C",
                        params: &[Param { name: "aHostname", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hostname",
                        abi: "C",
                        params: &[Param { name: "aHostname", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString port; */
                    Method {
                        name: "get_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString pathname; */
                    Method {
                        name: "get_pathname",
                        abi: "C",
                        params: &[Param { name: "aPathname", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_pathname",
                        abi: "C",
                        params: &[Param { name: "aPathname", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString search; */
                    Method {
                        name: "get_search",
                        abi: "C",
                        params: &[Param { name: "aSearch", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_search",
                        abi: "C",
                        params: &[Param { name: "aSearch", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString hash; */
                    Method {
                        name: "get_hash",
                        abi: "C",
                        params: &[Param { name: "aHash", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hash",
                        abi: "C",
                        params: &[Param { name: "aHash", ty: "*const nsAString" }],
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

                    /* attribute DOMString coords; */
                    Method {
                        name: "get_coords",
                        abi: "C",
                        params: &[Param { name: "aCoords", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_coords",
                        abi: "C",
                        params: &[Param { name: "aCoords", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString name; */
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

                    /* attribute DOMString shape; */
                    Method {
                        name: "get_shape",
                        abi: "C",
                        params: &[Param { name: "aShape", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_shape",
                        abi: "C",
                        params: &[Param { name: "aShape", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* DOMString toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

