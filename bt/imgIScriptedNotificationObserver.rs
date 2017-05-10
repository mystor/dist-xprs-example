//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIScriptedNotificationObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgIScriptedNotificationObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void sizeAvailable (in imgIRequest aRequest); */
                    Method {
                        name: "sizeAvailable",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void frameUpdate (in imgIRequest aRequest); */
                    Method {
                        name: "frameUpdate",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void frameComplete (in imgIRequest aRequest); */
                    Method {
                        name: "frameComplete",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void loadComplete (in imgIRequest aRequest); */
                    Method {
                        name: "loadComplete",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void decodeComplete (in imgIRequest aRequest); */
                    Method {
                        name: "decodeComplete",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void discard (in imgIRequest aRequest); */
                    Method {
                        name: "discard",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void isAnimated (in imgIRequest aRequest); */
                    Method {
                        name: "isAnimated",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void hasTransparency (in imgIRequest aRequest); */
                    Method {
                        name: "hasTransparency",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

