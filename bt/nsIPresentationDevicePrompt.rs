//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationDevicePrompt.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationDeviceRequest",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString origin; */
                    Method {
                        name: "get_origin",
                        abi: "C",
                        params: &[Param { name: "aOrigin", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray requestURLs; */
                    Method {
                        name: "get_requestURLs",
                        abi: "C",
                        params: &[Param { name: "aRequestURLs", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMEventTarget chromeEventHandler; */
                    Method {
                        name: "get_chromeEventHandler",
                        abi: "C",
                        params: &[Param { name: "aChromeEventHandler", ty: "*mut *const nsIDOMEventTarget" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "get_principal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* void select (in nsIPresentationDevice device); */
                    Method {
                        name: "select",
                        abi: "C",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }],
                        ret: "nsresult",
                    },

                    /* void cancel (in nsresult reason); */
                    Method {
                        name: "cancel",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationDevicePrompt",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void promptDeviceSelection (in nsIPresentationDeviceRequest request); */
                    Method {
                        name: "promptDeviceSelection",
                        abi: "C",
                        params: &[Param { name: "request", ty: "*const nsIPresentationDeviceRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

