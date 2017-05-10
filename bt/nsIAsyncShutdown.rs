//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncShutdown.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncShutdownBlocker",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void blockShutdown (in nsIAsyncShutdownClient aBarrierClient); */
                    Method {
                        name: "blockShutdown",
                        abi: "C",
                        params: &[Param { name: "aBarrierClient", ty: "*const nsIAsyncShutdownClient" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPropertyBag state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut *const nsIPropertyBag" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAsyncShutdownClient",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIAsyncShutdownCompletionCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void done (); */
                    Method {
                        name: "done",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAsyncShutdownBarrier",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIAsyncShutdownClient client; */
                    Method {
                        name: "get_client",
                        abi: "C",
                        params: &[Param { name: "aClient", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPropertyBag state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut *const nsIPropertyBag" }],
                        ret: "nsresult",
                    },

                    /* void wait (in nsIAsyncShutdownCompletionCallback aOnReady); */
                    Method {
                        name: "wait",
                        abi: "C",
                        params: &[Param { name: "aOnReady", ty: "*const nsIAsyncShutdownCompletionCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAsyncShutdownService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIAsyncShutdownBarrier makeBarrier (in AString aName); */
                    Method {
                        name: "makeBarrier",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIAsyncShutdownBarrier" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient profileBeforeChange; */
                    Method {
                        name: "get_profileBeforeChange",
                        abi: "C",
                        params: &[Param { name: "aProfileBeforeChange", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient profileChangeTeardown; */
                    Method {
                        name: "get_profileChangeTeardown",
                        abi: "C",
                        params: &[Param { name: "aProfileChangeTeardown", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient quitApplicationGranted; */
                    Method {
                        name: "get_quitApplicationGranted",
                        abi: "C",
                        params: &[Param { name: "aQuitApplicationGranted", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient sendTelemetry; */
                    Method {
                        name: "get_sendTelemetry",
                        abi: "C",
                        params: &[Param { name: "aSendTelemetry", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient webWorkersShutdown; */
                    Method {
                        name: "get_webWorkersShutdown",
                        abi: "C",
                        params: &[Param { name: "aWebWorkersShutdown", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient xpcomWillShutdown; */
                    Method {
                        name: "get_xpcomWillShutdown",
                        abi: "C",
                        params: &[Param { name: "aXpcomWillShutdown", ty: "*mut *const nsIAsyncShutdownClient" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

