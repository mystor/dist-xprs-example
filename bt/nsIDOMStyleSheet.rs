//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMStyleSheet.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMStyleSheet",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean disabled; */
                    Method {
                        name: "get_disabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_disabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode ownerNode; */
                    Method {
                        name: "get_ownerNode",
                        abi: "C",
                        params: &[Param { name: "aOwnerNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMStyleSheet parentStyleSheet; */
                    Method {
                        name: "get_parentStyleSheet",
                        abi: "C",
                        params: &[Param { name: "aParentStyleSheet", ty: "*mut *const nsIDOMStyleSheet" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString href; */
                    Method {
                        name: "get_href",
                        abi: "C",
                        params: &[Param { name: "aHref", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString title; */
                    Method {
                        name: "get_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMMediaList media; */
                    Method {
                        name: "get_media",
                        abi: "C",
                        params: &[Param { name: "aMedia", ty: "*mut *const nsIDOMMediaList" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

