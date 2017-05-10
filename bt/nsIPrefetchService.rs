//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefetchService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrefetchService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void prefetchURI (in nsIURI aURI, in nsIURI aReferrerURI, in nsIDOMNode aSource, in boolean aExplicit); */
                    Method {
                        name: "prefetchURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReferrerURI", ty: "*const nsIURI" }, Param { name: "aSource", ty: "*const nsIDOMNode" }, Param { name: "aExplicit", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* boolean hasMoreElements (); */
                    Method {
                        name: "hasMoreElements",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void cancelPrefetchURI (in nsIURI aURI, in nsIDOMNode aSource); */
                    Method {
                        name: "cancelPrefetchURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aSource", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

