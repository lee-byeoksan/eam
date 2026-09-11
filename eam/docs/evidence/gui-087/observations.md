# 0.8.7 GUI 관찰 — 2026-09-10

전용 앱 local.emacs-ai.app.gui087에서 개인 init 없이 설치본을 로드했다.
초기 Esc/x는 M-1 is undefined로 표시됐다. AX 클릭은 cannotClickOffscreenElement로 실패했다.
화면 좌표 (240, 200) 클릭 뒤 Esc/x는 정상 M-x를 열었다. 포커스가 모든 실패의 원인인지는 미확정이다.

M-x emacs-ai-help 문자열을 확인한 뒤 Return을 보냈다. 0.8.7 사용 안내가 열렸고 q로 시작 화면에 복귀했다.
M-x emacs-ai-new-session으로 가짜 입력/출력 분할도 확인했다. 메뉴는 사용하지 않았다.

한글 혼합 type_text는 Shell command 미니버퍼를 열어 Return 없이 Esc 3회로 취소했다.
좌표 클릭 뒤 paste도 clipboard-read timeout으로 실패했다. 후속 영문 대문자 입력은 comment 관련 미니버퍼를 열어 취소했다.
소문자 문자열도 기대대로 들어가지 않았다. 한글만의 문제라고 단정하지 않는다.
따라서 입력·undo·전송·스트리밍을 통과로 판정하지 않았다. 물리 한글 조합도 미검증이다.

새 AI 요청 0회. 개인 Emacs 조작 없음. 시험 창은 후속 진단을 위해 열어 두었다.
[설치 로그](package-install.log): 59개 ERT와 별도 프로세스 복원·패키지 수명 검사 통과.

## 원시 이벤트 진단

`tests/gui-input-events.el`을 별도 Emacs -Q에 로드했다. emacs-ai/Ghostel 패키지를
로드하지 않고 read-event로 받은 이벤트만 기록했다. 키를 명령에 전달하지 않아
잘못된 이벤트로 셸 명령을 실행할 수 없다. 180초 또는 128개 이벤트에서 종료한다.

`type_text("abc")`가 `M-ᆼ`, `M-ᅮ`, `M-ᅦ`로 기록됐다.
개별 press_key(a/b/c)는 조합 중간 상태를 포함해 처음 두 이벤트가 `ᆼ`, `ㅜ`로 기록됐다.
[원시 기록](raw-events.jsonl), [입력 소스](input-source.json).

macOS의 선택 입력 소스는 `com.apple.inputmethod.Korean.390Sebulshik`,
키보드 레이아웃은 `com.apple.keylayout.390Hangul`이었다. 설정은 읽기만 했다.
패키지 키맵 이전에 이벤트가 달라진다는 점은 확인됐지만, Computer Use의 이벤트 생성,
세벌식 IME 처리, Emacs 네이티브 입력 처리 중 어디의 결함인지는 아직 분리하지 못했다.
다음 대조는 사용자가 ABC 입력 소스로 전환한 상태에서 동일한 입력을 보내는 것이다.
제품에서 세벌식 설정을 강제로 바꾸거나 정상이라고 판정하지 않는다.

### 사용자 ABC 전환 후 대조

사용자가 ABC로 전환했음을 알렸고 defaults에서도 ABC를 확인했다.
새 원시 이벤트 GUI를 만들었지만 첫 type_text(abc)는 여전히 Meta+자모였다.
도구 JS 실행 세션을 초기화한 뒤 개별 a/b/c와 type_text(abc)가 모두 정상 ASCII 이벤트로
전달됐다. [원시 기록](raw-abc-events.jsonl), [대조 결과](abc-comparison.json).
입력 소스 변경과 초기화 외 시간·포커스 상태도 달라져 초기화만의 효과로 확정하지 않는다.

0.8.7 패키지 GUI에서는 입력 영역 클릭 후 abc 입력, M-x undo 실행으로 입력 제거를 확인했다.
클릭 없이 창/버퍼 전환 직후 보낸 명령은 실패한 경우가 있어, 이후 자동 GUI 시험은
화면을 확인하고 입력 영역 포커스를 확보해야 한다. 한글 paste는 여전히 clipboard-read
timeout이었다. 이 결과로 물리 한글 조합이나 Ctrl-/ 전달을 통과 판정하지 않는다.

## 키 입력과 독립적인 GUI 출력·스크롤

`tests/gui-scroll-fixture.el`로 0.8.7 설치본에서 가짜 스트림 4개를 시작했다.
초기화 코드로 시작했으므로 M-x/키보드 제출 시험으로 세지 않는다. 실제 AI 호출은 없다.
전용 앱은 `local.emacs-ai.app.scroll087`이다.

출력 중 텍스트 영역 좌표에 포커스를 주고 위·아래 스크롤을 수행했다.
AX의 스크롤 위치 변화와 창 시작점 표본(60807 → 11512 → 1)을 확인했다.
이는 1초 표본이므로 입력 지연의 정밀 측정이나 모든 GUI 흐름의 반응성 증명은 아니다.
빠른 출력으로 버퍼 앞부분이 잘리면서 읽던 오래된 내용도 밀려났다. 오래 읽을 내용은
제한된 디스크 기록 페이지로 조회해야 하며, 그 페이지 이동의 GUI 검증은 별도다.

125개 표본에서 4개 출력 버퍼 모두 최대 65,536자, undo 비활성화를 유지했다.
각 디스크 기록은 14,390,659바이트이며 마지막 표본의 모든 스트림 remaining은 0이다.
[표본](scroll-samples.jsonl), [요약](scroll-summary.json), [GUI 화면](stream-scroll.png).
물리 한글 조합·사용자 입력 undo·CLI 터미널 스크롤은 이 시험의 통과 범위에 포함하지 않는다.

## 붙여넣기 timeout의 후속 관찰

