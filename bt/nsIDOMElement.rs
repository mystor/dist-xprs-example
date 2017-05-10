//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMElement",
            base: Some("nsIDOMNode"),
            methods: Some(&[
                    /* readonly attribute DOMString tagName; */
                    Method {
                        name: "get_tagName",
                        abi: "C",
                        params: &[Param { name: "aTagName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMMozNamedAttrMap attributes; */
                    Method {
                        name: "get_attributes",
                        abi: "C",
                        params: &[Param { name: "aAttributes", ty: "*mut *const nsIDOMMozNamedAttrMap" }],
                        ret: "nsresult",
                    },

                    /* DOMString getAttribute (in DOMString name); */
                    Method {
                        name: "getAttribute",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setAttribute (in DOMString name, in DOMString value); */
                    Method {
                        name: "setAttribute",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean hasAttribute (in DOMString name); */
                    Method {
                        name: "hasAttribute",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMAttr getAttributeNode (in DOMString name); */
                    Method {
                        name: "getAttributeNode",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMAttr getAttributeNodeNS (in DOMString namespaceURI, in DOMString localName); */
                    Method {
                        name: "getAttributeNodeNS",
                        abi: "C",
                        params: &[Param { name: "namespaceURI", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

