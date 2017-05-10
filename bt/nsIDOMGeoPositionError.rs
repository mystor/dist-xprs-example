//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoPositionError.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMGeoPositionError",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute short code; */
                    Method {
                        name: "get_code",
                        abi: "C",
                        params: &[Param { name: "aCode", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString message; */
                    Method {
                        name: "get_message",
                        abi: "C",
                        params: &[Param { name: "aMessage", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