다음 관찰에서 `local.emacs-ai.app.gui087` 입력 버퍼에 정확히 `한글 입력 확인 abc 123`이
표시됐다. [화면](korean-after-timeout.png). 이는 앞서 timeout을 반환한 paste의 표본과 같다.
따라서 timeout은 실제 입력이 전혀 이루어지지 않았다는 증거가 아니다. 완료 시각이나
그 사이 다른 입력 여부를 계측하지 않아 지연 원인은 확정하지 않는다. 자동 재시도하면
중복 입력이 될 수 있으므로 상태를 먼저 확인한다. 물리 한글 조합의 통과 증거도 아니다.

이어 Codex 시작 명령을 준비했지만 M-x 미니버퍼가 열리지 않았고 버퍼에 `--`만 추가됐다.
Esc와 x를 별도 호출로 나누어 보내도 진입을 확인하지 못했다. Return으로 실행하지 않았고
실제 CLI 시작이나 AI 요청도 없었다. F01 GUI 검증은 이번 시도로 통과하지 않았다.
사용자에게 같은 시험 창에서 물리 Esc/x가 M-x를 여는지 확인을 요청했다.

## 사용자 직접 검증

사용자가 “Emacs AI GUI 0.8.7” 창에서 ABC 입력 소스, 입력 영역 클릭 후
Esc → x → emacs-ai-help → Return → q 순서를 직접 수행하고 **“모두 정상”**이라고 보고했다.
물리 키보드의 M-x 진입, 설치된 도움말 열기, q 복귀는 사용자 확인으로 통과했다.
이는 도구 입력에서 재현된 실패와 구분한다. 모든 터미널 단축키나 물리 한글 조합까지
확인한 것으로 확대하지 않는다. 운영체제·도구·Emacs 네이티브 입력 간의 정확한 원인은 미확정이다.

## 실제 Codex GUI 시작

M-x emacs-ai-codex → Project directory 미니버퍼를 실제 키로 열었다.
첫 경로 입력은 `~//-/`로 잘못 전달돼 제출하지 않았다. 미니버퍼 좌표 (200, 548)에
포커스를 두고 재입력했을 때 `~//Users/lee.byeoksan/workspace/emacs-ai/`가 표시됐다.
Emacs substitute-in-file-name이 이 입력을 프로젝트 절대 경로로 해석함을 확인한 뒤 Return을 보냈다.
Codex v0.154.0 시작 화면의 directory가 `~/workspace/emacs-ai`인 것을 확인했다.
[실제 시작 화면](codex-start.png). 신뢰/승인 선택이나 AI 요청 제출은 없었다.

세션 선택 명령은 후속 키 전달 실패로 미니버퍼 진입을 확인하지 못했다.
잘못 전달된 문자는 위쪽 가짜 입력 버퍼에만 추가됐으며 CLI에는 요청을 제출하지 않았다.
따라서 GUI 프로젝트 선택·Codex 시작은 확인했지만 F01 세션 목록/전환 전체는 아직 미검증이다.
새 Codex 터미널은 후속 시험을 위해 열어 두었다.

## F01 공통 키·후보 선택·취소

후속 시험에서 위쪽 가짜 입력 영역에 포커스를 두고 `C-c a s`로 `CLI session:`
미니버퍼를 열었다. Tab으로 다음 후보가 완성되는 것을 확인했다.

`*Codex terminal: ~/workspace/emacs-ai* | Codex | ~/workspace/emacs-ai/ | branch:— | process:run`

Return으로 해당 Codex 터미널로 전환됐다. 다시 가짜 입력 영역에서 같은 키로 목록을
열고 Tab을 누른 뒤 C-g로 취소했을 때 원래 가짜 입력 버퍼와 내용이 유지됐다.
[후보 화면](session-candidate.png).

실제 GUI에서 공통 키 진입, 단일 후보 정보 표시, 선택·이동, 취소를 확인했다.
이 시험은 이미 열린 CLI 하나만 사용했으며 새 CLI/AI 요청을 만들지 않았다.
여러 후보 사이 탐색, 긴 한글 경로 구분, 실제 Git 브랜치 표시, 물리 한글 completion은
이 단일 후보 결과로 통과 판정하지 않는다.

## 여러 후보 시험 준비

`tests/gui-selector-fixture.el`과 전용 앱 `local.emacs-ai.app.selector087`을 준비했다.
창 제목은 `Session selector GUI 0.8.7`이다. 실제 Git 저장소 3개를 시험 폴더에 생성하고
공통된 긴 한글·공백 경로의 끝을 Alpha/Beta/Gamma로 구분했다.
가짜 Python PTY만 실행하며 Alpha/Beta는 300초 뒤 종료, Gamma는 즉시 코드 7로 종료한다.
AI 요청은 없다.

시작 2초 후 실제 후보 함수 결과에서 세 경로, alpha/beta/gamma 브랜치, 두 run 및
exit(7)을 확인했다. [후보 데이터](selector-candidates.json). 이후 스크린샷에서 Beta의
exited(0) 표시도 관찰했다. 이는 가짜 프로세스의 유한 수명에 따른 종료다.

도구로 C-c a s 및 M-x를 시도했으나 선택창 진입을 확인하지 못했다. JS 실행 세션 초기화
후에도 같았다. 후보 데이터가 생성됐다는 사실을 GUI 목록 탐색의 통과로 세지 않는다.
사용자는 이 창의 아래쪽 참조 버퍼를 클릭하고 C-c a s → Tab → Tab으로 후보를 표시한 뒤,
C-g로 취소할 수 있다. 가짜 프로세스 종료 뒤에도 후보의 종료 상태와 브랜치를 비교할 수 있다.

## F01 여러 후보 사용자 확인 (2026-09-10)

사용자가 직전 안내의 전용 Session selector GUI 0.8.7 창에서 C-c a s → Tab 두 번,
Alpha/Beta/Gamma와 alpha/beta/gamma 브랜치 구분, C-g 취소를 수행하고
“모두 정상”이라고 보고했다. 긴 한글·공백 경로의 세 후보 표시와 브랜치 구분,
취소는 사용자 직접 확인으로 통과했다. 이는 물리 한글 조합 입력의 검증은 아니다.

## F02 알림 GUI 준비와 키 전달 한계 (2026-09-10)

