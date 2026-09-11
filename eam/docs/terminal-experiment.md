# 일반 초안 버퍼 + Ghostel 터미널 실험

2026-09-08. 기존 JSON 연결과 별도의 실험이다. 공식 CLI를 대화형 PTY에서 계속 실행하며 기본 입력은 터미널에서, 필요한 편집은 일반 Emacs 버퍼에서 한다. 매 턴 프로세스를 다시 시작하거나 대화 ID를 직접 복원하지 않는다. CLI의 화면·승인·슬래시 명령 처리를 그대로 사용한다.

## 터미널 선택

[Ghostel](https://github.com/dakra/ghostel)은 Ghostty의 libghostty-vt를 Emacs 네이티브 모듈로 연결한다. Ghostty 앱의 GPU 창을 그대로 내장하는 것은 아니다. Emacs 버퍼 표시와 네이티브 터미널 상태 처리를 결합한다. [vterm](https://github.com/akermu/emacs-libvterm)은 libvterm 기반이며, 입력을 별도 버퍼에 두면 vterm에서도 같은 실험 방향이 가능하다. 이번에는 사용자가 언급한 Ghostty 계열인 Ghostel부터 검증했다. 내장 term으로 작성 중이던 코드는 Ghostel 연결로 교체했고 term 테스트를 완료했다고 기록하지 않는다.

Ghostel v0.53.0을 `var/deps/ghostel`에 원본 그대로 설치했다. 우리 앱은 포크가 아니라 의존성으로 사용한다. 공식 릴리스의 Apple Silicon 모듈 SHA256:

`6e4a509c23fe6c39e90610dd17cb2543622436898f86daf80f47cde523240b9d`

`setup-ghostel.py`는 이 값을 확인한다. 현재 설치 스크립트는 Apple Silicon macOS용이다. Emacs for Mac OS X 30.1의 동적 모듈 지원 환경에서 로드했다. 개인 init.el이나 패키지 설치 디렉터리는 변경하지 않았다.

## 사용법

```sh
cd ~/workspace/emacs-ai
bash scripts/terminal-gui.sh
```

별도 `Emacs AI Terminal Test` GUI가 열리고 처음에는 가짜 CLI가 실행된다. 실행할 때마다 최신 코드를 로드한 새 시험 인스턴스를 연다. 패키지 다운로드가 필요한 첫 실행에는 네트워크를 사용하지만 가짜 CLI는 AI를 호출하지 않는다.

1. `Terminal Test → Start Claude CLI` 또는 `Start Codex CLI`를 선택한다.
2. 초기 로그인·신뢰 확인과 CLI 초기화가 끝나면 **터미널에 직접 입력**한다. 초안 버퍼는 자동으로 만들거나 표시하지 않는다.
3. 작성 중인 입력을 Emacs에서 편집하려면 `Terminal AI → Edit CLI input in Emacs`를 선택한다.
4. CLI가 임시 파일로 전달한 입력이 같은 Emacs의 일반 편집 버퍼에 열린다. 편집하고 저장한 다음 **`C-x #`** (`server-edit`)를 누른다.
5. 수정본이 CLI 입력란으로 돌아온다. 내용을 확인하고 터미널에서 Enter로 제출한다. 외부 편집 종료 자체는 AI 제출이 아니다.

이 동작은 CLI의 외부 편집기 단축키 Ctrl-G를 Ghostel 키 API로 전송한다. Emacs의 Ctrl-G는 취소 키로 처리될 수 있으므로 메뉴 명령을 권장한다. 공식 기능 설명: [Claude](https://code.claude.com/docs/en/interactive-mode), [Codex](https://learn.chatgpt.com/docs/codex/cli). 이 앱에서 새로 시작한 CLI의 EDITOR/VISUAL만 해당 Emacs의 emacsclient 명령으로 설정한다. 이미 실행 중인 이전 CLI는 재시작해야 새 환경을 받는다.

별도로 빈 초안을 쓰려면 `Terminal AI → Open draft`를 선택한다. `Transfer draft (no Enter)`로 CLI 입력란에 추가한 다음 `Enter / confirm`으로 제출한다. **수동 transfer는 기존 입력을 대체하지 않으므로**, 작성 중인 CLI 입력을 수정할 때는 위의 외부 편집기 왕복을 사용한다. 초안 버퍼를 닫아도 CLI는 계속 실행되며 다시 열 수 있다.

`Copy selection to draft`는 터미널에서 선택한 텍스트를 보조 초안 끝에 복사한다. 기존 초안을 지우지 않으며 CLI에도 삭제 키를 보내지 않는다. 선택 영역에는 응답이나 프롬프트 기호가 포함될 수 있으므로, 정확한 입력 원문을 가져오는 외부 편집기 기능과 구분한다.

방향키·Escape·Ctrl-C도 Terminal AI 메뉴로 보낼 수 있다. 보조 초안의 `C-c C-c`는 옮기기만 하고 `C-c C-s`가 Enter다. 여러 줄 수동 transfer는 bracketed paste가 활성화됐을 때만 가능하다. 수동 전달 상한은 64 KiB이며 ESC 등 제어 문자를 거절한다. 외부 편집기로 열린 파일과 초안의 undo는 유지한다.

터미널 버퍼의 Ghostel 기본 키에서는 `C-c C-c`가 Ctrl-C 전송이고, `C-c C-t`가 복사 모드, `C-c C-j`가 기본 터미널 입력 모드 복귀다. 같은 `C-c C-c`라도 보조 초안에서는 전송이라는 점을 구분한다. 복사 모드에서 `Terminal AI` 메뉴가 사라지는 앱 결함은 수정했으며, [모드 전환 검사와 GUI 확인 한계](startup-and-churn.md)를 기록했다.

일반 프로젝트에서는 `M-x emacs-ai-terminal-claude` 또는 `emacs-ai-terminal-codex`로 작업 디렉터리를 고를 수 있다. 테스트 메뉴의 Korean sample, /model, /goal은 보조 초안에 예제 문자열만 넣는다.

### 세션 전환·재개·종료

같은 Emacs에서 이미 열어 둔 CLI로 이동하려면 `C-x b`로 해당 `*Claude terminal*` 또는 `*Codex terminal*` 버퍼를 선택한다. 새로운 CLI를 다시 시작하는 동작과 구분한다. CLI/Emacs를 종료한 뒤에는 같은 작업 디렉터리에서 원하는 제공자를 시작하고 CLI의 `/resume` 목록에서 대화를 선택한다. 두 제공자의 서로 다른 대화 왕복과 새 Emacs에서 ID 재개는 [실제 검증](cli-validation.md)에서 확인했다. 같은 Codex 대화를 다른 CLI가 열고 있으면 active writer 오류가 날 수 있으므로 그 대화를 쓰는 기존 CLI부터 정상 종료한다.

종료 전 Claude는 `/tasks`, Codex는 `/ps`로 관리 중인 작업을 확인한다. Claude `/exit`에서 작업 종료를 원하면 `Exit and stop tasks`를 선택한다. Codex 작업 중단은 `/stop`, 정상 종료는 `/quit`다. [정상 종료 검증](cli-normal-exit.md)에서 시험 작업 정리를 확인했지만, 앱의 `Close experiment`나 Emacs 강제 종료는 같은 종료 절차가 아니다. CLI 종료 후 남은 출력 버퍼를 Close로 닫는 순서를 권장한다.

## 실행과 기록

입력 버퍼 → Ghostel의 paste/key API → PTY → 대화형 공식 CLI. 출력은 PTY → 우리 파일 기록 필터 → Ghostel의 libghostty-vt 상태 처리 → Emacs 터미널 버퍼 순서다. Python AI 브리지·JSON 이벤트 변환·앱의 대화 재개 로직은 사용하지 않는다.

원시 출력을 빠짐없이 기록하기 위해 Ghostel의 **Emacs 소유 PTY** 옵션을 사용한다. 기본 네이티브 PTY 스레드 경로와 성능이 같다고 가정하지 않는다. 출력 파싱은 수신 시 처리하고 화면 재표시는 Ghostel의 배치 갱신에 맡기며 타이머 간격을 50ms로 지정했다. 모든 프레임의 실행시간이 50ms 이하라는 보장은 아니다.

- `var/terminal-*.ansi`: 수신한 원시 터미널 출력. ANSI 커서 이동·지우기·색상 코드 포함.
- `*.ansi.input.jsonl`: 앱에서 전달한 초안과 메뉴 키. 터미널에 직접 누른 모든 키를 기록하는 키로거는 아니다.
- 스크롤백 엔진 예산: 세션당 128 KiB. **Emacs 버퍼 128 KiB 상한이나 기존 65,536문자 상한과 같은 값이 아니다.** 현재 화면·문자 속성·엔진 부가 구조는 별도다.
- 출력 undo 비활성. 입력 버퍼와 undo의 메모리는 제한하지 않는다.
- `Open raw archive (bounded pages)`는 기존 제한 페이지 뷰어로 원시 파일을 연다. 전체 파일을 버퍼에 넣지 않고 ANSI도 실행하지 않는다. 과거 터미널 화면을 재구성하는 재생기는 아직 없다.

일반 텍스트 transcript와 달리 ANSI 파일은 화면 덮어쓰기를 포함한다. 완전한 대화 문장 목록이나 보기 좋은 과거 화면을 제공하는 문제는 추가 작업이다. 파일 쓰기는 동기식이며 디스크 오류 시 해당 CLI를 중지한다. 디스크 용량 제한·전원 장애 시 fsync 보장은 없다.

이 경로는 공식 CLI의 기본 권한·설정·모델·스킬을 유지한다. 이전 JSON 어댑터의 읽기 전용 제한이나 스킬 비활성화 옵션을 적용하지 않는다. 앱이 프롬프트·컨텍스트를 추가하지 않아도 CLI 자체가 기존 설정과 스킬을 읽을 수 있다. CLI가 요청하는 도구 승인·파일 수정은 CLI 정책에 따른다. API 키 환경 변수 4개만 하위 환경에서 제거하지만 이 실험에는 이전 브리지의 인증 종류 사전 거절 검사가 없으므로 실제 CLI 화면에서 계정을 확인해야 한다.

## 검증 결과

- ERT 1개 통과: 실제 PTY 가짜 CLI에서 한글·이모지·여러 줄 원문 수신, paste와 Enter 분리, `/model` 문자열 전달, 입력 undo, 출력 undo 비활성, 제어 문자 거절, 기록 생성 및 프로세스 종료.
- 새 Lisp 파일은 경고를 오류로 처리한 byte compile 통과.
- Computer Use GUI: 가짜 CLI의 `PASTED 한글🙂 첫 줄 | 두 번째 줄`과 별도 ENTER 표시 확인.
- Computer Use GUI, Claude Code 2.1.263: 실제 초기 화면, `/model` 선택창, 방향키 이동과 Escape, `/goal`의 `No goal set` 상태창 확인. 모델 변경 확정이나 목표 실행은 하지 않았다. [goal 화면 텍스트](terminal-evidence/claude-goal.txt).
- Computer Use GUI, Codex 0.153.4: 디렉터리 신뢰 확인 화면까지 확인했다. GUI의 신뢰 선택은 남겨두었다.
- Codex 별도 PTY 배치 검사: 우리가 만든 테스트 디렉터리에만 프로세스별 trusted 설정을 전달하고 초기화 완료 후 `/model` 선택창을 확인했다. 개인 Codex 설정 파일의 신뢰 항목을 변경하지 않았다. 이 검사는 실제 GUI 조작 성공과 구분한다. [모델 선택창 텍스트](terminal-evidence/codex-model-batch.txt).
- Codex 초기화 도중의 첫 시도는 `Model selection is disabled until startup completes`로 거절됐다. 화면 준비 후에는 Enter 한 번으로 선택창이 열렸다.
- 이 단계에서는 AI에게 질문을 제출하거나 목표를 실행하지 않았다. CLI 자체의 시작 네트워크·기존 MCP 시작은 발생할 수 있다.

메뉴로 넣은 한글 문자열은 실제 물리 키보드의 IME 조합 검증이 아니다. Ghostel의 접근성 텍스트에서는 일부 화면 끝에 비표시 문자가 관찰됐고 스크린샷에서는 해당 문자가 보이지 않았다. 접근성 도구의 텍스트만으로 모든 렌더링 품질을 판정하지 않았다.

## 가짜 출력 부하 측정

동일 Emacs batch 프로세스에서 1/4/12세션 순서로 실행했다. 세션당 한글 80자짜리 10줄 청크를 600번 **기록 필터에 직접 주입**했다. PTY 처리량이나 실제 AI 속도 측정이 아니다. 200청크 후 GC/RSS, 600청크 후 각 버퍼를 표시 대상으로 바꿔 강제 재표시 후 GC/RSS를 측정했다. [원시 결과](terminal-benchmark.json).

| 세션 | 디스크 총 바이트 | 가장 큰 출력 버퍼 문자 | RSS KiB: 200청크 → 600청크+표시 | 필터 p95 / 최대 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1,452,000 | 31,612 | 49,152 → 49,744 | 0.309 / 2.254 |
| 4 | 5,808,000 | 31,612 | 54,672 → 56,880 | 0.332 / 9.291 |
| 12 | 17,424,000 | 31,612 | 65,552 → 70,224 | 0.235 / 10.368 |

필터 시간은 파일 쓰기와 VT 파싱을 포함하지만 GUI redisplay·IME 지연은 포함하지 않는다. 이전 일반 버퍼 벤치마크와 데이터·표시 시점이 달라 직접 우열 비교에 쓰면 안 된다. vterm과 동일 조건 비교, 장시간 누수, 이미지 출력, 실제 다중 CLI 메모리, 리사이즈·스크롤·한글 조합은 추가 검증 대상이다. 이 결과로 Ghostel이 vterm보다 가볍다고 결론 내리지 않는다.

```sh
bash scripts/test-terminal.sh # 설치 후, AI 호출·GUI 없음
/Applications/Emacs.app/Contents/MacOS/Emacs --batch -Q -L lisp \
  -l tests/terminal-benchmark.el > docs/terminal-benchmark.json
```

## 직접 입력 기본값과 외부 편집기 왕복 — 02:06 KST 추가 검증

새 세션은 터미널을 선택하고 보조 초안을 생성하지 않는다. `Open draft`에서 지연 생성하며, 초안 종료는 CLI 종료와 분리했다. Emacs에 서버가 없으면 프로젝트의 `var/editor-server` 아래에 프로세스별 이름의 로컬 Unix 소켓 서버를 만든다. 기존 서버가 있으면 그 서버를 사용한다. TCP 서버는 이 실험에서 지원하지 않는다. 개인 설정 파일을 수정하거나 다른 Emacs GUI를 실행하지 않는다.

가짜 PTY ERT를 확장해 기본 터미널 선택, 초안 지연 생성, 실제 emacsclient로 입력 파일 열기·수정·반환, 초안 종료 후 CLI 생존과 재열기를 확인했다. Lisp byte compile도 경고 없이 통과했다.

실제 Claude 2.1.263과 Codex 0.153.4의 PTY **배치 검사**에서 `한글 초안 첫 줄\n둘째 줄🙂`을 CLI 입력란에 붙여넣고 외부 편집기로 같은 원문을 받았다. `편집한 한글🙂\n돌려보낸 둘째 줄`로 수정한 뒤 두 CLI 입력란에 반환되는 것을 확인했다. Codex는 다시 외부 편집기를 열어 반환된 원문이 정확히 일치하는 것까지 확인했다. AI에 프롬프트를 제출하지 않았다.

Claude 첫 검사는 EDITOR 명령의 `--alternate-editor\=false`를 CLI가 그대로 인자로 전달해 실패했다. `-a false` 형식으로 수정한 뒤 왕복에 성공했다. Codex의 batch 터미널 화면에는 외부 편집기 종료 후 일부 이전 안내 문자가 남아 보였지만, 다시 가져온 입력 원문은 정확했다. 최신 GUI에서 이 왕복 동작과 화면 정리를 직접 확인하는 것은 별도 검증 대상이다.

이 경로는 텍스트 입력 원문을 CLI가 직접 전달한다. 화면 스크래핑으로 숨겨진 입력이나 줄바꿈을 추측하지 않는다. 이미지 첨부·파일 칩·CLI별 비텍스트 입력 상태의 왕복은 검증하지 않았다. CLI의 키 바인딩이나 외부 편집기 설정을 사용자가 변경했다면 동작이 달라질 수 있다.

## 실제 코딩과 세션 재개 — 후속 검증

두 CLI의 파일 수정·테스트·새 Emacs에서 ID로 재개·추가 수정과 중단을 검증했다. 정상 CLI 종료 시 출력 버퍼가 삭제되는 문제를 수정했다. Claude GUI 편집 왕복·스크롤·리사이즈는 확인했고 Codex GUI 최종 복귀는 도구의 창 접근 실패로 미완료다. [검증표·증거·한계](cli-validation.md)를 최신 결과로 참고한다.

11:57 KST 후속: Codex GUI의 최종 입력 복귀·원문 재확인·스크롤·리사이즈도 통과했다. Claude 쓰기 승인 거절과 이후 메뉴 응답, 기록 실패의 세션 격리 ERT를 추가 확인했다. 접근성 트리와 실제 스크린샷의 불일치, 한글 입력 소스에서 Computer Use 합성 키 문제는 별도 제한으로 기록했다. 최신 표는 [후속 검증 기록](cli-validation.md)을 참고한다.

세션 재개 후속: 두 CLI의 `/resume` 목록에서 서로 다른 대화 왕복을 실제 PTY로 확인했다. Codex는 같은 대화를 다른 CLI가 열고 있으면 active writer 오류를 반환했으며, 해당 시험 CLI를 닫은 뒤 재개에 성공했다. Codex 승인 거절·후속 메뉴 응답도 확인했다. [상세 증거](cli-validation.md).

## 실행 중 종료의 알려진 제한

실제 Claude·Codex 모두 CLI 강제 종료 뒤 시험 명령이 계속 실행되는 경우를 확인했다. **Close나 CLI 종료를 모든 하위 작업 종료로 해석하면 안 된다.** [시험 조건·정리 결과·사용 시 의미](terminal-exit-limits.md)를 참고한다.
