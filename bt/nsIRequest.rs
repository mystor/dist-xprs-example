//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRequest",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean isPending (); */
                    Method {
                        name: "isPending",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsresult status; */
                    Method {
                        name: "get_status",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "*mut nsresult" }],
                        ret: "nsresult",
                    },

                    /* void cancel (in nsresult aStatus); */
                    Method {
                        name: "cancel",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void suspend (); */
                    Method {
                        name: "suspend",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void resume (); */
                    Method {
                        name: "resume",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* attribute nsILoadGroup loadGroup; */
                    Method {
                        name: "get_loadGroup",
                        abi: "C",
                        params: &[Param { name: "aLoadGroup", ty: "*mut *const nsILoadGroup" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_loadGroup",
                        abi: "C",
                        params: &[Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }],
                        ret: "nsresult",
                    },

                    /* attribute nsLoadFlags loadFlags; */
                    Method {
                        name: "get_loadFlags",
                        abi: "C",
                        params: &[Param { name: "aLoadFlags", ty: "*mut nsLoadFlags" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_loadFlags",
                        abi: "C",
                        params: &[Param { name: "aLoadFlags", ty: "nsLoadFlags" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

