//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIReflowObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIReflowObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void reflow (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
                    Method {
                        name: "reflow",
                        abi: "C",
                        params: &[Param { name: "start", ty: "DOMHighResTimeStamp" }, Param { name: "end", ty: "DOMHighResTimeStamp" }],
                        ret: "nsresult",
                    },

                    /* void reflowInterruptible (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
                    Method {
                        name: "reflowInterruptible",
                        abi: "C",
                        params: &[Param { name: "start", ty: "DOMHighResTimeStamp" }, Param { name: "end", ty: "DOMHighResTimeStamp" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

