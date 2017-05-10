//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULDocument.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULDocument",
            base: Some("nsIDOMDocument"),
            methods: Some(&[
                    /* attribute nsIDOMNode popupNode; */
                    Method {
                        name: "get_popupNode",
                        abi: "C",
                        params: &[Param { name: "aPopupNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_popupNode",
                        abi: "C",
                        params: &[Param { name: "aPopupNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode popupRangeParent; */
                    Method {
                        name: "get_popupRangeParent",
                        abi: "C",
                        params: &[Param { name: "aPopupRangeParent", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long popupRangeOffset; */
                    Method {
                        name: "get_popupRangeOffset",
                        abi: "C",
                        params: &[Param { name: "aPopupRangeOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIDOMNode tooltipNode; */
                    Method {
                        name: "get_tooltipNode",
                        abi: "C",
                        params: &[Param { name: "aTooltipNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_tooltipNode",
                        abi: "C",
                        params: &[Param { name: "aTooltipNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMXULCommandDispatcher commandDispatcher; */
                    Method {
                        name: "get_commandDispatcher",
                        abi: "C",
                        params: &[Param { name: "aCommandDispatcher", ty: "*mut *const nsIDOMXULCommandDispatcher" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long height; */
                    Method {
                        name: "get_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNodeList getElementsByAttribute (in DOMString name, in DOMString value); */
                    Method {
                        name: "getElementsByAttribute",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMNodeList" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNodeList getElementsByAttributeNS (in DOMString namespaceURI, in DOMString name, in DOMString value); */
                    Method {
                        name: "getElementsByAttributeNS",
                        abi: "C",
                        params: &[Param { name: "namespaceURI", ty: "*const nsAString" }, Param { name: "name", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMNodeList" }],
                        ret: "nsresult",
                    },

                    /* void addBroadcastListenerFor (in nsIDOMElement broadcaster, in nsIDOMElement observer, in DOMString attr); */
                    Method {
                        name: "addBroadcastListenerFor",
                        abi: "C",
                        params: &[Param { name: "broadcaster", ty: "*const nsIDOMElement" }, Param { name: "observer", ty: "*const nsIDOMElement" }, Param { name: "attr", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void removeBroadcastListenerFor (in nsIDOMElement broadcaster, in nsIDOMElement observer, in DOMString attr); */
                    Method {
                        name: "removeBroadcastListenerFor",
                        abi: "C",
                        params: &[Param { name: "broadcaster", ty: "*const nsIDOMElement" }, Param { name: "observer", ty: "*const nsIDOMElement" }, Param { name: "attr", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void persist (in DOMString id, in DOMString attr); */
                    Method {
                        name: "persist",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsAString" }, Param { name: "attr", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIBoxObject getBoxObjectFor (in nsIDOMElement elt); */
                    Method {
                        name: "getBoxObjectFor",
                        abi: "C",
                        params: &[Param { name: "elt", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsIBoxObject" }],
                        ret: "nsresult",
                    },

                    /* void loadOverlay (in DOMString url, in nsIObserver aObserver); */
                    Method {
                        name: "loadOverlay",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsAString" }, Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

