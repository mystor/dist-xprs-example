//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedProcessor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedProcessor",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* attribute nsIFeedResultListener listener; */
                    Method {
                        name: "get_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIFeedResultListener" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIFeedResultListener" }],
                        ret: "nsresult",
                    },

                    /* void parseFromStream (in nsIInputStream stream, in nsIURI uri); */
                    Method {
                        name: "parseFromStream",
                        abi: "C",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }, Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void parseFromString (in AString str, in nsIURI uri); */
                    Method {
                        name: "parseFromString",
                        abi: "C",
                        params: &[Param { name: "str", ty: "*const nsAString" }, Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void parseAsync (in nsIRequestObserver requestObserver, in nsIURI uri); */
                    Method {
                        name: "parseAsync",
                        abi: "C",
                        params: &[Param { name: "requestObserver", ty: "*const nsIRequestObserver" }, Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

