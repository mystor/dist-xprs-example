//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIdleService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIdleService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long idleTime; */
                    Method {
                        name: "get_idleTime",
                        abi: "C",
                        params: &[Param { name: "aIdleTime", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void addIdleObserver (in nsIObserver observer, in unsigned long time); */
                    Method {
                        name: "addIdleObserver",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIObserver" }, Param { name: "time", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void removeIdleObserver (in nsIObserver observer, in unsigned long time); */
                    Method {
                        name: "removeIdleObserver",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIObserver" }, Param { name: "time", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

