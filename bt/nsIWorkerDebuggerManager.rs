//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWorkerDebuggerManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWorkerDebuggerManagerListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onRegister (in nsIWorkerDebugger debugger); */
                    Method {
                        name: "onRegister",
                        abi: "C",
                        params: &[Param { name: "debugger", ty: "*const nsIWorkerDebugger" }],
                        ret: "nsresult",
                    },

                    /* void onUnregister (in nsIWorkerDebugger debugger); */
                    Method {
                        name: "onUnregister",
                        abi: "C",
                        params: &[Param { name: "debugger", ty: "*const nsIWorkerDebugger" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWorkerDebuggerManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISimpleEnumerator getWorkerDebuggerEnumerator (); */
                    Method {
                        name: "getWorkerDebuggerEnumerator",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void addListener (in nsIWorkerDebuggerManagerListener listener); */
                    Method {
                        name: "addListener",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIWorkerDebuggerManagerListener" }],
                        ret: "nsresult",
                    },

                    /* void removeListener (in nsIWorkerDebuggerManagerListener listener); */
                    Method {
                        name: "removeListener",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIWorkerDebuggerManagerListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

