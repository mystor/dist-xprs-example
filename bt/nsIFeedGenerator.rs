//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedGenerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedGenerator",
            base: Some("nsIFeedElementBase"),
            methods: Some(&[
                    /* attribute AString agent; */
                    Method {
                        name: "get_agent",
                        abi: "C",
                        params: &[Param { name: "aAgent", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_agent",
                        abi: "C",
                        params: &[Param { name: "aAgent", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString version; */
                    Method {
                        name: "get_version",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_version",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "*const nsAString" }],
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

