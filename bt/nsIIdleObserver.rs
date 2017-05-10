//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIdleObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIdleObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long time; */
                    Method {
                        name: "get_time",
                        abi: "C",
                        params: &[Param { name: "aTime", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void onidle (); */
                    Method {
                        name: "onidle",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onactive (); */
                    Method {
                        name: "onactive",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

