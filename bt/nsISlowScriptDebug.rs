//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISlowScriptDebug.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISlowScriptDebugCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleSlowScriptDebug (in nsIDOMWindow aWindow); */
                    Method {
                        name: "handleSlowScriptDebug",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISlowScriptDebuggerStartupCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void finishDebuggerStartup (); */
                    Method {
                        name: "finishDebuggerStartup",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISlowScriptDebugRemoteCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleSlowScriptDebug (in nsIDOMEventTarget aBrowser, in nsISlowScriptDebuggerStartupCallback aCallback); */
                    Method {
                        name: "handleSlowScriptDebug",
                        abi: "C",
                        params: &[Param { name: "aBrowser", ty: "*const nsIDOMEventTarget" }, Param { name: "aCallback", ty: "*const nsISlowScriptDebuggerStartupCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISlowScriptDebug",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsISlowScriptDebugCallback activationHandler; */
                    Method {
                        name: "get_activationHandler",
                        abi: "C",
                        params: &[Param { name: "aActivationHandler", ty: "*mut *const nsISlowScriptDebugCallback" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_activationHandler",
                        abi: "C",
                        params: &[Param { name: "aActivationHandler", ty: "*const nsISlowScriptDebugCallback" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler; */
                    Method {
                        name: "get_remoteActivationHandler",
                        abi: "C",
                        params: &[Param { name: "aRemoteActivationHandler", ty: "*mut *const nsISlowScriptDebugRemoteCallback" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_remoteActivationHandler",
                        abi: "C",
                        params: &[Param { name: "aRemoteActivationHandler", ty: "*const nsISlowScriptDebugRemoteCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

