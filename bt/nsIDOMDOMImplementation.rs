//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDOMImplementation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMDOMImplementation",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean hasFeature (in DOMString feature, in DOMString version); */
                    Method {
                        name: "hasFeature",
                        abi: "C",
                        params: &[Param { name: "feature", ty: "*const nsAString" }, Param { name: "version", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentType createDocumentType (in DOMString qualifiedName, in DOMString publicId, in DOMString systemId) raises (DOMException); */
                    Method {
                        name: "createDocumentType",
                        abi: "C",
                        params: &[Param { name: "qualifiedName", ty: "*const nsAString" }, Param { name: "publicId", ty: "*const nsAString" }, Param { name: "systemId", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocumentType" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocument createDocument (in DOMString namespaceURI, in DOMString qualifiedName, in nsIDOMDocumentType doctype) raises (DOMException); */
                    Method {
                        name: "createDocument",
                        abi: "C",
                        params: &[Param { name: "namespaceURI", ty: "*const nsAString" }, Param { name: "qualifiedName", ty: "*const nsAString" }, Param { name: "doctype", ty: "*const nsIDOMDocumentType" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocument createHTMLDocument ([Null (Stringify)] in DOMString title); */
                    Method {
                        name: "createHTMLDocument",
                        abi: "C",
                        params: &[Param { name: "title", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

