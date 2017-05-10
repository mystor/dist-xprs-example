//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDocumentXBL.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMDocumentXBL",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDOMNodeList getAnonymousNodes (in nsIDOMElement elt); */
                    Method {
                        name: "getAnonymousNodes",
                        abi: "C",
                        params: &[Param { name: "elt", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsIDOMNodeList" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getAnonymousElementByAttribute (in nsIDOMElement elt, in DOMString attrName, in DOMString attrValue); */
                    Method {
                        name: "getAnonymousElementByAttribute",
                        abi: "C",
                        params: &[Param { name: "elt", ty: "*const nsIDOMElement" }, Param { name: "attrName", ty: "*const nsAString" }, Param { name: "attrValue", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getBindingParent (in nsIDOMNode node); */
                    Method {
                        name: "getBindingParent",
                        abi: "C",
                        params: &[Param { name: "node", ty: "*const nsIDOMNode" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void loadBindingDocument (in DOMString documentURL); */
                    Method {
                        name: "loadBindingDocument",
                        abi: "C",
                        params: &[Param { name: "documentURL", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

