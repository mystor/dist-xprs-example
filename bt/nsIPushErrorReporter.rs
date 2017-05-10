//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPushErrorReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPushErrorReporter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void reportDeliveryError (in DOMString messageId, [optional] in uint16_t reason); */
                    Method {
                        name: "reportDeliveryError",
                        abi: "C",
                        params: &[Param { name: "messageId", ty: "*const nsAString" }, Param { name: "reason", ty: "uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

