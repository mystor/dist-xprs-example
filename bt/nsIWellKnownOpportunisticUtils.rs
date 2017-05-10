//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWellKnownOpportunisticUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWellKnownOpportunisticUtils",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void verify (in ACString aJSON, in ACString aOrigin, in long aAlternatePort); */
                    Method {
                        name: "verify",
                        abi: "C",
                        params: &[Param { name: "aJSON", ty: "*const nsACString" }, Param { name: "aOrigin", ty: "*const nsACString" }, Param { name: "aAlternatePort", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute bool valid; */
                    Method {
                        name: "get_valid",
                        abi: "C",
                        params: &[Param { name: "aValid", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute bool mixed; */
                    Method {
                        name: "get_mixed",
                        abi: "C",
                        params: &[Param { name: "aMixed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute long lifetime; */
                    Method {
                        name: "get_lifetime",
                        abi: "C",
                        params: &[Param { name: "aLifetime", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

