use proc_macro2::{Ident, Span};
use std::{error::Error, fs, io::Write};
use syn;

use inflector::Inflector;

const COMMENT_PREFIX: &str = "= \" ";
const COMMENT_SUFFIX: &str = "\"";

const OPTION_PREFIX: &str = "Option < ";
const OPTION_SUFFIX: &str = " >";

const VEC_PREFIX: &str = "Vec < ";
const VEC_SUFFIX: &str = " >";

const HASH_MAP_PREFIX: &str = "HashMap < ";
const HASH_MAP_SUFFIX: &str = " >";

pub const ACRONYMS: &'static [&'static str] = &[
    "aaa", "aabb", "aac", "aal", "aalc", "aarp", "abac", "abcl", "abi", "abm", "abr", "ac", "acd", "ack", "acl", "acm", "acme", "acp", "acpi", "acr", "adb", "adc", "adccp", "ado",
    "adsl", "adt", "ae", "aes", "af", "afp", "agp", "ai", "aix", "alac", "algol", "alsa", "alu", "amd", "amoled", "amqp", "amr", "ann", "ansi", "aop", "apci", "api", "apic",
    "apipa", "apl", "apr", "arin", "aros", "arp", "arpa", "arpanet", "ascii", "aset", "asg", "asic", "asimo", "aslr", "asm", "asmp", "asp", "asr", "assp", "ast", "ata", "atag",
    "atapi", "atm", "av", "avc", "avi", "awfl", "awk", "awt", "bal", "bam", "bbp", "bbs", "bcd", "bcnf", "beep", "ber", "bfd", "bfs", "bft", "bgp", "bi", "binac", "bios", "bjt",
    "bmp", "bnc", "boinc", "bom", "bootp", "bpdu", "bpel", "bpl", "bpm", "brm", "brms", "brr", "brs", "bsa", "bsb", "bsd", "bss", "bt", "bw", "byod", "ca", "cad", "cae", "cai",
    "caid", "captcha", "caq", "cd", "cde", "cdfs", "cdma", "cdn", "cdp", "cdsa", "cert", "ces", "cf", "cfd", "cfg", "cg", "cga", "cgi", "cgt", "chs", "cidr", "cifs", "cim", "cio",
    "cir", "cisc", "cjk", "cjkv", "cli", "clr", "cm", "cmdb", "cmmi", "cmo", "cmos", "cms", "cn", "cnc", "cng", "cnr", "cobol", "com", "corba", "cots", "cpa", "cpan", "cpri",
    "cps", "cpu", "cr", "cran", "crc", "crlf", "crm", "crs", "crt", "crud", "cs", "cse", "csi", "csm", "csp", "csrf", "css", "csv", "ct", "ctan", "ctcp", "ctfe", "cti", "ctl",
    "ctm", "cts", "ctss", "cua", "cvs", "dac", "dal", "dao", "dap", "darpa", "dat", "db", "dba", "dbcs", "dbms", "dcc", "dcca", "dccp", "dcl", "dcmi", "dcom", "dcs", "dd", "dde",
    "ddi", "ddl", "ddr", "dec", "des", "dfa", "dfd", "dfs", "dgd", "dhcp", "dhtml", "dif", "dimm", "din", "dip", "dism", "divx", "dkim", "dl", "dll", "dlna", "dlp", "dma", "dmca",
    "dmi", "dml", "dmr", "dmz", "dn", "dnd", "dns", "doa", "docsis", "dom", "dos", "dp", "dpc", "dpi", "dpmi", "dpms", "dr", "dram", "dri", "drm", "dsa", "dsdl", "dsdm", "dsl",
    "dslam", "dsn", "dsp", "dsssl", "dtd", "dte", "dtp", "dtr", "dvd", "dvi", "dvr", "dw", "eai", "eap", "eas", "ebcdic", "ebml", "ecc", "ecma", "ecn", "ecos", "ecrs", "eda",
    "edi", "edo", "edsac", "edvac", "eeprom", "eff", "efi", "efm", "efs", "ega", "egp", "eide", "eigrp", "eisa", "elf", "emacs", "ems", "eniac", "eod", "eof", "eol", "eom", "eos",
    "eprom", "erd", "erm", "erp", "esb", "escon", "esd", "esr", "etl", "etw", "euc", "eula", "ewmh", "ext", "fap", "faq", "fasm", "fbdimm", "fcb", "fcs", "fdc", "fdd", "fddi",
    "fdm", "fdma", "fds", "fec", "femb", "fet", "fhs", "ficon", "fifo", "fips", "fl", "flac", "flops", "fmc", "fmo", "foldoc", "fosdem", "fosi", "foss", "fp", "fpga", "fps",
    "fpu", "fqdn", "fru", "fs", "fsb", "fsf", "fsm", "ftp", "ftta", "fttc", "ftth", "fttp", "fud", "fvek", "fws", "fxp", "fyi", "gb", "gcc", "gcj", "gcr", "gdb", "gdi", "geran",
    "gfdl", "gif", "gigo", "gimps", "gis", "gml", "gnu", "goms", "gpasm", "gpfs", "gpg", "gpgpu", "gpib", "gpl", "gprs", "gpt", "gpu", "gsm", "gui", "guid", "gwt", "gyr", "hal",
    "hasp", "hba", "hci", "hcl", "hd", "hdd", "hdl", "hdmi", "hf", "hfs", "hhd", "hid", "hig", "hird", "hlasm", "hls", "hma", "hp", "hpc", "hpfs", "hsdpa", "hsm", "ht", "htc",
    "htm", "html", "http", "https", "htx", "hurd", "hvd", "iana", "ibm", "ic", "icann", "ich", "icmp", "icp", "ics", "ict", "id", "ide", "idf", "idl", "ids", "iec", "ieee",
    "ietf", "ifl", "igmp", "igrp", "ihv", "iiop", "iis", "ike", "il", "im", "imap", "ime", "infosec", "ip", "ipam", "ipc", "ipl", "ipmi", "ipo", "ipp", "ips", "iptv", "ipx", "ir",
    "irc", "iri", "irp", "irq", "isa", "isam", "isatap", "isc", "isdn", "iso", "isp", "ispf", "isr", "isv", "itil", "itl", "itu", "ivcr", "ivrs", "jaxb", "jaxp", "jbod", "jce",
    "jcl", "jcp", "jdbc", "jdk", "jds", "jee", "jes", "jfc", "jfet", "jfs", "jini", "jit", "jme", "jms", "jmx", "jndi", "jni", "jnz", "jpeg", "jre", "js", "jse", "json", "jsp",
    "jtag", "jvm", "kb", "kde", "km", "krl", "kvm", "lacp", "lan", "lb", "lba", "lcd", "lcos", "lcr", "ldap", "le", "led", "lf", "lfs", "lga", "lgpl", "lib", "lif", "lifo",
    "lilo", "lisp", "lkml", "lm", "loc", "lpc", "lpi", "lpt", "lru", "lsb", "lsi", "lte", "ltl", "ltr", "lun", "lv", "lvd", "lvm", "lzw", "mac", "manet", "mapi", "mb", "mbcs",
    "mbd", "mbr", "mca", "mcad", "mcas", "mcdba", "mcdst", "mcitp", "mcm", "mcp", "mcpc", "mcpd", "mcsa", "mcsd", "mcse", "mct", "mcts", "mda", "mdf", "mdi", "mf", "mfc", "mfm",
    "mgcp", "mib", "micr", "midi", "mimd", "mime", "mimo", "minix", "mips", "mis", "misd", "mit", "mmc", "mmds", "mmf", "mmi", "mmio", "mmorpg", "mmu", "mmx", "mng", "mom", "mos",
    "mosfet", "motd", "mous", "mov", "mpaa", "mpeg", "mpl", "mpls", "mpu", "ms", "msa", "msb", "msdn", "msi", "msn", "mt", "mta", "mtbf", "mtu", "mua", "mvc", "mvp", "mvs", "mwc",
    "mx", "mxf", "nack", "nak", "nas", "nasm", "ncp", "ncq", "ncsa", "ndis", "ndps", "nds", "nep", "nfa", "nfc", "nfs", "ngl", "ngscb", "ni", "nic", "nim", "nio", "nist", "nlp",
    "nls", "nmi", "nntp", "noc", "nop", "nos", "np", "npl", "nptl", "npu", "ns", "nsa", "nsi", "nspr", "nss", "nt", "ntfs", "ntlm", "ntp", "numa", "nurbs", "nvr", "nvram", "oat",
    "obsai", "odbc", "oem", "oes", "ofdm", "oftc", "oid", "olap", "ole", "oled", "olpc", "oltp", "omf", "omg", "omr", "oo", "ooe", "oom", "oop", "ootb", "opml", "orb", "orm",
    "os", "oscon", "osdn", "osi", "ospf", "oss", "ostg", "oui", "pap", "parc", "pata", "pbs", "pc", "pcb", "pci", "pcl", "pcm", "pcmcia", "pcre", "pd", "pda", "pdf", "pdh", "pdp",
    "pe", "perl", "pfa", "pg", "pga", "pgo", "pgp", "php", "pid", "pim", "pio", "pkcs", "pki", "plc", "pld", "plt", "pmm", "png", "pnrp", "poid", "pojo", "posix", "ppc", "ppi",
    "ppp", "pptp", "pr", "ps", "psa", "psm", "psu", "psvi", "pv", "pvg", "pvr", "pxe", "pxi", "qa", "qdr", "qfp", "qotd", "qsop", "qtam", "racf", "rad", "raid", "raii", "rait",
    "ram", "rarp", "ras", "rc", "rcs", "rd", "rdbms", "rdc", "rdf", "rdm", "rdos", "rdp", "rds", "refal", "rest", "rf", "rfc", "rfi", "rfid", "rgb", "rgba", "rhel", "rhl", "ria",
    "riaa", "rip", "rir", "risc", "rje", "rle", "rll", "rmi", "rms", "rom", "romb", "rpc", "rpg", "rpm", "rras", "rsa", "rsi", "rss", "rtai", "rtc", "rte", "rtems", "rtl", "rtos",
    "rtp", "rts", "rtsp", "rtti", "rwd", "san", "sas", "sata", "sax", "sbod", "sbu", "scada", "scid", "scm", "scp", "scpc", "scpi", "scsa", "scsi", "sctp", "sd", "sddl", "sdh",
    "sdi", "sdio", "sdk", "sdl", "sdn", "sdp", "sdr", "sdram", "sdsl", "se", "sec", "sei", "seo", "sftp", "sgi", "sgml", "sgr", "sha", "shdsl", "sigcat", "siggraph", "simd",
    "simm", "sip", "sisd", "siso", "sles", "sli", "slm", "sloc", "sma", "smb", "smbios", "sme", "smf", "smil", "smp", "smps", "sms", "smt", "smtp", "sna", "snmp", "sntp", "soa",
    "soe", "soho", "soi", "sopa", "sp", "spa", "sparc", "spf", "spi", "spm", "spmd", "sql", "sram", "ssa", "ssd", "ssdp", "sse", "ssh", "ssi", "ssid", "ssl", "ssp", "ssse",
    "sssp", "sstp", "sus", "suse", "svc", "svd", "svg", "svga", "swf", "swt", "tao", "tapi", "tasm", "tb", "tcp", "tcu", "tdma", "tft", "tftp", "ti", "tla", "tld", "tls", "tlv",
    "tnc", "tpf", "tpm", "troff", "tron", "trsdos", "tso", "tsp", "tsr", "tta", "ttf", "ttl", "tts", "tty", "tucows", "twain", "uaag", "uac", "uart", "uat", "ucs", "uddi", "udma",
    "udp", "uefi", "uhf", "ui", "ul", "ula", "uma", "umb", "uml", "umpc", "umts", "unc", "univac", "ups", "uri", "url", "usb", "usr", "utc", "utf", "utp", "utran", "uucp", "uuid",
    "uun", "uvc", "uwp", "ux", "vax", "vb", "vba", "vbs", "vcpi", "vdm", "vdsl", "vesa", "vfat", "vfs", "vg", "vga", "vhf", "vlan", "vlb", "vlf", "vliw", "vlsi", "vlsm", "vm",
    "vmm", "vnc", "vod", "vpn", "vpu", "vr", "vram", "vrml", "vsam", "vsat", "vt", "vtam", "vtl", "wafs", "wai", "wais", "wan", "wap", "wasm", "wbem", "wcag", "wcf", "wdm", "wep",
    "wfi", "wins", "wlan", "wma", "wmi", "wmv", "wns", "wol", "wor", "wora", "wpa", "wpad", "wpan", "wpf", "wsdl", "wsfl", "wusb", "wwan", "wwdc", "wwid", "wwn", "www", "wysiwyg",
    "wzc", "xag", "xaml", "xcbl", "xdm", "xdmcp", "xhtml", "xilp", "xml", "xmms", "xmpp", "xms", "xns", "xp", "xpcom", "xpi", "xpidl", "xps", "xsd", "xsl", "xslt", "xss", "xtf",
    "xul", "xvga", "yaaf", "yacc", "yaml", "zcav", "zcs", "zif", "zifs", "zisc", "zma", "zoi", "zope", "zpl",
];

