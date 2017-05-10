//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMPaintRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMPaintRequest",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMClientRect clientRect; */
                    Method {
                        name: "get_clientRect",
                        abi: "C",
                        params: &[Param { name: "aClientRect", ty: "*mut *const nsIDOMClientRect" }],
                        ret: "nsresult",
                    },

                    /* [binaryname(XPCOMReason)] readonly attribute DOMString reason; */
                    Method {
                        name: "get_XPCOMReason",
                        abi: "C",
                        params: &[Param { name: "aReason", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

