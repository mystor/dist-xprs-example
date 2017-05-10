//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAppShellService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAppShellService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIXULWindow createTopLevelWindow (in nsIXULWindow aParent, in nsIURI aUrl, in uint32_t aChromeMask, in long aInitialWidth, in long aInitialHeight, in nsITabParent aOpeningTab, in mozIDOMWindowProxy aOpenerWindow); */
                    Method {
                        name: "createTopLevelWindow",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const nsIXULWindow" }, Param { name: "aUrl", ty: "*const nsIURI" }, Param { name: "aChromeMask", ty: "uint32_t" }, Param { name: "aInitialWidth", ty: "libc::int32_t" }, Param { name: "aInitialHeight", ty: "libc::int32_t" }, Param { name: "aOpeningTab", ty: "*const nsITabParent" }, Param { name: "aOpenerWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* nsIWindowlessBrowser createWindowlessBrowser ([optional] in bool aIsChrome); */
                    Method {
                        name: "createWindowlessBrowser",
                        abi: "C",
                        params: &[Param { name: "aIsChrome", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIWindowlessBrowser" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void createHiddenWindow (); */
                    Method {
                        name: "createHiddenWindow",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void destroyHiddenWindow (); */
                    Method {
                        name: "destroyHiddenWindow",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [noscript] void setScreenId (in uint32_t aScreenId); */
                    Method {
                        name: "setScreenId",
                        abi: "C",
                        params: &[Param { name: "aScreenId", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIXULWindow hiddenWindow; */
                    Method {
                        name: "get_hiddenWindow",
                        abi: "C",
                        params: &[Param { name: "aHiddenWindow", ty: "*mut *const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIDOMWindowProxy hiddenDOMWindow; */
                    Method {
                        name: "get_hiddenDOMWindow",
                        abi: "C",
                        params: &[Param { name: "aHiddenDOMWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIXULWindow hiddenPrivateWindow; */
                    Method {
                        name: "get_hiddenPrivateWindow",
                        abi: "C",
                        params: &[Param { name: "aHiddenPrivateWindow", ty: "*mut *const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIDOMWindowProxy hiddenPrivateDOMWindow; */
                    Method {
                        name: "get_hiddenPrivateDOMWindow",
                        abi: "C",
                        params: &[Param { name: "aHiddenPrivateDOMWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean applicationProvidedHiddenWindow; */
                    Method {
                        name: "get_applicationProvidedHiddenWindow",
                        abi: "C",
                        params: &[Param { name: "aApplicationProvidedHiddenWindow", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void registerTopLevelWindow (in nsIXULWindow aWindow); */
                    Method {
                        name: "registerTopLevelWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* void unregisterTopLevelWindow (in nsIXULWindow aWindow); */
                    Method {
                        name: "unregisterTopLevelWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* [noscript] readonly attribute boolean hasHiddenPrivateWindow; */
                    Method {
                        name: "get_hasHiddenPrivateWindow",
                        abi: "C",
                        params: &[Param { name: "aHasHiddenPrivateWindow", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* bool startEventLoopLagTracking (); */
                    Method {
                        name: "startEventLoopLagTracking",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void stopEventLoopLagTracking (); */
                    Method {
                        name: "stopEventLoopLagTracking",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