/// Identifier used in Rust structs, enums, and fields. It includes the `original` name and the `renamed` value after the transformation based on `serde` attributes.
#[derive(Clone)]
pub struct Id {
    pub original: String,
    pub renamed: String,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.original == self.renamed {
            write!(f, "({})", self.original)
        } else {
            write!(f, "({}, {})", self.original, self.renamed)
        }
    }
}

/// Rust struct.
pub struct RustStruct {
    pub id: Id,
    pub fields: Vec<RustField>,
    pub comments: Vec<String>,
}

/// Rust field defintion.
pub struct RustField {
    pub id: Id,
    pub ty: String,
    pub is_optional: bool,
    pub is_vec: bool,
    pub is_hash_map: bool,
    pub comments: Vec<String>,
}

/// Definition of enums in Rust
pub enum RustEnum {
    Constant(RustConstEnum),
    Algebraic(RustAlgebraicEnum),
}

/// Definition of constant enums.
pub struct RustConstEnum {
    pub id: Id,
    pub comments: Vec<String>,
    pub ty: Option<syn::Lit>,
    pub cases: Vec<RustConst>,
}

pub struct RustConst {
    pub id: Id,
    pub comments: Vec<String>,
    pub value: Option<syn::ExprLit>,
}

pub struct RustAlgebraicEnum {
    pub id: Id,
    pub comments: Vec<String>,
    pub cases: Vec<RustAlgebraicEnumCase>,
}

