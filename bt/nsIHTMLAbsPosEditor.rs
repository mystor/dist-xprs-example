//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTMLAbsPosEditor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTMLAbsPosEditor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean selectionContainerAbsolutelyPositioned; */
                    Method {
                        name: "get_selectionContainerAbsolutelyPositioned",
                        abi: "C",
                        params: &[Param { name: "aSelectionContainerAbsolutelyPositioned", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement positionedElement; */
                    Method {
                        name: "get_positionedElement",
                        abi: "C",
                        params: &[Param { name: "aPositionedElement", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean absolutePositioningEnabled; */
                    Method {
                        name: "get_absolutePositioningEnabled",
                        abi: "C",
                        params: &[Param { name: "aAbsolutePositioningEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_absolutePositioningEnabled",
                        abi: "C",
                        params: &[Param { name: "aAbsolutePositioningEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean snapToGridEnabled; */
                    Method {
                        name: "get_snapToGridEnabled",
                        abi: "C",
                        params: &[Param { name: "aSnapToGridEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_snapToGridEnabled",
                        abi: "C",
                        params: &[Param { name: "aSnapToGridEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long gridSize; */
                    Method {
                        name: "get_gridSize",
                        abi: "C",
                        params: &[Param { name: "aGridSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_gridSize",
                        abi: "C",
                        params: &[Param { name: "aGridSize", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement absolutelyPositionedSelectionContainer; */
                    Method {
                        name: "get_absolutelyPositionedSelectionContainer",
                        abi: "C",
                        params: &[Param { name: "aAbsolutelyPositionedSelectionContainer", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void absolutePositionSelection (in boolean aEnabled); */
                    Method {
                        name: "absolutePositionSelection",
                        abi: "C",
                        params: &[Param { name: "aEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void relativeChangeZIndex (in long aChange); */
                    Method {
                        name: "relativeChangeZIndex",
                        abi: "C",
                        params: &[Param { name: "aChange", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void absolutelyPositionElement (in nsIDOMElement aElement, in boolean aEnabled); */
                    Method {
                        name: "absolutelyPositionElement",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "aEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setElementPosition (in nsIDOMElement aElement, in long aX, in long aY); */
                    Method {
                        name: "setElementPosition",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "aX", ty: "libc::int32_t" }, Param { name: "aY", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* long getElementZIndex (in nsIDOMElement aElement); */
                    Method {
                        name: "getElementZIndex",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setElementZIndex (in nsIDOMElement aElement, in long aZorder); */
                    Method {
                        name: "setElementZIndex",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "aZorder", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* long relativeChangeElementZIndex (in nsIDOMElement aElement, in long aChange); */
                    Method {
                        name: "relativeChangeElementZIndex",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "aChange", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void showGrabberOnElement (in nsIDOMElement aElement); */
                    Method {
                        name: "showGrabberOnElement",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void hideGrabber (); */
                    Method {
                        name: "hideGrabber",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void refreshGrabber (); */
                    Method {
                        name: "refreshGrabber",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

