# Ghostel 기본 CLI 검증

2026-09-08. 새 기능보다 기존 CLI의 파일 수정·명령 실행·세션 재개를 먼저 검증하자는 사용자 요청(R15)에 따른 결과다. Emacs for Mac OS X 30.1, Ghostel v0.53.0, Claude Code 2.1.263, Codex 0.153.4에서 수행했다. 아래 결과는 이 시나리오의 통과이며 CLI의 모든 기능을 보증하지 않는다.

## 실제 CLI / PTY 검사

실제 공식 대화형 CLI를 Ghostel의 Emacs 소유 PTY에서 실행했다. JSON 어댑터는 사용하지 않았다. 배치 Emacs의 같은 터미널 코드에 paste/key를 전달하고 실제 파일·프로세스·원시 출력을 확인했다. 이 검사는 GUI 키보드 조작 성공과 구분한다.

| 항목 | Claude | Codex |
| --- | --- | --- |
| 기존 파일 읽기·수정 | 통과 | 통과 |
| 테스트 파일 신규 작성·명령 실행 | 3개 테스트 통과 | 3개 테스트 통과 |
| CLI 종료 후 새 Emacs/CLI에서 ID로 재개 | 통과 | 통과 |
| 파일에 저장하지 않은 이전 대화 확인 단어 회상 | 통과 | 통과 |
| 재개한 대화에서 추가 수정·테스트 실행 | 4개 통과 | 4개 통과 |
| 독립적으로 변경 결과 테스트 | 4개 통과 | 4개 통과 |
| 도구 승인 | 파일 수정·생성·셸 실행을 개별 승인 | workspace-write 정책으로 수행; 강제 승인 화면은 미검증 |
| 실행 중단 | Escape 후 실행 자식 종료 | Escape 후 백그라운드 자식 유지, `/stop` 후 종료 |
| 정상 CLI 종료 후 출력 보존 | 수정 후 통과 | 수정 후 통과 |

작업 디렉터리는 `var/cli-validation/20260908/{claude,codex}/workspace`다. 초기 `greet.py`를 한글 인사·앞뒤 공백 제거·빈 이름 오류 처리로 수정하게 하고 `test_greet.py`를 작성하게 했다. 첫 대화에서 확인 단어 `별빛-731`을 대화에만 기억하도록 요청했다. 프로세스를 종료하고 새 Emacs에서 네이티브 세션 ID로 재개한 뒤, 확인 단어를 다시 제공하지 않고 회상과 None 입력 처리 추가를 요청했다. 두 CLI 모두 단어를 회상하고 네 번째 테스트를 추가했다. Python 파일에 단어가 없는 것도 확인했다.

Claude는 테스트 프로세스에 `--permission-mode manual`을 지정했다. 화면의 실제 diff와 명령을 확인하고 해당 작업에만 Yes를 선택했다. Codex는 우리가 작성한 테스트 디렉터리만 프로세스별 trusted 설정으로 지정하고 `--sandbox workspace-write --ask-for-approval on-request`를 사용했다. 개인 CLI 설정 파일이나 개인 Emacs 설정은 수정하지 않았다. CLI 자체 설정·스킬·시작 동작은 유지한다.

재개 ID는 각 제공자 폴더의 `session-id.txt`, 화면과 상태는 `*-completed.txt`, `exit-state.json`, 전체 원시 출력은 해당 JSON의 `raw_file`에 보관했다. 계정 정보가 포함될 수 있는 전체 로그는 `var/`에만 둔다. [독립 재검사 결과](cli-validation-evidence/checks.json)는 계정 정보 없이 보관했다. 원시 출력의 확인 단어 존재 여부와 실제 관찰을 함께 근거로 삼는다.

## 중단의 차이

시험용 `wait_probe.py`는 시작 PID를 기록하고 180초 뒤에만 완료 파일을 만든다. Claude는 Escape 후 자식 프로세스가 사라지고 완료 파일이 없었으며 CLI는 계속 살아 있었다. Codex는 Escape 후에도 자식이 남고 화면에 background terminal 안내가 표시됐다. `/stop` 전송 후 자식이 사라지고 완료 파일이 없었으며 CLI는 살아 있었다.

