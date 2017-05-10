//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintStatusFeedback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrintStatusFeedback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void showStatusString (in wstring status); */
                    Method {
                        name: "showStatusString",
                        abi: "C",
                        params: &[Param { name: "status", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void startMeteors (); */
                    Method {
                        name: "startMeteors",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void stopMeteors (); */
                    Method {
                        name: "stopMeteors",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void showProgress (in long percent); */
                    Method {
                        name: "showProgress",
                        abi: "C",
                        params: &[Param { name: "percent", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setDocShell (in nsIDocShell shell, in mozIDOMWindowProxy window); */
                    Method {
                        name: "setDocShell",
                        abi: "C",
                        params: &[Param { name: "shell", ty: "*const nsIDocShell" }, Param { name: "window", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void closeWindow (); */
                    Method {
                        name: "closeWindow",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

