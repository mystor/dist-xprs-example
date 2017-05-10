//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoadGroupChild.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoadGroupChild",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsILoadGroup parentLoadGroup; */
                    Method {
                        name: "get_parentLoadGroup",
                        abi: "C",
                        params: &[Param { name: "aParentLoadGroup", ty: "*mut *const nsILoadGroup" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_parentLoadGroup",
                        abi: "C",
                        params: &[Param { name: "aParentLoadGroup", ty: "*const nsILoadGroup" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsILoadGroup childLoadGroup; */
                    Method {
                        name: "get_childLoadGroup",
                        abi: "C",
                        params: &[Param { name: "aChildLoadGroup", ty: "*mut *const nsILoadGroup" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsILoadGroup rootLoadGroup; */
                    Method {
                        name: "get_rootLoadGroup",
                        abi: "C",
                        params: &[Param { name: "aRootLoadGroup", ty: "*mut *const nsILoadGroup" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

