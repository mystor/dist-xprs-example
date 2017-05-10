//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMessageManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMessageListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void receiveMessage (); */
                    Method {
                        name: "receiveMessage",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIMessageListenerManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addMessageListener (in AString messageName, in nsIMessageListener listener, [optional] in boolean listenWhenClosed); */
                    Method {
                        name: "addMessageListener",
                        abi: "C",
                        params: &[Param { name: "messageName", ty: "*const nsAString" }, Param { name: "listener", ty: "*const nsIMessageListener" }, Param { name: "listenWhenClosed", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void removeMessageListener (in AString messageName, in nsIMessageListener listener); */
                    Method {
                        name: "removeMessageListener",
                        abi: "C",
                        params: &[Param { name: "messageName", ty: "*const nsAString" }, Param { name: "listener", ty: "*const nsIMessageListener" }],
                        ret: "nsresult",
                    },

                    /* void addWeakMessageListener (in AString messageName, in nsIMessageListener listener); */
                    Method {
                        name: "addWeakMessageListener",
                        abi: "C",
                        params: &[Param { name: "messageName", ty: "*const nsAString" }, Param { name: "listener", ty: "*const nsIMessageListener" }],
                        ret: "nsresult",
                    },

                    /* void removeWeakMessageListener (in AString messageName, in nsIMessageListener listener); */
                    Method {
                        name: "removeWeakMessageListener",
                        abi: "C",
                        params: &[Param { name: "messageName", ty: "*const nsAString" }, Param { name: "listener", ty: "*const nsIMessageListener" }],
                        ret: "nsresult",
                    },

                    /* [notxpcom] boolean markForCC (); */
                    Method {
                        name: "markForCC",
                        abi: "C",
                        params: &[],
                        ret: "bool",
                    },

                    ]),
        },


        Interface {
            name: "nsIMessageSender",
            base: Some("nsIMessageListenerManager"),
            methods: None,
        },


        Interface {
            name: "nsIMessageBroadcaster",
            base: Some("nsIMessageListenerManager"),
            methods: None,
        },


        Interface {
            name: "nsISyncMessageSender",
            base: Some("nsIMessageSender"),
            methods: None,
        },


        Interface {
            name: "nsIMessageManagerGlobal",
            base: Some("nsISyncMessageSender"),
            methods: Some(&[
                    /* void dump (in DOMString aStr); */
                    Method {
                        name: "dump",
                        abi: "C",
                        params: &[Param { name: "aStr", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void privateNoteIntentionalCrash (); */
                    Method {
                        name: "privateNoteIntentionalCrash",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* DOMString atob (in DOMString aAsciiString); */
                    Method {
                        name: "atob",
                        abi: "C",
                        params: &[Param { name: "aAsciiString", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* DOMString btoa (in DOMString aBase64Data); */
                    Method {
                        name: "btoa",
                        abi: "C",
                        params: &[Param { name: "aBase64Data", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentFrameMessageManager",
            base: Some("nsIMessageManagerGlobal"),
            methods: Some(&[
                    /* readonly attribute mozIDOMWindowProxy content; */
                    Method {
                        name: "get_content",
                        abi: "C",
                        params: &[Param { name: "aContent", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDocShell docShell; */
                    Method {
                        name: "get_docShell",
                        abi: "C",
                        params: &[Param { name: "aDocShell", ty: "*mut *const nsIDocShell" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIInProcessContentFrameMessageManager",
            base: Some("nsIContentFrameMessageManager"),
            methods: Some(&[
                    /* [notxpcom] nsIContent getOwnerContent (); */
                    Method {
                        name: "getOwnerContent",
                        abi: "C",
                        params: &[],
                        ret: "*const nsIContent",
                    },

                    /* [notxpcom] void cacheFrameLoader (in nsIFrameLoader aFrameLoader); */
                    Method {
                        name: "cacheFrameLoader",
                        abi: "C",
                        params: &[Param { name: "aFrameLoader", ty: "*const nsIFrameLoader" }],
                        ret: "libc::c_void",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentProcessMessageManager",
            base: Some("nsIMessageManagerGlobal"),
            methods: None,
        },


        Interface {
            name: "nsIFrameScriptLoader",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIProcessScriptLoader",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIGlobalProcessScriptLoader",
            base: Some("nsIProcessScriptLoader"),
            methods: None,
        },


        ]; D}

