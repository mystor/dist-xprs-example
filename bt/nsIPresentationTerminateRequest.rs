//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationTerminateRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationTerminateRequest",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIPresentationDevice device; */
                    Method {
                        name: "get_device",
                        abi: "C",
                        params: &[Param { name: "aDevice", ty: "*mut *const nsIPresentationDevice" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString presentationId; */
                    Method {
                        name: "get_presentationId",
                        abi: "C",
                        params: &[Param { name: "aPresentationId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPresentationControlChannel controlChannel; */
                    Method {
                        name: "get_controlChannel",
                        abi: "C",
                        params: &[Param { name: "aControlChannel", ty: "*mut *const nsIPresentationControlChannel" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isFromReceiver; */
                    Method {
                        name: "get_isFromReceiver",
                        abi: "C",
                        params: &[Param { name: "aIsFromReceiver", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

