# Env

Environment variables required to run the project

| Key         | Description                                                 | Default     |
|-------------|-------------------------------------------------------------|:------------|
| `LOG_LEVEL` | Log level. Default is not set. Can take `info`, `debug` ... | null        |
| `HOST`      | IP address or host name                                     | `localhost` |
| `PORT`      |                                                             | `8080`      |
| `WORKERS`   | Parallelism thread count                                    | as in cpu   |
| `CERT`      | TLS certificate file path.                                  | null        |
| `KEY`       | TLS key file path or nothing                                | null        |
| `USERNAME`  | Basic auth username                                         | `admin`     |
| `PASSWORD`  | Basic auth password                                         | `admin`     |