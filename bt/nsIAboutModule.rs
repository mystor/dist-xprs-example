//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAboutModule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAboutModule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
                    Method {
                        name: "newChannel",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* unsigned long getURIFlags (in nsIURI aURI); */
                    Method {
                        name: "getURIFlags",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