pub struct RustAlgebraicEnumCase {
    pub id: Id,
    pub comments: Vec<String>,
    pub value: RustField,
}

pub trait Language {
    fn begin_file(&mut self, _w: &mut dyn Write, _params: &Params) -> std::io::Result<()> {
        Ok(())
    }

    fn end_file(&mut self, _w: &mut dyn Write, _params: &Params) -> std::io::Result<()> {
        Ok(())
    }

    fn write_struct(&mut self, w: &mut dyn Write, params: &Params, rs: &RustStruct) -> std::io::Result<()>;

    fn write_const_enum(&mut self, _w: &mut dyn Write, _params: &Params, _e: &RustConstEnum) -> std::io::Result<()> {
        Ok(())
    }

    fn write_algebraic_enum(&mut self, _w: &mut dyn Write, _params: &Params, _e: &RustAlgebraicEnum) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct Params {
    pub use_marker: bool,
    pub swift_prefix: String,
    pub java_package: String,
}

pub struct Generator<'l> {
    params: Params,
    language: &'l mut dyn Language,
    serde_rename_all: Option<String>,

    structs: Vec<RustStruct>,
    enums: Vec<RustEnum>,
}

impl<'l> Generator<'l> {
    pub fn new(language: &'l mut dyn Language, params: Params) -> Self {
        Self {
            params,
            language,
            serde_rename_all: None,

            structs: Vec::new(),
            enums: Vec::new(),
        }
    }

