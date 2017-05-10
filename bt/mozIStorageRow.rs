//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageRow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageRow",
            base: Some("mozIStorageValueArray"),
            methods: Some(&[
                    /* nsIVariant getResultByIndex (in unsigned long aIndex); */
                    Method {
                        name: "getResultByIndex",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* nsIVariant getResultByName (in AUTF8String aName); */
                    Method {
                        name: "getResultByName",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