Codex의 첫 60초 시험은 `/stop` 확인 전에 자연 완료되어 중단 성공 근거로 쓰지 않았다. 180초로 다시 시험하여 Escape 이후 자식 생존과 `/stop` 이후 종료를 각각 확인했다. 따라서 **Codex의 Escape를 모든 실행 명령의 종료로 해석하면 안 된다.** 이것은 관찰한 CLI 동작이며 앱에서 임의로 전체 프로세스를 죽이는 기능을 추가하지 않았다.

## 발견한 문제와 수정

Ghostel 기본값은 프로세스 종료 시 터미널 버퍼도 삭제한다. 실제 `/exit`·`/quit` 검사에서 마지막 화면이 사라지고 검증 코드에서 `Selecting deleted buffer`가 발생했다. 앱이 만든 터미널에만 `ghostel-kill-buffer-on-exit`을 nil로 설정했다. 이제 정상 종료 후 마지막 출력과 종료 표시를 확인할 수 있다. 명시적인 Close는 기존처럼 정리한다.

가짜 PTY ERT에 실제 자식 종료 후 출력 버퍼 생존, 종료 문구, 출력 undo 비활성 검사를 추가했다. 제품 코드는 이 수정에 집중했으며 세션 관리자 등 새 기능을 추가하지 않았다.

## 실제 GUI 관찰 — Computer Use

별도 `Emacs AI Validation` 앱에서 `tests/cli-validation-gui.el`을 로드했다. 검증용 메뉴는 저장된 시험 세션을 재개하고 합성 한글 문자열을 넣는 도구다.

- Claude: 재개된 대화 화면 확인. `한글🙂 첫 줄\n두 번째 줄`을 CLI에 옮긴 뒤 외부 편집기로 같은 원문을 가져왔다. `GUI 왕복 확인`을 추가하고 저장·반환한 결과를 CLI 입력란에서 확인했다. AI에 제출하지 않았다. [왕복 화면](cli-validation-evidence/claude-editor-return.jpg).
- Claude: 터미널 위로 스크롤하여 이전 기록과 하단 이동 안내가 나타나는 것, 창 크기를 바꾼 뒤 더 많은 기록이 표시되는 것을 확인했다. 작은 표본의 육안 관찰이며 프레임 지연 측정은 아니다. [스크롤](cli-validation-evidence/claude-scroll-history.jpg), [리사이즈](cli-validation-evidence/claude-resize.jpg).
- Codex: 저장된 ID로 실행하고 합성 한글 두 줄을 외부 편집 버퍼로 정확히 가져오는 것까지 확인했다. 이후 Computer Use가 `cgWindowNotFound`를 반환했고 앱 ID로 재시도해도 같았다. **최종 GUI 반환 화면·스크롤·리사이즈는 미완료**다. 앞선 실제 PTY 배치 왕복 성공으로 GUI 성공을 대신하지 않는다.

초안에서 외부 편집을 요청하면 저장·반환 후 선택 창이 초안으로 돌아올 수 있다. `Focus terminal`로 CLI의 반환된 입력을 확인했다. 합성 문자열 삽입은 물리 키보드의 한글 조합 검증이 아니다.

## 회귀 검사와 재현

- `bash scripts/test.sh`: Python 5개 + ERT 8개 통과.
- `bash scripts/test-terminal.sh`: 확장한 실제 가짜 PTY ERT 1개 통과.
- 터미널 Lisp: 경고를 오류로 처리한 byte compile 통과. 생성한 elc는 제거해 다음 GUI에서 소스를 로드한다.
- 실제 CLI가 수정한 결과: 제공자별 unittest 4개를 독립 실행해 모두 통과.

자동 회귀 스크립트는 실제 AI를 호출하지 않는다. 실제 검증 도구 `tests/cli-validation-driver.el`과 `tests/cli-validation-control.py`는 기본 테스트에 포함하지 않았다. driver는 `EMACS_AI_VALIDATION_DIR`, `EMACS_AI_VALIDATION_PROVIDER`, 선택적 `EMACS_AI_VALIDATION_SESSION_ID` 및 `EMACS_AI_VALIDATION_RESUME`을 읽는다. Claude의 RESUME은 플래그, Codex의 RESUME은 세션 ID다. control의 paste는 제출하지 않으며 key return을 따로 사용한다. 재현 시 같은 폴더의 증거를 덮어쓰므로 새로운 시험 폴더를 준비한다.

GUI 도구는 `python3 scripts/prepare-gui-test.py --validation`로 준비한다. 이 명령은 앱을 준비하고 경로를 출력하며, 실제 GUI 실행은 별도다. 검증 메뉴의 Resume은 로컬 시험 ID가 필요하다. 제품의 세션 찾기 UI가 완성됐다는 뜻은 아니다.

