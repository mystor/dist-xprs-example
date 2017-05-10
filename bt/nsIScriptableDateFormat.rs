//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableDateFormat.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableDateFormat",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* wstring FormatDateTime (in wstring locale, in long dateFormatSelector, in long timeFormatSelector, in long year, in long month, in long day, in long hour, in long minute, in long second); */
                    Method {
                        name: "FormatDateTime",
                        abi: "C",
                        params: &[Param { name: "locale", ty: "*const libc::int16_t" }, Param { name: "dateFormatSelector", ty: "libc::int32_t" }, Param { name: "timeFormatSelector", ty: "libc::int32_t" }, Param { name: "year", ty: "libc::int32_t" }, Param { name: "month", ty: "libc::int32_t" }, Param { name: "day", ty: "libc::int32_t" }, Param { name: "hour", ty: "libc::int32_t" }, Param { name: "minute", ty: "libc::int32_t" }, Param { name: "second", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* wstring FormatDate (in wstring locale, in long dateFormatSelector, in long year, in long month, in long day); */
                    Method {
                        name: "FormatDate",
                        abi: "C",
                        params: &[Param { name: "locale", ty: "*const libc::int16_t" }, Param { name: "dateFormatSelector", ty: "libc::int32_t" }, Param { name: "year", ty: "libc::int32_t" }, Param { name: "month", ty: "libc::int32_t" }, Param { name: "day", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* wstring FormatTime (in wstring locale, in long timeFormatSelector, in long hour, in long minute, in long second); */
                    Method {
                        name: "FormatTime",
                        abi: "C",
                        params: &[Param { name: "locale", ty: "*const libc::int16_t" }, Param { name: "timeFormatSelector", ty: "libc::int32_t" }, Param { name: "hour", ty: "libc::int32_t" }, Param { name: "minute", ty: "libc::int32_t" }, Param { name: "second", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

