//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXXMLReader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXXMLReader",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* attribute nsIURI baseURI; */
                    Method {
                        name: "get_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISAXContentHandler contentHandler; */
                    Method {
                        name: "get_contentHandler",
                        abi: "C",
                        params: &[Param { name: "aContentHandler", ty: "*mut *const nsISAXContentHandler" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentHandler",
                        abi: "C",
                        params: &[Param { name: "aContentHandler", ty: "*const nsISAXContentHandler" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISAXDTDHandler dtdHandler; */
                    Method {
                        name: "get_dtdHandler",
                        abi: "C",
                        params: &[Param { name: "aDtdHandler", ty: "*mut *const nsISAXDTDHandler" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_dtdHandler",
                        abi: "C",
                        params: &[Param { name: "aDtdHandler", ty: "*const nsISAXDTDHandler" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISAXErrorHandler errorHandler; */
                    Method {
                        name: "get_errorHandler",
                        abi: "C",
                        params: &[Param { name: "aErrorHandler", ty: "*mut *const nsISAXErrorHandler" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_errorHandler",
                        abi: "C",
                        params: &[Param { name: "aErrorHandler", ty: "*const nsISAXErrorHandler" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIMozSAXXMLDeclarationHandler declarationHandler; */
                    Method {
                        name: "get_declarationHandler",
                        abi: "C",
                        params: &[Param { name: "aDeclarationHandler", ty: "*mut *const nsIMozSAXXMLDeclarationHandler" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_declarationHandler",
                        abi: "C",
                        params: &[Param { name: "aDeclarationHandler", ty: "*const nsIMozSAXXMLDeclarationHandler" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISAXLexicalHandler lexicalHandler; */
                    Method {
                        name: "get_lexicalHandler",
                        abi: "C",
                        params: &[Param { name: "aLexicalHandler", ty: "*mut *const nsISAXLexicalHandler" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_lexicalHandler",
                        abi: "C",
                        params: &[Param { name: "aLexicalHandler", ty: "*const nsISAXLexicalHandler" }],
                        ret: "nsresult",
                    },

                    /* void setFeature (in AString name, in boolean value); */
                    Method {
                        name: "setFeature",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "value", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* boolean getFeature (in AString name); */
                    Method {
                        name: "getFeature",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setProperty (in AString name, in nsISupports value); */
                    Method {
                        name: "setProperty",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* boolean getProperty (in AString name); */
                    Method {
                        name: "getProperty",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void parseFromString (in AString str, in string contentType); */
                    Method {
                        name: "parseFromString",
                        abi: "C",
                        params: &[Param { name: "str", ty: "*const nsAString" }, Param { name: "contentType", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void parseFromStream (in nsIInputStream stream, in string charset, in string contentType); */
                    Method {
                        name: "parseFromStream",
                        abi: "C",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }, Param { name: "charset", ty: "*const libc::c_char" }, Param { name: "contentType", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void parseAsync (in nsIRequestObserver observer); */
                    Method {
                        name: "parseAsync",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIRequestObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

