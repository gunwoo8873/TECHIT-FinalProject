ELK [Elasticsearch, Logstash, Kibana]
> ELK Stack은 세 가지 S/W 구성 요소가 있는 오픈 소스 로그 분석 솔루션  
> 클라우드에서 로그 데이터를 수집, 집계, 분석 및 시각화 하여 애플리케이션 모니터링 및 보안 분석과 같은 기능을 지원

---
* E : Elasticsearch
  * Apache Lucene에 구축되어 있는 배포된 검색 및 분석 엔진
  * 스키마가 없는 Json 문서로 다양한 로그 분석 및 검색 사용 사례
  * `Apache License Version 2.0`에서 `Elastic License, SSPL`로 변경되어  
    사용자에게 동일한 자유를 제공하진 않지만, ALv2의 갈래인 `OpenSearch` 프로젝트를 활용
---
* L : Logstash
  * 다양한 소스로부터 데이터 수집하고 전환하여 원하는 대상에 전송할 수 있도록 하는 오픈소스 데이터 도구 이다.
  * 사전 구축된 필터와 200개의 플러그인으로 사용자가 소스나 유형에 관계없이 데이터를 쉽게 수집할 수 있도록 지원한다.
  * 경량의 오픈 소스 서버측 데이터 처리 파이프라인 이기 때문에 데이터 분석 및 검색 엔진인 Elasticsearch의 파이프라인으로 사용
  * AWS CloudWatch API, AWS S3 버킷을 포함한 다양한 소스에서 로그및 이벤트 데이터 집계하여 수집 후 데이터 처리
  * 지원 예시
    * 비정형 데이터 로드
      * System, Web, Application Server 로그 등 다양한 데이터 소스에서 비정형 데이터를 수집 가능
    * 사전 구축된 필터
      * 사전 구축된 필터로, 사용자는 일반 데이터 유형을 쉽게 전환후 Elasticsearch로 인덱싱
      * 사용자 지정 데이터 전환 파이프라인이 필요하지 않고 쿼리로 시작 가능
    * 유연한 플러그인 아키텍처
      * 구축된 플러그인이 존재해, 데이터 파이프라인을 직접 지정하는 데 필요한 플러그인이 이미 구축
      * 별도로 사용자에 맞춤 커스텀 플러그인을 개발이 가능
---
* K : Kibana
  * 로그와 분석, 애플리케이션 모니터링, 운영 인텔리전스 사용 사례에 사용되는 데이터 시각화 및 탐색 도구
  * 히스토그램, 선형 그래프, 원형 차트, 열 지도, 내장 지리 공간적 지원과 함께 사용자 친화적인 기능을 제공
  * On-Pramise, Amazon Elastic Computer Cloude[EC2] 혹은 OpenSearch Service의 Apach 2.0 라이선스 버전에서 실행 가능
  * 지원 예시
    * 대화형 차트
      * 대량의 로그 데이터를 대화형으로 탐색이 가능하여 사용할 수 있는 이니셔티브 차트와 보고서를 제공
      * 스크린을 동적으로 드래그하고 특정 데이터 하위 집합을 확대하거나 축소하여 데이터에서 실천 가능한 인사이트 추출하기 위해 휠업다운 가능
    * 매핑 지원
      * 지리 공작적 기능이 포함되어 사용자는 데이터의 지리적 정보를 제일 위에 두고 결과를 지도에 시각화하는 계층화 작업을 수행이 가능
    * 구축된 집계 및 필터
      * 사전 구축된 집계 및 필터를 사용하면 히스토그램, 상위 쿼리, 추세와 같은 분석을 실행
    * 액세스 가능한 대시보드
      * 대시보드와 보고서를 쉽게 설정하고 다른 사람에게 공유가 가능하기 때문에, 데이터를 확인하고 탐색이 가능
---
사용사례
---
* Observability
  * Log, Metric, Traces
    * 애플리케이션, 시스템, 서비스에서 로그, 메트릭, 추적, 수집, 저장 및 분석 한다.
  * Application Performance Monitoring[APM]
    * S/W Application의 Performance 모니터링하고 분석 한다.
  * Real User Monitoring[RUM]
    * Web Application과 User간 상호작용을 모니터링, 분석, 정량화 한다.
  * OpenTelemetry
    * 기존 계측을 재사용하여 Open Telemetry 표준을 사용해 Elastic Stack으로 텔레메트리 데이터를 전송
---
* Search
  * Vector Database
    * 벡터화된 데이터를 저장하고 검색하여, 내장 및 타사 자연어 처리[NLP] 모델을 사용하여 벡터 임베딩을 생성
  * Full-Text Search
    * 역색인, 토큰환, 텍스트 분석을 사용하여 빠르고 관련성 높은 전체 텍스트 검색 솔루션 구축
  * Semantic Search
    * 동의어, 밀집 벡터 임베딩, 학습된 희소 쿼리-문서를 확장하여 사용하고 검색 쿼리의 의도와 맥락적 의미를 이해
  * Hybrid Search
    * 최신 알고리즘을 사용하여 전체 텍스트 검색과 벡터 검색을 결합
  * Build Search Experiences
    * 앱이나 웹시아트에 하이브리드 검색 기능을 추가허간 조직의 내부 데이터 소스를 대상으로 엔터프라이즈 검색 엔진 구축
  * Retrieval augmented Generation[RAG]
    * ElasticSearch를 검색 엔진으로 사용하여 생성 AI모델을 더 관련성 높고 최신의 또는 독점 데이터로 보완
  * Geospatial Search
    * 지리 공간 쿼리를 사용하여 위치를 검색하고 공간 관계를 계산
---
* Security
  * Security Information and Event management[SIEM]
    * 애플리케이션, 시스템, 서비스에서 보안 데이터를 수집, 저장, 분석 수행
  * Endpoint security
    * 엔드포인트 보안 데이터를 모니터링하고 분석
  * Threat Hunting
    * 데이터를 검색하고 분석하여 보안 위협을 탐지하고 대응