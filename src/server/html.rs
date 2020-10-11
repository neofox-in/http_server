pub fn generate(table_body: String) -> String {
    let start: String = String::from(r#"<!DOCTYPE html><html lang="en"><link rel="icon" href="https://neofox.in/files/img/Logo/neofox.webp" sizes="16x16 32x32" type="image/webp"><link href="https://fonts.googleapis.com/css2?family=Tajawal&display=swap" rel="stylesheet"><style>*{margin: 0; padding: 0; box-sizing: border-box; -webkit-box-sizing: border-box; -moz-box-sizing: border-box; -webkit-font-smoothing: antialiased;}body{background: #fefefe; font-size: 30px; font-family: 'Tajawal', sans-serif;}h2{text-align: left; font-size: 18px; text-transform: uppercase; letter-spacing: 1px; color: white; padding: 30px 0;}table{border-radius: 5px; font-weight: normal; border: none; border-collapse: collapse; width: 100%; max-width: 100%; white-space: nowrap; background-color: white;}td, th{text-align: left; padding: 8px;}td{border-right: 1px solid #f8f8f8;}thead th{color: #ffffff; background: #4fc3a1;}thead th:nth-child(odd){color: #ffffff; background: #324960;}tr:nth-child(even){background: #f8f8f8;}</style><head> <title>Server</title> <meta charset="UTF-8"> <meta name="viewport" content="width=device-width, initial-scale=1"></head><body><div><table>"#);
    let end: String = String::from(r#"</tbody></div></body></html>"#);

    let table: String = String::from(r#"
        <thead>
        <tr>
            <th>Name</th>
            <th>Path</th>
            <th>Size</th>
        </tr>
        </thead>
        <tbody>"#);
    format!("{}{}{}{}", start, table, table_body, end)
}