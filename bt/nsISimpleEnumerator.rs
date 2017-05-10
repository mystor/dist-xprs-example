//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISimpleEnumerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISimpleEnumerator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean hasMoreElements (); */
                    Method {
                        name: "hasMoreElements",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsISupports getNext (); */
                    Method {
                        name: "getNext",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

