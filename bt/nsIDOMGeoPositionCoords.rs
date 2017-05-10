//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoPositionCoords.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMGeoPositionCoords",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute double latitude; */
                    Method {
                        name: "get_latitude",
                        abi: "C",
                        params: &[Param { name: "aLatitude", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double longitude; */
                    Method {
                        name: "get_longitude",
                        abi: "C",
                        params: &[Param { name: "aLongitude", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double altitude; */
                    Method {
                        name: "get_altitude",
                        abi: "C",
                        params: &[Param { name: "aAltitude", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double accuracy; */
                    Method {
                        name: "get_accuracy",
                        abi: "C",
                        params: &[Param { name: "aAccuracy", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double altitudeAccuracy; */
                    Method {
                        name: "get_altitudeAccuracy",
                        abi: "C",
                        params: &[Param { name: "aAltitudeAccuracy", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double heading; */
                    Method {
                        name: "get_heading",
                        abi: "C",
                        params: &[Param { name: "aHeading", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double speed; */
                    Method {
                        name: "get_speed",
                        abi: "C",
                        params: &[Param { name: "aSpeed", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

