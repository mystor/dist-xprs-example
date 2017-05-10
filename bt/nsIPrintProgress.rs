//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintProgress.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrintProgress",
            base: Some("nsIWebProgressListener"),
            methods: Some(&[
                    /* void openProgressDialog (in mozIDOMWindowProxy parent, in string dialogURL, in nsISupports parameters, in nsIObserver openDialogObserver, out boolean notifyOnOpen); */
                    Method {
                        name: "openProgressDialog",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "dialogURL", ty: "*const libc::c_char" }, Param { name: "parameters", ty: "*const nsISupports" }, Param { name: "openDialogObserver", ty: "*const nsIObserver" }, Param { name: "notifyOnOpen", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void closeProgressDialog (in boolean forceClose); */
                    Method {
                        name: "closeProgressDialog",
                        abi: "C",
                        params: &[Param { name: "forceClose", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void registerListener (in nsIWebProgressListener listener); */
                    Method {
                        name: "registerListener",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIWebProgressListener" }],
                        ret: "nsresult",
                    },

                    /* void unregisterListener (in nsIWebProgressListener listener); */
                    Method {
                        name: "unregisterListener",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIWebProgressListener" }],
                        ret: "nsresult",
                    },

                    /* void doneIniting (); */
                    Method {
                        name: "doneIniting",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsIPrompt getPrompter (); */
                    Method {
                        name: "getPrompter",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIPrompt" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean processCanceledByUser; */
                    Method {
                        name: "get_processCanceledByUser",
                        abi: "C",
                        params: &[Param { name: "aProcessCanceledByUser", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_processCanceledByUser",
                        abi: "C",
                        params: &[Param { name: "aProcessCanceledByUser", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

