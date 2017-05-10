//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolProxyService2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtocolProxyService2",
            base: Some("nsIProtocolProxyService"),
            methods: Some(&[
                    /* void reloadPAC (); */
                    Method {
                        name: "reloadPAC",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsICancelable asyncResolve2 (in nsIChannel aChannel, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback); */
                    Method {
                        name: "asyncResolve2",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aCallback", ty: "*const nsIProtocolProxyCallback" }, Param { name: "_retval", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

