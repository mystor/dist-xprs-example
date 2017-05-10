//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSStyleDeclaration.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSStyleDeclaration",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString cssText; */
                    Method {
                        name: "get_cssText",
                        abi: "C",
                        params: &[Param { name: "aCssText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cssText",
                        abi: "C",
                        params: &[Param { name: "aCssText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* DOMString getPropertyValue (in DOMString propertyName); */
                    Method {
                        name: "getPropertyValue",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMCSSValue getPropertyCSSValue (in DOMString propertyName); */
                    Method {
                        name: "getPropertyCSSValue",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMCSSValue" }],
                        ret: "nsresult",
                    },

                    /* DOMString removeProperty (in DOMString propertyName) raises (DOMException); */
                    Method {
                        name: "removeProperty",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* DOMString getPropertyPriority (in DOMString propertyName); */
                    Method {
                        name: "getPropertyPriority",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setProperty (in DOMString propertyName, in DOMString value, [optional] in DOMString priority) raises (DOMException); */
                    Method {
                        name: "setProperty",
                        abi: "C",
                        params: &[Param { name: "propertyName", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }, Param { name: "priority", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* DOMString item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMCSSRule parentRule; */
                    Method {
                        name: "get_parentRule",
                        abi: "C",
                        params: &[Param { name: "aParentRule", ty: "*mut *const nsIDOMCSSRule" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