    pub fn process_file(&mut self, filename: &str, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let source = fs::read_to_string(filename)?;
        self.process_source(source, w)?;
        Ok(())
    }

    pub fn process_source(&mut self, source: String, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let source = syn::parse_file(&source)?;
        for item in source.items.iter() {
            match item {
                syn::Item::Struct(s) => self.parse_struct(&s)?,
                syn::Item::Enum(e) => self.parse_enum(&e)?,
                syn::Item::Fn(_) => {}
                _ => {}
            }
        }

        self.write(w)?;
        Ok(())
    }

    pub fn write(&mut self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        self.language.begin_file(w, &self.params)?;

        for s in &self.structs {
            self.language.write_struct(w, &self.params, &s)?;
        }

        for e in &self.enums {
            match e {
                RustEnum::Constant(const_enum) => self.language.write_const_enum(w, &self.params, &const_enum)?,
                RustEnum::Algebraic(algebraic_enum) => self.language.write_algebraic_enum(w, &self.params, &algebraic_enum)?,
            }
        }

        self.language.end_file(w, &self.params)?;
        Ok(())
    }

    fn parse_struct(&mut self, s: &syn::ItemStruct) -> std::io::Result<()> {
        if self.params.use_marker && !has_typeshare_marker(&s.attrs) {
            return Ok(());
        }

        self.serde_rename_all = serde_rename_all(&s.attrs);

        let mut rs = RustStruct {
            id: get_ident(Some(&s.ident), &s.attrs, &self.serde_rename_all),
            fields: Vec::new(),
            comments: Vec::new(),
        };
        self.parse_comment_attrs(&mut rs.comments, &s.attrs)?;

        for f in s.fields.iter() {
            self.parse_field(&mut rs, &f)?;
        }

        self.serde_rename_all = None;
        self.structs.push(rs);
        Ok(())
    }

