//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICaptivePortalDetector.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICaptivePortalCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void prepare (); */
                    Method {
                        name: "prepare",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void complete (in bool success); */
                    Method {
                        name: "complete",
                        abi: "C",
                        params: &[Param { name: "success", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICaptivePortalDetector",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void checkCaptivePortal (in wstring ifname, in nsICaptivePortalCallback callback); */
                    Method {
                        name: "checkCaptivePortal",
                        abi: "C",
                        params: &[Param { name: "ifname", ty: "*const libc::int16_t" }, Param { name: "callback", ty: "*const nsICaptivePortalCallback" }],
                        ret: "nsresult",
                    },

                    /* void abort (in wstring ifname); */
                    Method {
                        name: "abort",
                        abi: "C",
                        params: &[Param { name: "ifname", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void cancelLogin (in wstring eventId); */
                    Method {
                        name: "cancelLogin",
                        abi: "C",
                        params: &[Param { name: "eventId", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void finishPreparation (in wstring ifname); */
                    Method {
                        name: "finishPreparation",
                        abi: "C",
                        params: &[Param { name: "ifname", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

