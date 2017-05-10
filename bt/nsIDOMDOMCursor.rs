//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDOMCursor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICursorContinueCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleContinue (); */
                    Method {
                        name: "handleContinue",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDOMDOMCursor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean done; */
                    Method {
                        name: "get_done",
                        abi: "C",
                        params: &[Param { name: "aDone", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void continue (); */
                    Method {
                        name: "continue_",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

