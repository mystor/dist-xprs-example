//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecureBrowserUI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecureBrowserUI",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in mozIDOMWindowProxy window); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void setDocShell (in nsIDocShell docShell); */
                    Method {
                        name: "setDocShell",
                        abi: "C",
                        params: &[Param { name: "docShell", ty: "*const nsIDocShell" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

