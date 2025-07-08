use std::{collections::HashMap, fmt::{format, Display}, sync::{Arc, Mutex}, thread};
mod structs;
use structs::SiteUser;
fn main(){
    let mut isim:String = String::from("burak");
    println!("isim {}",isim);
    
    let mut randomveriler: Vec<Box<dyn Display>> =Vec::new();

    randomveriler.push(Box::new("kaya"));
    randomveriler.push(Box::new(1));
    randomveriler.push(Box::new(true));

    for eleman in &randomveriler{
        println!("random veri {}",eleman);
    }


    struct kişi{
        ad:String,
        yas:u16
    }

    let kullanıcı =kişi{
        ad:String::from("efe"),
        yas:17
    };

    println!("kullanıcı {} {}",kullanıcı.ad,kullanıcı.yas);

    let buyuk_harfli_isim = merhaba(&isim);
    println!("büyük harfli++ {} ",buyuk_harfli_isim);

    let mut çift:HashMap<i32,String> = HashMap::new();
    çift.insert(1,String::from("burak"));

    for (anahtar,değer) in &çift{
        println!("Anahtar: {}, Değer: {}", anahtar, değer);
    }

    let handletThread = thread::spawn(|| {
        println!("thread now working ")
    });

    handletThread.join().unwrap();

    let my_age = Arc::new(Mutex::new(21)); // Paylaşılan ve değiştirilebilir bir değişken

    let my_age_clone = Arc::clone(&my_age); // Arc'ı çoğalt

    let birth_year = thread::spawn(move || {
        let mut age = my_age_clone.lock().unwrap(); // Mutex'i kilitle
        *age = 2025 - *age; // Yaşı güncelle
    });

    birth_year.join().unwrap(); // İş parçacığının bitmesini bekle

    // Güncellenmiş yaşı yazdır
    let final_age = *my_age.lock().unwrap();
    println!("My birth year: {}", final_age);

    let final_result;
    {
        final_result = hos_geldin(&isim);
        println!("final_result {} ",final_result)
    }
    

    let title_name = String::from("rust book");
    let page_number:u16 = 200;
    
    let rust_book = Book{
        title:&title_name,
        page:&page_number
    };

    println!("book title {} page {} ",rust_book.title,rust_book.page);

    let mut user = SiteUser{
        UserName:String::from("emre"),
        UserAge:20
    };

    println!("Site User name {} Site User Age {} ",user.get_user_name(),user.get_user_age());
    user.set_user_name(String::from("ela"));
    println!("Site User name {} Site User Age {} ",user.get_user_name(),user.get_user_age());

    

}


fn merhaba<'a>(x: &'a String) -> String {
    x.to_uppercase()
}


fn hos_geldin<'a>(x: &'a str) -> String {
    let y = format!("merhaba {}", x); 
    y 
}
struct Book<'a>{
        title: &'a str,
        page: &'a u16,

    }