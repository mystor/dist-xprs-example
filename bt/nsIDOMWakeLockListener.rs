//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWakeLockListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMMozWakeLockListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void callback (in DOMString aTopic, in DOMString aState); */
                    Method {
                        name: "callback",
                        abi: "C",
                        params: &[Param { name: "aTopic", ty: "*const nsAString" }, Param { name: "aState", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

