//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILayoutDebuggingTools.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILayoutDebuggingTools",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in mozIDOMWindow win); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "win", ty: "*const mozIDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* void newURILoaded (); */
                    Method {
                        name: "newURILoaded",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* attribute boolean visualDebugging; */
                    Method {
                        name: "get_visualDebugging",
                        abi: "C",
                        params: &[Param { name: "aVisualDebugging", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_visualDebugging",
                        abi: "C",
                        params: &[Param { name: "aVisualDebugging", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean visualEventDebugging; */
                    Method {
                        name: "get_visualEventDebugging",
                        abi: "C",
                        params: &[Param { name: "aVisualEventDebugging", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_visualEventDebugging",
                        abi: "C",
                        params: &[Param { name: "aVisualEventDebugging", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean paintFlashing; */
                    Method {
                        name: "get_paintFlashing",
                        abi: "C",
                        params: &[Param { name: "aPaintFlashing", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_paintFlashing",
                        abi: "C",
                        params: &[Param { name: "aPaintFlashing", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean paintDumping; */
                    Method {
                        name: "get_paintDumping",
                        abi: "C",
                        params: &[Param { name: "aPaintDumping", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_paintDumping",
                        abi: "C",
                        params: &[Param { name: "aPaintDumping", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean invalidateDumping; */
                    Method {
                        name: "get_invalidateDumping",
                        abi: "C",
                        params: &[Param { name: "aInvalidateDumping", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_invalidateDumping",
                        abi: "C",
                        params: &[Param { name: "aInvalidateDumping", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean eventDumping; */
                    Method {
                        name: "get_eventDumping",
                        abi: "C",
                        params: &[Param { name: "aEventDumping", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_eventDumping",
                        abi: "C",
                        params: &[Param { name: "aEventDumping", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean motionEventDumping; */
                    Method {
                        name: "get_motionEventDumping",
                        abi: "C",
                        params: &[Param { name: "aMotionEventDumping", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_motionEventDumping",
                        abi: "C",
                        params: &[Param { name: "aMotionEventDumping", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean crossingEventDumping; */
                    Method {
                        name: "get_crossingEventDumping",
                        abi: "C",
                        params: &[Param { name: "aCrossingEventDumping", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_crossingEventDumping",
                        abi: "C",
                        params: &[Param { name: "aCrossingEventDumping", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean reflowCounts; */
                    Method {
                        name: "get_reflowCounts",
                        abi: "C",
                        params: &[Param { name: "aReflowCounts", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_reflowCounts",
                        abi: "C",
                        params: &[Param { name: "aReflowCounts", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void dumpWebShells (); */
                    Method {
                        name: "dumpWebShells",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void dumpContent (); */
                    Method {
                        name: "dumpContent",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void dumpFrames (); */
                    Method {
                        name: "dumpFrames",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void dumpViews (); */
                    Method {
                        name: "dumpViews",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void dumpStyleSheets (); */
                    Method {
                        name: "dumpStyleSheets",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void dumpStyleContexts (); */
                    Method {
                        name: "dumpStyleContexts",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void dumpReflowStats (); */
                    Method {
                        name: "dumpReflowStats",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