이번 실제 사용자 프롬프트 제출은 Claude 3회, Codex 4회다. Codex 중단 재시험 1회를 포함한다. GUI 편집 시험에서는 추가 질문을 제출하지 않았다. 이 횟수는 제공자 내부의 모델 요청·도구 호출·MCP 시작 횟수와 다르다.

## 남은 검증

1. Computer Use 창 접근 복구 후 Codex GUI 왕복의 최종 화면과 스크롤·리사이즈.
2. 물리 키보드 한글 조합·수정·직접 터미널 입력. [사용자 확인 절차](keyboard-handoff.md).
3. 승인 거절 후 복구, 세션 목록에서 선택하는 네이티브 재개 UX, 모델 변경 확정과 후속 동작. 기존 `/model` 선택창·`/goal` 상태창 확인은 전체 명령 검증이 아니다.
4. 실제 여러 CLI의 장시간 RSS·응답성, 비정상 종료·디스크 실패, 비텍스트 첨부와 백그라운드 작업의 다양한 종료 조건.

현재 단계는 **두 CLI의 작은 실제 개발 작업과 ID 기반 재개를 검증한 상태**다. 새로운 제품 기능보다 남은 기본 동작 검증을 우선한다.

## 후속 자율 검증 — 2026-09-08 11:57 KST

사용자는 큰 문제나 직접 개입이 필요한 경우 외에는 검토·검증·계획을 이어가도록 요청했다. 제품 기능 추가 없이 다음을 검증했다.

| 항목 | 결과와 증거 |
| --- | --- |
| Codex GUI 편집 왕복 | 재개한 CLI 입력을 일반 Emacs 버퍼로 가져와 한글 세 번째 줄을 추가하고 CLI에 반환. 다시 외부 편집기를 열어 세 줄 원문 일치 확인. [복귀 화면](cli-validation-evidence/codex-editor-return.jpg) |
| Codex 스크롤·리사이즈 | 위로 두 페이지 이동하여 이전 코드·테스트 로그 표시. 창 확대 후 더 많은 기록 표시. [스크롤](cli-validation-evidence/codex-scroll-history.jpg), [리사이즈](cli-validation-evidence/codex-resize.jpg) |
| Codex 네이티브 `/resume` | 현재 작업 디렉터리로 필터된 시험 대화 1개 표시. 선택 후 `Already viewing` 메시지와 입력란 복귀. 같은 세션 재선택이므로 서로 다른 세션 전환 성공으로 확대하지 않음. [목록](cli-validation-evidence/codex-resume-picker.jpg) |
| Claude 쓰기 승인 거절 | 시험 파일 `denied-check.txt` 쓰기를 한 번 요청하고 실제 승인 화면의 No 선택. `User rejected write to denied-check.txt` 표시, 실제 파일 부재 확인. [요청 화면](cli-validation-evidence/claude-denial-prompt.jpg) |
| 거절 후 CLI 복구 | 추가 AI 요청 없이 `/model` 선택창 정상 표시. 승인 대기 상태에 갇히지 않음. [거절 결과와 모델 메뉴](cli-validation-evidence/claude-after-denial-model.jpg) |
| 기록 실패와 세션 격리 | 새 ERT에서 두 실제 가짜 PTY를 시작. 한 세션의 archive 경로를 존재하지 않는 부모 아래로 바꾸어 쓰기 실패 유도. 그 PTY 종료·실패 청크 미표시·후속 Enter 거절을 확인. 다른 PTY는 계속 살아서 Enter 출력 기록. 통과 |

Codex의 이전 `cgWindowNotFound`는 이번 재시도에서 발생하지 않았다. 반면 접근성 트리가 초기 시작 화면을 반환하면서 스크린샷은 최신 대화·입력을 표시하는 차이가 재현됐다. 따라서 스크린샷과 외부 편집기의 실제 원문을 대조했다. 접근성 텍스트만으로 최신 출력이 없다고 판단하지 않는다.

검증용 Node 메뉴 선택 코드에서 부분 문자열 `Down`이 `ID: menuDown:`에도 일치하는 문제가 있었다. 실제 메뉴 이름을 ID·단축키와 분리해 완전 일치하도록 고쳤다. No 항목의 선택 표시를 스크린샷으로 확인한 뒤 Enter를 전송했다. 제품의 Down 명령 구현 결함은 아니었다.

