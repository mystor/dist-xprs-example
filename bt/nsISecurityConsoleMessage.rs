//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityConsoleMessage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecurityConsoleMessage",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AString tag; */
                    Method {
                        name: "get_tag",
                        abi: "C",
                        params: &[Param { name: "aTag", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_tag",
                        abi: "C",
                        params: &[Param { name: "aTag", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString category; */
                    Method {
                        name: "get_category",
                        abi: "C",
                        params: &[Param { name: "aCategory", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_category",
                        abi: "C",
                        params: &[Param { name: "aCategory", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

