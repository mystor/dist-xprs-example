//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIServiceWorkerManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIServiceWorkerUnregisterCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void unregisterSucceeded (in bool aState); */
                    Method {
                        name: "unregisterSucceeded",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void unregisterFailed (); */
                    Method {
                        name: "unregisterFailed",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIServiceWorkerInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString scriptSpec; */
                    Method {
                        name: "get_scriptSpec",
                        abi: "C",
                        params: &[Param { name: "aScriptSpec", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString cacheName; */
                    Method {
                        name: "get_cacheName",
                        abi: "C",
                        params: &[Param { name: "aCacheName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIWorkerDebugger debugger; */
                    Method {
                        name: "get_debugger",
                        abi: "C",
                        params: &[Param { name: "aDebugger", ty: "*mut *const nsIWorkerDebugger" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool handlesFetchEvents; */
                    Method {
                        name: "get_handlesFetchEvents",
                        abi: "C",
                        params: &[Param { name: "aHandlesFetchEvents", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime installedTime; */
                    Method {
                        name: "get_installedTime",
                        abi: "C",
                        params: &[Param { name: "aInstalledTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime activatedTime; */
                    Method {
                        name: "get_activatedTime",
                        abi: "C",
                        params: &[Param { name: "aActivatedTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime redundantTime; */
                    Method {
                        name: "get_redundantTime",
                        abi: "C",
                        params: &[Param { name: "aRedundantTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* void attachDebugger (); */
                    Method {
                        name: "attachDebugger",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void detachDebugger (); */
                    Method {
                        name: "detachDebugger",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIServiceWorkerRegistrationInfoListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onChange (); */
                    Method {
                        name: "onChange",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIServiceWorkerRegistrationInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "get_principal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString scope; */
                    Method {
                        name: "get_scope",
                        abi: "C",
                        params: &[Param { name: "aScope", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString scriptSpec; */
                    Method {
                        name: "get_scriptSpec",
                        abi: "C",
                        params: &[Param { name: "aScriptSpec", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime lastUpdateTime; */
                    Method {
                        name: "get_lastUpdateTime",
                        abi: "C",
                        params: &[Param { name: "aLastUpdateTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIServiceWorkerInfo installingWorker; */
                    Method {
                        name: "get_installingWorker",
                        abi: "C",
                        params: &[Param { name: "aInstallingWorker", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIServiceWorkerInfo waitingWorker; */
                    Method {
                        name: "get_waitingWorker",
                        abi: "C",
                        params: &[Param { name: "aWaitingWorker", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIServiceWorkerInfo activeWorker; */
                    Method {
                        name: "get_activeWorker",
                        abi: "C",
                        params: &[Param { name: "aActiveWorker", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "nsresult",
                    },

                    /* nsIServiceWorkerInfo getWorkerByID (in unsigned long long aID); */
                    Method {
                        name: "getWorkerByID",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "libc::uint64_t" }, Param { name: "_retval", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "nsresult",
                    },

                    /* void addListener (in nsIServiceWorkerRegistrationInfoListener listener); */
                    Method {
                        name: "addListener",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIServiceWorkerRegistrationInfoListener" }],
                        ret: "nsresult",
                    },

                    /* void removeListener (in nsIServiceWorkerRegistrationInfoListener listener); */
                    Method {
                        name: "removeListener",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIServiceWorkerRegistrationInfoListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIServiceWorkerManagerListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onRegister (in nsIServiceWorkerRegistrationInfo aInfo); */
                    Method {
                        name: "onRegister",
                        abi: "C",
                        params: &[Param { name: "aInfo", ty: "*const nsIServiceWorkerRegistrationInfo" }],
                        ret: "nsresult",
                    },

                    /* void onUnregister (in nsIServiceWorkerRegistrationInfo aInfo); */
                    Method {
                        name: "onUnregister",
                        abi: "C",
                        params: &[Param { name: "aInfo", ty: "*const nsIServiceWorkerRegistrationInfo" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIServiceWorkerManager",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

