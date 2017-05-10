//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBoxObject.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBoxObject",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMElement element; */
                    Method {
                        name: "get_element",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long x; */
                    Method {
                        name: "get_x",
                        abi: "C",
                        params: &[Param { name: "aX", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long y; */
                    Method {
                        name: "get_y",
                        abi: "C",
                        params: &[Param { name: "aY", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long screenX; */
                    Method {
                        name: "get_screenX",
                        abi: "C",
                        params: &[Param { name: "aScreenX", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long screenY; */
                    Method {
                        name: "get_screenY",
                        abi: "C",
                        params: &[Param { name: "aScreenY", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long height; */
                    Method {
                        name: "get_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsISupports getPropertyAsSupports (in wstring propertyName); */
                    Method {
                        name: "getPropertyAsSupports",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsSupports (in wstring propertyName, in nsISupports value); */
                    Method {
                        name: "setPropertyAsSupports",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const libc::int16_t" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* wstring getProperty (in wstring propertyName); */
                    Method {
                        name: "getProperty",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void setProperty (in wstring propertyName, in wstring propertyValue); */
                    Method {
                        name: "setProperty",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const libc::int16_t" }, Param { name: "propertyValue", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void removeProperty (in wstring propertyName); */
                    Method {
                        name: "removeProperty",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement parentBox; */
                    Method {
                        name: "get_parentBox",
                        abi: "C",
                        params: &[Param { name: "aParentBox", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement firstChild; */
                    Method {
                        name: "get_firstChild",
                        abi: "C",
                        params: &[Param { name: "aFirstChild", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement lastChild; */
                    Method {
                        name: "get_lastChild",
                        abi: "C",
                        params: &[Param { name: "aLastChild", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement nextSibling; */
                    Method {
                        name: "get_nextSibling",
                        abi: "C",
                        params: &[Param { name: "aNextSibling", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement previousSibling; */
                    Method {
                        name: "get_previousSibling",
                        abi: "C",
                        params: &[Param { name: "aPreviousSibling", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

