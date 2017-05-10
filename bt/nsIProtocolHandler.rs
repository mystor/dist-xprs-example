//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtocolHandlerWithDynamicFlags",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* unsigned long getFlagsForURI (in nsIURI aURI); */
                    Method {
                        name: "getFlagsForURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIProtocolHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString scheme; */
                    Method {
                        name: "get_scheme",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long defaultPort; */
                    Method {
                        name: "get_defaultPort",
                        abi: "C",
                        params: &[Param { name: "aDefaultPort", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long protocolFlags; */
                    Method {
                        name: "get_protocolFlags",
                        abi: "C",
                        params: &[Param { name: "aProtocolFlags", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI); */
                    Method {
                        name: "newURI",
                        abi: "C",
                        params: &[Param { name: "aSpec", ty: "*const nsACString" }, Param { name: "aOriginCharset", ty: "*const libc::c_char" }, Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannel2 (in nsIURI aURI, in nsILoadInfo aLoadinfo); */
                    Method {
                        name: "newChannel2",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLoadinfo", ty: "*const nsILoadInfo" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannel (in nsIURI aURI); */
                    Method {
                        name: "newChannel",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* boolean allowPort (in long port, in string scheme); */
                    Method {
                        name: "allowPort",
                        abi: "C",
                        params: &[Param { name: "port", ty: "libc::int32_t" }, Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

