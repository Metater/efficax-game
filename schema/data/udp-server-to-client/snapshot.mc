!vec entity-snapshots
    !type entity-snapshot
        id u32
        pos @pos
        data
            !enum entity-specific-snapshot
                !type none
                !type player
                    input-sequence u8