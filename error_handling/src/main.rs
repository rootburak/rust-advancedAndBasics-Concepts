#[derive(Debug)]
enum SayısalHata {
    GeçersizGiriş,
    SıfıraBölme,
}
#[derive(Debug)]
enum SözelHata {
    GeçersizGiriş,
}


fn bos_string_kotnrol(x:String)->Result<String,SözelHata>{
    if x.is_empty() {
        Err(SözelHata::GeçersizGiriş)        
    }
    else {
        Ok(x)
    }
}

fn karmaşık_hesap(x: f64) -> Result<f64, SayısalHata> {
    if x < 0.0 { return Err(SayısalHata::GeçersizGiriş); }
    
    let ara_sonuç = pozitif_kök(x)
        .ok_or(SayısalHata::GeçersizGiriş)?;
    
    if ara_sonuç == 0.0 { Err(SayısalHata::SıfıraBölme) }
    else { Ok(1.0 / ara_sonuç) }
}

fn pozitif_kök(x: f64) -> Option<f64> {
    if x >= 0.0 { Some(x.sqrt()) } else { None }
}

fn return_name(name: String) -> Option<String> {
    if name.is_empty() { None } else { Some(name) }
}

fn bölme(a: f32, b: f32) -> Result<f32, &'static str> {
    if b == 0.0 { Err("Sıfıra bölme hatası") } 
    else { Ok(a / b) }
}

fn main() {

    match karmaşık_hesap(16.0) {
        Ok(r) => println!("Sonuç: {}", r),
        Err(e) => match e {
            SayısalHata::GeçersizGiriş => println!("Negatif sayı"),
            SayısalHata::SıfıraBölme => println!("Sıfır bölen"),
        },
    };

    match pozitif_kök(20.0) {
        Some(kök) => println!("Pozitif kök: {}", kök),
        None => println!("Negatif bir sayı verildi."),
    }

    let sonuc = bölme(80.0, 40.0);
    match sonuc {
        Ok(değer) => println!("Sonuç: {}", değer),
        Err(hata) => println!("Hata: {}", hata),
    }

    let name = return_name(String::from("burak"));
    match name {
        Some(isim) => println!("İsim: {}", isim),
        None => println!("İsim boş."),
    }
    let ad = bos_string_kotnrol(String::from("burak"));
    match ad {
        Ok(kullanıcı_adı) => println!("kullanıcı adı {}",kullanıcı_adı),
        Err(_hata) => println!("kullanıcı adı yok")
    }

}
