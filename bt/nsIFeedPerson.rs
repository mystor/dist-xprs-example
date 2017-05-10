//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedPerson.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedPerson",
            base: Some("nsIFeedElementBase"),
            methods: Some(&[
                    /* attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString email; */
                    Method {
                        name: "get_email",
                        abi: "C",
                        params: &[Param { name: "aEmail", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_email",
                        abi: "C",
                        params: &[Param { name: "aEmail", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURI uri; */
                    Method {
                        name: "get_uri",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_uri",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

