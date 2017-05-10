//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLSourceElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLSourceElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString src; */
                    Method {
                        name: "get_src",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_src",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString srcset; */
                    Method {
                        name: "get_srcset",
                        abi: "C",
                        params: &[Param { name: "aSrcset", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_srcset",
                        abi: "C",
                        params: &[Param { name: "aSrcset", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString sizes; */
                    Method {
                        name: "get_sizes",
                        abi: "C",
                        params: &[Param { name: "aSizes", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sizes",
                        abi: "C",
                        params: &[Param { name: "aSizes", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString media; */
                    Method {
                        name: "get_media",
                        abi: "C",
                        params: &[Param { name: "aMedia", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_media",
                        abi: "C",
                        params: &[Param { name: "aMedia", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

