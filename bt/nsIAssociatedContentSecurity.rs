//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAssociatedContentSecurity.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAssociatedContentSecurity",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute long countSubRequestsBrokenSecurity; */
                    Method {
                        name: "get_countSubRequestsBrokenSecurity",
                        abi: "C",
                        params: &[Param { name: "aCountSubRequestsBrokenSecurity", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_countSubRequestsBrokenSecurity",
                        abi: "C",
                        params: &[Param { name: "aCountSubRequestsBrokenSecurity", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long countSubRequestsNoSecurity; */
                    Method {
                        name: "get_countSubRequestsNoSecurity",
                        abi: "C",
                        params: &[Param { name: "aCountSubRequestsNoSecurity", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_countSubRequestsNoSecurity",
                        abi: "C",
                        params: &[Param { name: "aCountSubRequestsNoSecurity", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void flush (); */
                    Method {
                        name: "flush",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

