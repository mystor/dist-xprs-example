//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXSLTProcessor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXSLTProcessor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void importStylesheet (in nsIDOMNode style); */
                    Method {
                        name: "importStylesheet",
                        abi: "C",
                        params: &[Param { name: "style", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentFragment transformToFragment (in nsIDOMNode source, in nsIDOMDocument output); */
                    Method {
                        name: "transformToFragment",
                        abi: "C",
                        params: &[Param { name: "source", ty: "*const nsIDOMNode" }, Param { name: "output", ty: "*const nsIDOMDocument" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocumentFragment" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocument transformToDocument (in nsIDOMNode source); */
                    Method {
                        name: "transformToDocument",
                        abi: "C",
                        params: &[Param { name: "source", ty: "*const nsIDOMNode" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    /* void setParameter (in DOMString namespaceURI, in DOMString localName, in nsIVariant value); */
                    Method {
                        name: "setParameter",
                        abi: "C",
                        params: &[Param { name: "namespaceURI", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* nsIVariant getParameter (in DOMString namespaceURI, in DOMString localName); */
                    Method {
                        name: "getParameter",
                        abi: "C",
                        params: &[Param { name: "namespaceURI", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void removeParameter (in DOMString namespaceURI, in DOMString localName); */
                    Method {
                        name: "removeParameter",
                        abi: "C",
                        params: &[Param { name: "namespaceURI", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void clearParameters (); */
                    Method {
                        name: "clearParameters",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void reset (); */
                    Method {
                        name: "reset",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

