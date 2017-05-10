//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSFontFeatureValuesRule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCSSFontFeatureValuesRule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString fontFamily; */
                    Method {
                        name: "get_fontFamily",
                        abi: "C",
                        params: &[Param { name: "aFontFamily", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_fontFamily",
                        abi: "C",
                        params: &[Param { name: "aFontFamily", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString valueText; */
                    Method {
                        name: "get_valueText",
                        abi: "C",
                        params: &[Param { name: "aValueText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_valueText",
                        abi: "C",
                        params: &[Param { name: "aValueText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

