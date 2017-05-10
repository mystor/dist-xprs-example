//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputStreamTee.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamTee",
            base: Some("nsIInputStream"),
            methods: Some(&[
                    /* attribute nsIInputStream source; */
                    Method {
                        name: "get_source",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_source",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIOutputStream sink; */
                    Method {
                        name: "get_sink",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*mut *const nsIOutputStream" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sink",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsIOutputStream" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIEventTarget eventTarget; */
                    Method {
                        name: "get_eventTarget",
                        abi: "C",
                        params: &[Param { name: "aEventTarget", ty: "*mut *const nsIEventTarget" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_eventTarget",
                        abi: "C",
                        params: &[Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

