//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINSSErrorsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINSSErrorsService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean isNSSErrorCode (in int32_t aNSPRCode); */
                    Method {
                        name: "isNSSErrorCode",
                        abi: "C",
                        params: &[Param { name: "aNSPRCode", ty: "int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsresult getXPCOMFromNSSError (in int32_t aNSPRCode); */
                    Method {
                        name: "getXPCOMFromNSSError",
                        abi: "C",
                        params: &[Param { name: "aNSPRCode", ty: "int32_t" }, Param { name: "_retval", ty: "*mut nsresult" }],
                        ret: "nsresult",
                    },

                    /* AString getErrorMessage (in nsresult aXPCOMErrorCode); */
                    Method {
                        name: "getErrorMessage",
                        abi: "C",
                        params: &[Param { name: "aXPCOMErrorCode", ty: "nsresult" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* uint32_t getErrorClass (in nsresult aXPCOMErrorCode); */
                    Method {
                        name: "getErrorClass",
                        abi: "C",
                        params: &[Param { name: "aXPCOMErrorCode", ty: "nsresult" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

