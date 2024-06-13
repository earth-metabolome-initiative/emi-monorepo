CREATE TABLE IF NOT EXISTS bio_ott_taxon_items (
    id INTEGER PRIMARY KEY,

    -- the name of the taxon item
    -- This must NOT be unique because there are some taxons
    -- that have the same name but are different species/genus/etc... 
    -- For example Aaleniella exists in both Mollusca and Arthropoda
    name TEXT NOT NULL,

    -- the identifier of the taxon item
    ott_id INTEGER UNIQUE NOT NULL,

    -- the rank of the taxon item. This id is still an ott_id
    ott_rank_id INTEGER NOT NULL,

    -- if available, the wikidata identifier of the taxon item
    wikidata_id INTEGER,

    -- if available, the ncbi identifier of the taxon item
    ncbi_id INTEGER,

    -- if available, the gbif identifier of the taxon item
    gbif_id INTEGER,

    -- if available, the irmng identifier Interim Register of Marine and Nonmarine Genera (IRMNG)
    irmng_id INTEGER,

    -- if available, the if identifier of World Register of Marine Species (WoRMS)
    worms_id INTEGER,

    domain_id INTEGER,

    kingdom_id INTEGER,

    phylum_id INTEGER,

    class_id INTEGER,

    order_id INTEGER,

    family_id INTEGER,

    genus_id INTEGER,

    -- the parent of the taxon item. This is a recursive relationship
    -- where the root is the parent of itself.
    parent_id INTEGER NOT NULL,

    -- the font awesome icon of the taxon item
    icon_id INTEGER NOT NULL,

    -- the color of the font awesome icon of the taxon item
    color_id INTEGER NOT NULL,
    FOREIGN KEY (ott_rank_id) REFERENCES bio_ott_ranks(id) ON DELETE CASCADE,
    FOREIGN KEY (domain_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,
    FOREIGN KEY (kingdom_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,
    FOREIGN KEY (phylum_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,
    FOREIGN KEY (order_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,
    FOREIGN KEY (family_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,
    FOREIGN KEY (genus_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id) ON DELETE CASCADE,
    FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE CASCADE
);