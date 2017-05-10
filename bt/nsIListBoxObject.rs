//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIListBoxObject.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIListBoxObject",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* long getRowCount (); */
                    Method {
                        name: "getRowCount",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getItemAtIndex (in long index); */
                    Method {
                        name: "getItemAtIndex",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* long getIndexOfItem (in nsIDOMElement item); */
                    Method {
                        name: "getIndexOfItem",
                        abi: "C",
                        params: &[Param { name: "item", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

