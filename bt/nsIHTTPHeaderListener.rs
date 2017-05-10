//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTTPHeaderListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTTPHeaderListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void newResponseHeader (in string headerName, in string headerValue); */
                    Method {
                        name: "newResponseHeader",
                        abi: "C",
                        params: &[Param { name: "headerName", ty: "*const libc::c_char" }, Param { name: "headerValue", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void statusLine (in string line); */
                    Method {
                        name: "statusLine",
                        abi: "C",
                        params: &[Param { name: "line", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

