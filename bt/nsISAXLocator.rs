//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXLocator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXLocator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long columnNumber; */
                    Method {
                        name: "get_columnNumber",
                        abi: "C",
                        params: &[Param { name: "aColumnNumber", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long lineNumber; */
                    Method {
                        name: "get_lineNumber",
                        abi: "C",
                        params: &[Param { name: "aLineNumber", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString publicId; */
                    Method {
                        name: "get_publicId",
                        abi: "C",
                        params: &[Param { name: "aPublicId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString systemId; */
                    Method {
                        name: "get_systemId",
                        abi: "C",
                        params: &[Param { name: "aSystemId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

