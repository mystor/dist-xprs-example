//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedElementBase.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedElementBase",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsISAXAttributes attributes; */
                    Method {
                        name: "get_attributes",
                        abi: "C",
                        params: &[Param { name: "aAttributes", ty: "*mut *const nsISAXAttributes" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_attributes",
                        abi: "C",
                        params: &[Param { name: "aAttributes", ty: "*const nsISAXAttributes" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURI baseURI; */
                    Method {
                        name: "get_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

