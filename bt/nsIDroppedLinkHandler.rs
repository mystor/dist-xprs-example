//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDroppedLinkHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDroppedLinkItem",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString url; */
                    Method {
                        name: "get_url",
                        abi: "C",
                        params: &[Param { name: "aUrl", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDroppedLinkHandler",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

