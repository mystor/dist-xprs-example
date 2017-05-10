//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationDeviceManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationDeviceManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean deviceAvailable; */
                    Method {
                        name: "get_deviceAvailable",
                        abi: "C",
                        params: &[Param { name: "aDeviceAvailable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void addDeviceProvider (in nsIPresentationDeviceProvider provider); */
                    Method {
                        name: "addDeviceProvider",
                        abi: "C",
                        params: &[Param { name: "provider", ty: "*const nsIPresentationDeviceProvider" }],
                        ret: "nsresult",
                    },

                    /* void removeDeviceProvider (in nsIPresentationDeviceProvider provider); */
                    Method {
                        name: "removeDeviceProvider",
                        abi: "C",
                        params: &[Param { name: "provider", ty: "*const nsIPresentationDeviceProvider" }],
                        ret: "nsresult",
                    },

                    /* void forceDiscovery (); */
                    Method {
                        name: "forceDiscovery",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsIArray getAvailableDevices ([optional] in nsIArray presentationUrls); */
                    Method {
                        name: "getAvailableDevices",
                        abi: "C",
                        params: &[Param { name: "presentationUrls", ty: "*const nsIArray" }, Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

