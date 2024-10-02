ELK [Elasticsearch, Logstash, Kibana]
---
* E : Elasticsearch
  * Apache Lucene에 구축되어 있는 배포된 검색 및 분석 엔진
  * 스키마가 없는 Json 문서로 다양한 로그 분석 및 검색 사용 사례
  * `Apache License Version 2.0`에서 `Elastic License, SSPL`로 변경되어  
    사용자에게 동일한 자유를 제공하진 않지만, ALv2의 갈래인 `OpenSearch` 프로젝트를 활용해야 한다.
---
* L : Logstash
  * 다양한 소스로부터 데이터 수집하고 전환하여 원하는 대상에 전송할 수 있도록 하는 오픈소스 데이터 도구 이다.
  * 사전 구축된 필터와 200개의 플러그인으로 사용자가 소스나 유형에 관계없이 데이터를 쉽게 수집할 수 있도록 지원한다.
  * 경량의 오픈 소스 서버측 데이터 처리 파이프라인 이기 때문에 데이터 분석 및 검색 엔진인 Elasticsearch의 파이프라인으로 사용
  * 대표적 지원 방식
    * 비정형 데이터 로드
    * 사전 구축된 필터
    * 유연한 플러그인 아키텍처
---
* K : Kibana
  * 