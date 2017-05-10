//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURI",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AUTF8String spec; */
                    Method {
                        name: "get_spec",
                        abi: "C",
                        params: &[Param { name: "aSpec", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_spec",
                        abi: "C",
                        params: &[Param { name: "aSpec", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String prePath; */
                    Method {
                        name: "get_prePath",
                        abi: "C",
                        params: &[Param { name: "aPrePath", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString scheme; */
                    Method {
                        name: "get_scheme",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_scheme",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String userPass; */
                    Method {
                        name: "get_userPass",
                        abi: "C",
                        params: &[Param { name: "aUserPass", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_userPass",
                        abi: "C",
                        params: &[Param { name: "aUserPass", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String username; */
                    Method {
                        name: "get_username",
                        abi: "C",
                        params: &[Param { name: "aUsername", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_username",
                        abi: "C",
                        params: &[Param { name: "aUsername", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String password; */
                    Method {
                        name: "get_password",
                        abi: "C",
                        params: &[Param { name: "aPassword", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_password",
                        abi: "C",
                        params: &[Param { name: "aPassword", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String hostPort; */
                    Method {
                        name: "get_hostPort",
                        abi: "C",
                        params: &[Param { name: "aHostPort", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hostPort",
                        abi: "C",
                        params: &[Param { name: "aHostPort", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setHostAndPort (in AUTF8String hostport); */
                    Method {
                        name: "setHostAndPort",
                        abi: "C",
                        params: &[Param { name: "hostport", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String host; */
                    Method {
                        name: "get_host",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_host",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute long port; */
                    Method {
                        name: "get_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String path; */
                    Method {
                        name: "get_path",
                        abi: "C",
                        params: &[Param { name: "aPath", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_path",
                        abi: "C",
                        params: &[Param { name: "aPath", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean equals (in nsIURI other); */
                    Method {
                        name: "equals",
                        abi: "C",
                        params: &[Param { name: "other", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean schemeIs (in string scheme); */
                    Method {
                        name: "schemeIs",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIURI clone (); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String resolve (in AUTF8String relativePath); */
                    Method {
                        name: "resolve",
                        abi: "C",
                        params: &[Param { name: "relativePath", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString asciiSpec; */
                    Method {
                        name: "get_asciiSpec",
                        abi: "C",
                        params: &[Param { name: "aAsciiSpec", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString asciiHostPort; */
                    Method {
                        name: "get_asciiHostPort",
                        abi: "C",
                        params: &[Param { name: "aAsciiHostPort", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString asciiHost; */
                    Method {
                        name: "get_asciiHost",
                        abi: "C",
                        params: &[Param { name: "aAsciiHost", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString originCharset; */
                    Method {
                        name: "get_originCharset",
                        abi: "C",
                        params: &[Param { name: "aOriginCharset", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String ref; */
                    Method {
                        name: "get_ref_",
                        abi: "C",
                        params: &[Param { name: "aRef", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_ref_",
                        abi: "C",
                        params: &[Param { name: "aRef", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean equalsExceptRef (in nsIURI other); */
                    Method {
                        name: "equalsExceptRef",
                        abi: "C",
                        params: &[Param { name: "other", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIURI cloneIgnoringRef (); */
                    Method {
                        name: "cloneIgnoringRef",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* nsIURI cloneWithNewRef (in AUTF8String newRef); */
                    Method {
                        name: "cloneWithNewRef",
                        abi: "C",
                        params: &[Param { name: "newRef", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String specIgnoringRef; */
                    Method {
                        name: "get_specIgnoringRef",
                        abi: "C",
                        params: &[Param { name: "aSpecIgnoringRef", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean hasRef; */
                    Method {
                        name: "get_hasRef",
                        abi: "C",
                        params: &[Param { name: "aHasRef", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String filePath; */
                    Method {
                        name: "get_filePath",
                        abi: "C",
                        params: &[Param { name: "aFilePath", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_filePath",
                        abi: "C",
                        params: &[Param { name: "aFilePath", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String query; */
                    Method {
                        name: "get_query",
                        abi: "C",
                        params: &[Param { name: "aQuery", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_query",
                        abi: "C",
                        params: &[Param { name: "aQuery", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

