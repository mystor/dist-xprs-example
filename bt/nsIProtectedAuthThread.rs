//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtectedAuthThread.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtectedAuthThread",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void login (in nsIObserver observer); */
                    Method {
                        name: "login",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPKCS11Slot slot; */
                    Method {
                        name: "get_slot",
                        abi: "C",
                        params: &[Param { name: "aSlot", ty: "*mut *const nsIPKCS11Slot" }],
                        ret: "nsresult",
                    },

                    /* AString getTokenName (); */
                    Method {
                        name: "getTokenName",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

