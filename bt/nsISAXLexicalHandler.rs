//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXLexicalHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXLexicalHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void comment (in AString chars); */
                    Method {
                        name: "comment",
                        abi: "C",
                        params: &[Param { name: "chars", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void startDTD (in AString name, in AString publicId, in AString systemId); */
                    Method {
                        name: "startDTD",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "publicId", ty: "*const nsAString" }, Param { name: "systemId", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void endDTD (); */
                    Method {
                        name: "endDTD",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void startCDATA (); */
                    Method {
                        name: "startCDATA",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void endCDATA (); */
                    Method {
                        name: "endCDATA",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void startEntity (in AString name); */
                    Method {
                        name: "startEntity",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void endEntity (in AString name); */
                    Method {
                        name: "endEntity",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

