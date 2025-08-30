pub mod first;
pub mod second;
pub mod third;
pub mod model;

#[allow(unused_imports)]
use first::say_hello;

#[allow(unused_imports)]
use second::say_hello as say_hello_second;


#[test]
fn test_use() {
    say_hello();
    say_hello_second();
   
}

































// use model::User;

fn main() {
    println!("Hello, world!");
}


#[test]
fn test_variabel() {
    let name = "Rido Sihombing" ;
    println!("Hello {}", name);

}

#[test]
fn test_mutable() {
    let mut   name = "Rido Sihombing" ;
    println!("Hello {}", name);

    name = "Lionel Messi" ;
    println!("Hello {}", name);
}

#[test]
fn test_shadowing() {
    let name = "Rido Sihombing" ;
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn explicit() {
    let age = 20;
    println!("{}", age);
}

#[test]
fn test_number() {
    let a = 100;
    println!("{}",a);

    let a = 10.5 ;
    println!("{}", a);
}

#[test]
fn test_number_convertion() {
    let a = 10;
    println!("{}", a);
    
    let b: i16 = a as i16; // a konversi ke i16
    println!("{}",b);

    let c = a as i32; // a konversi ke i32
    println!("{}", c); 
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 11;
    let c = a * b + b - a / b;
    println!("{}", c)
}

#[test]
fn boolean() {
    let a = true;
    let b = false;

    println!("{} {}", a, b)
}

#[test]
fn comprarison() {
    let a = 10;
    let b = 20;

    let result = a > b ;
    println!("{}", result);
}

#[test]
fn boolean_operator() {
    let absen = 80;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 80;
    let lulus_nilai_akhir = nilai_akhir >= 80;

    let lulus_final = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus_final);
}

#[test]
fn char_type() {
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}


#[test]
fn tuple() {
    let mut data = (10, 10.5, 'n');
    println!("{:?}", data);

    // mengakses data tuple
    /*
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);
     */

    // mengakses seluruh data tuple
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c );


    // mengubah data tuple (mut)
    data.0 = 30;
    data.1 = 40.5;
    data.2 = 'b';
    println!("{:?}", data);
}



#[test]
fn arry() {
    let mut array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    // mengakses array
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    // mengubah data array (mut)
    array[0] = 10;
    array[1] = 189;
    println!("{:?}", array);

    // mengakses panjang atau jumlah data array
    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensial_array() {
    let matrix  = [
        [1, 2],
        [3, 4]
    ];

    println!("{:?}", matrix);

    // mengakses data
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
    
}

#[allow(dead_code)]
const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM,MAXIMUM);
}

#[test]
fn variabel_scope() {
    let rido = 10; // variabel scope

    { // inner scope
        println!("inner rido: {}", rido);
        let sihombing = 2;
        println!("inner sihombing: {}", sihombing);
    }
}

#[test]
fn string_heap() {
    function_a();
    function_b();
}

#[allow(dead_code)]
fn function_a(){
    let a = 10;
    let b = String::from("Rido");
    println!("{} {}", a, b);
}

#[allow(dead_code)]
fn function_b(){
    let a = 10;
    let b = String::from("Sihombing");
    println!("{} {}", a, b);
}

#[test]
fn string() {
    let name = "  Rido Sihombing  " ;
    let trim = name.trim();

    println!("{}", name);
    println!("{}", trim);

}

#[test]
fn string_type() {
    let mut name = String::from("Rido");
    println!("{}", name);

    // menambahkan data baru
    name.push_str(" Sihombing");
    println!("{}", name);
}

#[test]
fn ownership_rules() {
    let a = 10;
    {
        let b = 18;
        println!("{}", b);
        println!("{}", a)
    }

    println!("{}", a)
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a; // copy data dari A ke B

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Rido");
    println!("{}", name1);

    let name2 = name1;
    println!("{}",name2);
  //  println!("{}", name1); // ini tidak bisa karna ownership sudah berpindah pada name2

}

#[test]
fn clone() {
    let name1 = String::from("Rido");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let nilai = 9;
    let result = if nilai >= 8 {
        "Good"
    } else if nilai >= 6 {
        "Not bad"
    } else {
        "Very bad"
    } ;

    println!("{}", result);
}

