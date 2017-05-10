//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIController",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean isCommandEnabled (in string command); */
                    Method {
                        name: "isCommandEnabled",
                        abi: "C",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean supportsCommand (in string command); */
                    Method {
                        name: "supportsCommand",
                        abi: "C",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void doCommand (in string command); */
                    Method {
                        name: "doCommand",
                        abi: "C",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void onEvent (in string eventName); */
                    Method {
                        name: "onEvent",
                        abi: "C",
                        params: &[Param { name: "eventName", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICommandController",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIControllerCommandGroup",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addCommandToGroup (in string aCommand, in string aGroup); */
                    Method {
                        name: "addCommandToGroup",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "aGroup", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void removeCommandFromGroup (in string aCommand, in string aGroup); */
                    Method {
                        name: "removeCommandFromGroup",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "aGroup", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* boolean isCommandInGroup (in string aCommand, in string aGroup); */
                    Method {
                        name: "isCommandInGroup",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "aGroup", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator getGroupsEnumerator (); */
                    Method {
                        name: "getGroupsEnumerator",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator getEnumeratorForGroup (in string aGroup); */
                    Method {
                        name: "getEnumeratorForGroup",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