설치본 0.8.7과 tests/gui-notifications-fixture.el로 Notifications GUI 0.8.7
전용 창을 열었다. Alpha/Beta 가짜 PTY가 기존 notification-fixture.py의 분할 UTF-8
OSC 9/777을 출력하고 종료한다. 실제 AI 요청과 OS 알림은 없다. 초기 목록 열기는
fixture 타이머가 수행했으므로 C-c a l 진입의 GUI 검증으로 세지 않는다.

스크린샷에서 NEW 네 행, Alpha/Beta 제목·세션과 한글 이벤트를 확인했다. 기본 폭에서는
긴 세션 경로와 본문이 잘린다. 좌표 클릭 후 도구 press_key r로 읽음 전환을 시도했으나
undefined 키 메시지가 나타났고 NEW가 유지됐다. 반복 입력은 중단했다.
[화면](notifications-key.png). 읽음 전환·RET 이동·다음 미읽음은 사용자 직접 확인 대기다.

## F02 사용자 키보드 확인 완료 (2026-09-10)

사용자가 Notifications GUI 0.8.7에서 안내한 다섯 단계를 수행한 뒤 “모두 정상”이라고
보고했다. r로 NEW/read 양방향 전환, RET로 해당 세션 이동, C-c a l로 목록 복귀 후
방문 행 read 확인, C-c a n으로 다른 미읽음 알림의 세션 이동을 사용자 확인으로 통과했다.
가짜 PTY 종료 표시는 정상이며 실제 AI 요청은 없다. 선택적 macOS 알림과 물리 한글
조합 입력은 이 결과에 포함하지 않는다.

## F03 GUI 준비 (2026-09-10)

tests/gui-review-fixture.el로 설치본 0.8.7의 Review GUI 0.8.7 창을 열었다.
example.py 한 줄 변경 diff와 즉시 종료하는 가짜 Review-test PTY를 준비했다.
[초기 화면](review-ready.png)에서 diff-mode의 여섯 줄을 확인했다.
영역 선택·의견 입력·취소·초안 undo는 사용자 직접 시험 대기다.
실제 AI/파일 변경 요청은 없으며 시험 파일과 기록은 var/gui-review-087에 격리했다.

## F03 Computer Use 직접 검증 완료 (2026-09-10)

기존 Review GUI 0.8.7 창을 조회한 뒤 마우스 드래그로 변경 두 줄(32자)을 선택했다.
C-c a v가 정상 진입했고 type_text로 입력한 Explain this change를 화면에서 확인했다.
Return → Tab으로 Review-test 후보를 확인한 뒤 Return으로 초안을 열었다.
[초안](review-draft.png): Old/New example.py:1, 정확한 의견, 선택한 두 줄만 포함한다.
전체 diff의 헤더는 선택 본문에 자동 첨부되지 않았다.

ctrl+slash로 초안이 빈 상태로 돌아오는 [undo](review-undo.png)를 확인했다.
다시 두 줄 선택 → C-c a v → C-g로 취소했고, diff로 복귀하며 빈 초안이 유지됐다
([취소](review-cancel.png)). 앱 입력 저널은 0바이트
([점검](review-input-audit.json)); 시험 중 CLI 전송 명령이나 실제 AI 요청은 없었다.

이 실행에서는 도구의 마우스·키·영문 입력이 정상 동작했다. 이전 전달 실패의 원인이
해결됐다고 단정하지 않는다. 물리 한글 조합과 Magit GUI는 별도 미검증이며, 일반
diff-mode GUI의 선택·의견·대상 선택·초안 확인·undo·취소는 직접 확인했다.

## F04 worktree 선택·파일 탐색 GUI 직접 확인 (2026-09-10)

var/gui-worktree-087에 로컬 Git 저장소(main)와 한글 작업 공간(gui-review) worktree를
준비했다. GUI 준비 데이터 생성은 shell Git으로 했으며, GUI 생성 명령 통과로 세지 않는다.
기존 시험 GUI에서 C-c a W o로 Repository 미니버퍼를 열고 ASCII 절대 경로를 입력했다.
Return → Tab 두 번으로 두 경로 및 refs/heads/main, refs/heads/gui-review 후보를 확인했다.
한글 후보를 더블클릭해 해당 Dired로 이동했고 example.txt에서 Return으로 파일을 열었다.
[파일 화면](worktree-file.png)의 내용은 worktree GUI fixture다. 실제 CLI/AI 요청은 없다.

GUI 삭제 동작이나 새 worktree 신뢰 승인은 수행하지 않았다. 기존 배치 삭제 보호 결과와
구분한다. 경로 입력·후보 선택·한글 디렉터리 및 파일 탐색은 이번 직접 GUI 결과로 확인했다.

## F05 프로젝트 명령 GUI 직접 검증 (2026-09-10)

기존 시험 GUI의 한글 worktree에서 C-c a T a로 gui-test / python3 ../task-fixture.py를
등록했다. 각 미니버퍼 입력을 화면에서 확인했고 Registered gui-test; not executed가
표시됐다. 설치 시험 프로필의 project-commands.json에서 정확한 프로젝트와 명령을 확인했다.
C-c a T r → 프로젝트 확인 → Tab 후보 확인 → Return으로 명시적으로 실행했다.
시험 Python은 3,000줄 한글 로그와 example.txt:1:1 오류를 출력하고 코드 7로 종료한다.

[종료 화면](task-end.png)에 LOG 2999, 오류 행, code 7이 표시됐다. 오류 행을 클릭하고
Return을 눌러 아래쪽 example.txt의 1번 줄 위치로 이동했다
([오류 이동](task-error-navigation.png)). 전체 로그는 264,042바이트, 로그 3,000줄과
오류 1줄을 빠짐없이 포함한다([디스크 점검](task-log-audit.json)).

