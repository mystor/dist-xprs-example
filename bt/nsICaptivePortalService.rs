//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICaptivePortalService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICaptivePortalServiceCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void complete (in bool success, in nsresult error); */
                    Method {
                        name: "complete",
                        abi: "C",
                        params: &[Param { name: "success", ty: "bool" }, Param { name: "error", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICaptivePortalService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void recheckCaptivePortal (); */
                    Method {
                        name: "recheckCaptivePortal",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute long state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long lastChecked; */
                    Method {
                        name: "get_lastChecked",
                        abi: "C",
                        params: &[Param { name: "aLastChecked", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

