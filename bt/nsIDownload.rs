//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownload.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDownload",
            base: Some("nsITransfer"),
            methods: Some(&[
                    /* readonly attribute nsIFile targetFile; */
                    Method {
                        name: "get_targetFile",
                        abi: "C",
                        params: &[Param { name: "aTargetFile", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long percentComplete; */
                    Method {
                        name: "get_percentComplete",
                        abi: "C",
                        params: &[Param { name: "aPercentComplete", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long amountTransferred; */
                    Method {
                        name: "get_amountTransferred",
                        abi: "C",
                        params: &[Param { name: "aAmountTransferred", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long size; */
                    Method {
                        name: "get_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI source; */
                    Method {
                        name: "get_source",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI target; */
                    Method {
                        name: "get_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsICancelable cancelable; */
                    Method {
                        name: "get_cancelable",
                        abi: "C",
                        params: &[Param { name: "aCancelable", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString displayName; */
                    Method {
                        name: "get_displayName",
                        abi: "C",
                        params: &[Param { name: "aDisplayName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long startTime; */
                    Method {
                        name: "get_startTime",
                        abi: "C",
                        params: &[Param { name: "aStartTime", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double speed; */
                    Method {
                        name: "get_speed",
                        abi: "C",
                        params: &[Param { name: "aSpeed", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIMIMEInfo MIMEInfo; */
                    Method {
                        name: "get_MIMEInfo",
                        abi: "C",
                        params: &[Param { name: "aMIMEInfo", ty: "*mut *const nsIMIMEInfo" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString guid; */
                    Method {
                        name: "get_guid",
                        abi: "C",
                        params: &[Param { name: "aGuid", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute short state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI referrer; */
                    Method {
                        name: "get_referrer",
                        abi: "C",
                        params: &[Param { name: "aReferrer", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean resumable; */
                    Method {
                        name: "get_resumable",
                        abi: "C",
                        params: &[Param { name: "aResumable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isPrivate; */
                    Method {
                        name: "get_isPrivate",
                        abi: "C",
                        params: &[Param { name: "aIsPrivate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void cancel (); */
                    Method {
                        name: "cancel",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void pause (); */
                    Method {
                        name: "pause",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void resume (); */
                    Method {
                        name: "resume",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void remove (); */
                    Method {
                        name: "remove",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void retry (); */
                    Method {
                        name: "retry",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

