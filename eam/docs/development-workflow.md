# 실제 개발 흐름 종합 검증

2026-09-09 G11 배치 PTY 흐름 완료. 실제 구독 Claude 2.1.263/Codex 0.153.4를 현재 Ghostel 제품 경로로 실행했다. 일상 앱의 GUI 시작·미니버퍼 입력 검증을 대신하지 않는다.

`python3 tests/prepare-workflow.py var/workflow-20260909`가 두 제공자에 동일한 결함·테스트·180초 한도의 중단 프로브를 준비했다. 기존 디렉터리는 덮어쓰지 않는다. 검증 대상 `label.py`는 처음에 입력을 그대로 반환했다. 초기 테스트 4개가 실패하는 것을 확인한 뒤 AI에 수정하게 했다.

| 단계 | Claude | Codex |
| --- | --- | --- |
| 기존 파일 수정 | label.py의 공백 정규화·빈 결과 ValueError 구현 | 동일 요구 구현 |
| 승인 | manual 모드에서 파일 수정과 테스트 명령 각각 Yes | 프로세스별 read-only/on-request에서 파일 변경 명령 1회 승인 |
| 테스트 | CLI 및 독립 실행 각각 4개 통과 | CLI 및 독립 실행 각각 4개 통과 |
| 중단 | heartbeat 변화 확인 후 Escape, PID 62662 부재 | Escape 뒤 PID 62675 생존, /stop 후 부재 |
| 다른 프로젝트 왕복 | 같은 Emacs의 실제 CLI 두 개, 선택 3회, 주 세션 생존 | 동일 |
| 정상 종료 | 보조 /exit 및 주 /exit, 시험 Emacs 종료 코드 0 | 보조 /quit 및 주 /quit, 시험 Emacs 종료 코드 0 |
| 새 실행에서 재개·추가 수정 | 확인 단어 회상, 타입 검사·테스트 추가, 6개 통과 | 동일 |

초기 테스트 파일은 두 프로젝트에서 동일하며 확인 단어 `은하-924`가 프로젝트 Python 파일에 없음을 검사했다. 확인 단어는 대화 재개 시 기억 확인에만 사용한다. 현재까지 명시적 모델 작업 요청은 제공자별 2회(구현, 중단 프로브)다. 보조 CLI에는 모델 요청을 제출하지 않았다. CLI 내부 호출 횟수나 시작 네트워크 트래픽을 측정한 것은 아니다.

프로젝트 왕복은 `tests/workflow-switch.el`이 실제 `emacs-ai-app-switch`에 결정적인 completion 답을 공급했다. 버퍼·프로젝트 경로·주/보조 CLI 생존을 확인하고 보조 CLI만 정상 종료했다. GUI 목록 선택 조작을 시험한 것이 아니다. 첫 드라이버는 bracketed-paste 활성화나 제목만으로 입력 준비를 판단해 정상 종료 검사가 실패했다. 실패한 보조 세션은 unwind-protect로 정리했다. Claude 원시 ANSI는 단어 사이를 커서 이동으로 구분했고, Codex는 제목이 나온 뒤에도 model: loading 단계가 있었다. 최종 드라이버는 보조 터미널을 선택하고 렌더링된 시작 완료 상태를 기다린다. 이 조건은 시험 버전에 맞춘 것이며 제품에 CLI 화면 파싱을 추가하지 않았다.

증거는 [workflow 디렉터리](cli-validation-evidence/workflow/)의 `*-edit-approval.txt`, `*-initial-tests.txt`, `initial-checks.json`, `*-after-escape.txt`, `*-switch.json`, `*-initial-exit.txt`에 있다. 실패 시도 로그와 원시 PTY 위치는 `var/workflow-20260909/{claude,codex}.log` 및 제공자별 `state.json`에 보관했다. 중단 후 완료 파일 `wait.finished`가 없는 것도 확인했다.

## 새 실행 재개 결과

새 배치 Emacs에서 기존 `tests/cli-validation-driver.el`을 사용한다. 작업 경로는 `var/workflow-20260909/PROVIDER/workspace`로 유지하고 출력 디렉터리만 새로 만든다. `session-id.txt`에 저장된 ID로 네이티브 재개한다.

- Claude: `719838cb-48a0-4f26-aa30-52067a678da6`
- Codex: `01a081d4-6bc9-7c81-83d8-79a1951e4452`

`var/workflow-20260909/resume.txt`는 확인 단어를 다시 제공하지 않고 회상·문자열 외 입력 TypeError·None/정수 테스트 추가를 요청했다. 새 Emacs에서 두 대화를 재개해 각각 1회 제출했다. Claude는 파일 두 개와 테스트 명령을 개별 승인했고, Codex는 시험 프로젝트에 workspace-write를 사용했다(첫 실행의 read-only 승인 검사와 구분).

두 제공자 모두 은하-924를 회상했고 후속 수정과 테스트 6개를 완료했다. `python3 tests/verify-workflow.py var/workflow-20260909 docs/cli-validation-evidence/workflow`로 각각 테스트 6개 재실행 및 함수 직접 호출 10개(한글/이모지·Unicode 공백·빈 값·None·정수·bytes·list·dict·float)를 확인했다. Python 파일에 확인 단어가 없음을 다시 검사했다. 두 CLI 정상 종료와 새 시험 Emacs 두 개의 종료 코드 0도 확인했다. 전체 명시적 모델 작업 요청은 제공자별 3회다.

최종 Claude 화면에서 이전 회상 문구가 사라져 최초 화면 발췌 검사가 실패했다. CLI 오류가 아니라 현재 화면만 읽은 증거 수집의 범위 문제다. 원시 PTY에서 실제 회상 응답과 완료 문장을 확인하고 CSI/OSC를 제거한 짧은 발췌 및 원본 SHA256을 보관했다. 이를 완전한 ANSI 화면 재구성으로 주장하지 않는다. 증거는 `*-resumed-raw-excerpt.txt`, `*-resumed-record.json`, `*-resumed-exit.txt`, `resumed-checks.json`, `*-resumed-tests.txt`, `*-final-*.py`다.

이로써 **G11의 배치 PTY 개발 흐름**은 완주했다. G01/G10 일상 GUI 진입·물리 IME·Caffeine UI, G08 장시간 GUI 반응, G04의 실제 goal 수행 등 다른 미완료 항목을 이 결과로 대체하지 않는다.
