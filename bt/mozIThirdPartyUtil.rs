//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIThirdPartyUtil.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIThirdPartyUtil",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean isThirdPartyURI (in nsIURI aFirstURI, in nsIURI aSecondURI); */
                    Method {
                        name: "isThirdPartyURI",
                        abi: "C",
                        params: &[Param { name: "aFirstURI", ty: "*const nsIURI" }, Param { name: "aSecondURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isThirdPartyWindow (in mozIDOMWindowProxy aWindow, [optional] in nsIURI aURI); */
                    Method {
                        name: "isThirdPartyWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isThirdPartyChannel (in nsIChannel aChannel, [optional] in nsIURI aURI); */
                    Method {
                        name: "isThirdPartyChannel",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getBaseDomain (in nsIURI aHostURI); */
                    Method {
                        name: "getBaseDomain",
                        abi: "C",
                        params: &[Param { name: "aHostURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsIURI getURIFromWindow (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "getURIFromWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindowProxy getTopWindowForChannel (in nsIChannel aChannel); */
                    Method {
                        name: "getTopWindowForChannel",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

