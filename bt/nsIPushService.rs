//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPushService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPushSubscription",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIPushSubscriptionCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onPushSubscription (in nsresult status, in nsIPushSubscription subscription); */
                    Method {
                        name: "onPushSubscription",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }, Param { name: "subscription", ty: "*const nsIPushSubscription" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUnsubscribeResultCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onUnsubscribe (in nsresult status, in bool success); */
                    Method {
                        name: "onUnsubscribe",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }, Param { name: "success", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPushClearResultCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onClear (in nsresult status); */
                    Method {
                        name: "onClear",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPushService",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIPushQuotaManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notificationForOriginShown (in string origin); */
                    Method {
                        name: "notificationForOriginShown",
                        abi: "C",
                        params: &[Param { name: "origin", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void notificationForOriginClosed (in string origin); */
                    Method {
                        name: "notificationForOriginClosed",
                        abi: "C",
                        params: &[Param { name: "origin", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