실행 버퍼의 C-c C-o로 전체 기록을 열었을 때 shown 59,580 chars가 표시됐다.
p로 HISTORY에 진입(198510–264042), 다시 p로 이전 범위(132978–198510)로 이동했다.
이전 페이지에서 도구 스크롤로 LOG 1551 이후가 표시되고 위치가 6%/L41로 바뀌었다
([이전 페이지·스크롤](task-history-older-scroll.png)). GUI에서 전체 디스크 파일을 한꺼번에
표시하지 않고 페이지를 이동하는 동작을 확인했다. 매 순간의 메모리/버퍼 제한과 undo
비활성화는 기존 자동 측정 근거와 구분하며, 이번 스크린샷만으로 상시 불변식을 증명하지 않는다.

실제 AI 요청은 없다. GUI 취소·재실행 키는 이번 시험에서 실행하지 않았으며 기존 자동
수명 테스트와 구분한다. 개인 설정은 변경하지 않았다.

## F05 GUI 재실행·취소 및 F06 복원 결함 발견

gui-test의 시험 스크립트에 유한 60초 대기를 추가하고 기존 실행 버퍼의 g로 재실행했다.
별도 실행 버퍼와 별도 로그가 생성됐다([로그 구분](task-rerun-audit.json)). 첫 취소 키는
기존 버퍼에 포커스가 남아 새 실행에 전달되지 않았다. 아래 새 실행 버퍼를 클릭한 뒤
C-c C-k로 대기 중 Python을 중단했고 KeyboardInterrupt 및 Compilation interrupt: 2가
표시됐다([화면](task-rerun-cancel.png)). 전체 시험은 AI 요청 없이 진행됐다.

C-c a S s로 두 창의 배치와 숨겨진 Review-test 세션을 저장했다
([메타데이터](workspace-saved.json)). 새로운 Workspace restore GUI 0.8.7에서
C-c a S r로 같은 파일을 명시적으로 복원했다. 두 창과 metadata only 상태는 복원됐지만,
원래 로그의 큰 point/start를 짧은 안내 버퍼에 적용해 첫 창이 빈 화면처럼 보이는 문제를
발견했다([수정 전](workspace-restored.png)). 원래 로그 내용은 복원 대상이 아니다.

소스 수정은 안내 버퍼의 point/start를 첫 줄로 설정하며, 기존 세션 버퍼의 저장 위치는
계속 적용한다. 기존 passive-layout 테스트에 20,000자 원본과 큰 스크롤 위치를 추가했다.
설치본 0.8.7에서 point=295로 실패하고, 수정 소스에서 작업 공간 테스트 5개가 통과했다.
수정 설치본 0.8.8의 GUI 재확인은 별도로 진행한다.

0.8.8의 전체 패키지 설치·회귀·삭제 후 기록 보존 검증이 종료 코드 0으로 통과했다
([로그](package-install-088.log), [산출물](package-088.json)). 새 Workspace restore GUI
0.8.8은 초기 참조 화면까지 확인했지만, C-c a S r 도구 입력이 undefined 키로 전달돼
복원 미니버퍼가 열리지 않았다. 같은 키 재시도를 반복하지 않았으며 수정 설치본의 GUI
재검증은 대기 상태다. 사용자에게 별도 수동 시험을 요청하지 않았다.

## 0.8.8 복원 안내 렌더링 확인

기존 restore088 창에서 C-c a S r 키 전달이 다시 실패했다. 반복 재시도 대신
tests/gui-workspace-fixture.el로 같은 workspace JSON을 설치본 복원 함수에 전달하는
전용 초기 상태를 준비했다. 이 fixture는 일반 실행에 자동 복원을 추가하지 않는다.

[수정 후 실제 GUI](workspace-fixed-088.png)에서 두 창 모두 L1부터 프로젝트와
metadata only 상태, 수동 방문·재개·기록 명령을 표시한다. 수정 전 동일 JSON으로
첫 창의 내용이 숨겨지던 문제는 재현되지 않는다. 창 분할 경계도 동일하게 유지됐다.
이는 설치본 함수의 실제 GUI 렌더링 확인이며, 0.8.8의 키 전달 성공으로 세지 않는다.
원본 로그 내용·스크롤 위치는 메타데이터 안내 화면에 복원하지 않는 것이 의도된 동작이다.

## F07 GUI lifecycle 첫 시험

설치본 0.8.8로 전용 GUI-Fake 지속 CLI를 시작하고 최초 화면에서 PID 72591 및
한글 출력이 표시됐다([초기](persistent-initial.png)). 이는 초기 fixture가 실행한 연결이다.
M-RET/C-c a P d 도구 키는 터미널 문자로 전달돼 분리 명령 통과로 세지 않는다.

정확한 실행 경로로 식별한 시험 Emacs PID 72584만 SIGTERM으로 종료했다. 이후
manager inspect는 running, recorder_pid 72589, attached_clients 0을 반환했다
([종료 후](persistent-after-emacs-exit.json)). Emacs 수명 이후 세션 생존은 확인했다.

별도 GUI에서 설치본 attach를 초기 fixture로 실행했지만 스크린샷에는 server exited가
표시됐다([재연결 관찰](persistent-reconnected.png)). 후속 inspect도 stopped였다.
시험에 300초/120초 자동 정리 타이머가 있으므로 이 관찰만으로 제품 재접속 실패라고
단정하지 않는다. 화면 상태 유지·동일 PID 재연결은 아직 입증하지 못했다.
명시적 stop과 inspect로 stopped 상태를 재확인했다([정리](persistent-cleanup.json)).
후속 fixture는 자동 정리 시간을 900초로 늘렸고, 여전히 시험 종료 시 명시적으로 정리한다.

## F07 별도 GUI 재접속 직접 화면 확인 (두 번째 시험)

0.8.8 설치본과 유한 수명 fixture로 GUI-Fake를 시작했다. 최초 GUI 화면의
PID 73442와 한글 출력([초기](persistent02-initial.png))을 확인한 뒤, 정확한 실행
경로로 식별한 시험 Emacs만 SIGTERM으로 종료했다. manager 상태는 running, 연결 수
0으로 유지됐다. 새 GUI의 초기 fixture에서 설치본 attach를 호출했고
[재연결 화면](persistent02-reconnected.png)에 동일 PID 73442와 동일 한글 출력이 보였다.

