//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGeolocationProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGeolocationUpdate",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void update (in nsIDOMGeoPosition position); */
                    Method {
                        name: "update",
                        abi: "C",
                        params: &[Param { name: "position", ty: "*const nsIDOMGeoPosition" }],
                        ret: "nsresult",
                    },

                    /* void notifyError (in unsigned short error); */
                    Method {
                        name: "notifyError",
                        abi: "C",
                        params: &[Param { name: "error", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIGeolocationProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void startup (); */
                    Method {
                        name: "startup",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void watch (in nsIGeolocationUpdate callback); */
                    Method {
                        name: "watch",
                        abi: "C",
                        params: &[Param { name: "callback", ty: "*const nsIGeolocationUpdate" }],
                        ret: "nsresult",
                    },

                    /* void shutdown (); */
                    Method {
                        name: "shutdown",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setHighAccuracy (in boolean enable); */
                    Method {
                        name: "setHighAccuracy",
                        abi: "C",
                        params: &[Param { name: "enable", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

