//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPKCS11Module.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPKCS11Module",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String libName; */
                    Method {
                        name: "get_libName",
                        abi: "C",
                        params: &[Param { name: "aLibName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsIPKCS11Slot findSlotByName (in AUTF8String name); */
                    Method {
                        name: "findSlotByName",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIPKCS11Slot" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator listSlots (); */
                    Method {
                        name: "listSlots",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