화면 확인 직후 inspect는 running/연결 수 1, OS는 CLI PID 생존을 확인했다.
명시적 stop 이후 stopped 및 해당 CLI PID 부재를 확인했다
([전체 수명 근거](persistent02-lifecycle.json)). 실제 AI 요청은 없다.

이 결과는 Emacs 종료 후 다른 GUI에서 동일 CLI 프로세스·화면 재접속의 근거다.
연결은 fixture 초기화로 수행했으므로 C-c a P a/d 키나 물리 한글 조합, CLI의
미제출 입력 편집 왕복을 통과했다고 확대하지 않는다. 개인 Emacs는 종료하지 않았다.

## worktree 생성 GUI 후속 시도와 문서 패키지

0.8.8 GUI에서 C-c a W c 진입과 Repository 경로 입력은 확인했다. New worktree path
단계에서 문자열이 잘못 전달됐고 미니버퍼 좌표 포커스 후에도 같았다. Return으로
생성을 실행하지 않고 C-g를 보냈다. 의도한 gui-created 경로는 생성되지 않았다.
GUI 생성 전체 통과로 세지 않는다.

사용 안내의 누락된 7개 전체 M-x 이름과 잘못된 설치 경로 정정을 설치본에 반영하기
위해 문서 갱신 패키지 0.8.9를 준비한다. 런타임 동작은 복원 수정본 0.8.8과 동일하다.

0.8.9 설치 검증은 38개 ERT, 별도 Emacs 복원, 패키지 삭제 후 기록 보존까지 통과했다.
런타임은 버전 헤더 외 0.8.8과 동일하며 PTY/Magit은 이번에 반복하지 않았다.
설치된 도움말에 누락 명령 7개가 포함됨을 확인했다.

생성 미니버퍼의 C-g도 잘못 전달돼 취소 완료는 확인하지 못했다. 현재 해당 시험 창에는
잘못된 경로 입력이 남아 있을 수 있으며 Return을 보내지 않았다. 생성 완료로 세지 않는다.

## 생성 미니버퍼 정리와 Magit GUI 관찰

restore088의 미완료 New worktree path 미니버퍼에 포커스를 두고 Escape 세 번을
보냈다. Quit 메시지와 일반 참조 화면 복귀를 확인했다. 생성은 실행하지 않았다.

0.8.9 설치본과 기존 원본 Magit 의존성을 사용해 전용 gui-magit-089 저장소를 준비했다.
이전 이름.txt → 새 이름.txt 이름 변경과 21번 줄 변경을 staging했다. 초기 fixture가
magit-diff-staged를 호출했고 실제 GUI에서 한글 파일명, rename 헤더, 변경 한 줄이 보였다.
마우스 드래그로 +changed 한글 줄을 선택했다([화면](magit-selection.png)).

C-c a v와 대체 Esc/x는 undefined 키로 전달돼 리뷰 명령 진입을 확인하지 못했다.
이미 통과한 Magit 배치의 경로·줄 위치 계산과 이번 GUI 표시/선택 관찰은 구분한다.
Magit GUI의 초안 생성·확인은 아직 미검증이다. 가짜 CLI는 즉시 종료했고 AI 요청은 없다.

## 자동 진행 차단 재확인

최근 세 목표 턴에서 worktree 경로 입력 실패, Magit 리뷰 진입 실패, 다시 Magit
리뷰 진입 실패로 같은 Computer Use 키 전달 장애를 확인했다. 마지막 시도도
선택 영역은 보이지만 C-c a v가 undefined 키로 전달됐고 의견 미니버퍼는 열리지 않았다.
원시 입력 대조는 이미 패키지를 로드하지 않은 Emacs에서도 실패 사례를 남겼다.
추가 제품 키 변경으로 이 도구 전달 장애를 우회할 근거는 없다.

0.8.9 패키지 검증은 종료했으며 기다릴 실행 중 테스트 핸들은 없다. 이번 GUI 관찰에
AI 요청이나 새 프로세스 실행은 없다. 기능 구현·자동 검증·확보한 GUI 근거는 유지하되,
남은 물리 한글 조합 및 수동 GUI 흐름을 완료로 판정하지 않는다. 자동 진행을 차단 상태로
전환하고 키 전달 환경 변화 또는 사용자의 남은 직접 관찰을 기다린다.

## 차단 후 목표 재개: 첫 재확인

목표 상태 active를 확인하고 Magit GUI에서 선택 → C-c a v를 보냈다. 리뷰 의견
미니버퍼 대신 Magit의 Reverse region? 확인창이 열렸다. 승인하지 않고 Escape 세 번으로
취소했다([후속 화면](magit-resume-input.png)). Git staged diff는 여전히 이름 변경과
1줄 추가/1줄 삭제이며 역적용을 실행하지 않았다.

따라서 도구의 문자 일부가 전달되더라도 의도한 접두사 명령이 실행된다는 보장은 없다.
재개 후 첫 동일 장애 확인으로 기록하며 목표 완료나 새 blocked 판정을 하지 않는다.

## 재개 후 두 번째 진단

설치본 0.8.9와 실제 Magit 의존성을 로드한 별도 배치 Emacs에서 magit-diff-mode 및
emacs-ai-keys-mode를 활성화하고 key-binding을 조회했다. C-c a v는 정확히
emacs-ai-review-selection이었다. 정적 모드 조합의 바인딩 충돌은 재현되지 않았다.
이 결과로 현재 GUI의 순간적인 입력 상태까지 동일하다고 가정하지 않는다.

GUI에서 Esc → x는 M-x 대신 Reset main to 미니버퍼를 열었다. 값을 입력하거나
Return을 누르지 않고 Escape 세 번으로 취소했다. 같은 키 전달 장애가 재개 후 두 번째
턴에서도 확인됐다. 제품 동작을 바꾸거나 기존 통과 테스트를 반복할 근거는 없다.

## 재개 후 세 번째 확인: 입력 성공과 실제 제품 오류

