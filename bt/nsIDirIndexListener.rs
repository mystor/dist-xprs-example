//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDirIndexListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDirIndexListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onIndexAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in nsIDirIndex aIndex); */
                    Method {
                        name: "onIndexAvailable",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCtxt", ty: "*const nsISupports" }, Param { name: "aIndex", ty: "*const nsIDirIndex" }],
                        ret: "nsresult",
                    },

                    /* void onInformationAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in AString aInfo); */
                    Method {
                        name: "onInformationAvailable",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCtxt", ty: "*const nsISupports" }, Param { name: "aInfo", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDirIndexParser",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* attribute nsIDirIndexListener listener; */
                    Method {
                        name: "get_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIDirIndexListener" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_listener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIDirIndexListener" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string comment; */
                    Method {
                        name: "get_comment",
                        abi: "C",
                        params: &[Param { name: "aComment", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* attribute string encoding; */
                    Method {
                        name: "get_encoding",
                        abi: "C",
                        params: &[Param { name: "aEncoding", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_encoding",
                        abi: "C",
                        params: &[Param { name: "aEncoding", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

