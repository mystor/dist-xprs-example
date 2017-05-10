//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationDevice.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationDevice",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsIPresentationControlChannel establishControlChannel (); */
                    Method {
                        name: "establishControlChannel",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIPresentationControlChannel" }],
                        ret: "nsresult",
                    },

                    /* void disconnect (); */
                    Method {
                        name: "disconnect",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean isRequestedUrlSupported (in DOMString requestedUrl); */
                    Method {
                        name: "isRequestedUrlSupported",
                        abi: "C",
                        params: &[Param { name: "requestedUrl", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