일반 참조 GUI에서 alt+x, emacs-ai-worktree-create, 저장소와 생성 경로, 브랜치,
HEAD 입력이 정확히 전달됐다. Git은 ~/workspace/... 저장소로 이동할 수 없다는
오류를 반환했다([화면](worktree-created.png): 이름과 달리 생성 실패 화면).
Git 조회에서도 gui-created worktree가 없었다. 이는 도구 실패가 아니라 제품이
read-directory-name의 축약 경로를 Git -C에 그대로 전달한 오류다.

Git 호출 전 expand-file-name 적용으로 수정했고, 실제 Git 회귀 테스트에 ~ 경로를
추가했다. 기존 0.8.9에서 실패, 수정 소스의 worktree 테스트 4개는 통과했다.
0.8.10 패키지의 설치 검증과 GUI 재확인은 후속 결과로 구분한다. 목표는 진행 중이다.

## 0.8.10 GUI 재검증 첫 시도

0.8.10 설치 프로필을 로드하는 Worktree GUI 0.8.10 창을 준비했다. 시작 디렉터리는
기존 시험 저장소다. 초기 참조 화면은 정상 표시됐으나 C-c a W c와 alt+x가 각각
undefined 키로 전달돼 생성 미니버퍼를 열지 못했다. 생성 요청은 실행하지 않았다.
0.8.10의 실제 Git 회귀/설치 테스트 통과와 GUI 재검증 미실행을 구분한다.

## 0.8.10 GUI worktree 생성 통과

준비된 Worktree GUI 0.8.10에서 M-x emacs-ai-worktree-create를 실행했다.
저장소 기본값 ~/workspace/emacs-ai/var/gui-worktree-087/repo/를 확인하고 Return,
새 경로에 ../gui-created를 덧붙여 확인하고 Return, 브랜치 gui-created, 기준 HEAD를
입력했다. 마지막 Return 후 생성 디렉터리의 Dired가 열렸다
([화면](worktree-created-0810.png)). 모든 입력은 Computer Use의 실제 키로 진행했다.

Git/파일 대조는 gui-created 브랜치, 깨끗한 상태, 기준 커밋과 example.txt 내용 일치를
확인했다([대조](worktree-created-0810.json)). 수정 전 ~ 경로 실패가 이번에는 발생하지
않았다. macOS ls의 --dired 미지원 안내는 표시됐으나 파일 목록은 정상이다.
개인 설정 변경, 신뢰 승인, CLI 시작 또는 AI 요청은 없다. 생성한 시험 worktree는 유지한다.

## Magit GUI 초안 생성·undo 통과

기존 Magit review GUI 0.8.9에서 +changed 한글 한 줄(11자)을 마우스로 선택하고
alt+x → emacs-ai-review-selection을 실행했다. 영문 의견 Explain the rename change를
입력하고 Magit-review 가짜 세션을 선택해 초안을 생성했다([초안](magit-draft.png)).
Old: 이전 이름.txt:none / New: 새 이름.txt:21, 정확한 의견과 선택한 한 줄만 포함됐다.
추가 줄만 선택했으므로 대응되는 이전 줄 번호 none은 정상이다.

초안에 포커스를 두고 ctrl+slash로 빈 상태로 되돌리는 것도 확인했다
([undo](magit-draft-undo.png)). 앱 입력 저널은 0바이트이며
([점검](magit-input-audit.json)), 실제 AI 요청·CLI 전송·Git 편집은 없었다.
물리 한글 의견 조합과는 별도로 Magit의 실제 GUI 영역 선택·명령 실행·초안·undo를
확인했다. 실행본은 0.8.9이며 0.8.10에서 리뷰 코드는 변경하지 않았다.

## 지속 수동 연결 GUI 첫 시도

0.8.10 설치본 manager로 전용 GUI-Manual 가짜 CLI를 준비했다. 기존 worktree0810
GUI에서 alt+x는 M-x를 열었지만 emacs-ai-persistent-attach 문자열이 잘못 전달됐다.
명령을 실행하지 않고 Escape 세 번으로 취소했다. 연결 키 검증은 완료하지 못했다.
가짜 세션은 명시적으로 stop했고 stopped를 확인했다
([정리](persistent-manual-cleanup.json)). 실제 AI 요청은 없다.

## 지속 수동 연결 검증 차단

최근 세 목표 턴에서 M-x의 명령 문자열, C-c a P a, 동일 단축키 재확인이 모두
잘못 전달됐다. 마지막 화면도 undefined 키 메시지이며 경로 입력창은 열리지 않았다.
이 세 턴 동안 새 제품 결함이나 수정 근거는 얻지 못했다. 첫 시험용 CLI는 이미
stopped로 정리했고 후속 두 턴에는 CLI를 시작하지 않았다.

작업 공간 복원 안내 및 ~ 경로 worktree 오류 수정은 설치·GUI 근거가 확보됐고,
Magit 초안·undo도 통과했다. 남은 물리 한글 조합과 수동 지속 GUI 흐름은
완료로 판정하지 않는다. 같은 입력 전달 장애로 자동 진행을 blocked로 전환한다.

## 0.8.10 사용자 직접 확인: 한글 입력과 Codex 지속 왕복

사용자가 직전의 두 묶음 안내를 수행한 뒤 “모두 정상”이라고 보고했다. 사용자 보고로
다음 범위를 통과 처리한다. 도구 입력이나 배치 시험의 관찰과 구분한다.

- 새 0.8.10 시험 GUI의 일반 입력 버퍼에서 직접 한글 타이핑, 받침 삭제·재입력,
  문장 중간 한영 전환과 수정, M-x undo가 정상이다.
- Codex 지속 세션 시작, 한글 미제출 문장 입력, 작업 공간 저장, C-c a P d 분리가 정상이다.
- 두 번째 GUI에서 저장 파일을 복원하면 안내가 먼저 표시되고 자동 연결되지 않는다.
  해당 항목의 r로 연결하면 기존 화면과 미제출 문장이 유지된다.
- 재접속한 GUI의 C-c a e로 입력을 편집 버퍼에서 열고 문구를 추가한 뒤
  C-x C-s / C-x #로 반환하면 두 번째 GUI의 Codex 입력란에 수정 내용이 돌아온다.
