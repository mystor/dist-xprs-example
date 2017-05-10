//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMModalContentWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMModalContentWindow",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIVariant dialogArguments; */
                    Method {
                        name: "get_dialogArguments",
                        abi: "C",
                        params: &[Param { name: "aDialogArguments", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIVariant returnValue; */
                    Method {
                        name: "get_returnValue",
                        abi: "C",
                        params: &[Param { name: "aReturnValue", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_returnValue",
                        abi: "C",
                        params: &[Param { name: "aReturnValue", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

