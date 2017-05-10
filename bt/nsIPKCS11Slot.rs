//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPKCS11Slot.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPKCS11Slot",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String desc; */
                    Method {
                        name: "get_desc",
                        abi: "C",
                        params: &[Param { name: "aDesc", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String manID; */
                    Method {
                        name: "get_manID",
                        abi: "C",
                        params: &[Param { name: "aManID", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String HWVersion; */
                    Method {
                        name: "get_HWVersion",
                        abi: "C",
                        params: &[Param { name: "aHWVersion", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String FWVersion; */
                    Method {
                        name: "get_FWVersion",
                        abi: "C",
                        params: &[Param { name: "aFWVersion", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long status; */
                    Method {
                        name: "get_status",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIPK11Token getToken (); */
                    Method {
                        name: "getToken",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIPK11Token" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String tokenName; */
                    Method {
                        name: "get_tokenName",
                        abi: "C",
                        params: &[Param { name: "aTokenName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

