3Day Project Dev
---
> Rust 작업은 추가로 Github Project란에 추가

* unwrap_or_else, unwrap_or 의 차이
  * unwrap_or_else : 기본값을 미리 지정되어 값이 없을 경우 그 값을 반환
  * unwrap_or : 값이 없을 경우 클로저를 실행하여 동적으로 기본값을 계산
  * unwrap_or_else과 unwrap_or을 사용하여 Server의 Host와 Port를 동적으로 설정이 가능하다라는 이론을 적용

우선순위 설정
---
| Seq | Title |
|:----|:------|
| FP1 | 높음    |
| FP2 | 중간    |
| FP3 | 낮음    |

Frontend
---
* FP1
  * Non
* FP2
  * ~~Signin, Register CSS 디자인~~
* FP3
  * ~~Index, Header, Footer CSS 디자인~~

Backend
---
* FP1 
  * Database postgrsql 적용방식 모색
    * Tokio-postgres : 컴파일 및 시간 중심
    * sqlx : SQL을 우선하되 Qurey 생을 위한 DSL
    * diesel : 비동기와 동기 인터페이스의 차이
  * DB AWS, Local 두 가지 방식을 연결해보고 테스트 환경 구축
* FP2
  * Non
* FP3
  * Non