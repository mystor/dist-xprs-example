//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFaviconService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFaviconService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIURI getFaviconLinkForIcon (in nsIURI aFaviconURI); */
                    Method {
                        name: "getFaviconLinkForIcon",
                        abi: "C",
                        params: &[Param { name: "aFaviconURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void expireAllFavicons (); */
                    Method {
                        name: "expireAllFavicons",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* unsigned short preferredSizeFromURI (in nsIURI aURI); */
                    Method {
                        name: "preferredSizeFromURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void addFailedFavicon (in nsIURI aFaviconURI); */
                    Method {
                        name: "addFailedFavicon",
                        abi: "C",
                        params: &[Param { name: "aFaviconURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void removeFailedFavicon (in nsIURI aFaviconURI); */
                    Method {
                        name: "removeFailedFavicon",
                        abi: "C",
                        params: &[Param { name: "aFaviconURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* boolean isFailedFavicon (in nsIURI aFaviconURI); */
                    Method {
                        name: "isFailedFavicon",
                        abi: "C",
                        params: &[Param { name: "aFaviconURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI defaultFavicon; */
                    Method {
                        name: "get_defaultFavicon",
                        abi: "C",
                        params: &[Param { name: "aDefaultFavicon", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFaviconDataCallback",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

