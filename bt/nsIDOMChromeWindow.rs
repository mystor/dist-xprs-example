//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMChromeWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMChromeWindow",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned short windowState; */
                    Method {
                        name: "get_windowState",
                        abi: "C",
                        params: &[Param { name: "aWindowState", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIBrowserDOMWindow browserDOMWindow; */
                    Method {
                        name: "get_browserDOMWindow",
                        abi: "C",
                        params: &[Param { name: "aBrowserDOMWindow", ty: "*mut *const nsIBrowserDOMWindow" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_browserDOMWindow",
                        abi: "C",
                        params: &[Param { name: "aBrowserDOMWindow", ty: "*const nsIBrowserDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* void getAttention (); */
                    Method {
                        name: "getAttention",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void getAttentionWithCycleCount (in long aCycleCount); */
                    Method {
                        name: "getAttentionWithCycleCount",
                        abi: "C",
                        params: &[Param { name: "aCycleCount", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setCursor (in DOMString cursor); */
                    Method {
                        name: "setCursor",
                        abi: "C",
                        params: &[Param { name: "cursor", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void maximize (); */
                    Method {
                        name: "maximize",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void minimize (); */
                    Method {
                        name: "minimize",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void restore (); */
                    Method {
                        name: "restore",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void notifyDefaultButtonLoaded (in nsIDOMElement defaultButton); */
                    Method {
                        name: "notifyDefaultButtonLoaded",
                        abi: "C",
                        params: &[Param { name: "defaultButton", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIMessageBroadcaster messageManager; */
                    Method {
                        name: "get_messageManager",
                        abi: "C",
                        params: &[Param { name: "aMessageManager", ty: "*mut *const nsIMessageBroadcaster" }],
                        ret: "nsresult",
                    },

                    /* nsIMessageBroadcaster getGroupMessageManager (in AString group); */
                    Method {
                        name: "getGroupMessageManager",
                        abi: "C",
                        params: &[Param { name: "group", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIMessageBroadcaster" }],
                        ret: "nsresult",
                    },

                    /* void beginWindowMove (in nsIDOMEvent mouseDownEvent, [optional] in nsIDOMElement panel); */
                    Method {
                        name: "beginWindowMove",
                        abi: "C",
                        params: &[Param { name: "mouseDownEvent", ty: "*const nsIDOMEvent" }, Param { name: "panel", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void setOpenerForInitialContentBrowser (in mozIDOMWindowProxy aOpener); */
                    Method {
                        name: "setOpenerForInitialContentBrowser",
                        abi: "C",
                        params: &[Param { name: "aOpener", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindowProxy takeOpenerForInitialContentBrowser (); */
                    Method {
                        name: "takeOpenerForInitialContentBrowser",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

