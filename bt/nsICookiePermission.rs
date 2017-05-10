//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookiePermission.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICookiePermission",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setAccess (in nsIURI aURI, in nsCookieAccess aAccess); */
                    Method {
                        name: "setAccess",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aAccess", ty: "nsCookieAccess" }],
                        ret: "nsresult",
                    },

                    /* nsCookieAccess canAccess (in nsIURI aURI, in nsIChannel aChannel); */
                    Method {
                        name: "canAccess",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut nsCookieAccess" }],
                        ret: "nsresult",
                    },

                    /* boolean canSetCookie (in nsIURI aURI, in nsIChannel aChannel, in nsICookie2 aCookie, inout boolean aIsSession, inout int64_t aExpiry); */
                    Method {
                        name: "canSetCookie",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aCookie", ty: "*const nsICookie2" }, Param { name: "aIsSession", ty: "*mut bool" }, Param { name: "aExpiry", ty: "*mut int64_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

