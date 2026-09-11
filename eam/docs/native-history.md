# CLI 원본 기반 history — 2026-09-13

## 변경과 저장 범위

- 기본 `eam-record-terminal=nil`: 새 지속 세션은 output.ansi와 Emacs 입력 진단 파일을 만들지 않는다. PTY 수명 관리와 알림 이벤트는 유지한다.
- 진단 설정은 새 세션에 적용한다. 이미 실행 중인 recorder는 바꾸지 않는다. 기존 기록도 삭제하지 않는다.
- history/prompt는 CLI의 저장소를 읽기만 한다. 프롬프트 수집 코드와 새 CLI의 수집 훅 주입을 제거했다. 개인 설정 파일과 CLI 저장소는 수정하지 않는다.
- 현재 프로젝트와 일치하는 대화도 사용자가 제목·시각·ID로 선택한다. 최근 항목을 현재 CLI 대화로 임의 연결하지 않는다.
- 세션 관리 JSON, 외부 편집기용 작업 파일, 알림 이벤트는 여전히 EAM에 필요하다. native history는 대화 사본이나 검색 인덱스를 EAM에 만들지 않는다.

## 지원한 로컬 형식

- Claude: `projects/*/*.jsonl`의 root user/assistant text. isMeta, isSidechain, tool_result, tool_use, thinking 제외.
- Codex: `state_5.sqlite`의 threads 목록. paginated는 `thread_history_1.sqlite`의 thread_items 중 userMessage/agentMessage, legacy는 rollout JSONL의 response_item/message.
- SQLite는 mode=ro, query_only 연결이며 조회 후 닫는다. 기록 끝의 미완성 JSONL 행은 다음 갱신까지 제외한다.
- Emacs 페이지 최대 65,536자(사용자 설정이 작으면 그 이하), 응답 worker 결과 최대 2Mi 문자, 개별 native 행 최대 8MiB. 목록 최대 300개, Claude 탐색 최대 10,000개 파일. 제한/DB 스키마 오류는 명시적으로 실패한다.
- CLI 내부 파일 형식은 공개 안정 API로 가정하지 않는다. 향후 형식 변경에 맞춘 어댑터 수정이 필요할 수 있다.

## 자동 검증과 측정

현재 재현: `bash scripts/test-native.sh`, `bash scripts/test-native-package.sh`. 실제 PTY와 가짜 CLI를 사용한다.

- Python fixture: 양쪽 형식, 다른 thread 격리, 한글·코드, 도구/추론 제외, 원본 불변, 페이지 합계 복원, 긴 프롬프트 이동, 잘못된/큰 행 오류.
- Emacs: readonly·undo 비활성·이전 페이지 상한·worker 오류 정리·기본 기록 OFF.
- 지속 세션: 기록 OFF의 attach/detach 및 workspace 복원, 기록 ON의 아카이브 보존, OFF의 CLI 종료 후 서버 정리. 상태 조회 중 서버가 종료되는 경쟁 조건을 수정하고, 종료 메타데이터 없이 stopped로 오판하지 않는 테스트 추가.
- 실제 로컬 기록은 텍스트를 보고서에 복사하지 않고 행 형식·문자 수만 집계했다. Claude 43개 목록과 3개 페이지 0.059초, Python tracemalloc peak 1,221,967 bytes. Codex 최근 300개 목록과 3개 페이지 3.249초, peak 33,710,665 bytes. 추적 오버헤드가 포함된 한 번의 측정이며 OS RSS가 아니다.
- Codex 실제 샘플: legacy 4,252,124자 → 65,536자 표시; paginated 248,556자 / 216,704자 → 각각 65,536자 표시.

`tests/native-history-benchmark.el`은 200만 한글 문자의 가짜 응답을 1/4/12개 조회 버퍼에서 읽는다.

| 조회 버퍼 수 | 누적 버퍼 문자 수 | 해당 단계의 추가 열기 시간 |
| --- | ---: | ---: |
| 1 | 65,536 | 0.043초 |
| 4 | 262,144 | 0.125초 |
| 12 | 786,432 | 0.337초 |

10ms 타이머 58회 실행, 최대 간격 67ms. 파일 생성·worker 시작 비용을 포함한 배치 관찰이다. 전체 Emacs 메모리 상한이나 실제 GUI 프레임 지연을 뜻하지 않는다. 각 worker는 조회 후 종료하며 숨은 응답 버퍼도 제거한다.

## 한계와 GUI 구분

- 사용자 요청과 assistant 텍스트의 저장 순서 뷰다. Claude 분기/압축/재생 항목의 전체 활성 분기 재구성, 모든 CLI 버전의 중복 제거는 보장하지 않는다.
- 저장 전 스트리밍 조각, 이미지/첨부 원본, 도구 결과는 표시하지 않는다. `eam-prompt`는 마지막 저장 사용자 텍스트이지 현재 처리 중이라는 증거가 아니다.
- `g` 수동 갱신. 매 페이지 조회 시 기록을 순회할 수 있어 매우 큰 대화에는 지연이 생긴다. 30초 제한, C-g 취소. native 전체 대화 검색은 아직 없고 C-s는 현재 페이지 검색이다.
- Markdown은 원문과 간단한 강조만 표시한다. CLI TUI나 코드별 syntax highlighting을 재현하지 않는다.
- 이번 변경은 배치 검증이며 실제 GUI 한글 조합·스크롤을 새로 검증하지 않았다. 기존 사용자 확인과 구분한다. 실제 AI 호출은 하지 않았다.
