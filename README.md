# ADS POSITION SCANNER

Бесплатный сканер позиций объявлений на платформе Авито.
Основная задача этого инструмента — собирать данные о количестве просмотров объявлений за сутки, а также о доле этих просмотров в общем количестве, анализ конкурентов в нише. Кроме того, он позволяет отслеживать позиции, на которых размещаются объявления.
Все собранные данные сохраняются в формате.csv.
Для корректной работы сканера необходимо ознакомиться с рекомендациями, которые можно найти в конфигурационном файле.env. После установки этот файл следует переименовать из .env.example.

![example](https://github.com/KirDontsov/crawler/blob/master/assets/example.jpeg?raw=true)



## Запуск под Linux

```apt install curl git-all``` - установка git

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` - установка языка Rust

```rustup default nightly``` - переключаем Rust на версию nightly

```wget https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.264/linux64/chromedriver-linux64.zip``` - скачиваем актуальный архив chromedriver соответствующий версии вашего браузера Chrome (его нужно распаковать в папку /crawler)
Ссылку берем от сюда: https://googlechromelabs.github.io/chrome-for-testing/

```git clone https://github.com/KirDontsov/crawler.git``` - клонируем репозиторий проекта

```сd crawler``` - спускаемся в директорию проекта

```mkdir output``` - создаем директорию для отчетов

```find . -type f -exec chmod 0664 {} ';'```

```chown -R USER:USER output``` - назначаем права на папку отчетов (если ее нет, ее нужно создать ```mkdir output```)

```chmod ug+rwx output```

```chmod u+x start.sh``` - файл start.sh делаем исполняемым

```./start.sh``` - запуск сканера

### Проверено на Ubuntu 22.04 и Ubuntu 25.04



## Запуск под Mac

```brew install git``` - установка git

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` - установка языка Rust

```rustup default nightly``` - переключаем Rust на версию nightly

```curl https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.264/linux64/chromedriver-linux64.zip --output -cromedriver.zip``` - скачиваем актуальный архив chromedriver соответствующий версии вашего браузера Chrome (его нужно распаковать в папку /crawler)
Ссылку берем от сюда: https://googlechromelabs.github.io/chrome-for-testing/

```git clone https://github.com/KirDontsov/crawler.git``` - клонируем репозиторий проекта

```сd crawler``` - спускаемся в директорию проекта

```mkdir output``` - создаем директорию для отчетов

```./start.sh``` - запуск сканера

### Проверено на MAC OS Sequoia 15.0.1

### !!!ВНИМАНИЕ по MAC OS может не работать актуальная версия Chrome браузера. Нужно использовать более раннюю версию. Например: Версия 123.0.6312.87



### В Mac Os есть возможность запуститься под Firefox, для этого нужно будет вместо cromedriver установить gekodriver

Вместо команды ```curl https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.264/linux64/chromedriver-linux64.zip --output -cromedriver.zip```

выполняем ```cargo install geckodriver```

и в директории проекта выполняем

```git checkout firefox```



### DEV NOTES

```cargo watch -q -c -w src/ -x run``` - run for dev

```cargo r -r``` - run for prod

```./chromedriver --port=9515 --disable-gpu --dns-prefetch-disable --disable-extensions --no-sandbox enable-automation``` - run chrome driver, если вылетает, нужно обновить на более новую версию chromedriver-mac-x64



### ВНИМАНИЕ!!! Под Windows программу не адаптировали, не проверяли.
