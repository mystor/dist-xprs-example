//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITextToSubURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITextToSubURI",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* string ConvertAndEscape (in string charset, in wstring text); */
                    Method {
                        name: "ConvertAndEscape",
                        abi: "C",
                        params: &[Param { name: "charset", ty: "*const libc::c_char" }, Param { name: "text", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* wstring UnEscapeAndConvert (in string charset, in string text); */
                    Method {
                        name: "UnEscapeAndConvert",
                        abi: "C",
                        params: &[Param { name: "charset", ty: "*const libc::c_char" }, Param { name: "text", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* AString unEscapeURIForUI (in ACString aCharset, in AUTF8String aURIFragment); */
                    Method {
                        name: "unEscapeURIForUI",
                        abi: "C",
                        params: &[Param { name: "aCharset", ty: "*const nsACString" }, Param { name: "aURIFragment", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString unEscapeNonAsciiURI (in ACString aCharset, in AUTF8String aURIFragment); */
                    Method {
                        name: "unEscapeNonAsciiURI",
                        abi: "C",
                        params: &[Param { name: "aCharset", ty: "*const nsACString" }, Param { name: "aURIFragment", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

