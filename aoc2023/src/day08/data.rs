pub const TEST_DATA_0_NAV: &[u8] = b"RL";

pub const TEST_DATA_0_NODES: [(&[u8; 3], &[u8; 3], &[u8; 3]); 7] = [
    (b"AAA", b"BBB", b"CCC"),
    (b"BBB", b"DDD", b"EEE"),
    (b"CCC", b"ZZZ", b"GGG"),
    (b"DDD", b"DDD", b"DDD"),
    (b"EEE", b"EEE", b"EEE"),
    (b"GGG", b"GGG", b"GGG"),
    (b"ZZZ", b"ZZZ", b"ZZZ"),
];

pub const TEST_DATA_1_NAV: &[u8] = b"LLR";

pub const TEST_DATA_1_NODES: [(&[u8; 3], &[u8; 3], &[u8; 3]); 3] = [
    (b"AAA", b"BBB", b"BBB"),
    (b"BBB", b"AAA", b"ZZZ"),
    (b"ZZZ", b"ZZZ", b"ZZZ"),
];

pub const TEST_DATA_2_NAV: &[u8] = b"LRLRLLRLRLRRLRLRLRRLRLRLLRRLRRLRLRLRLLRRRLRRRLLRRLRLRLRRRLRRLRRRLRLRLRRLRLLRLRLRRLRRRLRLRRLRRRLLRLRLRRRLRRRLRLRRRLRLRRRLLRRLLLRRRLLRRRLRRRLRRRLRLRLRLLRLRRLRLRLLLRRLRRLRRLRLRRLRRLLRRLRLRRRLRLRLLRRRLRRRLRRRLLLRRRLRLRLRRLRRRLRRRLRLRRRLRRLRRRLRLRRLLRRRLRRRLLLRRLRLRLRRLRRRLRRLRRLRLRRRR";

