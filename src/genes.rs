use global;

struct Genome {
    color: global::Color,
    size: u8,
    speed: u8,
}

struct Genes {
    haploid_m: [Genome; 100],
    haploid_p: [Genome; 100],
}