Claude `/resume` 추가 GUI 시험에서는 Computer Use의 영문 합성 입력이 현재 한글 입력 소스에서 의도와 다르게 전달됐다. 일반 편집 버퍼에서도 재현했고 미전송 진단 문자열을 비운 뒤 CLI에 복귀했다. 이 항목은 미완료로 유지한다. 물리 키보드 IME의 품질과는 별도이며, 사용자 입력 소스 변경을 요구하지 않았다.

이번 추가 AI 프롬프트는 Claude의 쓰기 거절 시험 1회뿐이다. Codex 편집·스크롤·재개 목록, Claude 모델 메뉴에는 모델 작업 요청을 제출하지 않았다. 전체 누계는 Claude 4회·Codex 4회이며 제공자 내부 호출 수와 같지 않다.

`bash scripts/test-terminal.sh`는 2개 ERT 모두 통과했다. 첫 실행은 현재 실행 샌드박스가 Unix 서버 소켓 bind를 금지하여 두 테스트 모두 시작 단계에서 실패했다. 자동 승인된 샌드박스 밖 재실행은 통과했다. 네트워크 AI 연결 없이 로컬 PTY·Unix 소켓만 사용했다. 기존 13개 회귀 검사는 직전 단계에서 통과했으며 이번에는 변경한 터미널 테스트만 실행했다. 실제 디스크 가득 참·fsync 내구성·모든 오류 유형을 검증한 것은 아니다.

### 다음 실행 순서

1. GUI 합성 입력에 의존하지 않는 실제 PTY 검사로 서로 다른 세션 간 네이티브 선택·복귀와 Claude 재개 목록을 검증한다.
2. Codex 승인 요청을 명시적으로 발생시키는 작은 격리 시나리오에서 거절·복구를 검증한다. 개인 설정이나 광범위한 권한 변경은 하지 않는다.
3. 모델 변경은 시험 세션 범위와 설정 저장 동작을 확인한 뒤 변경·원복과 후속 요청을 검증한다.
4. 실제 CLI 다중 세션의 시간별 RSS·반응성과 비정상 종료를 측정한다. 짧은 합성 필터 벤치마크와 구분한다.

물리 키보드 IME만 사용자 직접 관찰 항목으로 유지한다. 그 외 검증을 계속 진행하는 데 현재 사용자 답변은 필요하지 않다.

## 세션 왕복과 Codex 거절 검증 — 2026-09-08 후속

실제 PTY 배치 검사로 두 CLI의 서로 다른 대화 간 왕복 전환과 Codex 승인 거절·복구를 완료했다. 새 결과는 `var/cli-validation/20260908-followup`에 분리했다. 개인 Emacs·CLI 설정 파일은 수정하지 않았다.

| 항목 | 확인 결과 |
| --- | --- |
| Claude 네이티브 목록 | 빈 새 CLI에서 `/resume` → greet 시험 대화 선택 → 과거 거절 이력 복원. [목록](cli-validation-evidence/followup/claude-resume-picker.txt) |
| Claude 서로 다른 대화 왕복 | `/clear` 후 짧은 응답 `SESSION_B_902`만 요청하여 대화 B 생성. `/resume`로 greet 대화 A 복원, 다시 `/resume`에서 B 선택하여 같은 응답 복원. [B 선택 목록](cli-validation-evidence/followup/claude-picker-for-b.txt), [B 복원](cli-validation-evidence/followup/claude-switched-to-b.txt) |
| Codex 승인 거절 | read-only·on-request·user 승인으로 새 시험 대화 실행. `printf`로 `denied-check-2.txt`를 쓰려는 실제 승인 화면에서 Escape 선택. 파일 미생성 확인. [승인 화면](cli-validation-evidence/followup/codex-denial-approval.txt) |
| Codex 거절 후 복구 | 취소 안내 후 입력란 복귀, `/model` 선택창 표시. 모델 변경은 취소. [거절과 후속 메뉴](cli-validation-evidence/followup/codex-after-denial-model.txt) |
| Codex 서로 다른 대화 왕복 | `/resume`의 두 항목에서 greet 대화로 전환 후 승인 거절 대화로 복귀. 각각 해당 대화 이력 확인. [두 대화 목록](cli-validation-evidence/followup/codex-two-sessions.txt), [A 전환](cli-validation-evidence/followup/codex-switched-to-a.txt), [B 복원](cli-validation-evidence/followup/codex-switched-to-b.txt) |
| 정상 종료 | 이번 배치 CLI 둘 다 native exit, archive error 없음. 마지막 화면·ANSI 기록 보존. [검사 결과](cli-validation-evidence/followup/checks.json) |