#[test]
fn loop_expresion() {
    let mut counter = 0;

    loop{
        counter += 1; // tiap loop naikkan 1
        println!("Perulangan ke-{}", counter);

        if counter == 5{
            break;
        } else if counter %2 ==0 {
            continue;
        }
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }

    };

    println!("{}", result)
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop{
        let mut i = 1;
        loop {
            if number > 100 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }

          
        }

          number += 1;
    }
}


 #[test]
 fn while_loop() {
     let mut counter = 1;
     
     while counter <= 10 {

        println!("Counter: {}", counter);
        counter += 1;
     }
 }

 #[test]
 fn array_iteration() {
     let array = ["A", "B", "C", "D", "E"];
    

    for value in array{
        println!("Value {}", value);
    }
 }

 #[test]
 fn range() {
     let range = 0..5;
     println!("Star :{}", range.start);
     println!("End :{}", range.end);

      let array = ["A", "B", "C", "D", "E"];

     for i in range{
        println!("{}", array[i]);
     }
 }

#[allow(dead_code)]
fn say_hello1(name: &str) {
    println!("Hello, {}!", name);
}

#[test]
fn main2() {
    say_hello1("Rido");
    say_hello1("Gracia");
}

#[allow(dead_code)]
fn perkenalan(name: &str, umur: &str) {
    println!("Halo nama saya {} dan saya berumur {}", name, umur);
}

#[test]
fn main3() {
    perkenalan("Rido", "21 tahun");
}

#[allow(dead_code)]
fn say_hello_test(){
    println!("helo");

}

#[test]
fn test_function() {
    say_hello_test();
    say_hello_test();
}

#[allow(dead_code)]
fn say_goodbye(first_name: &str, last_name: &str){
    println!("good Bye {} {}", first_name, last_name)
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Rido", "Sihombing");
    say_goodbye("Lionel", "Messi");
}



#[allow(dead_code)]
fn kali_dua(a: i32) -> i32{
    a * 2
}


#[test]
fn test_kali_dua() {
    let hasil = kali_dua(5);
    println!("Hasil Perkalian {}", hasil);
}

#[allow(dead_code)]
fn luas_persegi_panjang(panjang: i32, lebar: i32) -> i32{
    panjang * lebar
}


#[test]
fn test_luas_luas_persegi_panjang() {
    let hasil = luas_persegi_panjang(4, 6);
    println!("Hasil Luas Persegi panjang {}", hasil);
}


#[allow(dead_code)]
fn celcius_ke_fahranheit(c : f64) -> f64{
    
 (c * 9.0 / 5.0) + 32.0
}

#[test]
fn test_celcius_ke_fahranheit() {
    let hasil1 = celcius_ke_fahranheit(0.0);
    println!("0°C = {}°F", hasil1)
}

#[allow(dead_code)]
fn print_number(number: i32){
    println!("Number {}", number)

}

#[allow(dead_code)]
fn hi(name: String){
    println!("Name {}", name)
}


#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);
    
    let name = String::from("Rido");
    hi(name);
    
}




#[allow(dead_code)]
fn ubah_nama(nama: &mut String) {
    nama.push_str(" Sihombing");
}

#[test]
fn main_ref_mut() {
    let mut s = String::from("Rido");
    ubah_nama(&mut s); // pinjam secara mutable
    println!("Nama setelah diubah: {}", s);
}



#[allow(dead_code)]
fn tambah_kata(kata: &mut String){
    kata.push_str(" Rust");
}

#[test]
fn test_tambah_kata() {
    let mut a = String::from("Belajar");
    tambah_kata(&mut a);
    println!("{}", a) ;
}

#[test]
fn slice() {
    let a = String::from("Rido Sihombing");

    let b = &a[1..5];
    let c = &a[5..9];
    let d = &a[..];
    let e = &a[1..=4];

    println!("{} {} {} {}", b, c, d, e);
}

#[test]
fn demo_debug_display() {
    let angka = 42;
    let teks = String::from("Halo");
    let arr = [1, 2, 3];

    // Angka
    println!("Display angka: {}", angka); // oke
    println!("Debug angka: {:?}", angka); // oke

    // String
    println!("Display string: {}", teks); // oke
    println!("Debug string: {:?}", teks); // oke

    // Array
    // println!("Display array: {}", arr); // ❌ error
    println!("Debug array: {:?}", arr);    // ✅ oke
}

#[test]
fn slice_referance() {
    let array = [1,2,3,4,5];

    let slice1 = &array[..];
    println!("{:?}", slice1);
}

#[test]
fn ambil_kata() {
    let a = String::from("Halo Rust");

    let b = &a[0..4];
    println!("{}", b)
}

#[test]
fn ambil_tengah() {
    let array = [10, 20, 30, 40, 50];

    let slice = &array[1..4];
    println!("{:?}", slice);
}

