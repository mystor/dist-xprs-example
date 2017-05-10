//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTMLObjectResizer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTMLObjectResizer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMElement resizedObject; */
                    Method {
                        name: "get_resizedObject",
                        abi: "C",
                        params: &[Param { name: "aResizedObject", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean objectResizingEnabled; */
                    Method {
                        name: "get_objectResizingEnabled",
                        abi: "C",
                        params: &[Param { name: "aObjectResizingEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_objectResizingEnabled",
                        abi: "C",
                        params: &[Param { name: "aObjectResizingEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void showResizers (in nsIDOMElement aResizedElement); */
                    Method {
                        name: "showResizers",
                        abi: "C",
                        params: &[Param { name: "aResizedElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void hideResizers (); */
                    Method {
                        name: "hideResizers",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void refreshResizers (); */
                    Method {
                        name: "refreshResizers",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void mouseDown (in long aX, in long aY, in nsIDOMElement aTarget, in nsIDOMEvent aMouseEvent); */
                    Method {
                        name: "mouseDown",
                        abi: "C",
                        params: &[Param { name: "aX", ty: "libc::int32_t" }, Param { name: "aY", ty: "libc::int32_t" }, Param { name: "aTarget", ty: "*const nsIDOMElement" }, Param { name: "aMouseEvent", ty: "*const nsIDOMEvent" }],
                        ret: "nsresult",
                    },

                    /* void mouseUp (in long aX, in long aY, in nsIDOMElement aTarget); */
                    Method {
                        name: "mouseUp",
                        abi: "C",
                        params: &[Param { name: "aX", ty: "libc::int32_t" }, Param { name: "aY", ty: "libc::int32_t" }, Param { name: "aTarget", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void mouseMove (in nsIDOMEvent aMouseEvent); */
                    Method {
                        name: "mouseMove",
                        abi: "C",
                        params: &[Param { name: "aMouseEvent", ty: "*const nsIDOMEvent" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

