//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIRefObject.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIRefObject",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIDOMNode node; */
                    Method {
                        name: "get_node",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_node",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void Reset (); */
                    Method {
                        name: "Reset",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* DOMString GetNextURI (); */
                    Method {
                        name: "GetNextURI",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void RewriteAllURIs (in DOMString aOldPat, in DOMString aNewPat, in boolean aMakeRel); */
                    Method {
                        name: "RewriteAllURIs",
                        abi: "C",
                        params: &[Param { name: "aOldPat", ty: "*const nsAString" }, Param { name: "aNewPat", ty: "*const nsAString" }, Param { name: "aMakeRel", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

