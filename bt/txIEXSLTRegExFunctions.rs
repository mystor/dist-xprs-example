//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/txIEXSLTRegExFunctions.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "txIEXSLTRegExFunctions",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* txINodeSet match (in txIFunctionEvaluationContext aContext, in DOMString aString, in DOMString aRegEx, in DOMString aFlags); */
                    Method {
                        name: "match_",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const txIFunctionEvaluationContext" }, Param { name: "aString", ty: "*const nsAString" }, Param { name: "aRegEx", ty: "*const nsAString" }, Param { name: "aFlags", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const txINodeSet" }],
                        ret: "nsresult",
                    },

                    /* DOMString replace (in DOMString aString, in DOMString aRegEx, in DOMString aFlags, in DOMString aReplace); */
                    Method {
                        name: "replace",
                        abi: "C",
                        params: &[Param { name: "aString", ty: "*const nsAString" }, Param { name: "aRegEx", ty: "*const nsAString" }, Param { name: "aFlags", ty: "*const nsAString" }, Param { name: "aReplace", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean test (in DOMString aString, in DOMString aRegEx, in DOMString aFlags); */
                    Method {
                        name: "test",
                        abi: "C",
                        params: &[Param { name: "aString", ty: "*const nsAString" }, Param { name: "aRegEx", ty: "*const nsAString" }, Param { name: "aFlags", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

