//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpPushListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpPushListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onPush (in nsIHttpChannel associatedChannel, in nsIHttpChannel pushChannel); */
                    Method {
                        name: "onPush",
                        abi: "C",
                        params: &[Param { name: "associatedChannel", ty: "*const nsIHttpChannel" }, Param { name: "pushChannel", ty: "*const nsIHttpChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

