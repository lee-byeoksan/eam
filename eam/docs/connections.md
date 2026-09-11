# 공식 CLI 연결과 검증 — 2026-09-08

일반 Emacs 버퍼에 Claude Code와 Codex의 실제 응답을 표시하는 최소 연결을 추가했다. 개인 Emacs 설정을 수정하지 않았고 공식 CLI를 포크하거나 인증 토큰을 추출하지 않았다. 기본 실행은 여전히 가짜 스트림이다.

## 공식 경로와 구독 조건

| 제공자 | 선택한 경로 | 이 컴퓨터에서 확인한 로그인 |
| --- | --- | --- |
| Claude Code 2.1.263 | 설치된 CLI의 `-p --output-format stream-json --include-partial-messages`, 다음 전송은 `--resume` | `claude.ai`, Max |
| Codex 0.153.4 | 설치된 CLI의 `app-server` stdio JSON-RPC, thread/start → turn/start, 후속 thread/resume | `chatgpt`, Pro |

Claude 공식 문서는 비대화형 CLI와 스트리밍·세션 재개를 지원한다. 자신의 구독 계정으로 수정하지 않은 공식 CLI를 실행하는 개인 도구로 사용하며, 다른 사용자들의 자격 증명을 수집하거나 공유하지 않는다. [Headless](https://code.claude.com/docs/en/headless), [인증](https://code.claude.com/docs/en/authentication), [공식 이용 조건 설명](https://code.claude.com/docs/en/legal-and-compliance).

Codex는 외부 클라이언트를 위한 app-server 프로토콜과 ChatGPT 로그인 경로를 제공한다. API 키 인증은 별도의 API 사용량 과금 경로이므로 이 연결은 account/read의 chatgpt 인증을 확인한 뒤에만 turn/start를 보낸다. [App Server](https://learn.chatgpt.com/docs/app-server), [인증](https://learn.chatgpt.com/docs/auth).

이 결과는 현재 계정의 구독 인증으로 요청이 성공했다는 뜻이다. 무제한 이용이나 추가 요금이 절대로 없다는 보장은 아니다. 구독의 모델·사용량 제한, 계정에서 활성화한 추가 사용량·크레딧 및 조직 정책은 제공자가 적용한다. API 키 환경 변수는 하위 프로세스에서 제거하고 구독 인증이 아니면 요청 전에 중지한다. 부모 환경과 개인 인증 파일은 수정하지 않는다. API SDK를 설치하거나 API 키를 발급하지 않았다.

## 자동 삽입과 호출 억제

앱은 입력 버퍼의 원문만 전송한다. 화면에 표시하는 `[사용자]` 같은 구분자는 전송하지 않는다. 파일 첨부, 선택 영역, 시스템 프롬프트, 스킬, 대화 요약, 제목 생성, 자동 재시도를 추가하지 않는다. 새 버퍼 생성·과거 기록 조회에는 프로세스나 AI 호출이 없다. 전송할 때 인증 상태 조회와 제공자 프로토콜 초기화는 수행한다.

Claude는 프로세스 시작 전 `CLAUDE_CODE_SAFE_MODE=1`을 지정하고 `--safe-mode`, `--disable-slash-commands`, 빈 `--setting-sources`, `--strict-mcp-config`를 함께 사용한다. 최초 `--safe-mode`만으로 수행한 실험에서는 init 이벤트에 스킬·플러그인 목록이 남았다. 수정 후 두 턴 모두 skills/plugins/mcp_servers/slash_commands가 빈 배열이고 도구는 Glob/Grep/Read뿐임을 관찰했다. 이 옵션은 설치된 버전의 `claude --help`와 [CLI 문서](https://code.claude.com/docs/en/cli-reference)를 함께 확인했다. `--bare`는 설치 버전에서 OAuth/Keychain을 읽지 않으므로 구독 연결에 사용하지 않았다.

Codex는 실행 인자로 hooks/plugins/apps/multi_agent/memories/skill_search를 끄고 host skill discovery를 건너뛰며 프로젝트 지침 읽기량을 0으로 설정한다. 추가 developer_instructions는 빈 값으로 덮어쓴다. 사용자·프로젝트 TOML의 MCP 이름만 읽어 해당 서버를 비활성화한다. 설정 파일에 쓰지 않는다. 실제 thread 시작 응답의 instruction_sources는 빈 배열이었다. MCP 이름은 현재 영숫자·밑줄·하이픈만 지원하며 다른 형식이면 시작 전에 오류로 중지한다. [설정 참고](https://learn.chatgpt.com/docs/config-file/config-reference).

이는 제공자의 전체 모델 요청 내용을 감청해 검증한 결과는 아니다. 공식 CLI의 내장 프롬프트·환경 정보·관리자 정책·자체 재시도·대화 압축 등은 남을 수 있다. 앱 추가 호출이 없다는 것과 제공자 내부 추론 호출이 항상 정확히 한 번이라는 것은 다르다.

## 저장·출력·취소

- `var/session-*.txt`: 전송 입력과 화면용 응답 전체. 미전송 초안은 저장하지 않는다.
- `*.txt.provider.json`: 제공자, 작업 디렉터리, 원격 대화 ID, 인증 종류/플랜, 상태. 인증 비밀은 저장하지 않는다.
- `*.turn-*.protocol.jsonl`: CLI에서 받은 원시 프로토콜 바이트. 화면에 생략한 도구 이벤트도 여기 보관한다. 중간 실패 시 마지막 프레임은 불완전할 수 있다.
- `*.stderr.log`, `*.bridge-stderr.log`: 제공자/브리지 진단. 대화 내용·작업 경로가 포함될 수 있다. 브리지 생성 파일은 소유자만 읽고 쓰도록 만든다.

Python 브리지는 원시 데이터를 디스크에 먼저 쓰고 최대 4 MiB JSONL 프레임을 해석한다. Emacs 필터도 화면용 문자열을 즉시 디스크에 쓰며, 50ms 타이머는 제한된 최신 구간만 읽는다. 무제한 메모리 대기열은 없다. 빠른 출력으로 화면이 건너뛴 내용도 디스크에는 남는다. 출력 상한 65,536 문자와 과거 페이지 상한, 출력 undo 비활성·입력 undo 유지가 적용된다.

프롬프트는 최대 1 MiB UTF-8다. 입력 버퍼/undo와 공식 CLI 자체의 메모리 사용량은 이 상한에 포함되지 않는다. 브리지는 프레임 크기를 제한하지만 Python/Emacs/CLI 전체 RSS가 고정되는 것은 아니다. 디스크 용량 자동 정리, fsync, 비정상 전원 종료 시 무손실은 보장하지 않는다.

취소는 브리지에 SIGTERM을 보내고 브리지가 공식 CLI와 같은 프로세스 그룹의 자식을 종료한다. Emacs 종료에 대비해 SIGHUP도 처리한다. 완료되지 않은 턴을 몰래 이어가지 않도록 취소·실패 시 backend_id를 비우며 다음 전송은 새 원격 대화다. 재연결 자동 반복은 없다. 정상 후속 전송은 같은 대화를 유지하지만 Emacs 재시작 후 원격 재개 UI는 아직 없다. 파일을 다른 프로세스에서 동시에 수정하는 것도 지원하지 않는다.

## 실제 결과와 자동 테스트

연결 확인 프롬프트는 도구 없이 확인 단어 `별빛`을 기억시키고, 다음 전송에서 단어만 묻는 방식이었다. Claude·Codex 모두 첫 턴 `확인`, 다음 턴 `별빛`을 반환했다.

- Claude: 최초 설정으로 2턴 성공, 스킬/플러그인 목록 억제를 보강한 최종 설정으로 2턴 성공. 총 4턴.
- Codex: 첫 프로세스는 MCP 비활성화 인자의 잘못된 이름 인용 때문에 모델 요청 전 종료했다. 이를 수정한 뒤 2턴 성공. 총 2턴.
- 원시 증거는 로컬 `var/live-check/`, `var/live-check-final/`에 보관한다. 개인 대화 ID와 경로가 포함돼 보고서에 전체 원문을 복사하지 않았다.
- Python unittest 5개: 두 제공자의 분할 UTF-8·완료 이벤트 중복 제거·원문 전송·재개, EOF/4 MiB 초과, 취소 시 하위 프로세스 종료, API 인증 거절, 환경 격리.
- Lisp byte compile: 경고를 오류로 처리한 검사 통과. 오래된 코드가 로드되지 않도록 생성된 `.elc`는 검사 후 제거했다.
- ERT 8개: 기존 6개와 실제 파이프 기반 가짜 CLI 전송/후속 전송/종료 및 디스크 기반 출력 폭주/과거 화면 고정/live 복귀 테스트. 출력 undo와 입력 undo, 타이머·stderr 프로세스 정리도 확인.

`bash scripts/test.sh`는 **GUI를 띄우지 않고 실제 AI도 호출하지 않는다.** Python fixture는 실제 subprocess/pipe 경계만 재현한다. 실제 CLI 확인은 별도 수동 실행이었으며 테스트 스크립트에 자동 네트워크 호출을 넣지 않았다.

## GUI 판정과 남은 범위

사용자는 앞선 GUI 시험 후 “별 문제 없는 것 같아”라고 보고했다. 이 관찰을 반영하되 모든 한글 조합·스크롤 항목을 계측 통과했다고 기록하지 않는다. Computer Use의 합성 키 입력 문제와 실제 키보드 IME는 [별도 조사](input-investigation.md)에 구분돼 있다.

이번 실제 연결은 브리지에서 검증했고 Emacs와 브리지 사이의 파이프는 가짜 CLI로 자동 검증했다. **실제 AI 응답 중 물리 키보드 한글 조합과 장시간 다중 CLI 세션 RSS/GUI 지연은 아직 측정하지 않았다.** 이전 가짜 스트림 성능 수치를 실제 Claude/Codex 프로세스 성능으로 대신하지 않는다.

현재 쓰기 도구·승인 UI가 없어 읽기와 질의 응답까지만 가능하다. 다음 구현 범위는 명시적 파일 변경 승인, 실행 중 요청/도구 상태 표시, 재시작 후 원격 대화 재개다.
