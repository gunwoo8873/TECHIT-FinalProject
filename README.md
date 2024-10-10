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
rustup install stable &&\
rustup default stable &&\
git clone https://github.com/gunwoo8873/TECHIT-FinalProject.git
```

Linux Install & Run Script [Stop]
---

* Docker Install
* Kubernetes Install
* Git Install
* Docker Run
* Kubernetes Run
* Exit

```bash
cd TECHIT_FinalProject &&\
 chmod +x run.sh &&\
./run.sh
```

Workspace Lib
---
| Lib          | Workspace               | Feature |
|:-------------|:------------------------|:--------|
| Serde        | Frontend, Backend, Tool | csr     |
| Serde_Json   | Frontend, Backend, Tool | -       |
| Serde_derive | Frontend, Backend, Tool | -       |
| Tokio        | Backend, Tool           | full    |
| Chrono       | Frontend, Backend, Tool | -       |

* Manual
    ```bash
    cargo clean \
    cargo build --release # Rust Build
    ```

Frontend [Stop]
---

| Lib       | Description                        | Feature |
|:----------|:-----------------------------------|:--------|
| Yew       | Frontend                           | csr     |
| wasm      | WebAssembly                        | -       |
| thiserror | Error Handling                     | -       |

* Manual
    ```bash
    cd frontend &&\
    trunk serve # Yew Run CMD
    ```

Backend [Stop]
---
| Lib       | Description | Feature |
|:----------|:------------|:--------|
| axum      | -           | -       |
| actix-web | -           | -       |
| sqlx      | -           | -       |

* Manual
    ```bash
    cd backend &&\
    cargo run # Yew Run CMD
    ```


Kubernetes
---
| Tool       | Description |
|:-----------|:------------|
| Kubernetes | Simple Tool |
| Linux      | Simple Tool |

| Lib         | Description    | Feature         |
|:------------|:---------------|:----------------|
| kube        | Kubectl Lib    | Derive, Runtime |
| k8s-openapi | Kubernetes API | Latest          |
| ratatui     | Terminal UI    | -               |
| crossterm   | -              | -               |

* Manual
  ```bash
  cd kubernetes &&\
  cargo run
  ```

AWS
---
| Lib            | Description         | Feature | Link              |
|:---------------|:--------------------|:--------|:------------------|
| lambda_runtime | -                   | -       | [Lambda Github](https://github.com/awslabs/aws-lambda-rust-runtime) |