#!/bin/bash

# Асоціативний масив з парами "ім'я файлу - команда"
declare -A commands=(
  ["app.yaml"]="kubectl ai 'Create a YAML manifest for deploying an application'"
  ["app-livenessProbe.yaml"]="kubectl ai 'Create a YAML manifest for defining a liveness probe for an application'"
  ["app-readinessProbe.yaml"]="kubectl ai 'Create a YAML manifest for defining a readiness probe for an application'"
  ["app-volumeMounts.yaml"]="kubectl ai 'Create a YAML manifest for defining volume mounts for an application'"
  ["app-cronjob.yaml"]="kubectl ai 'Create a YAML manifest for defining a cron job'"
  ["app-job-rsync.yaml"]="kubectl ai 'Create a YAML manifest for defining a job to perform an rsync operation'"
  ["app-multicontainer.yaml"]="kubectl ai 'Create a YAML manifest for defining a multi-container application'"
  ["app-resources.yaml"]="kubectl ai 'Create a YAML manifest for defining resource limits and requests for an application'"
  ["app-secret-env.yaml"]="kubectl ai 'Create a YAML manifest for defining environment variables from a secret for an application'"
)

# Створюємо папку "examples", якщо її не існує
mkdir -p examples

# Цикл для створення файлів і запису результатів
for filename in "${!commands[@]}"
do
  # Отримуємо команду для поточного файлу
  command="${commands[$filename]}"

  # Виконуємо команду, пропускаючи перші два рядки, та перенаправляємо результат у файл у папку "yaml"
  eval "$command" | tail -n +3 | sed -e '/\[?25l\[2K/,$ d' > "examples/$filename"

  # Виводимо повідомлення про виконання команди
  echo "Команда '$command' виконана. Результат записано в файл 'examples/$filename'."

  # Додатковий роздільник між командами
  echo "------------------------------------------------------------------------"
done
