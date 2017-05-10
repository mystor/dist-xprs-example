//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCommandEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCommandEvent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString command; */
                    Method {
                        name: "get_command",
                        abi: "C",
                        params: &[Param { name: "aCommand", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void initCommandEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean canCancelArg, in DOMString command); */
                    Method {
                        name: "initCommandEvent",
                        abi: "C",
                        params: &[Param { name: "typeArg", ty: "*const nsAString" }, Param { name: "canBubbleArg", ty: "bool" }, Param { name: "canCancelArg", ty: "bool" }, Param { name: "command", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

