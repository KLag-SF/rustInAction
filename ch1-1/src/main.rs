fn main() {
    let penguin_data = "\
    common name, length(c&m)
    Little penguin,33
    Yellow-Eyed penguin, 65
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record     // Vec<_> → ジェネリクス型を "_" にすることでRustの型推論を利用可能
            .split(',')          // カンマで分割
            .map(|field| field.trim())// 要素の前後の空白を削除
            .collect();                     // Vectorに収集
        
        if cfg!(debug_assertions){
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
           println!("{}, {}cm", name, length); 
        }
    }


}
