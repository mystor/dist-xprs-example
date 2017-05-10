//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXXMLFilter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXXMLFilter",
            base: Some("nsISAXXMLReader"),
            methods: Some(&[
                    /* attribute nsISAXXMLReader parent; */
                    Method {
                        name: "get_parent",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*mut *const nsISAXXMLReader" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_parent",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const nsISAXXMLReader" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

