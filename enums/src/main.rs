enum sıra {
    bir,
    iki,
    üç,
}

fn sıra_sayaç(sıra:sıra){
    match sıra{
        sıra::bir => println!("ilk sıra"),
        sıra::iki => println!("ikinci sıra"),
        sıra::üç =>println!("üçüncü sıra")
    }
}

fn main() {
    let üçüncü = sıra::üç;
    sıra_sayaç(üçüncü);
}
