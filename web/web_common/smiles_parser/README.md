# SMILES-parser
A parser that checks the validity of SMILES strings and converts them into molecular graph representations.

## Definition of SMILES
Definition from [Wikipedia](https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System). 

### Atoms
Atoms are represented by the standard abbreviation of the chemical elements, in square brackets, such as `[Au]` for gold. Brackets may be omitted in the common case of atoms which:
1. are in the "organic subset" of B, C, N, O, P, S, F, Cl, Br, or I, and
2. have no formal charge, and
3. have the number of hydrogens attached implied by the SMILES valence model (typically their normal valence, but for N and P it is 3 or 5, and for S it is 2, 4 or 6), and
4. are the normal isotopes, and
5. are not chiral centers.

All other elements must be enclosed in brackets, and have charges and hydrogens shown explicitly. For instance, the SMILES for water may be written as either `O` or `[OH2]`. Hydrogen may also be written as a separate atom; water may also be written as `[H]O[H]`.

When brackets are used, the symbol `H` is added if the atom in brackets is bonded to one or more hydrogen, followed by the number of hydrogen atoms if greater than 1, then by the sign `+` for a positive charge or by `-` for a negative charge. For example, [NH4+] for ammonium. If there is more than one charge, it is normally written as digit; however, it is also possible to repeat the sign as many times as the ion has charges: one may write either `[Ti+4]` or `[Ti++++]` for titanium(IV) Ti4+. Thus, the hydroxide anion (OH−) is represented by `[OH-]`, the hydronium cation (H3O+) is `[OH3+]` and the cobalt(III) cation (Co3+) is either `[Co+3]` or `[Co+++]`.

### Bonds
A bond is represented using one of the symbols `. - = # $ : / \`.

Bonds between aliphatic atoms are assumed to be single unless specified otherwise and are implied by adjacency in the SMILES string. Although single bonds may be written as `-`, this is usually omitted. For example, the SMILES for ethanol may be written as `C-C-O`, `CC-O` or `C-CO`, but is usually written `CCO`.

Double, triple, and quadruple bonds are represented by the symbols `=`, `#`, and `$` respectively as illustrated by the SMILES `O=C=O` (carbon dioxide CO2), `C#N` (hydrogen cyanide HCN) and `[Ga+]$[As-]` (gallium arsenide).

An additional type of bond is a "non-bond", indicated with `.`, to indicate that two parts are not bonded together. For example, aqueous sodium chloride may be written as `[Na+].[Cl-]` to show the dissociation.