    fn parse_field(&mut self, rs: &mut RustStruct, f: &syn::Field) -> std::io::Result<()> {
        let mut ty: &str = &type_as_string(&f.ty);
        let is_optional = ty.starts_with(OPTION_PREFIX);
        if is_optional {
            ty = remove_prefix_suffix(&ty, OPTION_PREFIX, OPTION_SUFFIX);
        }

        let is_vec = ty.starts_with(VEC_PREFIX);
        let is_hash_map = ty.starts_with(HASH_MAP_PREFIX);
        if is_vec {
            ty = &remove_prefix_suffix(&ty, VEC_PREFIX, VEC_SUFFIX);
        } else if is_hash_map {
            ty = &remove_prefix_suffix(&ty, HASH_MAP_PREFIX, HASH_MAP_SUFFIX);
        }

        let mut rf = RustField {
            id: get_ident(f.ident.as_ref(), &f.attrs, &self.serde_rename_all),
            ty: ty.to_owned(),
            is_optional,
            is_vec,
            is_hash_map,
            comments: Vec::new(),
        };
        self.parse_comment_attrs(&mut rf.comments, &f.attrs)?;

        rs.fields.push(rf);
        Ok(())
    }

    fn parse_enum(&mut self, e: &syn::ItemEnum) -> std::io::Result<()> {
        if self.params.use_marker && !has_typeshare_marker(&e.attrs) {
            return Ok(());
        }

        self.serde_rename_all = serde_rename_all(&e.attrs);
        if is_const_enum(e) {
            self.parse_const_enum(e)?;
        } else {
            self.parse_algebraic_enum(e)?;
        }
        self.serde_rename_all = None;
        Ok(())
    }

    fn parse_const_enum(&mut self, e: &syn::ItemEnum) -> std::io::Result<()> {
        let mut re = RustConstEnum {
            id: get_ident(Some(&e.ident), &e.attrs, &self.serde_rename_all),
            comments: Vec::new(),
            ty: get_const_enum_type(e).clone(),
            cases: Vec::new(),
        };
        self.parse_comment_attrs(&mut re.comments, &e.attrs)?;

        for v in e.variants.iter() {
            let mut rc = RustConst {
                id: get_ident(Some(&v.ident), &v.attrs, &self.serde_rename_all),
                value: get_discriminant(&v),
                comments: Vec::new(),
            };

            self.parse_comment_attrs(&mut rc.comments, &v.attrs)?;
            re.cases.push(rc);
        }

        self.enums.push(RustEnum::Constant(re));

        Ok(())
    }

