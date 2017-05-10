//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNavigatorUserMedia.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMediaDevice",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString mediaSource; */
                    Method {
                        name: "get_mediaSource",
                        abi: "C",
                        params: &[Param { name: "aMediaSource", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString rawId; */
                    Method {
                        name: "get_rawId",
                        abi: "C",
                        params: &[Param { name: "aRawId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean scary; */
                    Method {
                        name: "get_scary",
                        abi: "C",
                        params: &[Param { name: "aScary", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIGetUserMediaDevicesSuccessCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSuccess (in nsIVariant devices); */
                    Method {
                        name: "onSuccess",
                        abi: "C",
                        params: &[Param { name: "devices", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDOMGetUserMediaSuccessCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSuccess (in nsISupports value); */
                    Method {
                        name: "onSuccess",
                        abi: "C",
                        params: &[Param { name: "value", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDOMGetUserMediaErrorCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onError (in nsISupports error); */
                    Method {
                        name: "onError",
                        abi: "C",
                        params: &[Param { name: "error", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

