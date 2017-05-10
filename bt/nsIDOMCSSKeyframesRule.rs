//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSKeyframesRule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSKeyframesRule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMCSSRuleList cssRules; */
                    Method {
                        name: "get_cssRules",
                        abi: "C",
                        params: &[Param { name: "aCssRules", ty: "*mut *const nsIDOMCSSRuleList" }],
                        ret: "nsresult",
                    },

                    /* void appendRule (in DOMString rule); */
                    Method {
                        name: "appendRule",
                        abi: "C",
                        params: &[Param { name: "rule", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void deleteRule (in DOMString key); */
                    Method {
                        name: "deleteRule",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMCSSKeyframeRule findRule (in DOMString key); */
                    Method {
                        name: "findRule",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMCSSKeyframeRule" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

