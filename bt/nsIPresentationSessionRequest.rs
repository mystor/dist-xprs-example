//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationSessionRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationSessionRequest",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIPresentationDevice device; */
                    Method {
                        name: "get_device",
                        abi: "C",
                        params: &[Param { name: "aDevice", ty: "*mut *const nsIPresentationDevice" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString url; */
                    Method {
                        name: "get_url",
                        abi: "C",
                        params: &[Param { name: "aUrl", ty: "*mut nsAString" }],
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

                    ]),
        },


        ]; D}