### 같은 Codex 대화의 동시 재개 제한

첫 전환은 `already has an active writer`로 거절됐다. 앞선 검증 GUI가 같은 greet 대화를 열고 있었다. 기존 개인 Emacs나 다른 대화를 종료하지 않고, `Emacs AI Validation`의 해당 Codex 시험 버퍼만 `Close experiment`로 닫았다. 이후 같은 목록 선택으로 전환에 성공했다. [최초 오류](cli-validation-evidence/followup/codex-active-writer-error.txt).

이 오류는 세션 기록 소실이 아니라 중복 writer 제한으로 관찰됐다. 재개 실패 뒤에도 현재 Codex는 입력을 받아 다시 목록을 열 수 있었다. 따라서 세션 재개 사용법에 같은 대화를 열고 있는 시험 CLI를 먼저 닫는 조건을 명시한다. 앱의 세션 관리자나 잠금 자동 해제 기능은 추가하지 않았다.

### 승인 정책과 해석

공식 문서에서 `approvals_reviewer=user`는 수동 승인 수신자를 지정하며 sandbox 정책과 별개임을 확인했다. [공식 승인 설명](https://learn.chatgpt.com/docs/agent-approvals-security), [설정 레퍼런스](https://learn.chatgpt.com/docs/config-file/config-reference). 테스트 driver에만 workspace 경로·sandbox 선택 환경 변수를 추가하고 수동 reviewer를 명시했다. 이는 제품 터미널 어댑터의 기본 CLI 설정을 변경하지 않는다.

Codex 화면에는 거절 직후에도 `Ran printf ... (no output)`라는 요약이 함께 나타났다. 이 문구만으로 파일 쓰기 실행 여부를 판단하지 않았다. 취소 안내와 실제 `denied-check-2.txt` 부재를 함께 확인했다. 이 시험은 명시적인 승인 요청 1건의 거절이며 모든 도구·정책 조합을 검증하지 않는다.

이번 요청은 Claude 1회(별도 대화 식별용 응답), Codex 1회(승인 거절 시험)다. 누계는 각각 5회이며 CLI 내부 모델 호출 수가 아니다. 재개와 메뉴 검사에는 추가 모델 작업 요청을 보내지 않았다.

### 갱신한 다음 순서

1. 모델 선택 변경이 저장되는 범위를 먼저 확인하고, 개인 기본 설정을 건드리지 않는 시험 방법으로 변경·원복·후속 응답 검증.
2. 같은 Emacs에서 실제 CLI 여러 개를 실행한 시간별 RSS·화면 크기·메뉴 응답 측정. 유휴 상태와 출력 부하를 구분하고 장시간 누수 결론을 짧은 측정으로 대신하지 않음.
3. 비정상 종료·하위 작업 정리와 큰 출력의 제한 기록 조회 확인.
4. 실제 키보드 IME는 사용자 관찰로 별도 확인. GUI 합성 타이핑의 입력 소스 문제와 구분.

재현 시 `EMACS_AI_VALIDATION_WORKSPACE`로 시험 cwd를, `EMACS_AI_VALIDATION_SANDBOX=read-only`로 Codex sandbox를 선택할 수 있다. 두 변수 모두 검증 전용이며 기본 회귀 테스트나 제품 시작 흐름에는 사용하지 않는다.

## 모델·다중 세션 후속 결과

모델 변경/원복 후 응답, 설정 내용 불변, 같은 Emacs의 실제 CLI 4개에서 337초·메뉴 44회 검사, 유휴 CLI 강제 종료 격리를 확인했다. [측정값·증거·한계](cli-model-and-soak.md)를 최신 결과로 참고한다.

## 출력 부하·실행 중 종료 후속

실제 PTY의 가짜 생성기 2개로 10분·약 90 MiB 기록 보존, 입력 40건, 제한 페이지를 검증했다. 반면 실제 Claude·Codex 모두 CLI SIGKILL 뒤 시험 명령이 생존했다. [출력 측정](terminal-stream-validation.md), [현재 종료 제한](terminal-exit-limits.md)을 최신 안정성 결과로 참고한다.
