//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExtendedExpatSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExtendedExpatSink",
            base: Some("nsIExpatSink"),
            methods: Some(&[
                    /* void handleStartDTD (in wstring aDoctypeName, in wstring aSysid, in wstring aPubid); */
                    Method {
                        name: "handleStartDTD",
                        abi: "C",
                        params: &[Param { name: "aDoctypeName", ty: "*const libc::int16_t" }, Param { name: "aSysid", ty: "*const libc::int16_t" }, Param { name: "aPubid", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void handleStartNamespaceDecl (in wstring aPrefix, in wstring aUri); */
                    Method {
                        name: "handleStartNamespaceDecl",
                        abi: "C",
                        params: &[Param { name: "aPrefix", ty: "*const libc::int16_t" }, Param { name: "aUri", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void handleEndNamespaceDecl (in wstring aPrefix); */
                    Method {
                        name: "handleEndNamespaceDecl",
                        abi: "C",
                        params: &[Param { name: "aPrefix", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void handleNotationDecl (in wstring aNotationName, in wstring aSysid, in wstring aPubid); */
                    Method {
                        name: "handleNotationDecl",
                        abi: "C",
                        params: &[Param { name: "aNotationName", ty: "*const libc::int16_t" }, Param { name: "aSysid", ty: "*const libc::int16_t" }, Param { name: "aPubid", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void handleUnparsedEntityDecl (in wstring aName, in wstring aSysid, in wstring aPubid, in wstring aNotationName); */
                    Method {
                        name: "handleUnparsedEntityDecl",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const libc::int16_t" }, Param { name: "aSysid", ty: "*const libc::int16_t" }, Param { name: "aPubid", ty: "*const libc::int16_t" }, Param { name: "aNotationName", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

