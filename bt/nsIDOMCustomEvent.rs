//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCustomEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCustomEvent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIVariant detail; */
                    Method {
                        name: "get_detail",
                        abi: "C",
                        params: &[Param { name: "aDetail", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void initCustomEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIVariant detailArg); */
                    Method {
                        name: "initCustomEvent",
                        abi: "C",
                        params: &[Param { name: "typeArg", ty: "*const nsAString" }, Param { name: "canBubbleArg", ty: "bool" }, Param { name: "cancelableArg", ty: "bool" }, Param { name: "detailArg", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

