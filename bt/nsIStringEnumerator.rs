//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStringEnumerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStringEnumerator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean hasMore (); */
                    Method {
                        name: "hasMore",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AString getNext (); */
                    Method {
                        name: "getNext",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUTF8StringEnumerator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean hasMore (); */
                    Method {
                        name: "hasMore",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getNext (); */
                    Method {
                        name: "getNext",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

