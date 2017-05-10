//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThreadInternal.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIThreadInternal",
            base: Some("nsIThread"),
            methods: Some(&[
                    /* attribute nsIThreadObserver observer; */
                    Method {
                        name: "get_observer",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*mut *const nsIThreadObserver" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_observer",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIThreadObserver" }],
                        ret: "nsresult",
                    },

                    /* void addObserver (in nsIThreadObserver observer); */
                    Method {
                        name: "addObserver",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIThreadObserver" }],
                        ret: "nsresult",
                    },

                    /* void removeObserver (in nsIThreadObserver observer); */
                    Method {
                        name: "removeObserver",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIThreadObserver" }],
                        ret: "nsresult",
                    },

                    /* [noscript] nsIEventTarget pushEventQueue (); */
                    Method {
                        name: "pushEventQueue",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIEventTarget" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void popEventQueue (in nsIEventTarget aInnermostTarget); */
                    Method {
                        name: "popEventQueue",
                        abi: "C",
                        params: &[Param { name: "aInnermostTarget", ty: "*const nsIEventTarget" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIThreadObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onDispatchedEvent (in nsIThreadInternal thread); */
                    Method {
                        name: "onDispatchedEvent",
                        abi: "C",
                        params: &[Param { name: "thread", ty: "*const nsIThreadInternal" }],
                        ret: "nsresult",
                    },

                    /* void onProcessNextEvent (in nsIThreadInternal thread, in boolean mayWait); */
                    Method {
                        name: "onProcessNextEvent",
                        abi: "C",
                        params: &[Param { name: "thread", ty: "*const nsIThreadInternal" }, Param { name: "mayWait", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void afterProcessNextEvent (in nsIThreadInternal thread, in bool eventWasProcessed); */
                    Method {
                        name: "afterProcessNextEvent",
                        abi: "C",
                        params: &[Param { name: "thread", ty: "*const nsIThreadInternal" }, Param { name: "eventWasProcessed", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

