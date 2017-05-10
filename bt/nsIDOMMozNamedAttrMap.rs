//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMozNamedAttrMap.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMMozNamedAttrMap",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDOMAttr getNamedItem (in DOMString name); */
                    Method {
                        name: "getNamedItem",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMAttr setNamedItem (in nsIDOMAttr arg) raises (DOMException); */
                    Method {
                        name: "setNamedItem",
                        abi: "C",
                        params: &[Param { name: "arg", ty: "*const nsIDOMAttr" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMAttr removeNamedItem (in DOMString name) raises (DOMException); */
                    Method {
                        name: "removeNamedItem",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMAttr item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMAttr getNamedItemNS (in DOMString namespaceURI, in DOMString localName); */
                    Method {
                        name: "getNamedItemNS",
                        abi: "C",
                        params: &[Param { name: "namespaceURI", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMAttr setNamedItemNS (in nsIDOMAttr arg) raises (DOMException); */
                    Method {
                        name: "setNamedItemNS",
                        abi: "C",
                        params: &[Param { name: "arg", ty: "*const nsIDOMAttr" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMAttr removeNamedItemNS (in DOMString namespaceURI, in DOMString localName) raises (DOMException); */
                    Method {
                        name: "removeNamedItemNS",
                        abi: "C",
                        params: &[Param { name: "namespaceURI", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMAttr" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

