//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMClipboardEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMClipboardEvent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMDataTransfer clipboardData; */
                    Method {
                        name: "get_clipboardData",
                        abi: "C",
                        params: &[Param { name: "aClipboardData", ty: "*mut *const nsIDOMDataTransfer" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void initClipboardEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIDOMDataTransfer clipboardData); */
                    Method {
                        name: "initClipboardEvent",
                        abi: "C",
                        params: &[Param { name: "typeArg", ty: "*const nsAString" }, Param { name: "canBubbleArg", ty: "bool" }, Param { name: "cancelableArg", ty: "bool" }, Param { name: "clipboardData", ty: "*const nsIDOMDataTransfer" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

