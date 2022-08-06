use std::any::type_name;
use std::collections::HashMap;


fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // берем ссылку на третий елемент
    let third = &v[2];
    println!("{}", third);
    // дальше мы не сможем использовать ссылку third, так как добавили новое значение в вектор
    v.push(10);

    for i in &mut v {
        // разыменование, чтобы достучаться до значения
        *i *= 10;
    }

    print_vector(&v);

    // как хранить разные типы?
    // сделаем перечисление из всех возможных типов в будущем векторе
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String)
    }
    // наполняем вектор этими типами
    let data = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(1.2),
        SpreadsheetCell::Text("planet!".to_string())
    ];

    println!("typed vector:");
    for (index, item) in data.iter().enumerate() {
        match item {
            SpreadsheetCell::Int(i) => println!("{}: {}", index, i),
            SpreadsheetCell::Float(i) => println!("{}: {}", index, i),
            SpreadsheetCell::Text(i) => println!("{}: {}", index, i)
        }
    }

    //   и в чем блять отличие?))
    // кортежи для группировки разных типов в один составной
    let mut tup = (1, 2.3, "hello world");
    // массивы для хранения значений одного типа
    let mut arr = [1, 2, 3];
    println!("tup.0 = {}", tup.0);
    println!("arr[1] = {}", arr[1]);

    print_type_of(&tup);
    print_type_of(&arr);
    
    arr[0] = 100;
    println!("arr[0] = {}", arr[0]);

    tup.0 = 100;
    println!("tup.0 = {}", tup.0);

    println!("\n");
    // заполняем хешмапу вручную
    let mut dict: HashMap<&str, i32> = HashMap::new();
    dict.insert("Apple", 100);
    dict.insert("BMW", 120);
    dict.insert("Xiaomi", 229);
    for (key, value) in dict.iter() {
        println!("{}: {}", key, value);
    }

    println!("\n");
    // юзаем зип векторов
    let names = vec!["Daniel", "Mark", "John"];
    let scores = vec![1, 2, 3];
    // тут важно проаннотировать, чтобы метод коллект понял в какой тип конвернуть
    let mut table: HashMap<_, _> = names.into_iter().zip(scores.into_iter()).collect();

    for (k, v) in table.iter() {
        println!("{}: {}", k, v);
    }

    // println!("{:?}", names); ошибка
    table.entry("Michael").or_insert(10);
    table.entry("Michael").or_insert(25);
    println!("{:?}\n", table);

    // практика: подсчет слов в строке
    let text = "привет привет как дела как дела ахахаха у меня отлично а у тебя друг";
    let mut words = HashMap::new();
    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", words);
}

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn print_vector(v: &Vec<i32>) {
    println!("VECTOR:");
    for (index, item) in v.iter().enumerate() {
        println!("{}: {}", index, item);
    }
}