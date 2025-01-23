#!/bin/bash
#$ cat start.sh

echo "├➔ Проверяем от какого пользователя запускаем скрипт | $(whoami)"

echo "├➔ Проверяем директорию в которой находимся          | $(pwd)"

echo "├➔ Проверяем обновления                              | $(git pull)"

echo "├➔ Компилируем crawler                               | $(cargo build -r)"

# запускаем драйвер браузера

chromedriver_drv=$(pgrep chromedriver)
if [ -z $chromedriver_drv ]
then
./chromedriver --port=9515 --disable-gpu --dns-prefetch-disable --disable-extensions --no-sandbox enable-automation &
echo "├➔ Запускаем драйвер для работы браузера             |"
else
echo "├➔ Драйвер для работы браузер уже запущен            | $(pgrep chromedriver)"
fi
echo "└➔ Запускаем парсер                                  | crawler"
target/release/crawler
