# Enrichers

A response of Classyfire on GNPS API results in a json like below (this is for erythromycin): 
```json
{
    "identifier": "DB00199",
    "smiles": "CC[C@H]1OC(=O)[C@H](C)[C@@H](O[C@H]2C[C@@](C)(OC)[C@@H](O)[C@H](C)O2)[C@H](C)[C@@H](O[C@@H]2O[C@H](C)C[C@@H]([C@H]2O)N(C)C)[C@](C)(O)C[C@@H](C)C(=O)[C@H](C)[C@@H](O)[C@]1(C)O",
    "inchikey": "InChIKey=ULGZDMOVFRHVEP-RWJQBGPGSA-N",
    "kingdom": {
        "name": "Organic compounds",
        "description": "Compounds that contain at least one carbon atom, excluding isocyanide/cyanide and their non-hydrocarbyl derivatives, thiophosgene, carbon diselenide, carbon monosulfide, carbon disulfide, carbon subsulfide, carbon monoxide, carbon dioxide, Carbon suboxide, and dicarbon monoxide.",
        "chemont_id": "CHEMONTID:0000000",
        "url": "http://classyfire.wishartlab.com/tax_nodes/C0000000"
    },
    "superclass": {
        "name": "Organic oxygen compounds",
        "description": "Organic compounds that contain one or more oxygen atoms.",
        "chemont_id": "CHEMONTID:0004603",
        "url": "http://classyfire.wishartlab.com/tax_nodes/C0004603"
    },
    "class": {
        "name": "Organooxygen compounds",
        "description": "Organic compounds containing a bond between a carbon atom and an oxygen atom.",
        "chemont_id": "CHEMONTID:0000323",
        "url": "http://classyfire.wishartlab.com/tax_nodes/C0000323"
    },
    "subclass": {
        "name": "Carbohydrates and carbohydrate conjugates",
        "description": "Monosaccharides, disaccharides, oligosaccharides, polysaccharides, and their derivatives.",
        "chemont_id": "CHEMONTID:0000011",
        "url": "http://classyfire.wishartlab.com/tax_nodes/C0000011"
    },
    "intermediate_nodes": [
        {
            "name": "Aminosaccharides",
            "description": "Saccharides containing a sugar unit that bears an amino group.",
            "chemont_id": "CHEMONTID:0003305",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0003305"
        }
    ],
    "direct_parent": {
        "name": "Aminoglycosides",
        "description": "Molecules or a portion of a molecule composed of amino-modified sugars.",
        "chemont_id": "CHEMONTID:0000282",
        "url": "http://classyfire.wishartlab.com/tax_nodes/C0000282"
    },
    "alternative_parents": [
        {
            "name": "Macrolides and analogues",
            "description": "Organic compounds containing a lactone ring of at least twelve members.",
            "chemont_id": "CHEMONTID:0000147",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0000147"
        },
        {
            "name": "O-glycosyl compounds",
            "description": "Glycoside in which a sugar group is bonded through one carbon to another group via a O-glycosidic bond.",
            "chemont_id": "CHEMONTID:0002207",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0002207"
        },
        {
            "name": "Monosaccharides",
            "description": "Compounds containing one carbohydrate unit not glycosidically linked to another such unit, and no set of two or more glycosidically linked carbohydrate units. Monosaccharides have the general formula CnH2nOn.",
            "chemont_id": "CHEMONTID:0001540",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0001540"
        },
        {
            "name": "Oxanes",
            "description": "Compounds containing an oxane (tetrahydropyran) ring, which is a six-member saturated aliphatic heterocycle with one oxygen atom and five carbon atoms.",
            "chemont_id": "CHEMONTID:0002012",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0002012"
        },
        {
            "name": "Tertiary alcohols",
            "description": "Compounds in which a hydroxy group, -OH, is attached to a saturated carbon atom R3COH (R not H ).",
            "chemont_id": "CHEMONTID:0001670",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0001670"
        },
        {
            "name": "1,2-aminoalcohols",
            "description": "Organic compounds containing an alkyl chain with an amine group bound to the C1 atom and an alcohol group bound to the C2 atom.",
            "chemont_id": "CHEMONTID:0001897",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0001897"
        },
        {
            "name": "Trialkylamines",
            "description": "Organic compounds containing a trialkylamine group, characterized by exactly three alkyl groups bonded to the amino nitrogen.",
            "chemont_id": "CHEMONTID:0002239",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0002239"
        },
        {
            "name": "Lactones",
            "description": "Cyclic esters of hydroxy carboxylic acids, containing a 1-oxacycloalkan-2-one structure, or analogues having unsaturation or heteroatoms replacing one or more carbon atoms of the ring.",
            "chemont_id": "CHEMONTID:0000050",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0000050"
        },
        {
            "name": "Cyclic ketones",
            "description": "Organic compounds containing a ketone that is conjugated to a cyclic moiety.",
            "chemont_id": "CHEMONTID:0003487",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0003487"
        },
        {
            "name": "Carboxylic acid esters",
            "description": "Carboxylic acid derivatives in which the carbon atom from the carbonyl group is attached to an alkyl or an aryl moiety through an oxygen atom (forming an ester group).",
            "chemont_id": "CHEMONTID:0001238",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0001238"
        },
        {
            "name": "Amino acids and derivatives",
            "description": "Organic compounds containing an amine group, a carboxylic acid group (or a derivative thereof), and a side-chain that is specific to each amino acid.",
            "chemont_id": "CHEMONTID:0000347",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0000347"
        },
        {
            "name": "Secondary alcohols",
            "description": "Compounds containing a secondary alcohol functional group, with the general structure HOC(R)(R') (R,R'=alkyl, aryl).",
            "chemont_id": "CHEMONTID:0001661",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0001661"
        },
        {
            "name": "Acetals",
            "description": "Compounds having the structure R2C(OR')2 ( R' not Hydrogen) and thus diethers of geminal diols. Originally, the term was confined to derivatives of aldehydes (one R = H), but it now applies equally to derivatives of ketones (neither R = H ). Mixed acetals have different R' groups.",
            "chemont_id": "CHEMONTID:0001656",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0001656"
        },
        {
            "name": "Oxacyclic compounds",
            "description": "Organic compounds containing an heterocycle with at least one oxygen atom linked to a ring carbon.",
            "chemont_id": "CHEMONTID:0004140",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0004140"
        },
        {
            "name": "Monocarboxylic acids and derivatives",
            "description": "Carboxylic acids containing exactly one carboxyl groups.",
            "chemont_id": "CHEMONTID:0001137",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0001137"
        },
        {
            "name": "Dialkyl ethers",
            "description": "Organic compounds containing the dialkyl ether functional group, with the formula ROR', where R and R' are alkyl groups.",
            "chemont_id": "CHEMONTID:0001167",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0001167"
        },
        {
            "name": "Polyols",
            "description": "Organic compounds containing more than one hydroxyl groups.",
            "chemont_id": "CHEMONTID:0002286",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0002286"
        },
        {
            "name": "Organopnictogen compounds",
            "description": "Compounds containing a bond between carbon a pnictogen atom. Pnictogens are p-block element atoms that are in the group 15 of the periodic table.",
            "chemont_id": "CHEMONTID:0004557",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0004557"
        },
        {
            "name": "Organic oxides",
            "description": "Organic compounds containing an oxide group.",
            "chemont_id": "CHEMONTID:0003940",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0003940"
        },
        {
            "name": "Hydrocarbon derivatives",
            "description": "Derivatives of hydrocarbons obtained by substituting one or more carbon atoms by an heteroatom. They contain at least one carbon atom and heteroatom.",
            "chemont_id": "CHEMONTID:0004150",
            "url": "http://classyfire.wishartlab.com/tax_nodes/C0004150"
        }
    ],
    "molecular_framework": "Aliphatic heteromonocyclic compounds",
    "substituents": [
        "Aminoglycoside core",
        "Macrolide",
        "Glycosyl compound",
        "O-glycosyl compound",
        "Oxane",
        "Monosaccharide",
        "Tertiary alcohol",
        "1,2-aminoalcohol",
        "Amino acid or derivatives",
        "Carboxylic acid ester",
        "Ketone",
        "Cyclic ketone",
        "Tertiary aliphatic amine",
        "Tertiary amine",
        "Secondary alcohol",
        "Lactone",
        "Acetal",
        "Carboxylic acid derivative",
        "Dialkyl ether",
        "Ether",
        "Oxacycle",
        "Organoheterocyclic compound",
        "Polyol",
        "Monocarboxylic acid or derivatives",
        "Organic nitrogen compound",
        "Hydrocarbon derivative",
        "Alcohol",
        "Amine",
        "Organopnictogen compound",
        "Organic oxide",
        "Organonitrogen compound",
        "Carbonyl group",
        "Aliphatic heteromonocyclic compound"
    ],
    "description": "This compound belongs to the class of organic compounds known as aminoglycosides. These are molecules or a portion of a molecule composed of amino-modified sugars.",
    "external_descriptors": [
        {
            "source": "CHEBI",
            "source_id": "CHEBI:42355",
            "annotations": [
                "erythromycin"
            ]
        },
        {
            "source": "KEGG",
            "source_id": "C01912",
            "annotations": [
                "Macrolides and lactone polyketides",
                "Macrolides"
            ]
        },
        {
            "source": "LIPID MAPS",
            "source_id": "LMPK04000006",
            "annotations": [
                "Macrolides and lactone polyketides"
            ]
        }
    ],
    "ancestors": [
        "1,2-aminoalcohols",
        "Acetals",
        "Alcohols and polyols",
        "Alkanolamines",
        "Amines",
        "Amino acids and derivatives",
        "Amino acids, peptides, and analogues",
        "Aminoglycosides",
        "Aminosaccharides",
        "Carbohydrates and carbohydrate conjugates",
        "Carbonyl compounds",
        "Carboxylic acid derivatives",
        "Carboxylic acid esters",
        "Carboxylic acids and derivatives",
        "Chemical entities",
        "Cyclic ketones",
        "Dialkyl ethers",
        "Ethers",
        "Glycosyl compounds",
        "Hydrocarbon derivatives",
        "Ketones",
        "Lactones",
        "Macrolides and analogues",
        "Monocarboxylic acids and derivatives",
        "Monosaccharides",
        "O-glycosyl compounds",
        "Organic acids and derivatives",
        "Organic compounds",
        "Organic nitrogen compounds",
        "Organic oxides",
        "Organic oxygen compounds",
        "Organoheterocyclic compounds",
        "Organonitrogen compounds",
        "Organooxygen compounds",
        "Organopnictogen compounds",
        "Oxacyclic compounds",
        "Oxanes",
        "Phenylpropanoids and polyketides",
        "Polyols",
        "Secondary alcohols",
        "Tertiary alcohols",
        "Tertiary amines",
        "Trialkylamines"
    ],
    "predicted_chebi_terms": [
        "chemical entity (CHEBI:24431)",
        "cyclic ketone (CHEBI:3992)",
        "tertiary alcohol (CHEBI:26878)",
        "tertiary amino compound (CHEBI:50996)",
        "tertiary amine (CHEBI:32876)",
        "organic molecular entity (CHEBI:50860)",
        "carboxylic ester (CHEBI:33308)",
        "secondary alcohol (CHEBI:35681)",
        "organonitrogen compound (CHEBI:35352)",
        "ether (CHEBI:25698)",
        "glycoside (CHEBI:24400)",
        "amine (CHEBI:32952)",
        "oxacycle (CHEBI:38104)",
        "amino alcohol (CHEBI:22478)",
        "ketone (CHEBI:17087)",
        "aminoglycoside (CHEBI:47779)",
        "amino acid (CHEBI:33709)",
        "peptide (CHEBI:16670)",
        "macrolide (CHEBI:25106)",
        "lactone (CHEBI:25000)",
        "polyol (CHEBI:26191)",
        "organooxygen compound (CHEBI:36963)",
        "acetal (CHEBI:59769)",
        "oxanes (CHEBI:46942)",
        "amino sugar (CHEBI:28963)",
        "organic hydroxy compound (CHEBI:33822)",
        "alcohol (CHEBI:30879)",
        "organic molecule (CHEBI:72695)",
        "carbonyl compound (CHEBI:36586)",
        "organic heterocyclic compound (CHEBI:24532)",
        "carbohydrates and carbohydrate derivatives (CHEBI:78616)",
        "monosaccharide (CHEBI:35381)",
        "pnictogen molecular entity (CHEBI:33302)",
        "nitrogen molecular entity (CHEBI:51143)",
        "oxygen molecular entity (CHEBI:25806)",
        "organic oxide (CHEBI:25701)"
    ],
    "predicted_lipidmaps_terms": [
        "Macrolides and lactone polyketides (PK04)"
    ],
    "classification_version": "2.1"
}
```