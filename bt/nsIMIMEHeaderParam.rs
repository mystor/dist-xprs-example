//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMIMEHeaderParam.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMIMEHeaderParam",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString getParameter (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
                    Method {
                        name: "getParameter",
                        abi: "C",
                        params: &[Param { name: "aHeaderVal", ty: "*const nsACString" }, Param { name: "aParamName", ty: "*const libc::c_char" }, Param { name: "aFallbackCharset", ty: "*const nsACString" }, Param { name: "aTryLocaleCharset", ty: "bool" }, Param { name: "aLang", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getParameterHTTP (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
                    Method {
                        name: "getParameterHTTP",
                        abi: "C",
                        params: &[Param { name: "aHeaderVal", ty: "*const nsACString" }, Param { name: "aParamName", ty: "*const libc::c_char" }, Param { name: "aFallbackCharset", ty: "*const nsACString" }, Param { name: "aTryLocaleCharset", ty: "bool" }, Param { name: "aLang", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString decodeRFC5987Param (in ACString aParamVal, out ACString aLang); */
                    Method {
                        name: "decodeRFC5987Param",
                        abi: "C",
                        params: &[Param { name: "aParamVal", ty: "*const nsACString" }, Param { name: "aLang", ty: "*mut nsACString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* [noscript] string getParameterInternal (in string aHeaderVal, in string aParamName, out string aCharset, out string aLang); */
                    Method {
                        name: "getParameterInternal",
                        abi: "C",
                        params: &[Param { name: "aHeaderVal", ty: "*const libc::c_char" }, Param { name: "aParamName", ty: "*const libc::c_char" }, Param { name: "aCharset", ty: "*mut *const libc::c_char" }, Param { name: "aLang", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* [noscript] ACString decodeRFC2047Header (in string aHeaderVal, in string aDefaultCharset, in boolean aOverrideCharset, in boolean aEatContinuation); */
                    Method {
                        name: "decodeRFC2047Header",
                        abi: "C",
                        params: &[Param { name: "aHeaderVal", ty: "*const libc::c_char" }, Param { name: "aDefaultCharset", ty: "*const libc::c_char" }, Param { name: "aOverrideCharset", ty: "bool" }, Param { name: "aEatContinuation", ty: "bool" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [noscript] ACString decodeParameter (in ACString aParamValue, in string aCharset, in string aDefaultCharset, in boolean aOverrideCharset); */
                    Method {
                        name: "decodeParameter",
                        abi: "C",
                        params: &[Param { name: "aParamValue", ty: "*const nsACString" }, Param { name: "aCharset", ty: "*const libc::c_char" }, Param { name: "aDefaultCharset", ty: "*const libc::c_char" }, Param { name: "aOverrideCharset", ty: "bool" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

