//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFTPChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFTPChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute PRTime lastModifiedTime; */
                    Method {
                        name: "get_lastModifiedTime",
                        abi: "C",
                        params: &[Param { name: "aLastModifiedTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_lastModifiedTime",
                        abi: "C",
                        params: &[Param { name: "aLastModifiedTime", ty: "PRTime" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFTPEventSink",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void OnFTPControlLog (in boolean server, in string msg); */
                    Method {
                        name: "OnFTPControlLog",
                        abi: "C",
                        params: &[Param { name: "server", ty: "bool" }, Param { name: "msg", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

