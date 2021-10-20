use easy_scraper::Pattern;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

pub fn fetch_data() -> anyhow::Result<Vec<BTreeMap<String, String>>> {
  let data_dir = Path::new("data");
  let data_file = data_dir.join("data.json");

  if !data_file.exists() {
    fs::create_dir_all(data_dir)?;
  }

  if data_file.exists() {
    println!("read from file");
    let file = File::open(data_file)?;
    let reader = BufReader::new(file);
    let contents = serde_json::from_reader::<_, Vec<BTreeMap<String, String>>>(reader)?;
    Ok(contents)
  } else {
    let contents = fetch_remote_data()?;
    let serialized = serde_json::to_string(&contents)?;
    fs::write(data_file, serialized)?;
    Ok(contents)
  }
}

pub fn fetch_remote_data() -> anyhow::Result<Vec<BTreeMap<String, String>>> {
  let content_pattern: Pattern = Pattern::new(
    r#"
<div class="card border-0">
  <a>
        <img data-original="{{img_src}}">
  </a>
    <div class="card-body">
    <h5 class="card-title mb-2">
      <a href="{{link}}">{{title}}</a>
        </h5>
        <p>
            <span class="label label-light" style="margin-left:3px">
              <i class="fab fa-lg fa-fw fa fa-clock"></i>
              {{time}}</span>
        </p>
    <p class="card-text">{{description}}</p>
  </div>
</div>
"#,
  )
  .unwrap();

  let text = reqwest::blocking::get("https://novelgame.jp/player/fes")?.text()?;

  Ok(content_pattern.matches(&text))
}
