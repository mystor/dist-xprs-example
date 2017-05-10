//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMapElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLMapElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMHTMLCollection areas; */
                    Method {
                        name: "get_areas",
                        abi: "C",
                        params: &[Param { name: "aAreas", ty: "*mut *const nsIDOMHTMLCollection" }],
                        ret: "nsresult",
                    },

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

                    ]),
        },


        ]; D}

