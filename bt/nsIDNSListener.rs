//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDNSListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDNSListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onLookupComplete (in nsICancelable aRequest, in nsIDNSRecord aRecord, in nsresult aStatus); */
                    Method {
                        name: "onLookupComplete",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsICancelable" }, Param { name: "aRecord", ty: "*const nsIDNSRecord" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDNSListenerProxy",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDNSListener originalListener; */
                    Method {
                        name: "get_originalListener",
                        abi: "C",
                        params: &[Param { name: "aOriginalListener", ty: "*mut *const nsIDNSListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

