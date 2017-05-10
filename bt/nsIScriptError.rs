//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptError.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptErrorNote",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString errorMessage; */
                    Method {
                        name: "get_errorMessage",
                        abi: "C",
                        params: &[Param { name: "aErrorMessage", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString sourceName; */
                    Method {
                        name: "get_sourceName",
                        abi: "C",
                        params: &[Param { name: "aSourceName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t lineNumber; */
                    Method {
                        name: "get_lineNumber",
                        abi: "C",
                        params: &[Param { name: "aLineNumber", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t columnNumber; */
                    Method {
                        name: "get_columnNumber",
                        abi: "C",
                        params: &[Param { name: "aColumnNumber", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIScriptError",
            base: Some("nsIConsoleMessage"),
            methods: None,
        },


        ]; D}

