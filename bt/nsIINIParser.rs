//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIINIParser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIINIParser",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIUTF8StringEnumerator getSections (); */
                    Method {
                        name: "getSections",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIUTF8StringEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsIUTF8StringEnumerator getKeys (in AUTF8String aSection); */
                    Method {
                        name: "getKeys",
                        abi: "C",
                        params: &[Param { name: "aSection", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIUTF8StringEnumerator" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getString (in AUTF8String aSection, in AUTF8String aKey); */
                    Method {
                        name: "getString",
                        abi: "C",
                        params: &[Param { name: "aSection", ty: "*const nsACString" }, Param { name: "aKey", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIINIParserWriter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setString (in AUTF8String aSection, in AUTF8String aKey, in AUTF8String aValue); */
                    Method {
                        name: "setString",
                        abi: "C",
                        params: &[Param { name: "aSection", ty: "*const nsACString" }, Param { name: "aKey", ty: "*const nsACString" }, Param { name: "aValue", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void writeFile ([optional] in nsIFile aINIFile, [optional] in unsigned long aFlags); */
                    Method {
                        name: "writeFile",
                        abi: "C",
                        params: &[Param { name: "aINIFile", ty: "*const nsIFile" }, Param { name: "aFlags", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIINIParserFactory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIINIParser createINIParser (in nsIFile aINIFile); */
                    Method {
                        name: "createINIParser",
                        abi: "C",
                        params: &[Param { name: "aINIFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIINIParser" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

