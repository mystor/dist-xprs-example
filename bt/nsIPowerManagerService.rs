//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPowerManagerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPowerManagerService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void powerOff (); */
                    Method {
                        name: "powerOff",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void reboot (); */
                    Method {
                        name: "reboot",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void restart (); */
                    Method {
                        name: "restart",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void addWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
                    Method {
                        name: "addWakeLockListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIDOMMozWakeLockListener" }],
                        ret: "nsresult",
                    },

                    /* void removeWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
                    Method {
                        name: "removeWakeLockListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIDOMMozWakeLockListener" }],
                        ret: "nsresult",
                    },

                    /* DOMString getWakeLockState (in DOMString aTopic); */
                    Method {
                        name: "getWakeLockState",
                        abi: "C",
                        params: &[Param { name: "aTopic", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsISupports newWakeLock (in DOMString aTopic, [optional] in mozIDOMWindow aWindow); */
                    Method {
                        name: "newWakeLock",
                        abi: "C",
                        params: &[Param { name: "aTopic", ty: "*const nsAString" }, Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