    fn parse_algebraic_enum(&mut self, e: &syn::ItemEnum) -> std::io::Result<()> {
        let mut parsed_enum = RustAlgebraicEnum {
            id: get_ident(Some(&e.ident), &e.attrs, &self.serde_rename_all),
            comments: Vec::new(),
            cases: Vec::new(),
        };
        self.parse_comment_attrs(&mut parsed_enum.comments, &e.attrs)?;

        for variant in e.variants.iter() {
            let mut parsed_case = RustAlgebraicEnumCase {
                id: get_ident(Some(&variant.ident), &variant.attrs, &self.serde_rename_all),
                value: get_algebraic_enum_case_value(&variant, &self.serde_rename_all),
                comments: Vec::new(),
            };
            self.parse_comment_attrs(&mut parsed_case.comments, &variant.attrs)?;

            parsed_enum.cases.push(parsed_case);
        }
        self.enums.push(RustEnum::Algebraic(parsed_enum));

        Ok(())
    }

    //----

    fn parse_comment_attrs(&mut self, comments: &mut Vec<String>, attrs: &[syn::Attribute]) -> std::io::Result<()> {
        for a in attrs.iter() {
            let s = a.tts.to_string();
            if s.starts_with(COMMENT_PREFIX) {
                comments.push(remove_prefix_suffix(&s, COMMENT_PREFIX, COMMENT_SUFFIX).to_owned());
            }
        }

        Ok(())
    }
}

fn get_discriminant(v: &syn::Variant) -> Option<syn::ExprLit> {
    if let Some(d) = &v.discriminant {
        match &d.1 {
            syn::Expr::Lit(l) => {
                return Some(l.clone());
            }
            _ => {
                panic!("unexpected expr");
            }
        }
    }

    None
}

fn get_algebraic_enum_case_value(v: &syn::Variant, serde_rename_all: &Option<String>) -> RustField {
    match &v.fields {
        syn::Fields::Unnamed(associated_type) => {
            if associated_type.unnamed.len() > 1 {
                panic!("Unable to handle multiple unamed associated types yet");
            }

            let first_type = associated_type.unnamed.clone().first().unwrap().into_value().ty.clone();

            let mut ty: &str = &type_as_string(&first_type);
            let is_optional = ty.starts_with(OPTION_PREFIX);
            if is_optional {
                ty = remove_prefix_suffix(&ty, OPTION_PREFIX, OPTION_SUFFIX);
            }

            let is_vec = ty.starts_with(VEC_PREFIX);
            let is_hash_map = ty.starts_with(HASH_MAP_PREFIX);
            if is_vec {
                ty = &remove_prefix_suffix(&ty, VEC_PREFIX, VEC_SUFFIX);
            } else if is_hash_map {
                ty = &remove_prefix_suffix(&ty, HASH_MAP_PREFIX, HASH_MAP_SUFFIX);
            }

            RustField {
                id: get_ident(Some(&v.ident), &v.attrs, serde_rename_all),
                ty: ty.to_owned(),
                is_optional,
                is_vec,
                is_hash_map,
                comments: Vec::new(),
            }
        }
        _ => panic!("Call this method for Unnamed cases only"),
    }
}

fn is_const_enum(e: &syn::ItemEnum) -> bool {
    for v in e.variants.iter() {
        match v.fields {
            syn::Fields::Named(_) => {
                panic!("Can't handle complex enum");
            }
            syn::Fields::Unnamed(_) => return false,
            syn::Fields::Unit => {}
        }
    }

    true
}

fn get_const_enum_type(e: &syn::ItemEnum) -> Option<syn::Lit> {
    if is_const_enum(e) {
        if let Some(discriminant) = &e.variants.first().unwrap().into_value().discriminant {
            return match &discriminant.1 {
                syn::Expr::Lit(expr_lit) => Some(expr_lit.lit.clone()),
                _ => None,
            };
        }
    }
    None
}

fn type_as_string(ty: &syn::Type) -> String {
    use quote::ToTokens;

    let mut tokens = proc_macro2::TokenStream::new();
    ty.to_tokens(&mut tokens);
    tokens.to_string()
}

