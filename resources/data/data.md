# Data

To make the web crawler functional, we need to extract the data from the website.

The data is stored in the `data/raw` directory as a JSON file.
The processed data is stored in the `data/processed` directory as a Markdown file.

```bash
web-crawler/  # root
├── resources/
│   ├── data/  # store the data
│   │   ├── raw/  # store the data directly from the website
│   │   │   └── some_page_url.json  # only json files allowed
│   │   ├── processed/  # store the processed data
│   │   │   └── some_page_url.md  # only markdown files allowed
│   │   └── data.md  # this file
├── src/  # source code
│   └── main.rs  # app entry point
├── Cargo.toml  # cargo config file
├── LICENSE.md  # MIT license
├── .gitignore
└── README.md
```


### Note: You must create this directory structure yourself.
`todo (me): add code to create directory structure` ;)
