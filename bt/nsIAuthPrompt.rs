//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPrompt.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthPrompt",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean prompt (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, in wstring defaultText, out wstring result); */
                    Method {
                        name: "prompt",
                        abi: "C",
                        params: &[Param { name: "dialogTitle", ty: "*const libc::int16_t" }, Param { name: "text", ty: "*const libc::int16_t" }, Param { name: "passwordRealm", ty: "*const libc::int16_t" }, Param { name: "savePassword", ty: "uint32_t" }, Param { name: "defaultText", ty: "*const libc::int16_t" }, Param { name: "result", ty: "*mut *const libc::int16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring user, inout wstring pwd); */
                    Method {
                        name: "promptUsernameAndPassword",
                        abi: "C",
                        params: &[Param { name: "dialogTitle", ty: "*const libc::int16_t" }, Param { name: "text", ty: "*const libc::int16_t" }, Param { name: "passwordRealm", ty: "*const libc::int16_t" }, Param { name: "savePassword", ty: "uint32_t" }, Param { name: "user", ty: "*mut *const libc::int16_t" }, Param { name: "pwd", ty: "*mut *const libc::int16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean promptPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring pwd); */
                    Method {
                        name: "promptPassword",
                        abi: "C",
                        params: &[Param { name: "dialogTitle", ty: "*const libc::int16_t" }, Param { name: "text", ty: "*const libc::int16_t" }, Param { name: "passwordRealm", ty: "*const libc::int16_t" }, Param { name: "savePassword", ty: "uint32_t" }, Param { name: "pwd", ty: "*mut *const libc::int16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

