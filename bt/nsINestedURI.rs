//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINestedURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINestedURI",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIURI innerURI; */
                    Method {
                        name: "get_innerURI",
                        abi: "C",
                        params: &[Param { name: "aInnerURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI innermostURI; */
                    Method {
                        name: "get_innermostURI",
                        abi: "C",
                        params: &[Param { name: "aInnermostURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

