//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationDeviceProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationDeviceListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addDevice (in nsIPresentationDevice device); */
                    Method {
                        name: "addDevice",
                        abi: "C",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }],
                        ret: "nsresult",
                    },

                    /* void removeDevice (in nsIPresentationDevice device); */
                    Method {
                        name: "removeDevice",
                        abi: "C",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }],
                        ret: "nsresult",
                    },

                    /* void updateDevice (in nsIPresentationDevice device); */
                    Method {
                        name: "updateDevice",
                        abi: "C",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }],
                        ret: "nsresult",
                    },

                    /* void onSessionRequest (in nsIPresentationDevice device, in DOMString url, in DOMString presentationId, in nsIPresentationControlChannel controlChannel); */
                    Method {
                        name: "onSessionRequest",
                        abi: "C",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }, Param { name: "url", ty: "*const nsAString" }, Param { name: "presentationId", ty: "*const nsAString" }, Param { name: "controlChannel", ty: "*const nsIPresentationControlChannel" }],
                        ret: "nsresult",
                    },

                    /* void onTerminateRequest (in nsIPresentationDevice device, in DOMString presentationId, in nsIPresentationControlChannel controlChannel, in boolean aIsFromReceiver); */
                    Method {
                        name: "onTerminateRequest",
                        abi: "C",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }, Param { name: "presentationId", ty: "*const nsAString" }, Param { name: "controlChannel", ty: "*const nsIPresentationControlChannel" }, Param { name: "aIsFromReceiver", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void onReconnectRequest (in nsIPresentationDevice device, in DOMString url, in DOMString presentationId, in nsIPresentationControlChannel controlChannel); */
                    Method {
                        name: "onReconnectRequest",
                        abi: "C",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }, Param { name: "url", ty: "*const nsAString" }, Param { name: "presentationId", ty: "*const nsAString" }, Param { name: "controlChannel", ty: "*const nsIPresentationControlChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationDeviceProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIPresentationDeviceListener listener; */
                    Method {
                        name: "get_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIPresentationDeviceListener" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIPresentationDeviceListener" }],
                        ret: "nsresult",
                    },

                    /* void forceDiscovery (); */
                    Method {
                        name: "forceDiscovery",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

