use anyhow::Result;
use easy_scraper::Pattern;
use std::fs;
use std::path::Path;

pub fn fetch_data() -> Result<()> {
  let data_dir = Path::new("data");
  let data_file = data_dir.join("data.json");

  if data_file.exists() {
    return Ok(());
  }

  fs::create_dir_all(data_dir)?;

  Ok(())
}

pub fn fetch_remote_data() -> anyhow::Result<()> {
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
  let ms = content_pattern.matches(&text);

  Ok(())
}
