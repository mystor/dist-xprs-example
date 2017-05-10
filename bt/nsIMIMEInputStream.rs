//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMIMEInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMIMEInputStream",
            base: Some("nsIInputStream"),
            methods: Some(&[
                    /* attribute boolean addContentLength; */
                    Method {
                        name: "get_addContentLength",
                        abi: "C",
                        params: &[Param { name: "aAddContentLength", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_addContentLength",
                        abi: "C",
                        params: &[Param { name: "aAddContentLength", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addHeader (in string name, in string value); */
                    Method {
                        name: "addHeader",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void visitHeaders (in nsIHttpHeaderVisitor visitor); */
                    Method {
                        name: "visitHeaders",
                        abi: "C",
                        params: &[Param { name: "visitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "nsresult",
                    },

                    /* void setData (in nsIInputStream stream); */
                    Method {
                        name: "setData",
                        abi: "C",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIInputStream data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

