//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULCommandDispatcher.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULCommandDispatcher",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIDOMElement focusedElement; */
                    Method {
                        name: "get_focusedElement",
                        abi: "C",
                        params: &[Param { name: "aFocusedElement", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_focusedElement",
                        abi: "C",
                        params: &[Param { name: "aFocusedElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* attribute mozIDOMWindowProxy focusedWindow; */
                    Method {
                        name: "get_focusedWindow",
                        abi: "C",
                        params: &[Param { name: "aFocusedWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_focusedWindow",
                        abi: "C",
                        params: &[Param { name: "aFocusedWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void addCommandUpdater (in nsIDOMElement updater, in DOMString events, in DOMString targets); */
                    Method {
                        name: "addCommandUpdater",
                        abi: "C",
                        params: &[Param { name: "updater", ty: "*const nsIDOMElement" }, Param { name: "events", ty: "*const nsAString" }, Param { name: "targets", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void removeCommandUpdater (in nsIDOMElement updater); */
                    Method {
                        name: "removeCommandUpdater",
                        abi: "C",
                        params: &[Param { name: "updater", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void updateCommands (in DOMString eventName); */
                    Method {
                        name: "updateCommands",
                        abi: "C",
                        params: &[Param { name: "eventName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIController getControllerForCommand (in string command); */
                    Method {
                        name: "getControllerForCommand",
                        abi: "C",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIController" }],
                        ret: "nsresult",
                    },

                    /* nsIControllers getControllers (); */
                    Method {
                        name: "getControllers",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIControllers" }],
                        ret: "nsresult",
                    },

                    /* void advanceFocus (); */
                    Method {
                        name: "advanceFocus",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void rewindFocus (); */
                    Method {
                        name: "rewindFocus",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void advanceFocusIntoSubtree (in nsIDOMElement elt); */
                    Method {
                        name: "advanceFocusIntoSubtree",
                        abi: "C",
                        params: &[Param { name: "elt", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean suppressFocusScroll; */
                    Method {
                        name: "get_suppressFocusScroll",
                        abi: "C",
                        params: &[Param { name: "aSuppressFocusScroll", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_suppressFocusScroll",
                        abi: "C",
                        params: &[Param { name: "aSuppressFocusScroll", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void lock (); */
                    Method {
                        name: "lock",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void unlock (); */
                    Method {
                        name: "unlock",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

