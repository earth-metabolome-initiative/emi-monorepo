-- Your SQL goes here
CREATE TABLE IF NOT EXISTS taxa (
    -- The unique identifier for the taxon
    id INTEGER PRIMARY KEY,
    -- The scientific name of the taxon
    name TEXT NOT NULL,
    -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database
    -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.
    ncbi_taxon_id INTEGER,
    domain_id INTEGER REFERENCES organism_domains(id) ON DELETE CASCADE,
    kingdom_id INTEGER REFERENCES kingdoms(id) ON DELETE CASCADE,
    phylum_id INTEGER REFERENCES phylums(id) ON DELETE CASCADE,
    class_id INTEGER REFERENCES classes(id) ON DELETE CASCADE
);
