# ADS POSITION SCANNER

Сканнер позиций объявлений

## Запуск под Linux

```apt install curl git-all``` - установка git

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` - установка языка Rust

```rustup default nightly``` - переключаем Rust на версию nightly

```wget https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.264/linux64/chromedriver-linux64.zip``` - скачиваем архив chromedriver (его нужно распаковать)

```git clone https://github.com/KirDontsov/crawler.git``` - клонируем репозиторий проекта

```сd crawler``` - спускаемся в директорию проекта

```find . -type f -exec chmod 0664 {} ';'```

```chown -R USER:USER output``` - назначаем права на папку отчетов (если ее нет, ее нужно создать ```mkdir output```)

```chmod ug+rwx output```

```./start.sh``` - запуск сканера

## Запуск под Mac

```brew install git``` - установка git

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` - установка языка Rust

```rustup default nightly``` - переключаем Rust на версию nightly

```curl https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.264/linux64/chromedriver-linux64.zip --output -cromedriver.zip``` - скачиваем архив chromedriver (его нужно распаковать)

```git clone https://github.com/KirDontsov/crawler.git``` - клонируем репозиторий проекта

```сd crawler``` - спускаемся в директорию проекта

```mkdir output``` - создаем директорию для отчетов

```./start.sh``` - запуск сканера

### В Mac Os есть возможность запуститься под Firefox, для этого нужно будет вместо cromedriver установить gekodriver

Вместо команды ```curl https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.264/linux64/chromedriver-linux64.zip --output -cromedriver.zip```

выполняем ```cargo install geckodriver```

и в директории проекта выполняем

```git checkout firefox```

### DEV NOTES

```cargo watch -q -c -w src/ -x run``` - run for dev

```cargo r -r``` - run for prod

```./chromedriver --port=9515 --disable-gpu --dns-prefetch-disable --disable-extensions --no-sandbox enable-automation``` - run chrome driver, если вылетает, нужно обновить на более новую версию chromedriver-mac-x64
