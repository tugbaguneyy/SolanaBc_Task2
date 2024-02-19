//Book varyantı, title, author ve page_count alanlarına sahip bir struct içermelidir.
struct Book{
    title:String,
    author:String,
    page_count:u32,
}

//Magazine varyantı, title, issue ve topic alanlarına sahip bir struct içermelidir.
struct Magazine{
    title:String,
    issue:u32,
    topic:String,
}

//Publication adında bir enum tanımlayın. Bu enum iki varyant içermelidir: Book ve Magazine. Her ikisi de farklı veri tipleri içermelidir:
enum Publication{
    Book(Book),
    Magazine(Magazine),
}
//Dizideki her yayını, türüne (kitap veya dergi) göre farklı bir biçimde yazdıran bir fonksiyon yazın. Örneğin, kitaplar için "Kitap: [title] yazar: [author], [page_count] sayfa" ve dergiler için "Dergi: [title] - Sayı: [issue], Konu: [topic]" formatlarını kullanın.
fn print_pub(publications:Vec<Publication>){
    for pub_item in publications {
        match pub_item {
            Publication::Book(book) => {
                println!("Kitap: {} Yazar: {}, Sayfa Sayısı:{} ", book.title, book.author, book.page_count);
            }
            Publication::Magazine(magazine) => {
                println!("Dergi: {} - Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
            }
        }
    }
}

fn main() {
    //Kitap Örneği
    let book1=Book{
        title:"Aşk ve Öbür Cinler".to_string(),
        author:"Gabriel Garcia Marquez".to_string(),
        page_count:172,
    };
    //dergi örneği
    let magazine1=Magazine{
        title:"Rust Weekly".to_string(),
        issue:22,
        topic:"Programming".to_string(),
    };
    
    //vektör tanımlası
    let publications = vec![Publication::Book(book1), Publication::Magazine(magazine1)];
    
    print_pub(publications);
}
