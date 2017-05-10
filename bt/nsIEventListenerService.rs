//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEventListenerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEventListenerChange",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMEventTarget target; */
                    Method {
                        name: "get_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*mut *const nsIDOMEventTarget" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray changedListenerNames; */
                    Method {
                        name: "get_changedListenerNames",
                        abi: "C",
                        params: &[Param { name: "aChangedListenerNames", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIListenerChangeListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void listenersChanged (in nsIArray aEventListenerChanges); */
                    Method {
                        name: "listenersChanged",
                        abi: "C",
                        params: &[Param { name: "aEventListenerChanges", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIEventListenerInfo",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIEventListenerService",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

