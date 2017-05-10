//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMFontFace.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMFontFace",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean fromFontGroup; */
                    Method {
                        name: "get_fromFontGroup",
                        abi: "C",
                        params: &[Param { name: "aFromFontGroup", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean fromLanguagePrefs; */
                    Method {
                        name: "get_fromLanguagePrefs",
                        abi: "C",
                        params: &[Param { name: "aFromLanguagePrefs", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean fromSystemFallback; */
                    Method {
                        name: "get_fromSystemFallback",
                        abi: "C",
                        params: &[Param { name: "aFromSystemFallback", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString CSSFamilyName; */
                    Method {
                        name: "get_CSSFamilyName",
                        abi: "C",
                        params: &[Param { name: "aCSSFamilyName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMCSSFontFaceRule rule; */
                    Method {
                        name: "get_rule",
                        abi: "C",
                        params: &[Param { name: "aRule", ty: "*mut *const nsIDOMCSSFontFaceRule" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long srcIndex; */
                    Method {
                        name: "get_srcIndex",
                        abi: "C",
                        params: &[Param { name: "aSrcIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString URI; */
                    Method {
                        name: "get_URI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString localName; */
                    Method {
                        name: "get_localName",
                        abi: "C",
                        params: &[Param { name: "aLocalName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString format; */
                    Method {
                        name: "get_format",
                        abi: "C",
                        params: &[Param { name: "aFormat", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString metadata; */
                    Method {
                        name: "get_metadata",
                        abi: "C",
                        params: &[Param { name: "aMetadata", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

