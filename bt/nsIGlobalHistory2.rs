//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGlobalHistory2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGlobalHistory2",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addURI (in nsIURI aURI, in boolean aRedirect, in boolean aToplevel, in nsIURI aReferrer); */
                    Method {
                        name: "addURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aRedirect", ty: "bool" }, Param { name: "aToplevel", ty: "bool" }, Param { name: "aReferrer", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* boolean isVisited (in nsIURI aURI); */
                    Method {
                        name: "isVisited",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setPageTitle (in nsIURI aURI, in AString aTitle); */
                    Method {
                        name: "setPageTitle",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aTitle", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

