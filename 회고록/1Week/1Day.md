1 ~ 2일차
---
Frontend
---
  * Yew을 사용할 경우 Vertual Dom 요소를 어떻게 적용할지 고민
  * 프로젝트에 틀을 만들기 위해 `Signin`, `Register` 페이지는 미리 제작
    * Tutorial 기반으로 개발을 시작하고, 유연하게 새로운걸 개발을 목표
    * Element CSS 적용 방식을 확인결과, Style파일을 생성할지 혹은 Page파일에 적용할지 의문  
    => 결정 : 개별 Style파일을 생성하되 공통적인 Header, Footer 부분은 조금 고려해야 할거 같다.
    * Trunk 특성상 localhost Port 넘버를 8080으로 사용하기 때문에 Trunk.toml파일에서 8080외 포트넘버를 지정해야 하는 문제
    
      ```markdown
      /root
      └── Frontend
            └── src
                ├── main.rs
                ├── router.rs
                ├── app.rs
                ├── views
                │       └── auth
                └── api
      ```
  * 목표 : Yew을 사용하여 `Handler. Router, API` 개발

Backend
---
* Backend Server를 구축하기 위해 Actix와 Axum의 차이점을 파악

| -      |         Acctix          |          Axum           |
|--------|:-----------------------:|:-----------------------:|
| 스레드    |           싱글            |           멀티            |
| 비동기 지원 |      자체 비동기 런타임 지원      |     Tokio 런타임 기반 지원     |
| 문서화 |       규모가 있는 커뮤니티       |         규모가 성장중         |
| 확장성 | Actix-Actor를 사용해 다양한 구성 |  Tower를 통해 다양한 미들웨이 구성  |
| 안전성 |     안전성 위해서 주의가 필요      | 안전에 집중하여 주의가 크게 요구되지 않음 |
| 사용처 | 대규모 App과 높은 성능을 요구되는 시스템 | REST API 서버, 간단한 마이크로 서비스 |

* 목표 : Actix-web을 사용하여 퍼포먼스에 집중