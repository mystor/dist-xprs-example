//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUserInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUserInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute wstring fullname; */
                    Method {
                        name: "get_fullname",
                        abi: "C",
                        params: &[Param { name: "aFullname", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string emailAddress; */
                    Method {
                        name: "get_emailAddress",
                        abi: "C",
                        params: &[Param { name: "aEmailAddress", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string username; */
                    Method {
                        name: "get_username",
                        abi: "C",
                        params: &[Param { name: "aUsername", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string domain; */
                    Method {
                        name: "get_domain",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

