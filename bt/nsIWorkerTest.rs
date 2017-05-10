//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWorkerTest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWorkerTestCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onmessage (in DOMString data); */
                    Method {
                        name: "onmessage",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void onerror (in DOMString data); */
                    Method {
                        name: "onerror",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWorkerTest",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void postMessage (in DOMString data); */
                    Method {
                        name: "postMessage",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void terminate (); */
                    Method {
                        name: "terminate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* attribute nsIWorkerTestCallback callback; */
                    Method {
                        name: "get_callback",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*mut *const nsIWorkerTestCallback" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_callback",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIWorkerTestCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

