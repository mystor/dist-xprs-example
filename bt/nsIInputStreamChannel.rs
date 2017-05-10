//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputStreamChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setURI (in nsIURI aURI); */
                    Method {
                        name: "setURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIInputStream contentStream; */
                    Method {
                        name: "get_contentStream",
                        abi: "C",
                        params: &[Param { name: "aContentStream", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentStream",
                        abi: "C",
                        params: &[Param { name: "aContentStream", ty: "*const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* attribute AString srcdocData; */
                    Method {
                        name: "get_srcdocData",
                        abi: "C",
                        params: &[Param { name: "aSrcdocData", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_srcdocData",
                        abi: "C",
                        params: &[Param { name: "aSrcdocData", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isSrcdocChannel; */
                    Method {
                        name: "get_isSrcdocChannel",
                        abi: "C",
                        params: &[Param { name: "aIsSrcdocChannel", ty: "*mut bool" }],
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