#[test]
fn print_data() {
    let array = [1, 2, 3, 4];
    let slice = &array[..];
    println!("{:?}", slice)
}

#[test]
fn string_slice() {
    let full_name= String::from("Rido SIhombing");

    let call_name = &full_name[..4];
    println!("{}", call_name);
}


#[allow(dead_code)]
struct Biodata{
    nama: String,
    umur: u32,
    jurusan: String,
}

#[test]
fn test_biodata() {
    let bidata = Biodata{
        nama: String::from("RidoSihombing"),
        umur: 21,
         jurusan: String::from("Informatika"),

    };

    println!("Nama: {}", bidata.nama);
    println!("umur: {}", bidata.umur);
    println!("jurusan: {}", bidata.jurusan);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Mahasiswa2 {
    nama: String,
    umur: u32,
}

#[test]
fn demo_struct_debug() {
    let mhs = Mahasiswa2 {
        nama: String::from("Ido"),
        umur: 22,
    };

    println!("Debug: {:?}", mhs);
    println!("Pretty Debug: {:#?}", mhs);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Mahasiswa {
    nama: String,
    jurusan: String,
    umur: u32,
}

#[test]
fn test_mahasiswa() {
    let mhs = Mahasiswa{
        nama: String::from("ido"),
        umur: 21,
        jurusan: String::from("Rekayasa Perangkat Lunak"),
    };

    println!("{:#?}", mhs);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Mahasiswa3{
    nama: String,
    umur: u32,
}

#[allow(dead_code)]
impl Mahasiswa3 {
    fn perkenalan(&self){
        println!("Halo, nama saya {} umur {}", self.nama, self.umur);
    }
}

#[test]
fn test_method() {
    let mhs = Mahasiswa3{
        nama : String::from("ido"),
        umur: 21,
    };

    mhs.perkenalan();

}

#[allow(dead_code)]
#[derive(Debug)]
struct Perkenalan1{
    nama: String,
    umur:u32,
    hobby:String,
}

#[allow(dead_code)]
impl Perkenalan1  {
    fn perkenalan(&self) {
        println!("Perkenalkan nama saya {} umur saya {} hobby saya{}", self.nama, self.umur, self.hobby)
    }
}

#[test]
fn test_perkenalan_method() {
    let biodata = Perkenalan1{
        nama: String::from("ido"),
        umur: 21,
        hobby:String::from("Playing Pool")
    };

    biodata.perkenalan();
}



#[test]
fn test_mach() {
    let angka = 90;

    match angka {
        1 => println!("Angka satu"),
        2 | 3 => println!("Angka dua atau 3"),
       4..=10 => println!("Antara empat sampai sepuluh"),
          _ => println!("Angka lainnya"),
    }
}


#[allow(dead_code)]
#[derive(Debug)]
enum Warna {
    Merah,
    Biru,
    Hijau,
}

#[test]
fn test_enum_match() {
    let warna = Warna::Biru;

    match warna {
        Warna::Merah => println!("Warna merah"),
        Warna::Biru => println!("Warna biru"),
        Warna::Hijau => println!("Warna hijau"),
        // gak perlu _ karena semua kemungkinan udah ditangani
    }
}


#[allow(dead_code)]
#[derive(Debug)]
enum Hari {
    Sabtu,
    Minggu,
    Senin,
}


#[allow(dead_code)]
#[derive(Debug)]
enum Cuaca {
    Cerah,
    Hujan,
}

#[test]
fn test_hari_cuaca() {
    let hari = Hari::Sabtu;
    let cuaca = Cuaca::Cerah;

    match (hari, cuaca) {
        (Hari::Sabtu, Cuaca::Cerah) => println!("Main bola bareng temen ⚽"),
        (Hari::Sabtu, Cuaca::Hujan) => println!("Nonton film di rumah 🎬"),
        (Hari::Minggu, Cuaca::Cerah) => println!("Jalan-jalan sama keluarga 🚶‍♂️"),
        (Hari::Minggu, Cuaca::Hujan) => println!("Tidur siang lebih lama 😴"),
        (Hari::Senin, _) => println!("Mulai kerja/belajar 💻 (cuaca apapun)"),
    }
}


#[allow(dead_code)]
enum Lampu {
    Merah,
    Kuning,
    Hijau,
}

#[test]
fn test_lampu() {
    let lampu = Lampu::Kuning;

    match lampu {
        Lampu::Merah => println!("Berhenti"),
        Lampu::Kuning => println!("Hati hati"),
        Lampu::Hijau => println!("Jalan")
    }
}

#[test]
fn test_genap_ganjil() {
    let angka = 7;

    match angka % 2{
        0 => println!("{} adalah Genap", angka),
        1 => println!("{} adalah Ganjil", angka),
        _=> println!("Tidak mungkin"),
    }
}

#[test]
fn test_genap_ganjil1() {
    let angka = 7;

    match angka % 2 {
        0 => println!("{} adalah GENAP", angka),
        1 => println!("{} adalah GANJIL", angka),
        _ => println!("Tidak mungkin sampai sini 😅"),
    }
}

#[allow(dead_code)]
enum Payment {
    CreditCard(String),
    BankTransfer(String, String), // nomor rekening & bank
    Ewallet(String, u32),         // nama e-wallet & saldo
}

#[test]
fn test_payment() {
    let pay = Payment::Ewallet(String::from("Dana"), 50000);

    match pay {
        Payment::CreditCard(number) => {
            println!("Bayar pakai kartu kredit: {}", number);
        }
        Payment::BankTransfer(account, bank) => {
            println!("Transfer ke {} di bank {}", account, bank);
        }
        Payment::Ewallet(name, saldo) => {
            println!("Bayar pakai e-wallet {} dengan saldo {}", name, saldo);
        }
    }
}


#[allow(dead_code)]
enum Pesan {
    Teks(String),
    Gambar { nama: String, ukuran: u32 },
    Quit,
}

#[test]
fn test_pesan() {
    let pesan = Pesan::Gambar {
        nama: String::from("foto.png"),
        ukuran: 512,
    };

     match pesan {
        Pesan::Teks(teks) => {
            println!("Pesan teks: {}", teks);
        }
        Pesan::Gambar { nama, ukuran } => {
            println!("Pesan gambar {} dengan ukuran {}", nama, ukuran);
        }
        Pesan::Quit => {
            println!("Keluar aplikasi");
        }
    }
}

#[allow(dead_code)]
enum Notifikasi{
    Email { dari: String, subject: String},
    SMS (String),
    Panggilan(String)
}

#[test]
fn test_notifikas() {
    let notif = Notifikasi::Email {
         dari: String::from ("@Rido.com"), 
         subject: String::from ("Belajar Rust")
         };

    match notif {
        Notifikasi::Email { dari, subject } =>{
            println!("Email dari {} dengan subjek {}", dari, subject);
        }
        Notifikasi::Panggilan(panggilan) =>{
            println!("Ada panggilan {}", panggilan);
        }
        Notifikasi::SMS(sms) =>{
            println!("ada sms {}", sms)
        }
    }
}


#[allow(dead_code)]
enum HasilUjian {
    Nilai(u32),
    Absen(String),
    Curang { nama: String, catatan: String },
}

#[test]
fn test_hasil_ujian() {
    let hasil = HasilUjian::Curang {
        nama: String::from("Rido"),
        catatan: String::from("Buka contekan di HP"),
    };

    match hasil {
        HasilUjian::Curang { nama, catatan } => {
            println!("{} ketahuan curang {}", nama, catatan);
        }
        HasilUjian::Absen(alasan) =>{
            println!("Tidak hadir karena {}", alasan);
        }
        HasilUjian::Nilai(angka) =>{
            println!("Nilai ujian {}", angka);

        }
    }
}

#[test]
fn test_match_guard() {
    let nilai = 45;

    match nilai {
        n if n >= 90 => println!("Nilai A"),
        n if n >= 75 => println!("Nilai B"),
        n if n >= 60 => println!("Nilai C"),
        _ => println!("Nilai D"),
    }
}


#[allow(dead_code)]
type Meter = u32;

#[test]
fn test_alias() {
    let jarak: Meter = 100;
    println!("Jaraknya {} meter", jarak);
}


#[allow(dead_code)]
enum VeryLOngEnumName{
 Satu,
 Dua,
}


#[allow(dead_code)]
type A = VeryLOngEnumName;

#[test]
fn test_enum_alias() {
    let x = A::Satu;

    match x {
        A::Satu => println!("Inii Satu"),
        A::Dua => println!("Inii Satu"),

    }
}


#[allow(dead_code)]
type Age = u8;
type IdentifiNumber = String;
type Name = String;

#[allow(dead_code)]
struct Customer{
    id: IdentifiNumber,
    name: Name,
    age: Age,
}

#[test]
fn test_customer() {
    let customer = Customer{ 
    id: String::from("21104031"),
    name: String::from("Rido"),
    age: 20 ,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age)
}


