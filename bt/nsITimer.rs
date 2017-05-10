//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITimer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITimerCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notify (in nsITimer timer); */
                    Method {
                        name: "notify",
                        abi: "C",
                        params: &[Param { name: "timer", ty: "*const nsITimer" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsITimer",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

