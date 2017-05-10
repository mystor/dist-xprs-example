//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNavigator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMNavigator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString appCodeName; */
                    Method {
                        name: "get_appCodeName",
                        abi: "C",
                        params: &[Param { name: "aAppCodeName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString language; */
                    Method {
                        name: "get_language",
                        abi: "C",
                        params: &[Param { name: "aLanguage", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString vendor; */
                    Method {
                        name: "get_vendor",
                        abi: "C",
                        params: &[Param { name: "aVendor", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString vendorSub; */
                    Method {
                        name: "get_vendorSub",
                        abi: "C",
                        params: &[Param { name: "aVendorSub", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString product; */
                    Method {
                        name: "get_product",
                        abi: "C",
                        params: &[Param { name: "aProduct", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString productSub; */
                    Method {
                        name: "get_productSub",
                        abi: "C",
                        params: &[Param { name: "aProductSub", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString doNotTrack; */
                    Method {
                        name: "get_doNotTrack",
                        abi: "C",
                        params: &[Param { name: "aDoNotTrack", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

