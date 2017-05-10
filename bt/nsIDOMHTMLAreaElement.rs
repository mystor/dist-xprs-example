//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLAreaElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLAreaElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString alt; */
                    Method {
                        name: "get_alt",
                        abi: "C",
                        params: &[Param { name: "aAlt", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_alt",
                        abi: "C",
                        params: &[Param { name: "aAlt", ty: "*const nsAString" }],
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

                    /* attribute boolean noHref; */
                    Method {
                        name: "get_noHref",
                        abi: "C",
                        params: &[Param { name: "aNoHref", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_noHref",
                        abi: "C",
                        params: &[Param { name: "aNoHref", ty: "bool" }],
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