- C-c a P k 및 yes 후 지속 서버 종료·디스크 기록 유지 안내가 정상이다.

안내는 AI 요청을 제출하지 않는 범위였다. Claude의 동일 GUI 왕복, completion
미니버퍼의 한글 조합, 선택적 macOS 알림까지 확인한 것으로 확대하지 않는다.
Computer Use 키 전달 자체가 복구됐다는 의미도 아니다.

## Claude 지속 GUI 실제 시작 확인

기존 worktree0810 GUI에서 M-x emacs-ai-persistent-start, Claude, ~/workspace/emacs-ai/를
화면으로 확인하며 입력했다. 실제 Claude Code v2.1.266의 Claude Max 및 프로젝트 루트가
표시됐다([시작 화면](claude-manual-start.png)). 새 worktree 신뢰 승인은 선택하지 않았다.

GUI draft keep 123 문자열이 일부만 전달됐고 C-c a e도 편집기를 열지 못했다.
Return으로 요청을 제출하지 않았다. 편집·분리·재접속 전체 왕복은 미검증이다.
이번에 생성한 정확한 세션 경로만 stop하고 stopped를 확인했다
([정리](claude-manual-cleanup.json)). 제품 코드는 수정하지 않았다.

## 사용자 정상 보고 이후 도구 입력 재확인

기존 worktree0810 창에서 이전 Claude 시험의 [server exited] 표시를 확인했다.
상단 Dired에 포커스를 둔 뒤 Alt+x는 undefined로 전달됐고, Escape 다음 x도
M-x 미니버퍼를 열지 못했다([화면](claude-followup-key-block.png)).
새 세션·AI 요청·신뢰 승인은 실행하지 않았다. Claude GUI 왕복 검증은 진전이 없다.
사용자의 정상 보고 범위를 축소하거나 같은 수동 시험을 다시 요청하지 않는다.
직전 턴은 현황 문서의 오래된 남은 항목 정정이며, 추가 기능 검증 진전으로 세지 않는다.

후속 읽기 전용 점검에서 `defaults read com.apple.HIToolbox AppleSelectedInputSources`는
`com.apple.inputmethod.Korean.390Sebulshik`을 반환했다. 이는 환경 설정의 관찰이며
Emacs 창의 실제 키 이벤트나 장애 원인을 확정하는 증거는 아니다. 사용자 입력 소스를
변경하지 않았다. 이미 관찰한 도구 키 전달 실패 상태에서 GUI 시험을 다시 시작하지 않았다.
선택적 OS 배너는 alert 미설치 상태이고, 물리 한글 조합은 도구 문자열 삽입으로
대체할 수 없으므로 두 항목도 자동 시험 통과로 처리하지 않는다.

## 후속 자동 진행 차단 판정

세 번째 연속 점검에서는 M-x가 열린 화면이 보였다. `sky.paste`로
emacs-ai-persistent-start를 넣으려 했으나 clipboard 읽기 시간 초과(-10005)가
발생했다. 재조회 화면은 명령 문자열 없이 undefined 키 메시지를 보였다.
시간 초과 후 붙여넣기를 중복 실행하거나 Return을 누르지 않았다.
새 CLI 또는 AI 요청은 없다. 세 턴 연속 도구 입력 장애로 남은 GUI 검증에 진전이
없으며, 물리 입력·OS 표시의 미검증을 배치 결과로 대체할 수 없다.
도구 입력 전달 복구 또는 해당 잔여 항목의 직접 확인 없이는 전체 완료를 입증할 수
없어 목표를 blocked로 전환한다. 이미 통과한 사용자 시험의 반복을 요청하지 않는다.

## Claude 실제 GUI 편집·분리·새 GUI 재접속 통과

후속 실행에서 도구의 M-x와 문자열 전달이 정상 동작했다. 0.8.10 worktree0810
GUI에서 emacs-ai-persistent-start, Claude, 기존 신뢰 프로젝트 루트를 선택했다.
Claude Code v2.1.267 / Claude Max 화면에서 `GUI draft keep 123`을 입력했고
요청 제출용 Return은 누르지 않았다. M-RET, C-c a e로 편집 버퍼를 열어
` edited`를 추가하고 C-x C-s, C-x #로 반환했다
([첫 반환](claude-edit-return.png)). M-x emacs-ai-persistent-detach 실행 후
Display detached; persistent CLI continues running 안내를 확인했다
([분리](claude-detached.png)).

개인 init을 읽지 않는 새 GUI claude-reconnect0811은 설치본 0.8.11 bootstrap을
로드했다. M-x emacs-ai-persistent-attach에서 방금 세션의 정확한 경로를 지정했다.
같은 미제출 입력 `GUI draft keep 123 edited`가 표시됐다
([재접속](claude-reconnected.png)). 새 GUI에서 M-RET, C-c a e로 열린 편집기에
` second GUI`를 추가하고 저장·반환하자 같은 GUI의 Claude 입력란에
`GUI draft keep 123 edited second GUI`가 표시됐다
([두 번째 반환](claude-second-editor-return.png)).

재접속 상태는 running, attached_clients 1이었다
([조회](claude-second-gui-inspect.json)). M-x emacs-ai-persistent-stop에서 해당
시험 세션 경로를 확인한 뒤 yes로 종료했다. GUI는 서버 종료·디스크 기록 유지 안내를
표시했고([화면](claude-stopped.png)), manager 재조회는 stopped, attached_clients 0이었다
([조회](claude-second-gui-stopped.json)). output.ansi 12,715바이트와 빈 events.jsonl이
남았다. 새 AI 요청이나 신뢰 승인은 제출하지 않았다. 이 결과는 실제 GUI 명령 왕복이며
물리 한글 조합 시험이나 모든 입력 전달 장애의 영구 해소를 뜻하지 않는다.

## 선택적 OS 알림 한 건 승인 후 입력 장애

