//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEncodedChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEncodedChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIUTF8StringEnumerator contentEncodings; */
                    Method {
                        name: "get_contentEncodings",
                        abi: "C",
                        params: &[Param { name: "aContentEncodings", ty: "*mut *const nsIUTF8StringEnumerator" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean applyConversion; */
                    Method {
                        name: "get_applyConversion",
                        abi: "C",
                        params: &[Param { name: "aApplyConversion", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_applyConversion",
                        abi: "C",
                        params: &[Param { name: "aApplyConversion", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void doApplyContentConversions (in nsIStreamListener aNextListener, out nsIStreamListener aNewNextListener, in nsISupports aCtxt); */
                    Method {
                        name: "doApplyContentConversions",
                        abi: "C",
                        params: &[Param { name: "aNextListener", ty: "*const nsIStreamListener" }, Param { name: "aNewNextListener", ty: "*mut *const nsIStreamListener" }, Param { name: "aCtxt", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

