//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURILoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURILoader",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void registerContentListener (in nsIURIContentListener aContentListener); */
                    Method {
                        name: "registerContentListener",
                        abi: "C",
                        params: &[Param { name: "aContentListener", ty: "*const nsIURIContentListener" }],
                        ret: "nsresult",
                    },

                    /* void unRegisterContentListener (in nsIURIContentListener aContentListener); */
                    Method {
                        name: "unRegisterContentListener",
                        abi: "C",
                        params: &[Param { name: "aContentListener", ty: "*const nsIURIContentListener" }],
                        ret: "nsresult",
                    },

                    /* void openURI (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "openURI",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* nsIStreamListener openChannel (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "openChannel",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "_retval", ty: "*mut *const nsIStreamListener" }],
                        ret: "nsresult",
                    },

                    /* void stop (in nsISupports aLoadCookie); */
                    Method {
                        name: "stop",
                        abi: "C",
                        params: &[Param { name: "aLoadCookie", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

