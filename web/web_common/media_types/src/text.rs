//! Submodule providing the `text` media type and its subtypes. This module is generated from the IANA registry at `https://www.iana.org/assignments/media-types/text.csv`.
/// The `1d-interleaved-parityfec` subtype of the `text` media type.
pub const MT_1D_INTERLEAVED_PARITYFEC: &str = "1d-interleaved-parityfec";
/// The `cache-manifest` subtype of the `text` media type.
pub const CACHE_MANIFEST: &str = "cache-manifest";
/// The `calendar` subtype of the `text` media type.
pub const CALENDAR: &str = "calendar";
/// The `cql` subtype of the `text` media type.
pub const CQL: &str = "cql";
/// The `cql-expression` subtype of the `text` media type.
pub const CQL_EXPRESSION: &str = "cql-expression";
/// The `cql-identifier` subtype of the `text` media type.
pub const CQL_IDENTIFIER: &str = "cql-identifier";
/// The `css` subtype of the `text` media type.
pub const CSS: &str = "css";
/// The `csv` subtype of the `text` media type.
pub const CSV: &str = "csv";
/// The `csv-schema` subtype of the `text` media type.
pub const CSV_SCHEMA: &str = "csv-schema";
/// The `directory` subtype of the `text` media type.
#[deprecated(since = "0.1.0", note = "This subtype is deprecated and should not be used.")]
pub const DIRECTORY: &str = "directory";
/// The `dns` subtype of the `text` media type.
pub const DNS: &str = "dns";
/// The `ecmascript` subtype of the `text` media type.
#[deprecated(since = "0.1.0", note = "Use `media_types_builder::text::JAVASCRIPT` instead")]
pub const ECMASCRIPT: &str = "ecmascript";
/// The `encaprtp` subtype of the `text` media type.
pub const ENCAPRTP: &str = "encaprtp";
/// The `enriched` subtype of the `text` media type.
pub const ENRICHED: &str = "enriched";
/// The `example` subtype of the `text` media type.
pub const EXAMPLE: &str = "example";
/// The `fhirpath` subtype of the `text` media type.
pub const FHIRPATH: &str = "fhirpath";
/// The `flexfec` subtype of the `text` media type.
pub const FLEXFEC: &str = "flexfec";
/// The `fwdred` subtype of the `text` media type.
pub const FWDRED: &str = "fwdred";
/// The `gff3` subtype of the `text` media type.
pub const GFF3: &str = "gff3";
/// The `grammar-ref-list` subtype of the `text` media type.
pub const GRAMMAR_REF_LIST: &str = "grammar-ref-list";
/// The `hl7v2` subtype of the `text` media type.
pub const HL7V2: &str = "hl7v2";
/// The `html` subtype of the `text` media type.
pub const HTML: &str = "html";
/// The `javascript` subtype of the `text` media type.
pub const JAVASCRIPT: &str = "javascript";
/// The `jcr-cnd` subtype of the `text` media type.
pub const JCR_CND: &str = "jcr-cnd";
/// The `markdown` subtype of the `text` media type.
pub const MARKDOWN: &str = "markdown";
/// The `mizar` subtype of the `text` media type.
pub const MIZAR: &str = "mizar";
/// The `n3` subtype of the `text` media type.
pub const N3: &str = "n3";
/// The `parameters` subtype of the `text` media type.
pub const PARAMETERS: &str = "parameters";
/// The `parityfec` subtype of the `text` media type.
pub const PARITYFEC: &str = "parityfec";
/// The `plain` subtype of the `text` media type.
pub const PLAIN: &str = "plain";
/// The `provenance-notation` subtype of the `text` media type.
pub const PROVENANCE_NOTATION: &str = "provenance-notation";
/// The `prs.fallenstein.rst` subtype of the `text` media type.
pub const PRS_FALLENSTEIN_RST: &str = "prs.fallenstein.rst";
/// The `prs.lines.tag` subtype of the `text` media type.
pub const PRS_LINES_TAG: &str = "prs.lines.tag";
/// The `prs.prop.logic` subtype of the `text` media type.
pub const PRS_PROP_LOGIC: &str = "prs.prop.logic";
/// The `prs.texi` subtype of the `text` media type.
pub const PRS_TEXI: &str = "prs.texi";
/// The `raptorfec` subtype of the `text` media type.
pub const RAPTORFEC: &str = "raptorfec";
/// The `RED` subtype of the `text` media type.
pub const RED: &str = "RED";
/// The `rfc822-headers` subtype of the `text` media type.
pub const RFC822_HEADERS: &str = "rfc822-headers";
/// The `richtext` subtype of the `text` media type.
pub const RICHTEXT: &str = "richtext";
/// The `rtf` subtype of the `text` media type.
pub const RTF: &str = "rtf";
/// The `rtp-enc-aescm128` subtype of the `text` media type.
pub const RTP_ENC_AESCM128: &str = "rtp-enc-aescm128";
/// The `rtploopback` subtype of the `text` media type.
pub const RTPLOOPBACK: &str = "rtploopback";
/// The `rtx` subtype of the `text` media type.
pub const RTX: &str = "rtx";
/// The `SGML` subtype of the `text` media type.
pub const SGML: &str = "SGML";
/// The `shaclc` subtype of the `text` media type.
pub const SHACLC: &str = "shaclc";
/// The `shex` subtype of the `text` media type.
pub const SHEX: &str = "shex";
/// The `spdx` subtype of the `text` media type.
pub const SPDX: &str = "spdx";
/// The `strings` subtype of the `text` media type.
pub const STRINGS: &str = "strings";
/// The `t140` subtype of the `text` media type.
pub const T140: &str = "t140";
/// The `tab-separated-values` subtype of the `text` media type.
pub const TAB_SEPARATED_VALUES: &str = "tab-separated-values";
/// The `troff` subtype of the `text` media type.
pub const TROFF: &str = "troff";
/// The `turtle` subtype of the `text` media type.
pub const TURTLE: &str = "turtle";
/// The `ulpfec` subtype of the `text` media type.
pub const ULPFEC: &str = "ulpfec";
/// The `uri-list` subtype of the `text` media type.
pub const URI_LIST: &str = "uri-list";
/// The `vcard` subtype of the `text` media type.
pub const VCARD: &str = "vcard";
/// The `vnd.a` subtype of the `text` media type.
pub const VND_A: &str = "vnd.a";
/// The `vnd.abc` subtype of the `text` media type.
pub const VND_ABC: &str = "vnd.abc";
/// The `vnd.ascii-art` subtype of the `text` media type.
pub const VND_ASCII_ART: &str = "vnd.ascii-art";
/// The `vnd.curl` subtype of the `text` media type.
pub const VND_CURL: &str = "vnd.curl";
/// The `vnd.debian.copyright` subtype of the `text` media type.
pub const VND_DEBIAN_COPYRIGHT: &str = "vnd.debian.copyright";
/// The `vnd.DMClientScript` subtype of the `text` media type.
pub const VND_DMCLIENTSCRIPT: &str = "vnd.DMClientScript";
/// The `vnd.dvb.subtitle` subtype of the `text` media type.
pub const VND_DVB_SUBTITLE: &str = "vnd.dvb.subtitle";
/// The `vnd.esmertec.theme-descriptor` subtype of the `text` media type.
pub const VND_ESMERTEC_THEME_DESCRIPTOR: &str = "vnd.esmertec.theme-descriptor";
/// The `vnd.exchangeable` subtype of the `text` media type.
pub const VND_EXCHANGEABLE: &str = "vnd.exchangeable";
/// The `vnd.familysearch.gedcom` subtype of the `text` media type.
pub const VND_FAMILYSEARCH_GEDCOM: &str = "vnd.familysearch.gedcom";
/// The `vnd.ficlab.flt` subtype of the `text` media type.
pub const VND_FICLAB_FLT: &str = "vnd.ficlab.flt";
/// The `vnd.fly` subtype of the `text` media type.
pub const VND_FLY: &str = "vnd.fly";
/// The `vnd.fmi.flexstor` subtype of the `text` media type.
pub const VND_FMI_FLEXSTOR: &str = "vnd.fmi.flexstor";
/// The `vnd.gml` subtype of the `text` media type.
pub const VND_GML: &str = "vnd.gml";
/// The `vnd.graphviz` subtype of the `text` media type.
pub const VND_GRAPHVIZ: &str = "vnd.graphviz";
/// The `vnd.hans` subtype of the `text` media type.
pub const VND_HANS: &str = "vnd.hans";
/// The `vnd.hgl` subtype of the `text` media type.
pub const VND_HGL: &str = "vnd.hgl";
/// The `vnd.in3d.3dml` subtype of the `text` media type.
pub const VND_IN3D_3DML: &str = "vnd.in3d.3dml";
/// The `vnd.in3d.spot` subtype of the `text` media type.
pub const VND_IN3D_SPOT: &str = "vnd.in3d.spot";
/// The `vnd.IPTC.NewsML` subtype of the `text` media type.
pub const VND_IPTC_NEWSML: &str = "vnd.IPTC.NewsML";
/// The `vnd.IPTC.NITF` subtype of the `text` media type.
pub const VND_IPTC_NITF: &str = "vnd.IPTC.NITF";
/// The `vnd.latex-z` subtype of the `text` media type.
pub const VND_LATEX_Z: &str = "vnd.latex-z";
/// The `vnd.motorola.reflex` subtype of the `text` media type.
pub const VND_MOTOROLA_REFLEX: &str = "vnd.motorola.reflex";
/// The `vnd.ms-mediapackage` subtype of the `text` media type.
pub const VND_MS_MEDIAPACKAGE: &str = "vnd.ms-mediapackage";
/// The `vnd.net2phone.commcenter.command` subtype of the `text` media type.
pub const VND_NET2PHONE_COMMCENTER_COMMAND: &str = "vnd.net2phone.commcenter.command";
/// The `vnd.radisys.msml-basic-layout` subtype of the `text` media type.
pub const VND_RADISYS_MSML_BASIC_LAYOUT: &str = "vnd.radisys.msml-basic-layout";
/// The `vnd.senx.warpscript` subtype of the `text` media type.
pub const VND_SENX_WARPSCRIPT: &str = "vnd.senx.warpscript";
/// The `vnd.si.uricatalogue` subtype of the `text` media type.
#[deprecated(since = "0.1.0", note = "This subtype is deprecated and should not be used.")]
pub const VND_SI_URICATALOGUE: &str = "vnd.si.uricatalogue";
/// The `vnd.sun.j2me.app-descriptor` subtype of the `text` media type.
pub const VND_SUN_J2ME_APP_DESCRIPTOR: &str = "vnd.sun.j2me.app-descriptor";
/// The `vnd.sosi` subtype of the `text` media type.
pub const VND_SOSI: &str = "vnd.sosi";
/// The `vnd.typst` subtype of the `text` media type.
pub const VND_TYPST: &str = "vnd.typst";
/// The `vnd.trolltech.linguist` subtype of the `text` media type.
pub const VND_TROLLTECH_LINGUIST: &str = "vnd.trolltech.linguist";
/// The `vnd.vcf` subtype of the `text` media type.
pub const VND_VCF: &str = "vnd.vcf";
/// The `vnd.wap.si` subtype of the `text` media type.
pub const VND_WAP_SI: &str = "vnd.wap.si";
/// The `vnd.wap.sl` subtype of the `text` media type.
pub const VND_WAP_SL: &str = "vnd.wap.sl";
/// The `vnd.wap.wml` subtype of the `text` media type.
pub const VND_WAP_WML: &str = "vnd.wap.wml";
/// The `vnd.wap.wmlscript` subtype of the `text` media type.
pub const VND_WAP_WMLSCRIPT: &str = "vnd.wap.wmlscript";
/// The `vnd.zoo.kcl` subtype of the `text` media type.
pub const VND_ZOO_KCL: &str = "vnd.zoo.kcl";
/// The `vtt` subtype of the `text` media type.
pub const VTT: &str = "vtt";
/// The `wgsl` subtype of the `text` media type.
pub const WGSL: &str = "wgsl";
/// The `xml` subtype of the `text` media type.
pub const XML: &str = "xml";
/// The `xml-external-parsed-entity` subtype of the `text` media type.
pub const XML_EXTERNAL_PARSED_ENTITY: &str = "xml-external-parsed-entity";
/// Set of all subtypes of the `text` media type. This set is generated from the IANA registry at `https://www.iana.org/assignments/media-types/text.csv`.
pub static SUBTYPES: ::phf::Set<&'static str> = ::phf::phf_set! { "1d-interleaved-parityfec" , "cache-manifest" , "calendar" , "cql" , "cql-expression" , "cql-identifier" , "css" , "csv" , "csv-schema" , "directory" , "dns" , "ecmascript" , "encaprtp" , "enriched" , "example" , "fhirpath" , "flexfec" , "fwdred" , "gff3" , "grammar-ref-list" , "hl7v2" , "html" , "javascript" , "jcr-cnd" , "markdown" , "mizar" , "n3" , "parameters" , "parityfec" , "plain" , "provenance-notation" , "prs.fallenstein.rst" , "prs.lines.tag" , "prs.prop.logic" , "prs.texi" , "raptorfec" , "RED" , "rfc822-headers" , "richtext" , "rtf" , "rtp-enc-aescm128" , "rtploopback" , "rtx" , "SGML" , "shaclc" , "shex" , "spdx" , "strings" , "t140" , "tab-separated-values" , "troff" , "turtle" , "ulpfec" , "uri-list" , "vcard" , "vnd.a" , "vnd.abc" , "vnd.ascii-art" , "vnd.curl" , "vnd.debian.copyright" , "vnd.DMClientScript" , "vnd.dvb.subtitle" , "vnd.esmertec.theme-descriptor" , "vnd.exchangeable" , "vnd.familysearch.gedcom" , "vnd.ficlab.flt" , "vnd.fly" , "vnd.fmi.flexstor" , "vnd.gml" , "vnd.graphviz" , "vnd.hans" , "vnd.hgl" , "vnd.in3d.3dml" , "vnd.in3d.spot" , "vnd.IPTC.NewsML" , "vnd.IPTC.NITF" , "vnd.latex-z" , "vnd.motorola.reflex" , "vnd.ms-mediapackage" , "vnd.net2phone.commcenter.command" , "vnd.radisys.msml-basic-layout" , "vnd.senx.warpscript" , "vnd.si.uricatalogue" , "vnd.sun.j2me.app-descriptor" , "vnd.sosi" , "vnd.typst" , "vnd.trolltech.linguist" , "vnd.vcf" , "vnd.wap.si" , "vnd.wap.sl" , "vnd.wap.wml" , "vnd.wap.wmlscript" , "vnd.zoo.kcl" , "vtt" , "wgsl" , "xml" , "xml-external-parsed-entity" };
