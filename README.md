# TECHIT FinalProject
* Title   : `-`  
* Date    : `2024. 09. 30 ~ 11. 01`
* Version : `a1.0.0`

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

* Manual
    ```bash
    rustup install stable &&\ # Rust Install
    rustup default stable &&\ # Rust Run Default
    cargo build --release &&\ # Rust Build
    cd frontend &&\
    trunk serve --open # Yew Run CMD
    ```

Backend
---