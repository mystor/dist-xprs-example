//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamListenerTee.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamListenerTee",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* void init (in nsIStreamListener listener, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIStreamListener" }, Param { name: "sink", ty: "*const nsIOutputStream" }, Param { name: "requestObserver", ty: "*const nsIRequestObserver" }],
                        ret: "nsresult",
                    },

                    /* void initAsync (in nsIStreamListener listener, in nsIEventTarget eventTarget, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
                    Method {
                        name: "initAsync",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsIStreamListener" }, Param { name: "eventTarget", ty: "*const nsIEventTarget" }, Param { name: "sink", ty: "*const nsIOutputStream" }, Param { name: "requestObserver", ty: "*const nsIRequestObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

