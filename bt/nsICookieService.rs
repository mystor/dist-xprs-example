//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookieService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICookieService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* string getCookieString (in nsIURI aURI, in nsIChannel aChannel); */
                    Method {
                        name: "getCookieString",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* string getCookieStringFromHttp (in nsIURI aURI, in nsIURI aFirstURI, in nsIChannel aChannel); */
                    Method {
                        name: "getCookieStringFromHttp",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aFirstURI", ty: "*const nsIURI" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setCookieString (in nsIURI aURI, in nsIPrompt aPrompt, in string aCookie, in nsIChannel aChannel); */
                    Method {
                        name: "setCookieString",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPrompt", ty: "*const nsIPrompt" }, Param { name: "aCookie", ty: "*const libc::c_char" }, Param { name: "aChannel", ty: "*const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* void setCookieStringFromHttp (in nsIURI aURI, in nsIURI aFirstURI, in nsIPrompt aPrompt, in string aCookie, in string aServerTime, in nsIChannel aChannel); */
                    Method {
                        name: "setCookieStringFromHttp",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aFirstURI", ty: "*const nsIURI" }, Param { name: "aPrompt", ty: "*const nsIPrompt" }, Param { name: "aCookie", ty: "*const libc::c_char" }, Param { name: "aServerTime", ty: "*const libc::c_char" }, Param { name: "aChannel", ty: "*const nsIChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

