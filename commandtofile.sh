#!/bin/bash

# Асоціативний масив з парами "ім'я файлу - команда"
declare -A commands=(
  ["app.yaml"]="kubectl ai 'Create a YAML manifest for deploying an application. The manifest should include the necessary specifications to define the deployment, including the container image, ports, and any additional configuration required.'"
  ["app-livenessProbe.yaml"]="kubectl ai 'Create a YAML manifest that defines a liveness probe for an application. This manifest specifies parameters for periodically checking the state of the application to determine if it is functioning correctly. By using a liveness probe, you can detect if your application has stopped or is in an unhealthy state and take appropriate actions to restore or restart it'"
  ["app-readinessProbe.yaml"]="kubectl ai 'Create a YAML manifest for defining a readiness probe for an application. The manifest should include the necessary specifications to configure the readiness probe, such as the probe type (e.g., HTTP or TCP), endpoint path, port, initial delay, period, success threshold, and failure threshold'"
  ["app-volumeMounts.yaml"]="kubectl ai 'Create a YAML manifest for defining volume mounts for an application. Specify the required fields to configure the volume mounts in the manifest'"
  ["app-cronjob.yaml"]="kubectl ai 'Create a YAML manifest for defining a cron job in a Kubernetes cluster. Specify the schedule, command, and other configuration options according to your requirements'"
  ["app-job-rsync.yaml"]="kubectl ai 'Create a YAML manifest for defining a job to perform an rsync operation'"
  ["app-multicontainer.yaml"]="kubectl ai 'Create a YAML manifest for defining a multi-container application. The frontend container serves a web application on port 80. The backend container handles API requests on port 8080'"
  ["app-resources.yaml"]="kubectl ai 'Create a YAML manifest for defining resource limits and requests for an application. In this manifest, you can specify the resource requirements for CPU and memory that the application needs to run optimally. By setting resource limits and requests, you can allocate resources effectively and ensure the application has the necessary compute capacity'"
  ["app-secret-env.yaml"]="kubectl ai 'CCreate a YAML manifest that defines environment variables from a secret source for your application. This manifest allows you to securely pass confidential data, such as passwords or access keys, as environment variables in your application. By utilizing secrets, you can ensure the security of your application and protect confidential data from unauthorized access'"
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
