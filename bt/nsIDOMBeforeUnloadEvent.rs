//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMBeforeUnloadEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMBeforeUnloadEvent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString returnValue; */
                    Method {
                        name: "get_returnValue",
                        abi: "C",
                        params: &[Param { name: "aReturnValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_returnValue",
                        abi: "C",
                        params: &[Param { name: "aReturnValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