사용자가 “가짜 알림 한 건 실행 승인해”라고 명시적으로 승인했다. 이미 fixture를
로드한 claude-reconnect0811 GUI에서 M-x 실행을 시도했지만 Alt+x, Escape 다음 x,
포커스 재설정 후 Alt+x 모두 undefined 키 메시지로 전달됐다. 시험 명령 실행은
확인되지 않았고 var/desktop-notification-probe.json도 존재하지 않았다.
알림 한 건 실행 승인은 유효하며 다시 요청할 필요가 없다. 현재 차단 원인은 승인이
아닌 Computer Use 키 전달이다. OS 배너나 백엔드 호출을 통과 처리하지 않는다.

## 선택적 macOS 배너 사용자 직접 확인 통과

사용자가 시험 GUI에서 M-x emacs-ai-test-desktop-notification을 한 번 실행하는
안내에 따라 “배너 보임”이라고 보고했다. [사용자 확인](desktop-probe-user.json)과
[같은 실행의 백엔드 기록](desktop-probe-backend.json)을 함께 보존한다.
백엔드는 osx-notifier, 호출·반환 true, error null, AI 요청 0이다. 원시 기록의
visual_banner_verified false는 코드가 화면을 판정하지 않도록 고정한 값이며,
사용자 보고는 별도 파일에 기록했다. 원시 기록을 화면 통과 값으로 덮어쓰지 않았다.
기본 설치본의 자동 활성화를 의미하지 않는다. 별도 alert와 osx-notifier 설정이
있는 이번 격리 GUI에서 확인했으며, 같은 알림을 다시 보내지 않는다.

## 미니버퍼 사용자 보고와 Space 안내 정정

사용자는 리뷰 의견 시험이 정상이라고 보고했다. 리뷰 의견의 직접 입력·수정·취소는
사용자 확인 완료로 기록한다. 세션 선택은 일치 후보가 없어 Space가 입력되지 않았다고
보고했으므로 전체 문장 시험 통과로 확대하지 않는다. 설치본 0.8.11을 로드한 배치
조회에서 minibuffer-local-completion-map의 SPC는 minibuffer-complete-word였다.
이 명령은 일치하는 후보가 있을 때만 완성 뒤 공백 또는 하이픈을 추가한다.
따라서 관찰된 Space 동작은 기본 completion 동작과 일치하며 한글 조합 결함의
증거가 아니다. 임의 문장을 Space로 입력하도록 한 시험 안내가 부정확했다.
같은 사용자 시험을 즉시 반복 요청하지 않고 제품의 전역 키맵도 변경하지 않는다.

## 점검한 worktree의 Claude 신뢰 화면 준비

도구 입력이 정상 전달된 후 worktree0810 GUI에서 M-x emacs-ai-worktree-start를
실행했다. Repository 기본값 var/gui-worktree-087/repo를 선택하고, Worktree는
Tab 완성의 절대 경로 gui-created 후보와 refs/heads/gui-created를 확인해 선택했다.
Worktree는 파일 경로 입력란이 아닌 completion 목록이므로 ~ 경로 입력은 일치하지
않았다. Provider Claude 선택 후 해당 절대 경로의 실제 신뢰 화면이 표시됐다
([화면](worktree-claude-current.png)). 선택지는 No, exit 및 Yes, I trust this folder다.
아직 신뢰 선택·AI 요청을 하지 않았다. 파일 사전 점검은 worktree-handoff.json에 있다.

## gui-created 폴더 Claude 신뢰 선택 및 입력 화면 통과

사용자가 “gui-created 폴더 신뢰 승인해”라고 명시적으로 승인했다. 같은 절대 경로를
화면에서 재확인했다. 처음 Down은 Emacs 탐색 모드에서 화면 커서만 움직였으므로
신뢰 선택을 확정하지 않았다. C-c a Down으로 CLI에 방향키를 전달해 Yes 선택을
확인한 뒤 C-c a RET로 확정했다. 후속 화면에 Claude Code v2.1.267, Claude Max,
gui-created 프로젝트 경로와 빈 입력 프롬프트가 표시됐다
([신뢰 후 화면](worktree-claude-trusted.png)). AI 작업 요청은 제출하지 않았다.
이는 Claude의 worktree 생성·선택·시작·사용자 승인 신뢰 이후 입력 화면까지의
GUI 근거다. Codex의 같은 신뢰 이후 흐름이나 이 worktree에서의 파일 수정까지
통과한 것으로 확대하지 않는다. 해당 CLI는 빈 입력 상태로 열어 두었다.

## Codex gui-created 입력 화면 사용자 확인 및 최종 정리

사용자가 안내된 worktree 시작 절차 후 “Codex 입력창 보임, 경로 gui-created 맞음”이라고
보고했다. 후속 Computer Use 화면에서도 Codex v0.154.0과 해당 프로젝트 경로 및
빈 입력 화면을 확인했다([화면](worktree-codex-user-confirmed.png)). 신뢰 선택의
개별 키 조작을 도구가 관찰한 것은 아니며 사용자 절차 보고와 최종 화면을 근거로 한다.

정리 전 ps/lsof로 시험 Emacs PID 85926의 직접 자식 Claude 92524와 Codex 93883의
cwd가 정확한 gui-created 경로임을 확인했다. 두 프로세스만 TERM으로 종료하고
PID 부재 및 Git 변경 없음까지 확인했다([정리](worktree-final-cleanup.json)).
기록 파일이나 worktree를 삭제하지 않았다. 이번 흐름은 AI 작업 요청 없이 확인했다.

## 세션 선택 한글 입력·수정 최종 사용자 확인

사용자는 이전 시험 관찰을 명확히 하며 “Space만 안 들어갔고 한글 입력·수정은
정상이었음”이라고 답했다([보고](completion-ime-user.json)). 따라서 세션 선택의
물리 한글 입력·수정도 사용자 확인 완료다. Space는 확인된 기본 단어 완성 동작이며
한글 입력 결함으로 분류하지 않는다. 재시험은 하지 않았다. 리뷰 의견, 선택적 OS
배너, 두 제공자 worktree 및 지속 GUI 흐름의 별도 근거와 함께 마지막 미확인 항목을
닫는다. 제품 구현 변경이나 추가 AI 요청은 없다.