An aromatic "one and a half" bond may be indicated with `:`; see [Aromaticity](#aromaticity) below.

Single bonds adjacent to double bonds may be represented using `/` or `\` to indicate stereochemical configuration; see [Stereochemistry](#stereochemistry) below.

### Rings

Ring structures are written by breaking each ring at an arbitrary point (although some choices will lead to a more legible SMILES than others) to make an acyclic structure and adding numerical ring closure labels to show connectivity between non-adjacent atoms.

For example, cyclohexane and dioxane may be written as `C1CCCCC1` and `O1CCOCC1` respectively. For a second ring, the label will be 2. For example, decalin (decahydronaphthalene) may be written as `C1CCCC2C1CCCC2`.

SMILES does not require that ring numbers be used in any particular order, and permits ring number zero, although this is rarely used. Also, it is permitted to reuse ring numbers after the first ring has closed, although this usually makes formulae harder to read. For example, bicyclohexyl is usually written as `C1CCCCC1C2CCCCC2`, but it may also be written as `C0CCCCC0C0CCCCC0`.

Multiple digits after a single atom indicate multiple ring-closing bonds. For example, an alternative SMILES notation for decalin is `C1CCCC2CCCCC12`, where the final carbon participates in both ring-closing bonds 1 and 2. If two-digit ring numbers are required, the label is preceded by `%`, so `C%12` is a single ring-closing bond of ring 12.

Either or both of the digits may be preceded by a bond type to indicate the type of the ring-closing bond. For example, cyclopropene is usually written `C1=CC1`, but if the double bond is chosen as the ring-closing bond, it may be written as `C=1CC1`, `C1CC=1`, or `C=1CC=1`. (The first form is preferred.) `C=1CC-1` is illegal, as it explicitly specifies conflicting types for the ring-closing bond.

Ring-closing bonds may not be used to denote multiple bonds. For example, `C1C1` is not a valid alternative to `C=C` for ethylene. However, they may be used with non-bonds; `C1.C2.C12` is a peculiar but legal alternative way to write propane, more commonly written `CCC`.

Choosing a ring-break point adjacent to attached groups can lead to a simpler SMILES form by avoiding branches. For example, cyclohexane-1,2-diol is most simply written as `OC1CCCCC1O`; choosing a different ring-break location produces a branched structure that requires parentheses to write.

### Aromaticity

Aromatic rings such as benzene may be written in one of three forms:

1. In Kekulé form with alternating single and double bonds, e.g. `C1=CC=CC=C1`,
2. Using the aromatic bond symbol :, e.g. `C1:C:C:C:C:C1`, or
3. Most commonly, by writing the constituent B, C, N, O, P and S atoms in lower-case forms `b`, `c`, `n`, `o`, `p` and `s`, respectively.

In the latter case, bonds between two aromatic atoms are assumed (if not explicitly shown) to be aromatic bonds. Thus, benzene, pyridine and furan can be represented respectively by the SMILES `c1ccccc1`, `n1ccccc1` and `o1cccc1`.
Aromatic nitrogen bonded to hydrogen, as found in pyrrole must be represented as `[nH]`; thus imidazole is written in SMILES notation as `n1c[nH]cc1`.

When aromatic atoms are singly bonded to each other, such as in biphenyl, a single bond must be shown explicitly: `c1ccccc1-c2ccccc2`. This is one of the few cases where the single bond symbol `-` is required. (In fact, most SMILES software can correctly infer that the bond between the two rings cannot be aromatic and so will accept the nonstandard form `c1ccccc1c2ccccc2`.)

The Daylight and OpenEye algorithms for generating canonical SMILES differ in their treatment of aromaticity.

### Branching
Branches are described with parentheses, as in `CCC(=O)O` for propionic acid and `FC(F)F` for fluoroform. The first atom within the parentheses, and the first atom after the parenthesized group, are both bonded to the same branch point atom. The bond symbol must appear inside the parentheses; outside (e.g. `CCC=(O)O`) is invalid.

Substituted rings can be written with the branching point in the ring as illustrated by the SMILES `COc(c1)cccc1C#N` (see depiction) and `COc(cc1)ccc1C#N` which encode the 3 and 4-cyanoanisole isomers. Writing SMILES for substituted rings in this way can make them more human-readable.

Branches may be written in any order. For example, bromochlorodifluoromethane may be written as `FC(Br)(Cl)F`, `BrC(F)(F)Cl`, `C(F)(Cl)(F)Br`, or the like. Generally, a SMILES form is easiest to read if the simpler branch comes first, with the final, unparenthesized portion being the most complex. The only caveats to such rearrangements are:

- If ring numbers are reused, they are paired according to their order of appearance in the SMILES string. Some adjustments may be required to preserve the correct pairing.
- If stereochemistry is specified, adjustments must be made; see [Stereochemistry](#stereochemistry) below.

The one form of branch which does not require parentheses are ring-closing bonds: the SMILES fragment `C1N` is equivalent to `C(1)N`, both denoting a bond between the `C` and the `N`. Choosing ring-closing bonds adjacent to branch points can reduce the number of parentheses required. For example, toluene is normally written as `Cc1ccccc1` or `c1ccccc1C`, avoiding the parentheses required if written as `c1cc(C)ccc1` or `c1cc(ccc1)C`.

### Stereochemistry
SMILES permits, but does not require, specification of stereoisomers.

Configuration around double bonds is specified using the characters `/` and `\` to show directional single bonds adjacent to a double bond. For example, `F/C=C/F` is one representation of trans-1,2-difluoroethylene, in which the fluorine atoms are on opposite sides of the double bond (as shown in the figure), whereas `F/C=C\F` is one possible representation of cis-1,2-difluoroethylene, in which the fluorines are on the same side of the double bond.

Bond direction symbols always come in groups of at least two, of which the first is arbitrary. That is, `F\C=C\F` is the same as `F/C=C/F`. When alternating single-double bonds are present, the groups are larger than two, with the middle directional symbols being adjacent to two double bonds. For example, the common form of (2,4)-hexadiene is written `C/C=C/C=C/C`.

As a more complex example, beta-carotene has a very long backbone of alternating single and double bonds, which may be written `CC1CCC/C(C)=C1/C=C/C(C)=C/C=C/C(C)=C/C=C/C=C(C)/C=C/C=C(C)/C=C/C2=C(C)/CCCC2(C)C`.

Configuration at tetrahedral carbon is specified by `@` or `@@`. Consider the four bonds in the order in which they appear, left to right, in the SMILES form. Looking toward the central carbon from the perspective of the first bond, the other three are either clockwise or counter-clockwise. These cases are indicated with `@@` and `@`, respectively (because the `@` symbol itself is a counter-clockwise spiral).

For example, consider the amino acid alanine. One of its SMILES forms is `NC(C)C(=O)O`, more fully written as `N[CH](C)C(=O)O`. L-Alanine, the more common enantiomer, is written as `N[C@@H](C)C(=O)O`. Looking from the nitrogen–carbon bond, the hydrogen (`H`), methyl (`C`), and carboxylate (`C(=O)O`) groups appear clockwise. D-Alanine can be written as `N[C@H](C)C(=O)O`.

While the order in which branches are specified in SMILES is normally unimportant, in this case it matters; swapping any two groups requires reversing the chirality indicator. If the branches are reversed so alanine is written as `NC(C(=O)O)C`, then the configuration also reverses; L-alanine is written as `N[C@H](C(=O)O)C`. Other ways of writing it include `C[C@H](N)C(=O)O`, `OC(=O)[C@@H](N)C` and `OC(=O)[C@H](C)N`.

Normally, the first of the four bonds appears to the left of the carbon atom, but if the SMILES is written beginning with the chiral carbon, such as `C(C)(N)C(=O)O`, then all four are to the right, but the first to appear (the `[CH]` bond in this case) is used as the reference to order the following three: L-alanine may also be written as `[C@@H](C)(N)C(=O)O`.

The SMILES specification includes elaborations on the `@` symbol to indicate stereochemistry around more complex chiral centers, such as trigonal bipyramidal molecular geometry.

### Isotopes
Isotopes are specified with a number equal to the integer isotopic mass preceding the atomic symbol. Benzene in which one atom is carbon-14 is written as `[14cH]1ccccc1` and deuterochloroform is `[2H]C(Cl)(Cl)Cl`.