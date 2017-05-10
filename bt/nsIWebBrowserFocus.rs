//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserFocus.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserFocus",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void activate (); */
                    Method {
                        name: "activate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void deactivate (); */
                    Method {
                        name: "deactivate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setFocusAtFirstElement (); */
                    Method {
                        name: "setFocusAtFirstElement",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setFocusAtLastElement (); */
                    Method {
                        name: "setFocusAtLastElement",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* attribute mozIDOMWindowProxy focusedWindow; */
                    Method {
                        name: "get_focusedWindow",
                        abi: "C",
                        params: &[Param { name: "aFocusedWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_focusedWindow",
                        abi: "C",
                        params: &[Param { name: "aFocusedWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIDOMElement focusedElement; */
                    Method {
                        name: "get_focusedElement",
                        abi: "C",
                        params: &[Param { name: "aFocusedElement", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_focusedElement",
                        abi: "C",
                        params: &[Param { name: "aFocusedElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

