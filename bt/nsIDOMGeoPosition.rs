//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGeoPosition.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMGeoPosition",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMTimeStamp timestamp; */
                    Method {
                        name: "get_timestamp",
                        abi: "C",
                        params: &[Param { name: "aTimestamp", ty: "*mut DOMTimeStamp" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMGeoPositionCoords coords; */
                    Method {
                        name: "get_coords",
                        abi: "C",
                        params: &[Param { name: "aCoords", ty: "*mut *const nsIDOMGeoPositionCoords" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

