//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addCommandObserver (in nsIObserver aCommandObserver, in string aCommandToObserve); */
                    Method {
                        name: "addCommandObserver",
                        abi: "C",
                        params: &[Param { name: "aCommandObserver", ty: "*const nsIObserver" }, Param { name: "aCommandToObserve", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void removeCommandObserver (in nsIObserver aCommandObserver, in string aCommandObserved); */
                    Method {
                        name: "removeCommandObserver",
                        abi: "C",
                        params: &[Param { name: "aCommandObserver", ty: "*const nsIObserver" }, Param { name: "aCommandObserved", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* boolean isCommandSupported (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
                    Method {
                        name: "isCommandSupported",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aTargetWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isCommandEnabled (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
                    Method {
                        name: "isCommandEnabled",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aTargetWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void getCommandState (in string aCommandName, in mozIDOMWindowProxy aTargetWindow, in nsICommandParams aCommandParams); */
                    Method {
                        name: "getCommandState",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aTargetWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aCommandParams", ty: "*const nsICommandParams" }],
                        ret: "nsresult",
                    },

                    /* void doCommand (in string aCommandName, in nsICommandParams aCommandParams, in mozIDOMWindowProxy aTargetWindow); */
                    Method {
                        name: "doCommand",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandParams", ty: "*const nsICommandParams" }, Param { name: "aTargetWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