pub const TEST_DATA_2_NODES: [(&[u8; 3], &[u8; 3], &[u8; 3]); 738] = [
    (b"GXT", b"MQM", b"CHN"),
    (b"MBK", b"RCK", b"RCK"),
    (b"HBS", b"QHS", b"RXC"),
    (b"SXK", b"FDB", b"FKP"),
    (b"NJB", b"BSB", b"KJM"),
    (b"SPD", b"FNL", b"RSH"),
    (b"FJF", b"NFH", b"XJN"),
    (b"GHV", b"LSV", b"BTS"),
    (b"QDT", b"HXV", b"PDX"),
    (b"MDH", b"XDK", b"DKN"),
    (b"AAA", b"FKL", b"CFC"),
    (b"GRB", b"VDP", b"LMM"),
    (b"CXK", b"DVB", b"CRJ"),
    (b"FDB", b"FTD", b"CNK"),
    (b"LQT", b"BJV", b"SMQ"),
    (b"TSK", b"NQD", b"VSG"),
    (b"VLF", b"NDS", b"CTV"),
    (b"PGP", b"DKC", b"CKL"),
    (b"PVJ", b"FDB", b"FKP"),
    (b"VSV", b"NFP", b"QHX"),
    (b"KXN", b"XJN", b"NFH"),
    (b"KMQ", b"VBH", b"XXH"),
    (b"QXR", b"RMD", b"TLT"),
    (b"DLN", b"TPD", b"KBG"),
    (b"BHK", b"GRP", b"RXF"),
    (b"TSX", b"HQP", b"SHK"),
    (b"PTV", b"VSG", b"NQD"),
    (b"QVN", b"XBH", b"DHC"),
    (b"DDM", b"TCB", b"XRQ"),
    (b"NKD", b"CDR", b"BJM"),
    (b"JNR", b"FMC", b"SQN"),
    (b"VPQ", b"JGC", b"VCJ"),
    (b"HPB", b"STQ", b"DDM"),
    (b"HRT", b"JNR", b"BGH"),
    (b"CNQ", b"HQV", b"PJQ"),
    (b"PMG", b"LRB", b"XXP"),
    (b"RKV", b"XGN", b"VCG"),
    (b"KVQ", b"KHS", b"SLV"),
    (b"MDM", b"VDX", b"NSF"),
    (b"VHT", b"PGP", b"GJS"),
    (b"BPD", b"NBF", b"VNH"),
    (b"JCQ", b"JCB", b"XVR"),
    (b"CFJ", b"PQP", b"CBJ"),
    (b"DSX", b"BXN", b"VDS"),
    (b"MGH", b"PFV", b"NLQ"),
    (b"MPK", b"FND", b"BJX"),
    (b"QFR", b"HFC", b"CNG"),
    (b"PHS", b"VNH", b"NBF"),
    (b"KTT", b"MTG", b"SQM"),
    (b"JBK", b"CSR", b"VXV"),
    (b"BKL", b"DLB", b"SHQ"),
    (b"GQH", b"PCH", b"LDZ"),
    (b"XVJ", b"CQM", b"SLF"),
    (b"VBN", b"FFF", b"PDL"),
    (b"KQH", b"SLT", b"XLG"),
    (b"SSN", b"PQH", b"LFT"),
    (b"MQM", b"SBH", b"TTC"),
    (b"SCR", b"HGH", b"QGS"),
    (b"XTZ", b"XKM", b"JTJ"),
    (b"ZZZ", b"CFC", b"FKL"),
    (b"PRA", b"GRB", b"MDB"),
    (b"VTV", b"LXQ", b"HKP"),
    (b"PPX", b"SQM", b"MTG"),
    (b"PVA", b"MRJ", b"CVH"),
    (b"BJH", b"VTR", b"RKG"),
    (b"KPH", b"RFD", b"NJP"),
    (b"HXG", b"NVJ", b"HNG"),
    (b"LRX", b"DCX", b"MGH"),
    (b"NFD", b"PFR", b"FVQ"),
    (b"TDG", b"TDH", b"ZZZ"),
    (b"XKM", b"XDG", b"VHT"),
    (b"CKT", b"LDC", b"VPF"),
    (b"QQR", b"BFQ", b"FXC"),
    (b"RGJ", b"KTV", b"GFR"),
    (b"XDK", b"MVC", b"SPX"),
    (b"TLT", b"MBK", b"DGL"),
    (b"CDR", b"JBG", b"KPH"),
    (b"LTH", b"DFN", b"BHK"),
    (b"PVV", b"BFQ", b"FXC"),
    (b"FCK", b"FRJ", b"KVG"),
    (b"FXB", b"MDH", b"VSJ"),
    (b"DHD", b"LDJ", b"RNH"),
    (b"HHR", b"HSK", b"CJD"),
    (b"LSV", b"LNP", b"NMD"),
    (b"JSX", b"DPF", b"SNN"),
    (b"SBH", b"XKD", b"GTX"),
    (b"BHJ", b"JSX", b"TMN"),
    (b"CQX", b"XPG", b"RLB"),
    (b"XRQ", b"FJK", b"NCF"),
    (b"GMF", b"JBX", b"DRV"),
    (b"KBD", b"HCC", b"TND"),
    (b"KGT", b"NLK", b"FRX"),
    (b"RKM", b"RTM", b"FMQ"),
    (b"GNM", b"GML", b"NLB"),
    (b"SXP", b"RKG", b"VTR"),
    (b"PMS", b"XBH", b"DHC"),
    (b"PQH", b"PJD", b"CXK"),
    (b"XXH", b"BMB", b"JGJ"),
    (b"RHL", b"QHS", b"RXC"),
    (b"DFS", b"JVK", b"VBN"),
    (b"QCS", b"XGN", b"VCG"),
    (b"MJJ", b"DDM", b"STQ"),
    (b"SPV", b"PXX", b"RRT"),
    (b"FRJ", b"SNX", b"MPQ"),
    (b"FKC", b"CBF", b"RKM"),
    (b"SRG", b"QFH", b"LGP"),
    (b"VVQ", b"BJV", b"SMQ"),
    (b"BQC", b"DSP", b"TXQ"),
    (b"SHQ", b"NKR", b"RGJ"),
    (b"KCS", b"QGS", b"HGH"),
    (b"MTV", b"BXN", b"VDS"),
    (b"CNC", b"HPB", b"MJJ"),
    (b"SNF", b"XXP", b"LRB"),
    (b"KTP", b"QRX", b"KML"),
    (b"GSC", b"HSK", b"CJD"),
    (b"QRX", b"CRB", b"TXL"),
    (b"MDB", b"VDP", b"LMM"),
    (b"PFV", b"CLM", b"CTN"),
    (b"RRB", b"HHR", b"GSC"),
    (b"DDQ", b"PXX", b"RRT"),
    (b"TJB", b"LSX", b"XNR"),
    (b"BSP", b"TXC", b"LKB"),
    (b"MMV", b"DKK", b"NFD"),
    (b"DCB", b"XKP", b"TNJ"),
    (b"QKV", b"CTT", b"SFM"),
    (b"XDG", b"PGP", b"GJS"),
    (b"JMJ", b"JLK", b"NJB"),
    (b"VBJ", b"HJV", b"JFD"),
    (b"QHM", b"CLJ", b"QGP"),
    (b"STR", b"HXG", b"RFC"),
    (b"DPF", b"VNQ", b"DQQ"),
    (b"RXF", b"NHV", b"GBX"),
    (b"FXC", b"JLX", b"LBS"),
    (b"XVR", b"MVT", b"QHD"),
    (b"KNM", b"BJD", b"QCK"),
    (b"GTX", b"MSB", b"FNP"),
    (b"XCS", b"PTN", b"NRH"),
    (b"CGS", b"FRX", b"NLK"),
    (b"RCK", b"CQM", b"CQM"),
    (b"CDS", b"PRR", b"GCJ"),
    (b"BXN", b"KQC", b"MSV"),
    (b"PTN", b"VST", b"QHJ"),
    (b"TPD", b"HJF", b"TSS"),
    (b"RNH", b"TTK", b"DBN"),
    (b"TSM", b"TPD", b"KBG"),
    (b"KTG", b"GPT", b"BMD"),
    (b"PJP", b"XBR", b"TCM"),
    (b"QPF", b"KJF", b"NJX"),
    (b"HKP", b"PLS", b"PGK"),
    (b"KBG", b"HJF", b"TSS"),
    (b"SQK", b"LSD", b"KNM"),
    (b"XKD", b"MSB", b"FNP"),
    (b"QQP", b"LFB", b"LTK"),
    (b"MDX", b"KTJ", b"FXF"),
    (b"JLK", b"KJM", b"BSB"),
    (b"FBR", b"PMG", b"SNF"),
    (b"GSJ", b"TDH", b"TDH"),
    (b"DQQ", b"FKK", b"KRF"),
    (b"CTV", b"CQD", b"RQX"),
    (b"MNT", b"RLG", b"XXQ"),
    (b"GFL", b"FGJ", b"KMD"),
    (b"BJK", b"VQX", b"JCQ"),
    (b"SPN", b"NKS", b"CNQ"),
    (b"LNL", b"XLM", b"XLC"),
    (b"HKF", b"HKJ", b"NNC"),
    (b"FTD", b"CCX", b"MSS"),
    (b"RPV", b"MDQ", b"DGS"),
    (b"DCX", b"NLQ", b"PFV"),
    (b"JGB", b"DVT", b"DRH"),
    (b"CGM", b"CHN", b"MQM"),
    (b"JNQ", b"PPX", b"KTT"),
    (b"BJM", b"JBG", b"KPH"),
    (b"NFP", b"SPD", b"XCT"),
    (b"TPL", b"LSD", b"KNM"),
    (b"XTV", b"CTJ", b"HPX"),
    (b"LKB", b"PDF", b"SLK"),
    (b"QVK", b"KMQ", b"MGF"),
    (b"QVP", b"VCX", b"JNQ"),
    (b"NSR", b"TJB", b"NBB"),
    (b"XLA", b"DLB", b"SHQ"),
    (b"QLP", b"HBS", b"RHL"),
    (b"BGH", b"FMC", b"SQN"),
    (b"BJX", b"LRX", b"PNM"),
    (b"MVC", b"BPD", b"PHS"),
    (b"DXQ", b"TMN", b"JSX"),
    (b"PBK", b"QKV", b"CTH"),
    (b"GBJ", b"RNH", b"LDJ"),
    (b"BMB", b"HJS", b"TJT"),
    (b"SHT", b"MFQ", b"LCT"),
    (b"HSK", b"CGS", b"KGT"),
    (b"DRH", b"PKN", b"PBN"),
    (b"PQF", b"XKP", b"TNJ"),
    (b"RPX", b"HCT", b"HQT"),
    (b"FPH", b"VQL", b"NPJ"),
    (b"GLC", b"PMS", b"QVN"),
    (b"VDP", b"SXK", b"PVJ"),
    (b"JFD", b"QBD", b"DGT"),
    (b"LQL", b"NPT", b"GVS"),
    (b"TPQ", b"NHL", b"MTD"),
    (b"KRF", b"HVT", b"MPK"),
    (b"JQG", b"QDT", b"PCK"),
    (b"PJQ", b"KVH", b"LBV"),
    (b"VBB", b"XXQ", b"RLG"),
    (b"PSH", b"SQV", b"TLB"),
    (b"VXH", b"VCX", b"JNQ"),
    (b"VQD", b"NBB", b"TJB"),
    (b"JVD", b"GDJ", b"JGQ"),
    (b"SDR", b"FTF", b"NQL"),
    (b"NQQ", b"TFM", b"CMC"),
    (b"FNL", b"LTV", b"CSX"),
    (b"FVS", b"NSF", b"VDX"),
    (b"VTJ", b"VXH", b"QVP"),
    (b"KQC", b"LGG", b"LGG"),
    (b"SCS", b"MTV", b"DSX"),
    (b"QPS", b"HMV", b"QQP"),
    (b"CQM", b"JDD", b"JDD"),
    (b"FJX", b"VBB", b"MNT"),
    (b"GBQ", b"BXV", b"LFR"),
    (b"HGH", b"SQK", b"TPL"),
    (b"SLT", b"FXB", b"NGN"),
    (b"PBD", b"HDC", b"FCX"),
    (b"CKL", b"MQS", b"NND"),
    (b"LSX", b"MBF", b"SPN"),
    (b"HNG", b"JQG", b"GKD"),
    (b"MFQ", b"PSH", b"TGK"),
    (b"SNX", b"GCS", b"KSF"),
    (b"FMQ", b"DXT", b"DFS"),
    (b"TXC", b"SLK", b"PDF"),
    (b"VST", b"SSN", b"LSR"),
    (b"BFB", b"VSV", b"SVC"),
    (b"VNX", b"TMF", b"QFS"),
    (b"VPK", b"VBB", b"MNT"),
    (b"NVR", b"XPG", b"RLB"),
    (b"FLS", b"FSL", b"CKH"),
    (b"FKP", b"FTD", b"CNK"),
    (b"MGF", b"XXH", b"VBH"),
    (b"LNM", b"NFD", b"DKK"),
    (b"FNP", b"VTT", b"MVG"),
    (b"HDC", b"PXP", b"GVH"),
    (b"KVL", b"LCT", b"MFQ"),
    (b"XGP", b"RHG", b"RTK"),
    (b"JCT", b"NJB", b"JLK"),
    (b"XSL", b"SVC", b"VSV"),
    (b"SRS", b"HPD", b"XBL"),
    (b"FSL", b"JMG", b"TFD"),
    (b"CXB", b"KQV", b"QGK"),
    (b"QRP", b"CGF", b"CQF"),
    (b"PRG", b"MDX", b"KPL"),
    (b"VLL", b"MRB", b"MHN"),
    (b"VNQ", b"KRF", b"FKK"),
    (b"NPJ", b"VPK", b"FJX"),
    (b"MHN", b"SRG", b"TTL"),
    (b"KFN", b"DFN", b"BHK"),
    (b"FVQ", b"LJM", b"TJV"),
    (b"GKD", b"QDT", b"PCK"),
    (b"GVS", b"FCK", b"NGC"),
    (b"MBF", b"CNQ", b"NKS"),
    (b"SMQ", b"KSV", b"KMF"),
    (b"CRJ", b"GSJ", b"TDG"),
    (b"LHT", b"BTL", b"KMB"),
    (b"KJM", b"KJC", b"BGC"),
    (b"TTC", b"GTX", b"XKD"),
    (b"QFT", b"PRM", b"KXS"),
    (b"SDS", b"JMP", b"TSR"),
    (b"BLX", b"PQF", b"DCB"),
    (b"PXX", b"LJT", b"QFK"),
    (b"SQV", b"FCB", b"HXF"),
    (b"RQX", b"DBJ", b"VTV"),
    (b"THV", b"GJL", b"MND"),
    (b"SLK", b"XBP", b"CXB"),
    (b"LVS", b"CTJ", b"HPX"),
    (b"TSS", b"CGL", b"JST"),
    (b"MTG", b"RKT", b"GTJ"),
    (b"FFF", b"GBQ", b"VTM"),
    (b"XMV", b"KVL", b"SHT"),
    (b"FDJ", b"MLG", b"MLG"),
    (b"HPD", b"MQK", b"FLS"),
    (b"TLB", b"HXF", b"FCB"),
    (b"BGC", b"SCR", b"KCS"),
    (b"FBM", b"PBD", b"MFK"),
    (b"TJP", b"JSS", b"GHV"),
    (b"TXL", b"NQC", b"RXH"),
    (b"TKM", b"GLQ", b"TKP"),
    (b"RNS", b"BJK", b"PHJ"),
    (b"RRT", b"LJT", b"QFK"),
    (b"DHC", b"XVF", b"STR"),
    (b"RXM", b"KVL", b"SHT"),
    (b"JBX", b"GBJ", b"DHD"),
    (b"TBR", b"HJV", b"JFD"),
    (b"MFK", b"HDC", b"FCX"),
    (b"TFD", b"QXN", b"QSK"),
    (b"HMV", b"LFB", b"LTK"),
    (b"RMD", b"MBK", b"DGL"),
    (b"RJP", b"CTX", b"TPQ"),
    (b"CBF", b"FMQ", b"RTM"),
    (b"XXG", b"JDH", b"TJP"),
    (b"PDX", b"XGQ", b"BSP"),
    (b"XBL", b"MQK", b"FLS"),
    (b"GMT", b"PCB", b"GTK"),
    (b"DGT", b"GNH", b"KVQ"),
    (b"CHN", b"TTC", b"SBH"),
    (b"NKS", b"HQV", b"PJQ"),
    (b"HJC", b"DXQ", b"BHJ"),
    (b"CSR", b"TSX", b"HSM"),
    (b"XTK", b"BJM", b"CDR"),
    (b"DRT", b"MPB", b"PSV"),
    (b"HQT", b"CRM", b"XTZ"),
    (b"HNM", b"SXJ", b"GMF"),
    (b"NLQ", b"CLM", b"CTN"),
    (b"LTK", b"PGS", b"SCS"),
    (b"DNR", b"DRT", b"QLK"),
    (b"QVR", b"DLN", b"TSM"),
    (b"VLD", b"GSP", b"CKT"),
    (b"LFT", b"PJD", b"CXK"),
    (b"KVG", b"SNX", b"MPQ"),
    (b"FBS", b"JSN", b"KFT"),
    (b"GTK", b"XHH", b"LCL"),
    (b"KJZ", b"MDB", b"GRB"),
    (b"QGK", b"PKK", b"VFK"),
    (b"VDS", b"KQC", b"MSV"),
    (b"KHS", b"TRF", b"JVD"),
    (b"BHH", b"MLG", b"RPX"),
    (b"JVL", b"CQF", b"CGF"),
    (b"RGQ", b"CQJ", b"KTG"),
    (b"VTM", b"LFR", b"BXV"),
    (b"GML", b"VFF", b"VLD"),
    (b"CTJ", b"FKC", b"PDT"),
    (b"BJV", b"KSV", b"KSV"),
    (b"SPX", b"BPD", b"PHS"),
    (b"CTX", b"NHL", b"MTD"),
    (b"LRB", b"HHP", b"CDS"),
    (b"XHH", b"MKX", b"KQH"),
    (b"FDL", b"QFT", b"MNG"),
    (b"CCX", b"HKF", b"VPS"),
    (b"GPT", b"VHX", b"QPF"),
    (b"VFK", b"VJX", b"QPS"),
    (b"QFH", b"LHP", b"CKD"),
    (b"KMD", b"QFR", b"XVX"),
    (b"MSB", b"VTT", b"MVG"),
    (b"TRF", b"JGQ", b"GDJ"),
    (b"PHJ", b"VQX", b"JCQ"),
    (b"DBN", b"NRQ", b"GBN"),
    (b"JPM", b"PMG", b"SNF"),
    (b"TND", b"HQB", b"XXT"),
    (b"FJK", b"XHV", b"FBS"),
    (b"PNM", b"DCX", b"MGH"),
    (b"QFK", b"RKR", b"QVK"),
    (b"VNV", b"NRC", b"XXN"),
    (b"QXN", b"PSG", b"CFJ"),
    (b"NND", b"HJC", b"MPN"),
    (b"DQM", b"TJP", b"JDH"),
    (b"PGK", b"XQK", b"PRG"),
    (b"HMK", b"BJK", b"PHJ"),
    (b"DLB", b"RGJ", b"NKR"),
    (b"PXP", b"XTK", b"NKD"),
    (b"LGG", b"CNP", b"CNP"),
    (b"HKJ", b"VQD", b"NSR"),
    (b"TMN", b"SNN", b"DPF"),
    (b"FRX", b"FBR", b"JPM"),
    (b"PTA", b"JTJ", b"XKM"),
    (b"NRH", b"VST", b"QHJ"),
    (b"BQD", b"KTP", b"GFF"),
    (b"VTT", b"KFN", b"LTH"),
    (b"GMV", b"VCJ", b"JGC"),
    (b"XLG", b"FXB", b"NGN"),
    (b"JDH", b"GHV", b"JSS"),
    (b"TJT", b"JCT", b"JMJ"),
    (b"HCT", b"CRM", b"CRM"),
    (b"BGR", b"JMP", b"TSR"),
    (b"CKH", b"JMG", b"TFD"),
    (b"DHX", b"QGP", b"CLJ"),
    (b"BQN", b"TBN", b"BTQ"),
    (b"RSH", b"CSX", b"LTV"),
    (b"SXJ", b"DRV", b"JBX"),
    (b"MRB", b"SRG", b"TTL"),
    (b"GTG", b"SPV", b"DDQ"),
    (b"VQL", b"VPK", b"FJX"),
    (b"RXC", b"GXT", b"CGM"),
    (b"MSV", b"LGG", b"HLF"),
    (b"PSV", b"GBL", b"RRB"),
    (b"LRV", b"XTV", b"LVS"),
    (b"MVR", b"NQL", b"FTF"),
    (b"JDD", b"GRB", b"MDB"),
    (b"TKG", b"LHT", b"VLM"),
    (b"NHL", b"MMV", b"LNM"),
    (b"XBR", b"DHX", b"QHM"),
    (b"DNK", b"TXQ", b"DSP"),
    (b"GBX", b"SDS", b"BGR"),
    (b"PDT", b"CBF", b"RKM"),
    (b"PGH", b"FGJ", b"KMD"),
    (b"GTR", b"RPT", b"CXM"),
    (b"GBP", b"LNB", b"DNR"),
    (b"DKC", b"MQS", b"NND"),
    (b"HSM", b"SHK", b"HQP"),
    (b"VVV", b"GLQ", b"TKP"),
    (b"TTJ", b"DNK", b"BQC"),
    (b"CJD", b"CGS", b"KGT"),
    (b"VSG", b"RMJ", b"BQN"),
    (b"XGQ", b"TXC", b"LKB"),
    (b"CNP", b"PCH", b"PCH"),
    (b"JPT", b"NDS", b"CTV"),
    (b"MVT", b"GBP", b"RLR"),
    (b"HJF", b"CGL", b"JST"),
    (b"KGB", b"GML", b"NLB"),
    (b"MPN", b"DXQ", b"BHJ"),
    (b"KFT", b"DTS", b"DJP"),
    (b"QCK", b"RLF", b"QVR"),
    (b"MJR", b"MJD", b"RPS"),
    (b"LBV", b"FPH", b"RFH"),
    (b"JMG", b"QSK", b"QXN"),
    (b"LDJ", b"DBN", b"TTK"),
    (b"XBP", b"KQV", b"QGK"),
    (b"RTM", b"DFS", b"DXT"),
    (b"VBH", b"BMB", b"JGJ"),
    (b"LHP", b"MMH", b"BPV"),
    (b"PMR", b"MDQ", b"DGS"),
    (b"NNR", b"JDF", b"NVP"),
    (b"GJL", b"JDT", b"QLP"),
    (b"JST", b"SBL", b"LNL"),
    (b"KML", b"TXL", b"CRB"),
    (b"FBA", b"KQG", b"JSC"),
    (b"KSF", b"TLH", b"JBK"),
    (b"VFF", b"CKT", b"GSP"),
    (b"PSG", b"PQP", b"CBJ"),
    (b"LDZ", b"JSC", b"KQG"),
    (b"DKN", b"MVC", b"SPX"),
    (b"DRV", b"DHD", b"GBJ"),
    (b"MJB", b"HNM", b"LKH"),
    (b"MNG", b"KXS", b"PRM"),
    (b"SNN", b"VNQ", b"DQQ"),
    (b"GNH", b"KHS", b"SLV"),
    (b"BJF", b"HPD", b"XBL"),
    (b"LGP", b"LHP", b"CKD"),
    (b"XVZ", b"CVH", b"MRJ"),
    (b"GRP", b"NHV", b"GBX"),
    (b"NGC", b"FRJ", b"KVG"),
    (b"BJD", b"QVR", b"RLF"),
    (b"FKL", b"LRV", b"DSH"),
    (b"KSV", b"VHK", b"VHK"),
    (b"XQK", b"KPL", b"MDX"),
    (b"NRC", b"DMT", b"GLC"),
    (b"VNH", b"LQT", b"VVQ"),
    (b"CGL", b"SBL", b"SBL"),
    (b"GVH", b"NKD", b"XTK"),
    (b"DBJ", b"LXQ", b"HKP"),
    (b"KVC", b"PMX", b"XPR"),
    (b"NJP", b"VXG", b"TTJ"),
    (b"VXG", b"BQC", b"DNK"),
    (b"CLJ", b"JVL", b"QRP"),
    (b"RFH", b"VQL", b"NPJ"),
    (b"GFR", b"KGB", b"GNM"),
    (b"KQV", b"VFK", b"PKK"),
    (b"SQM", b"GTJ", b"RKT"),
    (b"MQS", b"MPN", b"HJC"),
    (b"BXV", b"RJP", b"CLN"),
    (b"VSJ", b"DKN", b"XDK"),
    (b"SHK", b"XMV", b"RXM"),
    (b"GSP", b"LDC", b"VPF"),
    (b"MSS", b"VPS", b"HKF"),
    (b"SFM", b"NVR", b"CQX"),
    (b"FMC", b"BJF", b"SRS"),
    (b"DXS", b"RMD", b"TLT"),
    (b"QHS", b"CGM", b"GXT"),
    (b"DJP", b"FDL", b"JHP"),
    (b"MPT", b"PQF", b"DCB"),
    (b"LXQ", b"PLS", b"PGK"),
    (b"RFD", b"TTJ", b"VXG"),
    (b"JGJ", b"HJS", b"TJT"),
    (b"XGN", b"KVT", b"LQL"),
    (b"NBB", b"LSX", b"XNR"),
    (b"CKD", b"MMH", b"BPV"),
    (b"CTT", b"CQX", b"NVR"),
    (b"CBJ", b"RRG", b"KBD"),
    (b"CNG", b"THV", b"QBK"),
    (b"BMD", b"QPF", b"VHX"),
    (b"XLC", b"BKL", b"SCZ"),
    (b"KQG", b"CPT", b"VLL"),
    (b"KTJ", b"KVC", b"VBM"),
    (b"JBG", b"NJP", b"RFD"),
    (b"KVT", b"NPT", b"GVS"),
    (b"QGP", b"JVL", b"QRP"),
    (b"HFC", b"THV", b"QBK"),
    (b"JDT", b"HBS", b"RHL"),
    (b"MNF", b"NRS", b"XVZ"),
    (b"GKR", b"GTK", b"PCB"),
    (b"NVP", b"NQQ", b"RRQ"),
    (b"MPQ", b"GCS", b"KSF"),
    (b"PLS", b"PRG", b"XQK"),
    (b"JDF", b"NQQ", b"RRQ"),
    (b"CQD", b"DBJ", b"VTV"),
    (b"LNP", b"TPK", b"SCP"),
    (b"HQP", b"XMV", b"RXM"),
    (b"RJM", b"MJD", b"RPS"),
    (b"MQK", b"FSL", b"CKH"),
    (b"KXS", b"HBJ", b"HMP"),
    (b"CSX", b"QCS", b"RKV"),
    (b"VCG", b"LQL", b"KVT"),
    (b"NVJ", b"JQG", b"GKD"),
    (b"NLB", b"VLD", b"VFF"),
    (b"CQN", b"NVP", b"JDF"),
    (b"PDL", b"VTM", b"GBQ"),
    (b"SJM", b"JNR", b"BGH"),
    (b"VTR", b"HMK", b"RNS"),
    (b"QGS", b"SQK", b"TPL"),
    (b"NFH", b"XRH", b"DKD"),
    (b"TXQ", b"GFL", b"PGH"),
    (b"GCJ", b"XSL", b"BFB"),
    (b"RKT", b"CQN", b"NNR"),
    (b"BFQ", b"LBS", b"JLX"),
    (b"PKN", b"FGD", b"TKG"),
    (b"XKP", b"FDJ", b"BHH"),
    (b"QHX", b"SPD", b"XCT"),
    (b"NKR", b"GFR", b"KTV"),
    (b"GLQ", b"FBM", b"KLD"),
    (b"PRR", b"XSL", b"BFB"),
    (b"XLT", b"HPB", b"MJJ"),
    (b"HRD", b"QVP", b"VXH"),
    (b"SNL", b"VSF", b"GTG"),
    (b"TBN", b"XCS", b"LJX"),
    (b"RPS", b"NFM", b"VNX"),
    (b"MKX", b"XLG", b"SLT"),
    (b"NQC", b"MPT", b"BLX"),
    (b"DKK", b"FVQ", b"PFR"),
    (b"LCT", b"TGK", b"PSH"),
    (b"XHV", b"JSN", b"KFT"),
    (b"PRM", b"HMP", b"HBJ"),
    (b"NQL", b"BQD", b"TTN"),
    (b"HQV", b"KVH", b"LBV"),
    (b"RKG", b"HMK", b"RNS"),
    (b"LCL", b"KQH", b"MKX"),
    (b"LMM", b"PVJ", b"SXK"),
    (b"NCF", b"XHV", b"FBS"),
    (b"NRS", b"MRJ", b"CVH"),
    (b"RTK", b"RJM", b"MJR"),
    (b"RFC", b"NVJ", b"HNG"),
    (b"XXQ", b"SXP", b"BJH"),
    (b"RPT", b"NMV", b"VNV"),
    (b"QBR", b"CTH", b"QKV"),
    (b"FXF", b"VBM", b"KVC"),
    (b"CRB", b"NQC", b"RXH"),
    (b"NQD", b"BQN", b"RMJ"),
    (b"RKR", b"KMQ", b"MGF"),
    (b"DSP", b"GFL", b"PGH"),
    (b"DSH", b"XTV", b"LVS"),
    (b"DMT", b"QVN", b"PMS"),
    (b"VPS", b"NNC", b"HKJ"),
    (b"JSC", b"CPT", b"VLL"),
    (b"XKS", b"DVT", b"DRH"),
    (b"TTV", b"LKH", b"HNM"),
    (b"MJD", b"VNX", b"NFM"),
    (b"SCZ", b"SHQ", b"DLB"),
    (b"XVX", b"CNG", b"HFC"),
    (b"KTV", b"KGB", b"GNM"),
    (b"QHJ", b"LSR", b"SSN"),
    (b"CQJ", b"GPT", b"BMD"),
    (b"NMD", b"TPK", b"SCP"),
    (b"DVB", b"GSJ", b"GSJ"),
    (b"JMP", b"MQP", b"RGQ"),
    (b"BSB", b"BGC", b"KJC"),
    (b"CPT", b"MRB", b"MHN"),
    (b"PQP", b"RRG", b"KBD"),
    (b"RLG", b"BJH", b"SXP"),
    (b"VQX", b"XVR", b"JCB"),
    (b"XJN", b"XRH", b"DKD"),
    (b"JHP", b"QFT", b"MNG"),
    (b"KMB", b"HRT", b"SJM"),
    (b"QSK", b"PSG", b"CFJ"),
    (b"DKD", b"VLF", b"JPT"),
    (b"NHV", b"BGR", b"SDS"),
    (b"HXV", b"XGQ", b"BSP"),
    (b"XXN", b"DMT", b"GLC"),
    (b"VLM", b"KMB", b"BTL"),
    (b"RLR", b"DNR", b"LNB"),
    (b"PCH", b"KQG", b"JSC"),
    (b"XRH", b"VLF", b"JPT"),
    (b"SBL", b"XLM", b"XLM"),
    (b"CRM", b"JTJ", b"XKM"),
    (b"JSS", b"BTS", b"LSV"),
    (b"CXM", b"NMV", b"VNV"),
    (b"MLG", b"HCT", b"HCT"),
    (b"JXH", b"MVR", b"SDR"),
    (b"XBH", b"XVF", b"STR"),
    (b"DXT", b"VBN", b"JVK"),
    (b"JSN", b"DTS", b"DJP"),
    (b"KLD", b"MFK", b"PBD"),
    (b"KVH", b"RFH", b"FPH"),
    (b"NJX", b"GMV", b"VPQ"),
    (b"NRQ", b"GKR", b"GMT"),
    (b"HXF", b"DXS", b"QXR"),
    (b"DGS", b"SNL", b"BVF"),
    (b"LJT", b"RKR", b"QVK"),
    (b"JLX", b"XGP", b"TKS"),
    (b"FCX", b"PXP", b"GVH"),
    (b"VDX", b"VBJ", b"TBR"),
    (b"VSF", b"SPV", b"DDQ"),
    (b"VCX", b"KTT", b"PPX"),
    (b"MMH", b"MDM", b"FVS"),
    (b"MPB", b"GBL", b"RRB"),
    (b"RLF", b"TSM", b"DLN"),
    (b"RMJ", b"BTQ", b"TBN"),
    (b"VJX", b"QQP", b"HMV"),
    (b"XVF", b"RFC", b"HXG"),
    (b"CTN", b"VVV", b"TKM"),
    (b"QHD", b"GBP", b"RLR"),
    (b"JGC", b"JXH", b"QCQ"),
    (b"KPL", b"FXF", b"KTJ"),
    (b"NGN", b"MDH", b"VSJ"),
    (b"XPG", b"MJB", b"TTV"),
    (b"MQP", b"KTG", b"CQJ"),
    (b"JCB", b"QHD", b"MVT"),
    (b"MNM", b"RPT", b"CXM"),
    (b"RXH", b"BLX", b"MPT"),
    (b"HHP", b"GCJ", b"PRR"),
    (b"SLV", b"JVD", b"TRF"),
    (b"HVT", b"BJX", b"FND"),
    (b"MTD", b"MMV", b"LNM"),
    (b"TTL", b"QFH", b"LGP"),
    (b"LTV", b"QCS", b"RKV"),
    (b"VXV", b"TSX", b"HSM"),
    (b"TCM", b"QHM", b"DHX"),
    (b"NBF", b"LQT", b"VVQ"),
    (b"BTQ", b"LJX", b"XCS"),
    (b"TKP", b"KLD", b"FBM"),
    (b"LDC", b"HSD", b"PJP"),
    (b"VHK", b"NRS", b"NRS"),
    (b"NFM", b"TMF", b"QFS"),
    (b"GTJ", b"NNR", b"CQN"),
    (b"LSD", b"QCK", b"BJD"),
    (b"HJV", b"QBD", b"DGT"),
    (b"QBK", b"MND", b"GJL"),
    (b"XNR", b"MBF", b"SPN"),
    (b"HQB", b"KXN", b"FJF"),
    (b"TNJ", b"FDJ", b"BHH"),
    (b"TGK", b"SQV", b"TLB"),
    (b"KMF", b"VHK", b"MNF"),
    (b"JVK", b"PDL", b"FFF"),
    (b"NSF", b"VBJ", b"TBR"),
    (b"SCP", b"QQR", b"PVV"),
    (b"XXP", b"HHP", b"CDS"),
    (b"VHX", b"NJX", b"KJF"),
    (b"VCJ", b"JXH", b"QCQ"),
    (b"KJC", b"KCS", b"SCR"),
    (b"XXT", b"FJF", b"KXN"),
    (b"NNC", b"VQD", b"NSR"),
    (b"VBM", b"XPR", b"PMX"),
    (b"JTJ", b"XDG", b"VHT"),
    (b"CNK", b"CCX", b"MSS"),
    (b"FTF", b"TTN", b"BQD"),
    (b"LKH", b"SXJ", b"GMF"),
    (b"TLH", b"CSR", b"VXV"),
    (b"GFF", b"KML", b"QRX"),
    (b"LFR", b"CLN", b"RJP"),
    (b"DTS", b"FDL", b"JHP"),
    (b"TTN", b"KTP", b"GFF"),
    (b"BTL", b"HRT", b"SJM"),
    (b"HJS", b"JCT", b"JMJ"),
    (b"HBJ", b"JVR", b"GSF"),
    (b"SLF", b"JDD", b"KJZ"),
    (b"CVH", b"CNC", b"XLT"),
    (b"JGQ", b"JGB", b"XKS"),
    (b"FGJ", b"QFR", b"XVX"),
    (b"STQ", b"XRQ", b"TCB"),
    (b"GCS", b"JBK", b"TLH"),
    (b"FKK", b"HVT", b"MPK"),
    (b"LJM", b"DQM", b"XXG"),
    (b"BTS", b"NMD", b"LNP"),
    (b"SVC", b"NFP", b"QHX"),
    (b"VPF", b"PJP", b"HSD"),
    (b"NPT", b"NGC", b"FCK"),
    (b"TTK", b"GBN", b"NRQ"),
    (b"XCT", b"FNL", b"RSH"),
    (b"RRG", b"TND", b"HCC"),
    (b"HMP", b"GSF", b"JVR"),
    (b"TJV", b"XXG", b"DQM"),
    (b"CTH", b"CTT", b"SFM"),
    (b"TPK", b"QQR", b"PVV"),
    (b"LSR", b"PQH", b"LFT"),
    (b"XLM", b"BKL", b"BKL"),
    (b"FGD", b"VLM", b"LHT"),
    (b"GSF", b"VTJ", b"HRD"),
    (b"PBN", b"FGD", b"TKG"),
    (b"NMV", b"NRC", b"XXN"),
    (b"CLN", b"TPQ", b"CTX"),
    (b"TMF", b"TSK", b"PTV"),
    (b"HPX", b"FKC", b"PDT"),
    (b"GBN", b"GMT", b"GKR"),
    (b"PCK", b"PDX", b"HXV"),
    (b"TCB", b"NCF", b"FJK"),
    (b"LJX", b"NRH", b"PTN"),
    (b"QBD", b"KVQ", b"GNH"),
    (b"CMC", b"QBR", b"PBK"),
    (b"GBL", b"HHR", b"GSC"),
    (b"SQN", b"BJF", b"SRS"),
    (b"FCB", b"QXR", b"DXS"),
    (b"GDJ", b"XKS", b"JGB"),
    (b"PCB", b"LCL", b"XHH"),
    (b"BPV", b"MDM", b"FVS"),
    (b"PJD", b"DVB", b"DVB"),
    (b"RHG", b"MJR", b"RJM"),
    (b"DVT", b"PKN", b"PBN"),
    (b"TFM", b"QBR", b"PBK"),
    (b"HCC", b"HQB", b"XXT"),
    (b"PGS", b"DSX", b"MTV"),
    (b"TSR", b"MQP", b"RGQ"),
    (b"KJF", b"GMV", b"VPQ"),
    (b"GJS", b"DKC", b"CKL"),
    (b"LNB", b"QLK", b"DRT"),
    (b"PKK", b"QPS", b"VJX"),
    (b"TKS", b"RHG", b"RTK"),
    (b"QFS", b"TSK", b"PTV"),
    (b"DGL", b"RCK", b"XVJ"),
    (b"XPR", b"MNM", b"GTR"),
    (b"TDH", b"FKL", b"CFC"),
    (b"PDF", b"XBP", b"CXB"),
    (b"DFN", b"RXF", b"GRP"),
    (b"JVR", b"HRD", b"VTJ"),
    (b"LBS", b"TKS", b"XGP"),
    (b"QCQ", b"MVR", b"SDR"),
    (b"MND", b"JDT", b"QLP"),
    (b"PFR", b"LJM", b"TJV"),
    (b"QLK", b"MPB", b"PSV"),
    (b"CFC", b"LRV", b"DSH"),
    (b"PMX", b"GTR", b"MNM"),
    (b"CGF", b"PMR", b"RPV"),
    (b"MVG", b"LTH", b"KFN"),
    (b"NLK", b"FBR", b"JPM"),
    (b"LFB", b"SCS", b"PGS"),
    (b"BVF", b"VSF", b"GTG"),
    (b"MDQ", b"BVF", b"SNL"),
    (b"CQF", b"PMR", b"RPV"),
    (b"FND", b"LRX", b"PNM"),
    (b"MRJ", b"CNC", b"XLT"),
    (b"HSD", b"XBR", b"TCM"),
    (b"HLF", b"CNP", b"GQH"),
    (b"RRQ", b"CMC", b"TFM"),
    (b"CLM", b"TKM", b"VVV"),
    (b"RLB", b"TTV", b"MJB"),
    (b"NDS", b"RQX", b"CQD"),
];

pub const TEST_DATA_3_NAV: &[u8] = b"LR";

pub const TEST_DATA_3_NODES: [(&[u8; 3], &[u8; 3], &[u8; 3]); 8] = [
    (b"11A", b"11B", b"XXX"),
    (b"11B", b"XXX", b"11Z"),
    (b"11Z", b"11B", b"XXX"),
    (b"22A", b"22B", b"XXX"),
    (b"22B", b"22C", b"22C"),
    (b"22C", b"22Z", b"22Z"),
    (b"22Z", b"22B", b"22B"),
    (b"XXX", b"XXX", b"XXX"),
];