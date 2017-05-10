//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWorkerDebugger.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWorkerDebuggerListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onClose (); */
                    Method {
                        name: "onClose",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onError (in DOMString filename, in unsigned long lineno, in DOMString message); */
                    Method {
                        name: "onError",
                        abi: "C",
                        params: &[Param { name: "filename", ty: "*const nsAString" }, Param { name: "lineno", ty: "libc::uint32_t" }, Param { name: "message", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void onMessage (in DOMString message); */
                    Method {
                        name: "onMessage",
                        abi: "C",
                        params: &[Param { name: "message", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWorkerDebugger",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute bool isClosed; */
                    Method {
                        name: "get_isClosed",
                        abi: "C",
                        params: &[Param { name: "aIsClosed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool isChrome; */
                    Method {
                        name: "get_isChrome",
                        abi: "C",
                        params: &[Param { name: "aIsChrome", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool isInitialized; */
                    Method {
                        name: "get_isInitialized",
                        abi: "C",
                        params: &[Param { name: "aIsInitialized", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIWorkerDebugger parent; */
                    Method {
                        name: "get_parent",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*mut *const nsIWorkerDebugger" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString url; */
                    Method {
                        name: "get_url",
                        abi: "C",
                        params: &[Param { name: "aUrl", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIDOMWindow window; */
                    Method {
                        name: "get_window",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*mut *const mozIDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "get_principal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long serviceWorkerID; */
                    Method {
                        name: "get_serviceWorkerID",
                        abi: "C",
                        params: &[Param { name: "aServiceWorkerID", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void initialize (in DOMString url); */
                    Method {
                        name: "initialize",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* [binaryname(PostMessageMoz)] void postMessage (in DOMString message); */
                    Method {
                        name: "PostMessageMoz",
                        abi: "C",
                        params: &[Param { name: "message", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void addListener (in nsIWorkerDebuggerListener listener); */
                    Method {
                        name: "addListener",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIWorkerDebuggerListener" }],
                        ret: "nsresult",
                    },

                    /* void removeListener (in nsIWorkerDebuggerListener listener); */
                    Method {
                        name: "removeListener",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIWorkerDebuggerListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

