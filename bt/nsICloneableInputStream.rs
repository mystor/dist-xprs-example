//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICloneableInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICloneableInputStream",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [infallible] readonly attribute boolean cloneable; */
                    Method {
                        name: "get_cloneable",
                        abi: "C",
                        params: &[Param { name: "aCloneable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream clone (); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

