pub fn index_html() -> String {
  format!(
    r#"
  <!DOCTYPE html>
  <html lang="ko">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Server Page</title>
  </head>
  <body style="background: black;">
    <main style="color: white;">
      <div>Run Server ^--------^</div>
    </main>
  </body>
  </html>
"#
  )
}
