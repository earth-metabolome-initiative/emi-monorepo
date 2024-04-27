CREATE TABLE bio_ott_taxon_items (
    id INTEGER PRIMARY KEY,

    -- the name of the taxon item
    -- This must NOT be unique because there are some taxons
    -- that have the same name but are different species/genus/etc... 
    -- For example Aaleniella exists in both Mollusca and Arthropoda
    name TEXT NOT NULL,

    -- the identifier of the taxon item
    ott_id INTEGER UNIQUE NOT NULL,

    -- the rank of the taxon item. This id is still an ott_id
    ott_rank_id INTEGER NOT NULL REFERENCES bio_ott_ranks(id) ON DELETE CASCADE,

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

    domain_id INTEGER REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,

    kingdom_id INTEGER REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,

    phylum_id INTEGER REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,

    class_id INTEGER REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,

    order_id INTEGER REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,

    family_id INTEGER REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,

    genus_id INTEGER REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,

    -- the parent of the taxon item. This is a recursive relationship
    -- where the root is the parent of itself.
    parent_id INTEGER NOT NULL REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,

    -- the font awesome icon of the taxon item
    font_awesome_icon_id INTEGER NOT NULL REFERENCES font_awesome_icons(id) ON DELETE CASCADE,

    -- the color of the font awesome icon of the taxon item
    color_id INTEGER NOT NULL REFERENCES colors(id) ON DELETE CASCADE
);