# TECHIT FinalProject
* Title       : `Hybrid Cloud & Multi Cloud`
* Description : `-`
* Date        : 
  * Setup     : `2024. 09. 27 ~ 30`
  * Devploy   : `2024. 09. 30 ~ 11. 01`
* Member      : `박종찬, 손채린, 윤현석, 정소은, 최건우`
* Position    : `PM, PL, CI/CD, Monitoring, Infra`
* Version     : `a0.1.1`

| Skill | Description                          |
|:------|:-------------------------------------|
| Rust  | Frontend & Backend `Server, Handler` |
| Shell | `Install & Run` script               |
| AWS   | Cloud Infra `EC2, EKS, S3, RDS`      |

```bash
git clone https://github.com/gunwoo8873/TECHIT-FinalProject.git
```

Install & Run Script
---
```bash
cd TECHIT_FinalProject && chmod +x run.sh
./run.sh
```
* Docker Install
* Kubernetes Install
* Git Install
* Docker Run
* Kubernetes Run
* Exit

Frontend
---

| Lib       | Description                        | Feature |
|:----------|:-----------------------------------|:--------|
| Yew       | Frontend                           | csr     |
| wasm      | WebAssembly                        | -       |
| chrono    | Date & Time                        | -       |
| serde     | Serialization & Deserialization    | default |
| thiserror | Error Handling                     | -       |

* Manual
    ```bash
    rustup install stable &&\ # Rust Install
    rustup default stable &&\ # Rust Run Default
    cd frontend &&\
    cargo build --release &&\ # Rust Build
    trunk serve --open # Yew Run CMD
    ```

Backend
---
| Lib        | Description | Feature |
|:-----------|:------------|:--------|
| axum       | -           | -       |
| actix-web  | -           | -       |
| tokio      | -           | -       |
| Postgresql | -           | -       |

AWS
---
| Lib            | Description         | Feature | Link              |
|:---------------|:--------------------|:--------|:------------------|
| lambda_runtime | -                   | -       | [Lambda Github](https://github.com/awslabs/aws-lambda-rust-runtime) |