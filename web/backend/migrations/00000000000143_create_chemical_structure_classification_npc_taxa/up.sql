CREATE TABLE IF NOT EXISTS chemical_structure_classification_npc_taxa (
    id SERIAL PRIMARY KEY,
    npc_pathway VARCHAR(255) NOT NULL,  -- NPC#pathway
    npc_superclass VARCHAR(255) NOT NULL,  -- NPC#superclass
    npc_class VARCHAR(255) NOT NULL  -- NPC#class
);