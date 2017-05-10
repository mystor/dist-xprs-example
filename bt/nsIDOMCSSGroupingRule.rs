//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSGroupingRule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSGroupingRule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMCSSRuleList cssRules; */
                    Method {
                        name: "get_cssRules",
                        abi: "C",
                        params: &[Param { name: "aCssRules", ty: "*mut *const nsIDOMCSSRuleList" }],
                        ret: "nsresult",
                    },

                    /* unsigned long insertRule (in DOMString rule, in unsigned long index) raises (DOMException); */
                    Method {
                        name: "insertRule",
                        abi: "C",
                        params: &[Param { name: "rule", ty: "*const nsAString" }, Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void deleteRule (in unsigned long index) raises (DOMException); */
                    Method {
                        name: "deleteRule",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

