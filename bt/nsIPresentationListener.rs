//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationAvailabilityListener",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIPresentationSessionListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notifyStateChange (in DOMString sessionId, in unsigned short state, in nsresult reason); */
                    Method {
                        name: "notifyStateChange",
                        abi: "C",
                        params: &[Param { name: "sessionId", ty: "*const nsAString" }, Param { name: "state", ty: "libc::uint16_t" }, Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void notifyMessage (in DOMString sessionId, in ACString data, in boolean isBinary); */
                    Method {
                        name: "notifyMessage",
                        abi: "C",
                        params: &[Param { name: "sessionId", ty: "*const nsAString" }, Param { name: "data", ty: "*const nsACString" }, Param { name: "isBinary", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationRespondingListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notifySessionConnect (in unsigned long long windowId, in DOMString sessionId); */
                    Method {
                        name: "notifySessionConnect",
                        abi: "C",
                        params: &[Param { name: "windowId", ty: "libc::uint64_t" }, Param { name: "sessionId", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

