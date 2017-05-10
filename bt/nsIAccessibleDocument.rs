//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleDocument.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleDocument",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString URL; */
                    Method {
                        name: "get_URL",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString title; */
                    Method {
                        name: "get_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString mimeType; */
                    Method {
                        name: "get_mimeType",
                        abi: "C",
                        params: &[Param { name: "aMimeType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString docType; */
                    Method {
                        name: "get_docType",
                        abi: "C",
                        params: &[Param { name: "aDocType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMDocument DOMDocument; */
                    Method {
                        name: "get_DOMDocument",
                        abi: "C",
                        params: &[Param { name: "aDOMDocument", ty: "*mut *const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIDOMWindowProxy window; */
                    Method {
                        name: "get_window",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessibleDocument parentDocument; */
                    Method {
                        name: "get_parentDocument",
                        abi: "C",
                        params: &[Param { name: "aParentDocument", ty: "*mut *const nsIAccessibleDocument" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long childDocumentCount; */
                    Method {
                        name: "get_childDocumentCount",
                        abi: "C",
                        params: &[Param { name: "aChildDocumentCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessiblePivot virtualCursor; */
                    Method {
                        name: "get_virtualCursor",
                        abi: "C",
                        params: &[Param { name: "aVirtualCursor", ty: "*mut *const nsIAccessiblePivot" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessibleDocument getChildDocumentAt (in unsigned long index); */
                    Method {
                        name: "getChildDocumentAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleDocument" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

