//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITaskbarProgress.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITaskbarProgress",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setProgressState (in nsTaskbarProgressState state, [optional] in unsigned long long currentValue, [optional] in unsigned long long maxValue); */
                    Method {
                        name: "setProgressState",
                        abi: "C",
                        params: &[Param { name: "state", ty: "nsTaskbarProgressState" }, Param { name: "currentValue", ty: "libc::uint64_t" }, Param { name: "maxValue", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