fn get_ident(ident: Option<&proc_macro2::Ident>, attrs: &[syn::Attribute], rename_all: &Option<String>) -> Id {
    let original = ident.map_or("???".to_string(), |id| id.to_string().replace("r#", ""));
    let mut renamed = match rename_all {
        None => original.clone(),
        Some(value) => match value.as_str() {
            "lowercase" => original.to_lowercase(),
            "UPPERCASE" => original.to_uppercase(),
            "PascalCase" => original.to_pascal_case(),
            "camelCase" => original.to_camel_case(),
            "snake_case" => original.to_snake_case(),
            "SCREAMING_SNAKE_CASE" => original.to_screaming_snake_case(),
            "kebab-case" => original.to_kebab_case(),
            "SCREAMING-KEBAB-CASE" => original.to_kebab_case(),
            _ => original.clone(),
        },
    };

    if let Some(s) = serde_rename(attrs) {
        renamed = s;
    }

    Id { original, renamed }
}

fn serde_rename(attrs: &[syn::Attribute]) -> Option<String> {
    const PREFIX: &str = r##"rename = ""##;
    const SUFFIX: &str = r##"""##;
    attr_value(attrs, PREFIX, SUFFIX)
}

fn serde_rename_all(attrs: &[syn::Attribute]) -> Option<String> {
    const PREFIX: &str = r##"rename_all = ""##;
    const SUFFIX: &str = r##"""##;
    attr_value(attrs, PREFIX, SUFFIX)
}

fn has_typeshare_marker(attrs: &[syn::Attribute]) -> bool {
    const TYPESHARE_MARKER: &str = "typeshare";
    let typeshare_ident = Ident::new(TYPESHARE_MARKER, Span::call_site());
    for a in attrs {
        if let Some(segment) = a.path.segments.iter().next() {
            if segment.ident == typeshare_ident {
                return true;
            }
        }
    }

    false
}

/*
    Process attributes and return value of the matching attribute, if found.
    ```
    [
    Attribute
        {
            pound_token: Pound,
            style: Outer,
            bracket_token: Bracket,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment { ident: Ident(doc), arguments: None }
                ]
            },
            tts: TokenStream [
                Punct { op: '=', spacing: Alone },
                Literal { lit: " This is a comment." }]
        },

    Attribute
        {
            pound_token: Pound,
            style: Outer,
            bracket_token: Bracket,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment { ident: Ident(serde), arguments: None }
                ]
            }
            tts: TokenStream [
                Group {
                    delimiter: Parenthesis,
                    stream: TokenStream [
                        Ident { sym: default },
                        Punct { op: ',', spacing: Alone },
                        Ident { sym: rename_all },
                        Punct { op: '=', spacing: Alone },
                        Literal { lit: "camelCase" }
                    ]
                }
            ]
        }
    ]
    ```
*/
fn attr_value(attrs: &[syn::Attribute], prefix: &'static str, suffix: &'static str) -> Option<String> {
    for a in attrs {
        if let Some(segment) = a.path.segments.iter().next() {
            if segment.ident != Ident::new("serde", Span::call_site()) {
                continue;
            }

            let attr_as_string = a.tts.to_string();
            let values = parse_attr(&attr_as_string)?;

            for v in values {
                if v.starts_with(prefix) && v.ends_with(suffix) {
                    return Some(remove_prefix_suffix(&v, prefix, suffix).to_string());
                }
            }
        }
    }

    None
}

fn parse_attr<'x>(attr: &'x str) -> Option<Vec<&'x str>> {
    const ATTR_PREFIX: &str = "( ";
    const ATTR_SUFFIX: &str = " )";

    if attr.starts_with(ATTR_PREFIX) && attr.ends_with(ATTR_SUFFIX) {
        let attr = remove_prefix_suffix(attr, ATTR_PREFIX, ATTR_SUFFIX);
        return Some(attr.split(" , ").collect());
    }

    None
}

fn remove_prefix_suffix<'a>(src: &'a str, prefix: &'static str, suffix: &'static str) -> &'a str {
    if src.starts_with(prefix) && src.ends_with(suffix) {
        return &src[prefix.len()..src.len() - suffix.len()];
    }
    src
}
