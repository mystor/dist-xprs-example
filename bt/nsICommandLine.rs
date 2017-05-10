//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandLine.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandLine",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* AString getArgument (in long aIndex); */
                    Method {
                        name: "getArgument",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* long findFlag (in AString aFlag, in boolean aCaseSensitive); */
                    Method {
                        name: "findFlag",
                        abi: "C",
                        params: &[Param { name: "aFlag", ty: "*const nsAString" }, Param { name: "aCaseSensitive", ty: "bool" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void removeArguments (in long aStart, in long aEnd); */
                    Method {
                        name: "removeArguments",
                        abi: "C",
                        params: &[Param { name: "aStart", ty: "libc::int32_t" }, Param { name: "aEnd", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean handleFlag (in AString aFlag, in boolean aCaseSensitive); */
                    Method {
                        name: "handleFlag",
                        abi: "C",
                        params: &[Param { name: "aFlag", ty: "*const nsAString" }, Param { name: "aCaseSensitive", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AString handleFlagWithParam (in AString aFlag, in boolean aCaseSensitive); */
                    Method {
                        name: "handleFlagWithParam",
                        abi: "C",
                        params: &[Param { name: "aFlag", ty: "*const nsAString" }, Param { name: "aCaseSensitive", ty: "bool" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean preventDefault; */
                    Method {
                        name: "get_preventDefault",
                        abi: "C",
                        params: &[Param { name: "aPreventDefault", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_preventDefault",
                        abi: "C",
                        params: &[Param { name: "aPreventDefault", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile workingDirectory; */
                    Method {
                        name: "get_workingDirectory",
                        abi: "C",
                        params: &[Param { name: "aWorkingDirectory", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMWindow windowContext; */
                    Method {
                        name: "get_windowContext",
                        abi: "C",
                        params: &[Param { name: "aWindowContext", ty: "*mut *const nsIDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* nsIFile resolveFile (in AString aArgument); */
                    Method {
                        name: "resolveFile",
                        abi: "C",
                        params: &[Param { name: "aArgument", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* nsIURI resolveURI (in AString aArgument); */
                    Method {
                        name: "resolveURI",
                        abi: "C",
                        params: &[Param { name: "aArgument", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

