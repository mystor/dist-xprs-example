//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMLocation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMLocation",
            base: Some("nsISupports"),
            methods: Some(&[
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

                    /* readonly attribute DOMString origin; */
                    Method {
                        name: "get_origin",
                        abi: "C",
                        params: &[Param { name: "aOrigin", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void reload ([optional] in boolean forceget); */
                    Method {
                        name: "reload",
                        abi: "C",
                        params: &[Param { name: "forceget", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void replace (in DOMString url); */
                    Method {
                        name: "replace",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void assign (in DOMString url); */
                    Method {
                        name: "assign",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* DOMString toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMLocation valueOf (); */
                    Method {
                        name: "valueOf",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMLocation" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

