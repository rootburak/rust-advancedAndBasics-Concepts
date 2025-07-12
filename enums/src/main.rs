enum sıra {
    Bir,
    Iki,
    Uç,
}

fn sıra_sayaç(sıra:sıra){
    match sıra{
        sıra::Bir => println!("ilk sıra"),
        sıra::Iki => println!("ikinci sıra"),
        sıra::Uç =>println!("üçüncü sıra")
    }
}

enum Mat{
    Topla(f32,f32),
    Carp(f32,f32)
}

impl Mat{
    fn sum_values(&self) ->f32{
        match self {
            Mat::Topla(x, y)=> x+y,
            Mat::Carp(x, y) => x*y
        }
    }
}

fn main() {
    let üçüncü = sıra::Uç;
    sıra_sayaç(üçüncü);

    let toplama = Mat::Topla(20.2,20.2);

    println!("toplam {}",toplama.sum_values());

    let çarpma = Mat::Carp(20.2,20.2);
    
    println!("çarpma {}",çarpma.sum_values());


}
