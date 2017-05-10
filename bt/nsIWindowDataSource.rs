//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowDataSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowDataSource",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDOMWindow getWindowForResource (in string inResource); */
                    Method {
                        name: "getWindowForResource",
                        abi: "C",
                        params: &[Param { name: "inResource", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIDOMWindow" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

