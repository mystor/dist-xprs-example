//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMenuBuilder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMenuBuilder",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void openContainer (in DOMString aLabel); */
                    Method {
                        name: "openContainer",
                        abi: "C",
                        params: &[Param { name: "aLabel", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void addItemFor (in nsIDOMHTMLMenuItemElement aElement, in boolean aCanLoadIcon); */
                    Method {
                        name: "addItemFor",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMHTMLMenuItemElement" }, Param { name: "aCanLoadIcon", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addSeparator (); */
                    Method {
                        name: "addSeparator",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void undoAddSeparator (); */
                    Method {
                        name: "undoAddSeparator",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void closeContainer (); */
                    Method {
                        name: "closeContainer",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* AString toJSONString (); */
                    Method {
                        name: "toJSONString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void click (in DOMString aGeneratedItemId); */
                    Method {
                        name: "click",
                        abi: "C",
                        params: &[Param { name: "aGeneratedItemId", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

