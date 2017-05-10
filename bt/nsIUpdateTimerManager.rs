//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUpdateTimerManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUpdateTimerManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void registerTimer (in AString id, in nsITimerCallback callback, in unsigned long interval); */
                    Method {
                        name: "registerTimer",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsAString" }, Param { name: "callback", ty: "*const nsITimerCallback" }, Param { name: "interval", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void unregisterTimer (in AString id); */
                    Method {
                        name: "unregisterTimer",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

