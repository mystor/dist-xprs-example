//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINativeAppSupport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINativeAppSupport",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean start (); */
                    Method {
                        name: "start",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void enable (); */
                    Method {
                        name: "enable",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean stop (); */
                    Method {
                        name: "stop",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void quit (); */
                    Method {
                        name: "quit",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onLastWindowClosing (); */
                    Method {
                        name: "onLastWindowClosing",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void ReOpen (); */
                    Method {
                        name: "ReOpen",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

