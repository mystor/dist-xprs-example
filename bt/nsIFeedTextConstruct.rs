//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedTextConstruct.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedTextConstruct",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIURI base; */
                    Method {
                        name: "get_base",
                        abi: "C",
                        params: &[Param { name: "aBase", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_base",
                        abi: "C",
                        params: &[Param { name: "aBase", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute AString lang; */
                    Method {
                        name: "get_lang",
                        abi: "C",
                        params: &[Param { name: "aLang", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_lang",
                        abi: "C",
                        params: &[Param { name: "aLang", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString text; */
                    Method {
                        name: "get_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString plainText (); */
                    Method {
                        name: "plainText",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentFragment createDocumentFragment (in nsIDOMElement element); */
                    Method {
                        name: "createDocumentFragment",
                        abi: "C",
                        params: &[Param { name: "element", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocumentFragment" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

