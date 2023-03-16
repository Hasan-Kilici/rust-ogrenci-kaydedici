use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut input_line = String::new();
    println!("Kaydedilecek olan öğrencinin adını giriniz");
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let isim = input_line.trim();
    let mut input_line2 = String::new();
    println!("Kaydedilecek olan öğrencinin soyadını giriniz");
    std::io::stdin()
        .read_line(&mut input_line2)
        .expect("Failed to read line");
    let soyisim = input_line2.trim();
    let mut input_line3 = String::new();
    println!("Kaydedilecek olan öğrencinin sınıfını giriniz");
    std::io::stdin()
        .read_line(&mut input_line3)
        .expect("Failed to read line");
    let sinif = input_line3.trim();
    let mut input_line4 = String::new();
    println!("Kaydedilecek olan öğrencinin numarasını giriniz");
    std::io::stdin()
        .read_line(&mut input_line4)
        .expect("Failed to read line");
    let no = input_line4.trim();
    let mut input_line5 = String::new();
    println!("Kaydedilecek olan öğrencinin yaşını giriniz");
    std::io::stdin()
        .read_line(&mut input_line5)
        .expect("Failed to read line");
    let yas = input_line5.trim();
    println!("Öğrencinin adı : {} \n Öğrencinin soyadı : {} \n Öğrencinin Sınıfı : {} \n Öğrencinin Okul Numarası : {} \n Öğrencinin Yaşı : {} \n Öğrencinin Bilgileri Şu dosyaya kaydedildi {}-{}-{}",isim,soyisim,sinif,no,yas,isim,soyisim,no);
    kaydet(isim, soyisim, sinif, no, yas);
}

fn kaydet(isim: &str, soyisim: &str, sinif: &str, no: &str, yas: &str) {
    let filename = format!("{}-{}-{}.txt", isim, soyisim, no);
    let path = Path::new(&filename);
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    match file.write_all(format!("{} {} {} {} {}", isim, soyisim, sinif, no, yas).as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", path.display(), why),
        Ok(_) => println!("{} dosyasına öğrenci bilgileri kaydedildi.", filename),
    };
    main();
}
