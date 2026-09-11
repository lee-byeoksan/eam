# F07 — 독립 터미널과 재접속

> 과거 구현·검증 기록이다. 현재 제품은 PTY 전용이며 이 문서의 예전 백엔드는 지원하지 않는다. 현재 범위는 [native runtime](native-runtime.md)을 따른다.


2026-09-10. 현재: **지속 세션 명령을 0.8.x 패키지 소스에 편입**했다.
일반 CLI와 별도로 명시적 시작·연결·분리·종료·상태 명령을 제공한다.
[현재 사용법](user-guide.md#20-emacs-종료-후에도-cli-유지하기)을 따른다.

아래는 실험·실패·수정의 시간순 기록이다. 각 절의 “미구현/실험 전용” 표시는 해당 단계의
상태이며 현재 상태는 이 문단과 마지막 절을 따른다. 실제 GUI·장시간 다중 세션·장애 조건의
남은 검증 때문에 전체 F07 완료로 판정하지 않는다.

## 비교와 임시 결정

사용자의 자율 설계·검증 위임에 따라 기존 요구사항에서 대안을 비교했다.
추가 모델 호출·독립 에이전트 리뷰는 수행하지 않았다.

| 대안 | 얻는 것 | 비용과 실패 조건 |
| --- | --- | --- |
| 전용 tmux 서버에 CLI를 두고 Ghostel에서 붙기 | PTY·화면 재구성·재접속을 기존 로컬 도구에 맡김 | 중첩 터미널의 키·OSC·외부 편집기 호환성, 별도 설치 의존성 |
| 자체 PTY 데몬 + 클라이언트 | 원본 바이트와 입력·창 크기 프로토콜 직접 제어 | VT 상태 복원, 느린 클라이언트, 인증·수명·장애 처리를 직접 구현해야 함 |
| 독립 Emacs daemon을 계속 유지 | 기존 Ghostel 상태·버퍼 유지 | Emacs 자체 충돌에서 보호되지 않으며 사용자가 닫는 Emacs와 실행 주체 구분 필요 |

우선 tmux 경로에서 가장 기본적인 가정인 “표시 Emacs의 죽음과 CLI 생존을
분리하면서 원본을 중복 없이 기록할 수 있는가”를 시험했다. 최종 채택은 아직 아니다.
다른 프로젝트를 포크하지 않았으며 tmux 설정·Ghostel 소스·개인 Emacs 설정은 수정하지 않았다.

[tmux 공식 매뉴얼](https://man.openbsd.org/tmux#pipe-pane)은 `pipe-pane -O`를
프로그램의 출력 연결로 설명한다. 이번 실험은 이 기록을 재접속 클라이언트의 화면
재그리기 기록과 분리했다. 로컬 설치본은 tmux 3.6a다.

## 재현과 결과

```sh
python3 experiments/persistence/probe.py var/새-persistence-시험경로
```

실험은 `/tmp`의 새 전용 소켓과 `/dev/null` 설정으로 서버를 만들고 최종 정리한다.
개인 tmux 서버를 조회하거나 종료하지 않는다. 출력 전에 가짜 CLI를 대기시켜
기록 연결 직전의 출력 유실을 차단했다. 이는 실제 CLI 시작 단계에서도 필요한 조건이다.

실제 배치 Emacs/Ghostel 첫 클라이언트를 SIGKILL한 후, 표시 클라이언트가 없는 상태에서
가짜 CLI가 6,000줄을 출력했다. 두 번째 Emacs에서 붙고 마지막 READY 화면을 확인했다.
Ghostel의 `ghostel-send-string`으로 보낸 한글이 가짜 CLI에서 동일한 UTF-8 바이트로 수신됐다.
이후 제어기가 EXIT를 전송했고 tmux의 pane 종료 상태를 확인했다.

- 원본 기록: 1,260,015바이트, ROW 6,000개. 재접속 전후 바이트 전체 동일.
- 두 번째 Emacs 화면: 1,473자, undo 비활성.
- AI 호출 0회, GUI 실행 없음.
- [기계 판독 결과](evidence/persistence/result.json), 전체 시험 파일 `var/persistence-probe-04/`.

첫 실행은 격리 환경에서 서버 유지에 실패했고, 허용된 전용 소켓 시험으로 다시 실행했다.
다음 두 실행은 측정 코드의 Ghostel 저수준 redraw 호출 오류였다. 기존 배치 시험과 같은
`ghostel--redraw-now` 경로로 수정한 네 번째 실행이 통과했다. 실패를 제품 호환성 통과로 세지 않는다.

## 제품 연결 전 남은 필수 작업

1. 세션별 소유권·메타데이터, 분리와 명시적 종료의 별도 M-x 명령, 실행 중 worktree 삭제 방지.
2. 원본 기록의 유일한 작성자와 기록 준비 완료 후 CLI 시작. 현재 일반 앱 필터는 클라이언트의
   재그리기도 기록하므로 그대로 원본 경로에 연결하면 안 된다.
3. 기록기 실패·디스크 오류 시 중단, 느린 기록기에서 tmux 내부 대기 메모리 측정.
   `pipe-pane`만 붙였다는 이유로 기록 무손실·메모리 제한을 보장할 수 없다.
4. tmux 스크롤백 및 Ghostel의 각각의 제한, 장시간 다중 세션 RSS·타이머·응답 지연.
   이번 1,473자 측정은 연결이 끊긴 동안의 전체 프로세스 메모리 상한을 입증하지 않는다.
5. 재접속 대상 없음·죽은 서버·동시 중복 연결·한글 경로·시작 실패의 정리.
6. Claude/Codex 각각의 slash 명령·파일 수정·승인·OSC 알림·bracketed paste·창 크기 변경.
7. 외부 편집기는 기존 Emacs 서버 주소를 상속할 수 있으므로 재접속 후 새 Emacs로의 연결 경로 필요.
8. 실제 GUI의 물리 한글 조합·스크롤·반응성은 별도 검증.

기존 감시자 실험에서 확인된 “분리된 모든 하위 작업을 완전히 정리하지 못함”은 여전히
제약이다. tmux 세션 종료를 전체 후손 작업 종료라고 안내하지 않는다.

이 선택이 틀렸다는 신호는 원본 기록의 실패를 감지·제어할 수 없거나, 공식 CLI 기능이
중첩 터미널에서 손상되는 것이다. 그 경우 제품 기본값을 바꾸지 않고 자체 PTY 데몬의
동일한 반증 시험으로 넘어간다. 이 문서는 채택 완료 ADR을 대신하지 않는다.

## 후속: 느린 기록기 반례와 기록 경로 변경

`experiments/persistence/pipe-failure.py`에서 기록기가 아무 바이트도 읽지 않는 동안
가짜 CLI가 32MiB를 출력했다. tmux RSS는 3,488KiB에서 38,272KiB로 증가했다.
기록기를 오류 종료시켜 `pane_pipe=0`이 된 뒤에도 CLI는 살아 있었다.
[반례 결과](evidence/persistence/pipe-failure.json). 이 결과로 **pipe-pane 단독 기록은 채택하지 않는다**.

대체 실험은 tmux의 화면 유지·재접속 기능을 유지하면서, pane 안에서
`recorder.py → 자체 소유 PTY → CLI`를 실행한다. 기록 중계기가 원본을 유일하게 작성한다.
자체 데몬·소켓·VT 화면 재구성까지 구현하는 방식은 아니다.

- 파일을 독점 생성하고 나서 CLI를 시작한다. 기존 파일·심볼릭 링크는 거부한다.
- 받은 출력을 디스크에 완전히 쓴 뒤에만 화면 전달 큐에 넣는다.
- 입력·출력 큐는 각각 65,536바이트 이하. 화면이 막히면 PTY 읽기도 멈춘다.
  디스크 쓰기가 막히면 그 자리에서 기다려 자식 출력에 역압력을 전달한다.
- 쓰기 오류나 종료 요청에서 아직 회수하지 않은 직접 자식의 프로세스 그룹을 정리한다.
  `setsid`로 분리된 모든 후손의 종료를 보장하지 않는다.
- 원본 바이트 기록이며 문자 디코딩·프롬프트 삽입·AI 호출을 하지 않는다.

[실제 PTY 시험 4개](evidence/persistence/recorder-tests.log):

| 시험 | 관측 |
| --- | --- |
| 32MiB 출력 중 화면 읽기 중단 | 기록 크기 81,920바이트에서 정지, 중계기 RSS 16,992KiB → 16,992KiB |
| 화면 읽기 재개 | 32MiB 전체 전달, 화면 수신과 원본 SHA-256 일치 |
| 파일 크기 제한 4,096바이트 | 쓰기 오류를 보고하며 종료 코드 74, 직접 자식 PID 종료 확인 |
| 기존 기록 파일 | 기존 내용 보존, 자식 시작 안 함 |
| 한글 입력·자식 종료 코드 | UTF-8 동일, 자식 종료 코드 7 보존 |

첫 두 행은 같은 시험이다. 파일 크기 제한은 EFBIG 경로 검증이며 실제 디스크 용량 소진
ENOSPC나 물리 디스크 지연을 직접 재현한 것은 아니다. RSS 두 번의 관측은 장시간 누수나
전체 프로세스 트리 메모리 상한을 입증하지 않는다. OS 파일 캐시와 전원 장애 내구성은
별도이며 매 출력마다 fsync하지 않는다.

새 중계기를 tmux에 넣은 [두 배치 Emacs 재접속 시험](evidence/persistence/recorder-reattach.json)도
통과했다. 첫 Emacs SIGKILL 이후 6,000줄/1,260,015바이트 기록, 재접속 중복 0,
두 번째 화면 1,473자·undo 비활성, Ghostel 한글 전송 일치다.

```sh
python3 experiments/persistence/pipe-failure.py var/새-pipe-반례경로
python3 experiments/persistence/recorder-test.py
python3 experiments/persistence/probe.py var/새-recorder-재접속경로 --recorder
```

다음 검증은 창 크기·신호·터미널 제어 시퀀스, 기록 중계기 자체 강제 종료,
실제 CLI·외부 편집기·알림 호환성이다. 이후에만 이 경로를 제품의 명시적 시작·분리·재접속·종료
명령에 연결한다. 현재 제품 0.7.1의 기본 실행 동작은 변경하지 않았다.

## 후속: 터미널 제어와 종료 코드

중계기 시험을 8개로 확장했다. [결과](evidence/persistence/recorder-terminal-tests.log).

- 외부 PTY 28행×93열이 내부 자식에 동일하게 전달됨.
- 외부 크기를 37행×111열로 바꾸고 중계기에 SIGWINCH를 보내면 내부 크기도 변경됨.
- Ctrl-C 바이트가 자식의 전경 프로세스 그룹에 SIGINT로 전달되며 자식의 종료 코드 23 유지.
- OSC 9와 bracketed paste의 제어 바이트, 한글·개행을 원본 그대로 보존.
  **이는 중계기만의 시험으로, tmux를 지난 뒤 앱 알림 수신을 뜻하지 않는다.**
- SIGTERM으로 종료한 자식은 셸에서 사용하는 143으로 변환.
- 중계기에 명시적 종료 요청을 보내면 직접 자식 종료 확인.

초기 확장 시험에서 신호 종료가 241로 잘못 변환되고, 이미 종료한 자식 그룹에
SIGKILL을 보내면서 EPERM을 오류로 보고하는 문제를 발견해 수정했다.
EPERM은 직접 자식의 종료를 `waitpid`로 확인한 경우에만 수용한다.
실행 중 자식에 대한 신호 권한 오류를 무시하지 않는다. 임의의 분리 후손 정리는
여전히 보장하지 않는다. 중계기 자체 SIGKILL은 정리 코드를 실행할 수 없어 별도 검증이 남았다.

## 후속: tmux 알림 호환성 실패

가짜 CLI가 재접속 후 입력을 받으면 OSC 9와 OSC 777 알림을 각각 하나씩 출력하도록
재접속 시험을 확장했다. 원본 기록·재접속·한글 입력은 다시 통과했지만
앱의 `emacs-ai-notifications--entries`에는 1초 관측 동안 **0개**가 들어왔다.
[결과](evidence/persistence/recorder-notices.json). 이는 시험한 tmux 3.6a 기본 경로의 결과이며
다른 버전·설정에서의 불가능성을 주장하지 않는다.

따라서 현재 중첩 터미널 경로를 실제 CLI의 완전한 호환 경로로 제공할 수 없다.
다음 작업은 명시적 OSC 이벤트를 제한된 크기로 보존·전달하는 경로를 구현하고,
연결 중·연결 해제 중 이벤트의 세션 귀속·중복·재접속 처리를 검증하는 것이다.
출력 문자열이나 침묵으로 알림 상태를 추측하는 대안은 사용하지 않는다.

## 후속: 명시적 이벤트의 디스크 전달

tmux 경유 알림 유실을 우회하는 실험 경로를 추가했다. `recorder.py --events 파일`은
원본 출력 기록 직후 `events.py`로 OSC 9 / OSC 777의 명시적 알림만 추출하고,
전용 JSONL 파일에 버전·순번·원본 끝 바이트 위치·제목·본문을 저장한다.
원본 PTY 스트림은 수정하지 않는다. 새 이벤트 파일도 CLI 시작 전에 독점 생성한다.
이벤트 기록 실패는 상위 중계기의 오류 경로로 전달된다.

- OSC 파싱 대기 메모리는 8,192바이트 이하. 조각난 UTF-8·BEL/ST 종료를 처리한다.
- 긴/잘못된/중첩 OSC, DCS 등의 문자열 내부 알림, 취소된 제어 문자열은 알림으로 만들지 않는다.
- OSC 9;4 진행률과 OSC 9;9 디렉터리 보고는 알림에서 제외한다. 의미 구분은
  설치된 Ghostel `src/handler.zig`의 progress/PWD/notification 분리와 대조했다.
- `events.el` 조회기는 최초 최신 64KiB에서 완성된 줄부터 시작한다. 조회당 최대 64KiB,
  미완성 줄은 32KiB 미만, 표시 알림은 기존 앱의 기본 100개 제한을 따른다.
- 같은 살아 있는 버퍼에서 재연결하면 기존 파일 위치·순번을 재사용한다. 반복 조회로
  알림 개수나 중복 횟수가 증가하지 않는다. 파일 교체·잘림·잘못된 형식은 조회를 중단한다.
- 원본 이벤트 전체는 디스크에 남는다. 새 Emacs의 최초 조회에서 오래된 모든 알림을
  한꺼번에 복원하지 않는다. Emacs 재시작 간 읽음 상태 보존은 아직 구현하지 않았다.

[추출기 3개](evidence/persistence/events-python.log),
[Emacs 조회기 4개](evidence/persistence/events-emacs.log),
[기존 중계기 8개 회귀 시험](evidence/persistence/recorder-with-events-regression.log)이 통과했다.
조회기 시험에는 3,000개 기록의 제한된 최신 부분 읽기와 세션 버퍼 귀속도 포함된다.

[실제 두 배치 Emacs 재접속 시험](evidence/persistence/journal-reattach.json)에서는
연결 해제 중 알림 1개와 재접속 후 알림 2개를 합해 3개 수신했다.
반복 조회 1초 동안 중복 증가 없이 3개를 유지했고, 원본 재접속 중복 0과 한글 입력도 통과했다.
이 시험은 네이티브 OSC의 tmux 직접 통과를 증명하는 것이 아니라, 디스크 전달 경로의 결과다.

```sh
python3 experiments/persistence/events-test.py
/Applications/Emacs.app/Contents/MacOS/Emacs --batch -Q -L lisp \
  -l experiments/persistence/events-test.el -f ert-run-tests-batch-and-exit
python3 experiments/persistence/probe.py var/새-이벤트-재접속경로 --recorder
```

아직 `experiments/`의 기능이며 설치 패키지에 포함하지 않았다. 자동 조회 타이머 수명,
재시작 간 읽음 상태, 다른 Emacs의 동시 연결, 이벤트 쓰기 중 프로세스 강제 종료,
실제 CLI 훅과 외부 편집기 호환성은 제품 연결 단계에서 추가 검증해야 한다.

## 후속: 재접속 후 외부 편집기 주소 변경

기존 `emacs-ai-terminal--editor-command`는 실행 시점의 Emacs 소켓을 `EDITOR`에 넣는다.
그 Emacs가 종료된 후에도 살아 있는 CLI는 이전 주소를 유지하므로, 지속 세션에서는
호출할 때마다 현재 접속 대상을 찾는 별도 경로가 필요하다.

실험용 `editor.py ROUTE FILE...`은 CLI가 외부 편집기를 요청할 때마다 4KiB 이하의
소유자 전용 JSON 파일을 읽어 emacsclient와 소켓을 선택한다. Lisp 평가나 셸 평가를
수행하지 않고 인자를 배열로 전달한다. `-a false`로 죽은 소켓의 대체 Emacs 시작을 막는다.
파일은 Emacs의 `server-edit`가 완료될 때까지 기다린다. 재시도·자동 제출은 하지 않는다.

[실제 배치 Emacs 서버 두 개의 시험](evidence/persistence/editor-routing.json)에서:

- 가짜 CLI 한 프로세스와 변경하지 않은 `EDITOR`로 각각의 Emacs에서 한글 경로 파일을 편집했다.
- 첫 Emacs 종료 후 이전 주소는 실패하고 새 GUI를 만들지 않았다.
- 둘째 Emacs 준비 후 주소 파일을 원자적으로 교체해 두 번째 편집이 완료됐다.
- 타 사용자 접근 비트가 켜진 주소 파일은 거절했다.

첫 시험의 기대값은 Emacs 저장 시 추가되는 마지막 개행을 빠뜨렸다. 실제 저장 내용을
확인해 기대값을 수정한 두 번째 시험이 통과했다. 개인 init·개인 Emacs 서버는 사용하지 않았다.

```sh
python3 experiments/persistence/editor-test.py var/새-editor-시험경로
```

주소 파일 교체는 아직 시험 제어기가 수행한다. 제품의 명시적 연결·분리 명령과 연결해야
하며, 동시에 여러 Emacs가 붙을 때 편집 대상을 누가 소유하는지도 결정·검증해야 한다.
이미 열려 있는 편집 도중 Emacs가 죽는 상황을 자동 복구한다고 주장하지 않는다.

## 후속: 실제 Claude·Codex /model

실제 설치 CLI를 전용 tmux → PTY 중계기 경로로 실행했다. 기존 구독 로그인 상태를
사용하고 API 키 환경 변수는 제외했다. 프로젝트 신뢰나 로그인 선택은 수행하지 않는다.
`/model`만 입력하고 선택 화면이 나타나면 Escape로 닫았다. AI 요청·모델 변경은 하지 않았다.

- Claude: 첫 실행에서 모델 선택 화면 통과.
- Codex: 첫 실행은 시작 화면의 준비 판별 문자열이 달라 입력을 보내지 않았다.
  실제 화면의 배너·프롬프트에 맞게 판별을 고친 재시험에서 모델 선택 화면 통과.
- [최초 결과](evidence/persistence/cli-model-initial.json),
  [Codex 재시험](evidence/persistence/codex-model-retest.json).
- 전체 화면은 `var/persistence-cli-model-01/` 및 `var/persistence-cli-model-02/`에 보관한다.

```sh
python3 experiments/persistence/cli-model-probe.py var/새-model-시험경로
```

이 시험의 화면 관측은 tmux `capture-pane`이며 실제 GUI/Ghostel 표시 검증이 아니다.
Ghostel 재접속은 앞선 가짜 CLI 시험, 외부 편집기 주소 변경은 별도 가짜 CLI 시험이다.
실제 CLI의 Ctrl-G 편집기 왕복·알림 훅·파일 수정·대화 재개를 이 경로에서 모두 검증한
것으로 합쳐 주장하지 않는다. 이 통합 검증과 제품 M-x 명령 연결이 다음 단계다.

## 후속: 실제 CLI의 외부 편집기 왕복

`cli-editor-probe.py`에서 실제 Codex와 Claude 각각에 고유 시험 문자열을 입력한 뒤
Ctrl-G로 외부 편집기를 열었다. 첫 번째 배치 Emacs가 문자열을 편집하고 `server-edit`로
반환했다. 첫 Emacs를 종료하고 두 번째 Emacs의 주소로 바꾼 뒤 같은 CLI에 Ctrl-G를
다시 보냈고 두 번째 편집도 CLI 입력란에 반영됐다.

[결과](evidence/persistence/cli-editor.json): 두 제공자 모두 2회 왕복 통과.
각 제공자의 tmux pane에 있는 중계기 PID가 전후 동일함을 확인했다.
CLI 재실행 명령은 보내지 않았고, Enter도 한 번도 보내지 않아 시험 문장은 제출하지 않았다.
전체 화면·배치 Emacs 로그는 `var/persistence-cli-editor-01/`에 보관한다.

```sh
python3 experiments/persistence/cli-editor-probe.py var/새-cli-editor-시험경로
```

편집은 시험 서버가 고유 문자열이 있는 CLI 편집 파일에만 수행한다. 물리 키보드 조합·GUI
편집 조작은 시험하지 않았다. 화면 복귀 관측은 tmux capture-pane이며 실제 Ghostel GUI
표시 검증과 별개다. 이 시험은 미제출 입력 편집이며 프로젝트 파일 수정 AI 작업이 아니다.

## 후속: 세션 관리 계층

`manager.py`에 명시적 시작·상태 조회·종료 함수를 구현했다. 아직 제품 M-x 명령에서
호출하지 않는다. 세션마다 별도 소유자 전용 디렉터리와 임시 런타임 디렉터리, tmux 서버를
만든다. JSON 소유 ID, 런타임 마커, tmux 서버 내부 소유 ID를 대조한 뒤 종료한다.
개인 tmux 설정을 읽지 않으며 원본·이벤트 기록과 외부 편집기 주소는 세션별로 분리한다.

상태 조회는 기존 서버에 질의하며 CLI를 시작하지 않는다. 서버 응답이 없으면
`unavailable`로 표시하고 임의로 재시작하지 않는다. `running`은 중계기 pane의 실행 상태이며
AI 작업 진행·완료 상태를 뜻하지 않는다. 종료 후에도 기록은 남는다.

[실제 tmux 시험 2개](evidence/persistence/manager-tests.log)에서 서로 다른 두 세션의
PID·종료 격리, 소유 ID 불일치 거부, 기존 디렉터리 보존, 잘못된 실행 경로 거부를 확인했다.
한 세션을 종료한 뒤 다른 세션은 실행 상태를 유지했다. 시험은 가짜 CLI만 사용했다.

```sh
python3 experiments/persistence/manager-test.py
```

연결 소유권(동시 Emacs·편집기 대상), Ghostel 표시 연결, 이벤트 조회 타이머,
worktree 삭제 보호, 작업 공간 메타데이터와의 연결, 패키징은 아직 남았다.
현재 설치본의 동작은 바뀌지 않았다.

## 후속: M-x 명령과 실제 Ghostel 연결

`experiments/persistence/commands.el`을 별도로 로드하면 다음 명령을 사용할 수 있다.
현재 패키지 기본 경로에는 아직 포함하지 않았다. 소스를 로드한 별도 시험 Emacs에서
`M-x load-file`로 위 파일을 지정한다. 키는 기존 `emacs-ai-keys-mode`를 켰을 때만 유효하다.

| M-x 명령 | 키 | 동작 |
| --- | --- | --- |
| `emacs-ai-persistent-start` | `C-c a P s` | 제공자·프로젝트를 골라 독립 CLI 시작 후 연결 |
| `emacs-ai-persistent-attach` | `C-c a P a` | 저장된 세션 디렉터리의 실행 중 CLI에 연결 |
| `emacs-ai-persistent-detach` | `C-c a P d` | 표시와 조회 타이머 종료, CLI 유지 |
| `emacs-ai-persistent-stop` | `C-c a P k` | 확인 후 해당 소유 서버 종료, 기록 유지 |
| `emacs-ai-persistent-status` | `C-c a P i` | 로컬 상태·경로·PID·연결 수 조회 |

`P`는 대문자다. 연결 디렉터리는 기록 디렉터리의 `persistent/` 아래에 생성된다.
이 실험의 출력 버퍼에서 기존 `emacs-ai-terminal-close` / `C-c a q`도 표시만 닫는다.
독립 CLI 종료는 반드시 위 `persistent-stop` 또는 CLI 자체 종료를 사용한다.
초안에서는 기존 키로 입력을 전송할 수 있지만 연결/종료 명령은 출력 버퍼에서 실행한다.

[실제 배치 Ghostel 통합 시험](evidence/persistence/commands.log)에서 다음을 확인했다.

- 가짜 CLI 시작, READY 화면 표시, 명시적 알림 수신.
- 추가 연결 거부, 표시 분리 뒤 조회 타이머 제거·CLI 생존.
- 새 Ghostel 버퍼에서 READY 화면 재복원.
- 재접속 전후 원본 바이트 동일, 알림 개수·중복 횟수 증가 없음.
- 명시적 종료 뒤 원본 파일 보존.

표시 클라이언트의 필터는 원본 기록기를 거치지 않고 Ghostel로 보낸다. 원본 조회는
중계기 소유 파일을 가리킨다. 초반 연결 안내와 명시적 입력 동작은 별도 attachment 파일에
남으며, 원본에는 tmux 화면 재그리기를 추가하지 않는다.

관리 계층의 `attach`는 표시 프로세스 수명 동안 OS 파일 잠금을 잡는다. 다른 연결은
잠금 또는 이미 붙은 tmux 클라이언트 검사에서 거부한다. 편집기 주소에는 이 잠금 파일을
기록하고, 외부 편집기 호출 시 활성 연결이 없으면 실패한다. 정상 분리는 주소를 지우고,
프로세스 강제 종료 시에도 OS가 잠금을 해제한다. 동시 경쟁·강제 종료 조건의 별도 시험은 남았다.

이 단계는 실제 M-x 명령과 Ghostel 연결까지의 실험이다. 설치 패키지 편입 전에는
작업 공간 복원과 worktree 삭제 보호, 재접속 실패·다중 세션 장시간 측정,
실제 CLI의 이 명령 경로 통합과 GUI 검증을 계속해야 한다.

## 후속: worktree 삭제 보호와 작업 공간 복원 연결

지속 세션 명령을 로드하면 worktree 삭제 직전에 기록 디렉터리의 `persistent/`를 검사한다.
해당 worktree 또는 하위 디렉터리에 속한 세션이 명시적으로 종료되지 않았다면 삭제를
거부한다. 연결이 끊겼거나 상태를 확인할 수 없는 세션도 종료로 간주하지 않는다.
`persistent-stop`이 소유 서버 종료에 성공했을 때만 메타데이터에 종료 상태를 저장한다.
서버에 응답이 없다는 사실만으로 이 상태를 쓰지 않는다.

작업 공간 항목에 선택적 `persistent` 경로를 저장한다. 기존 버전 1 파일의 다른 필드는
유지하며, 새 필드도 문자열 크기 제한을 검사한다. 복원 자체는 서버 조회·CLI 시작·연결을
하지 않고 연결 상태 미확인으로 표시한다. 연결이 끊긴 표시 버퍼가 남아 있더라도
이를 서버 자체 종료와 동일시하지 않는다.

복원 항목의 `M-x emacs-ai-workspace-resume` / `r`은 지속 경로가 있으면 기존 서버에
명시적으로 연결한다. 새 CLI나 UUID 재개로 대체하지 않는다. 지속 명령이 로드되지
않았다면 연결 기능을 로드하라는 오류를 표시한다. 기존 일반 CLI 항목은 종전 재개 경로다.

[통합 11개 시험](evidence/persistence/integration.log)에서 다음을 확인했다.

- 임시 Git worktree에서 가짜 CLI를 실행하고 표시를 분리한 뒤 worktree 삭제 거부.
- 저장한 배치를 복원하는 동안 프로세스 시작·조회·자동 연결 함수를 금지해도 복원 성공.
- 복원한 지속 항목의 명시적 재접속으로 기존 READY 화면 복원.
- 명시적 종료 뒤 worktree 삭제 허용.
- 기존 창 배치·롤백·일반 worktree 보호 회귀 시험 통과.

별도 [관리 계층 2개 회귀 시험](evidence/persistence/manager-stop-state.log)도 통과했다.

```sh
/Applications/Emacs.app/Contents/MacOS/Emacs --batch -Q -L lisp \
  -l experiments/persistence/integration-test.el -f ert-run-tests-batch-and-exit
```

현재 삭제 보호 연결은 지속 명령을 별도로 로드한 시험 Emacs에 적용된다. 표준 0.7.1
설치본은 이 모듈을 자동으로 로드하지 않는다. 패키지 편입 시에는 지속 세션을 사용한 뒤
다시 연 Emacs에서도 보호 기능이 빠지지 않도록 해야 한다. 외부 터미널의 직접 Git 삭제나
CLI가 별도 세션으로 분리한 모든 후손 작업까지 보호하는 기능은 아니다.

분리 뒤 모든 표시 버퍼를 없앤 상태에서 새로 작업 공간을 저장할 때 지속 세션 목록을
추가 수집하는 기능은 아직 없다. 이번 시험은 연결된 상태에서 저장 → 분리 → 복원이다.
목록 수집과 새 Emacs의 연결 메타데이터 검증, 패키지 편입을 다음 단계로 남긴다.

## 현재: 정식 소스 경로와 분리 세션 수집

구현은 `lisp/emacs-ai-persistent.el`, `lisp/emacs-ai-persistent-events.el`,
`bridge/persistence/`로 이동했다. 기존 `experiments/persistence/` 실행 경로는 이 소스를
호출하는 호환 진입점이며 구현을 두 벌로 유지하지 않는다. 앱을 로드하면 삭제 보호와
작업 공간 메타데이터 연결도 로드한다. 로딩 자체는 CLI 실행·재접속·AI 호출을 하지 않는다.

작업 공간 저장은 표시 버퍼가 없는 지속 세션도 디스크에서 수집한다. 세션 메타데이터는
파일당 16KiB, 후보는 최대 128개로 제한하고 이미 캡처한 지속 경로는 중복 추가하지 않는다.
잘못된 파일이나 한도를 넘는 목록은 저장을 거부해 기존 작업 공간을 보존한다.
이 수집은 서버 상태를 확인하지 않으므로 복원 화면에서도 연결 가능 여부는 미확인이다.

분리 메타데이터 캡처 시험 2개가 통과했다. 소스 이전 후 앱 15개·이벤트 추출 3개도 통과했다.
패키지 설치본에서는 실제 bridge 경로와 Lisp 함수의 설치 위치를 검증하도록 시험을 추가했다.
동시 연결 경합·강제 종료, 실제 CLI의 파일 수정·재개 통합, 장시간 부하 및 GUI 검증은 남았다.

## 0.8.1 패키지 설치 검증

설치본에서 57개 ERT 시험과 별도 프로세스의 작업 공간 저장/복원 시험을 통과했다.
지속 세션 8개, 기존 앱·리뷰·worktree·작업 명령·작업 공간·HTTP 서버·알림·터미널·Magit
시험을 포함한다. 지속 세션의 Lisp와 Python bridge가 설치 디렉터리에 있는지 확인했다.
패키지 삭제 뒤 기록 보존과 manifest 일치도 통과했다.
[설치 로그](evidence/persistence/package-0.8.1.log), [아티팩트 정보](evidence/persistence/package-0.8.1.json).

0.8.0 첫 설치 시험은 테스트 파일 중복 로드로 중단됐다. 0.8.1 첫 설치 시험에서는
사용자가 제거를 요청한 GUI 메뉴를 요구하던 기존 시험이 실패했다. 테스트를 현재 명령
중심 동작에 맞게 수정한 뒤 **같은 0.8.1 아티팩트**를 재검증해 통과했다.

```sh
bash ~/workspace/emacs-ai/var/package-profile-0.8.1/start.sh
```

프로필 준비·설치 검증은 GUI를 띄우거나 AI 요청을 보내지 않았다. 위 실행 명령을 사용자가
실행하면 별도 GUI가 열린다. tmux는 현재 로컬 3.6a 설치를 사용하며 Ghostel은 기존에
검증한 0.53.0 의존성을 재사용한다. 다른 Mac의 의존성 자동 설치까지 검증한 것은 아니다.

## 후속: 연결 경합과 외부 PTY 소실

두 실제 PTY 클라이언트가 같은 소유 세션에 동시에 연결하도록 시험했다.
[결과](evidence/persistence/attachment-race.json): 하나만 연결되고 나머지는 종료 코드 1로 거절됐다.
연결된 쪽의 외부 PTY를 닫자 잠금과 tmux 표시 연결이 해제됐고 독립 CLI는 계속 실행됐다.
강제 연결 소실로 편집기 주소 파일이 남았지만, 활성 잠금이 없어 외부 편집기 호출은 거부됐다.
새 연결은 같은 중계기 PID를 유지하면서 새 편집기 주소로 갱신했다.

```sh
python3 experiments/persistence/attachment-test.py var/새-attachment-시험경로
```

이 시험은 실제 PTY 소실을 재현했으며 GUI 앱을 직접 강제 종료한 시험과 구분한다.
임의의 시점에 발생하는 모든 경쟁 조건을 입증한 것은 아니다.

재접속 시 기존 알림의 대상 버퍼가 닫힌 표시 버퍼에 남는 문제도 발견했다.
소스에서 같은 지속 세션의 기존 알림을 새 버퍼로 연결하도록 수정했고, 읽음·개수·중복 횟수는
유지한다. [회귀 시험](evidence/persistence/notice-rebind.log)이 통과했다.

GUI 도구는 이번에도 `Sky Computer Use native pipe startup failed`로 시작하지 못했다.
따라서 GUI 검증을 통과 항목으로 올리지 않는다.

## 후속: 0.8.1 설치본 4세션·120초 부하

설치본의 Ghostel 출력·이벤트 조회를 사용해 가짜 CLI 네 개를 동시에 실행했다.
40초에 두 표시를 분리하고 80초에 재접속했다. 10초 간격 표본 12개를 저장했다.
[원시 표본](evidence/persistence/soak-120s.json).

- 113.28초의 마지막 표본에서 원본 기록 합계 35,742,666바이트.
- 표본 화면 버퍼 최대 1,704자, 모든 출력 버퍼 undo 비활성, 알림 최대 100개.
- Emacs RSS: 첫 표본 47,808KiB → 마지막 57,648KiB.
- 네 기록 중계기 RSS는 각각 32KiB 증가. tmux 서버·가짜 CLI 자체 RSS는 이 수치에 포함하지 않았다.
- 20ms 타이머의 최대 늦음은 431.88ms. 시작 전부터 측정을 시작한 초기 지연이 포함되며,
  표본 수집의 동기 상태 조회·ps 실행과 다른 검증 작업도 있어 순수 GUI 지연값이 아니다.
- 시험 종료 후 네 세션에 명시적 종료를 수행했다. GUI·AI 요청은 없었다.

초기 실행은 측정 스크립트의 변수 범위 오류로 중단됐다. 네 시험 세션의 종료 기록을 확인한 뒤
스크립트를 수정해 새 결과 경로에서 다시 실행했다. 통과 결과는 두 번째 실행이다.

```sh
EMACS_AI_PERSIST_SOAK=/절대경로/새-결과-폴더 \
/Applications/Emacs.app/Contents/MacOS/Emacs --batch -Q \
  -L var/deps/ghostel/lisp -L var/package-profile-0.8.1/elpa/emacs-ai-0.8.1 \
  -l experiments/persistence/soak.el
```

120초는 초기 부하 확인이며 장시간 누수 판정이 아니다. 전체 프로세스 트리, GC 후 RSS,
정상 사용자 입력 중 반응성·물리 한글 조합·GUI 스크롤은 별도 검증해야 한다.

## 0.8.2 알림 이동 수정 설치 검증

기존 알림이 재접속한 버퍼를 가리키는 수정까지 포함한 0.8.2 설치본에서 57개 ERT 시험,
별도 프로세스 복원, manifest·패키지 삭제 후 기록 보존을 통과했다.
[설치 로그](evidence/persistence/package-0.8.2.log), [아티팩트 정보](evidence/persistence/package-0.8.2.json).
준비된 최신 실행 경로는 `var/package-profile-0.8.2/start.sh`다. 자동 GUI·AI 실행은 하지 않았다.

## 후속: 연결 설정 실패 복구

이벤트 기록 조회기를 만드는 단계에서 실패하면 표시 버퍼가 남던 경로를 수정했다.
연결 설정을 완료하지 못하면 해당 표시 프로세스·버퍼·타이머만 정리하고 이전 창 배치를
복원한다. 독립 CLI에는 종료나 재시작 명령을 보내지 않는다. 읽기 위치와 기존 알림의
버퍼 소유권은 실패 가능한 설정이 끝난 후에만 갱신한다.

가짜 CLI에서 이벤트 조회 실패를 주입해 새 표시 버퍼·타이머가 남지 않고 원래 창으로
돌아오는지, 독립 CLI가 실행 상태를 유지하는지 확인했다. 이후 정상 연결·분리·재접속도
통과했다. [회귀 시험](evidence/persistence/attach-failure.log).

이 시험은 Emacs 쪽 설정 실패를 다룬다. 서버가 상태 조회 직후 종료되는 등 모든 시점의
비동기 연결 실패를 다 검증한 것은 아니다.

0.8.3 설치본에서도 57개 ERT 시험과 별도 프로세스 복원·패키지 수명 검증을 통과했다.
[설치 로그](evidence/persistence/package-0.8.3.log), [아티팩트 정보](evidence/persistence/package-0.8.3.json).
최신 시험 실행 경로는 `var/package-profile-0.8.3/start.sh`다.

## 후속: 비동기 연결 완료 확인

상태 조회에서 연결 수가 0이었더라도 실제 attach 직전에 다른 연결이 먼저 잠금을 얻을 수 있다.
이 경우 표시 프로세스 생성만으로 연결 성공을 반환하던 경로를 보완했다.
관리 계층은 자신의 tmux 클라이언트 PID가 서버에 등록된 것을 확인한 뒤 연결 완료를 기록한다.
Emacs는 표시 프로세스 PID와 시도별 식별값이 모두 일치하는 완료 기록을 기다린다.
이전 주소 파일이나 PID 재사용만으로 새 연결을 성공으로 오인하지 않도록 했다.

관리 측 확인 대기는 최대 3초, Emacs 측 대기는 최대 4초다. Emacs 대기 중에는 프로세스
이벤트를 처리한다. 실패하면 새 표시만 정리하고 이전 창을 복원하며 기존 CLI를 재시작하지 않는다.

오래된 상태 조회 결과를 주입한 경쟁 조건 시험에서 두 번째 호출이 실패하고 기존 연결이
유지되는 것을 확인했다. 정상 연결·분리·재접속·알림 이동도 통과했다.
[통합 결과](evidence/persistence/attachment-confirmation.log).
동시 실제 PTY 연결·PTY 소실 시험도 재실행해 통과했다.
[결과](evidence/persistence/attachment-confirmation-race.json).

시험 추가 과정의 중복 삽입을 수정했고, Ghostel이 이미 예약한 일회성 링크 탐색 타이머는
처리가 끝나는 것을 기다린 뒤 잔여 여부를 검사하도록 했다. 제품의 반복 조회 타이머를
검사 대상에서 제외한 것은 아니다.

현재 이 수정은 소스 검증을 마쳤으며 0.8.3 설치본에는 아직 포함하지 않았다.
진행 중인 600초 부하 시험은 변경 전 0.8.3 설치본을 계속 사용한다.

## 0.8.4 패키지 반영

시도별 식별값·PID·tmux 실제 클라이언트 등록을 대조하는 연결 완료 확인을 0.8.4 소스에
반영했다. 0.8.3을 대상으로 이미 시작한 600초 측정은 해당 설치본을 그대로 사용한다.

## 0.8.4 설치 검증과 0.8.3의 600초 측정 완료

연결 완료 확인을 추가한 0.8.4 설치본에서 57개 ERT, 별도 프로세스 작업 공간 복원,
manifest·삭제 후 기록 보존을 통과했다.
[설치 로그](evidence/persistence/package-0.8.4.log), [아티팩트 정보](evidence/persistence/package-0.8.4.json).
최신 시험 실행 경로는 `var/package-profile-0.8.4/start.sh`다.

이미 실행 중이던 0.8.3의 4세션 부하 시험도 같은 실행에서 600초를 마쳤다.
200초에 두 표시를 분리하고 400초에 재접속했으며 총 59개 표본을 수집했다.

| 관측 | 결과 |
| --- | --- |
| 종료 후 원본 크기 합계 | 189,659,641바이트 |
| 화면 버퍼 최대 / undo | 1,704자 / 전부 비활성 |
| 알림 최대 | 100개 |
| 처음·마지막 연결 상태의 프로세스 수 | 각각 21개 |
| 전체 소유 트리 RSS | 270,640KiB → 285,792KiB |
| GC 후 Emacs RSS | 47,440KiB → 60,336KiB |
| 마지막 150초 GC 후 Emacs RSS 범위 | 58,800–60,336KiB |
| 표본 구간의 최대 타이머 늦음 | 99.72ms |
| 종료 후 조회 | 측정 중 수집한 25개 PID 모두 없음 |

프로세스 트리는 시험 Emacs, 전용 tmux 서버, 기록 중계기, 표시 클라이언트와 가짜 CLI를
포함하며 측정 Python/ps 자체는 제외했다. GC·표본 수집 후 타이머 기준을 다시 잡았다.
시험 중 다른 로컬 검증 작업도 수행했으므로 수치는 격리된 성능 벤치마크가 아니다.
분리 중에는 표시 관련 프로세스 수가 줄어드므로 총 RSS의 첫·마지막 비교는 네 표시가
연결된 상태끼리 비교했다.

[전체 표본](evidence/persistence/soak-600s-result.json),
[요약](evidence/persistence/soak-600s-summary.json),
[종료 PID 조회](evidence/persistence/soak-600s-cleanup.json).
실행 핸들의 종료 코드 0과 최종 결과 파일을 확인했다. 각 소유 세션에 종료 명령을 수행했고,
그 기록만 믿지 않고 표본 PID의 부재도 별도 ps 조회로 확인했다.

측정 요구사항에 대한 근거이며 “누수 없음”의 증명은 아니다. 마지막 구간에도 Emacs RSS의
소폭 증가가 있어 allocator 보유·측정 자료 보관·실제 누수를 이 데이터만으로 구분할 수 없다.
물리 한글 조합·GUI 스크롤/입력 반응성은 계속 미검증이다. AI 호출은 없었다.

## 실제 파일 수정 통합에서 발견한 tmux DCS 알림

0.8.4 설치본의 `emacs-ai-persistent-start`로 실제 Claude·Codex 각각에 요청 한 번을
제출했다. 프로젝트 `var/persistence-native-{제공자}-01/proof.txt` 한 파일에 지정한 문자열과
마지막 개행을 쓰도록 요청했다. 두 파일 모두 정확히 일치했고 승인 선택을 자동 수행하지 않았다.

그러나 완료 알림은 0.8.4 앱에서 수신하지 못했다.
[Codex 실제 결과](evidence/persistence/native-codex-workflow.json),
[Claude 실제 결과](evidence/persistence/native-claude-workflow.json).
두 결과의 `completion_event=false`는 이 실패를 그대로 보존한다.

원본 PTY에는 두 제공자 모두 다음 형태로 알림을 출력했다.

```text
ESC P tmux; ESC ESC ] 9;DONE BEL ESC \
ESC P tmux; ESC ESC ] 777;notify;Claude Code;Stop BEL ESC \
```

기존 추출기는 임의 DCS 내부 알림을 무시했으므로 이 정상적인 tmux 포장도 놓쳤다.
정확한 `tmux;` 포장이고, 내부 ESC가 이중화되어 있으며, 완전한 OSC 알림 하나만 들어 있을
때에 한해 처리하도록 수정했다. 임의 중첩 문자열·여러 알림·앞뒤 부가 출력·잘못된 포장은
계속 거부한다. OSC 대기는 8,192바이트, DCS 대기는 최대 16,392바이트로 제한한다.

- 추출기 5개 시험 통과: 조각난 UTF-8/ESC, BEL/ST, 잘못된 포장·크기 초과·복구.
- 실제 Codex 원본 재생에서 DONE 알림 1개 추출.
  [결과](evidence/persistence/native-codex-replayed-events.json).
- 실제 Claude 원본 재생에서 Stop 및 Notification 추출.
  [결과](evidence/persistence/native-claude-replayed-events.json).
- 가짜 CLI의 tmux 포장 알림을 두 실제 배치 Ghostel 사이에서 전달해 총 3개 수신·중복 없음.
  [결과](evidence/persistence/wrapped-notice-integration.json).

원인 확인 후 같은 AI 요청은 반복하지 않았다. 실제 파일 수정 요청은 합계 2회다.
수정 후 검증은 실제 원본 재생 및 가짜 PTY 통합이며 새 AI 응답을 받은 것으로 주장하지 않는다.
GUI는 실행하지 않았다. 재현 스크립트는 `experiments/persistence/native-workflow.el`이다.

0.8.5 설치본의 통합 시험도 tmux 포장 OSC 알림을 사용하도록 바꿨으며, 전체 57개 ERT와
별도 프로세스 복원·패키지 수명 검증이 통과했다.
[설치 로그](evidence/persistence/package-0.8.5.log), [아티팩트 정보](evidence/persistence/package-0.8.5.json).
최신 실행 경로는 `var/package-profile-0.8.5/start.sh`다.

## 실제 대화 재개 — 0.8.5 설치본

이전 파일 수정 시험의 대화 ID를 명시해 Codex `resume ID`, Claude `--resume ID`를
지속 프로세스 관리자로 시작하고, 설치본의 `emacs-ai-persistent-attach`로 Ghostel에 연결했다.
두 화면에서 이전 파일 수정 요청과 **별도 응답 줄의 DONE**을 확인했다. 요청문 속 DONE만으로
통과시키지 않는다. 새 요청·터미널 입력·승인 선택은 0회이며 소유한 시험 서버는 종료했다.

- [Codex 결과](evidence/persistence/native-codex-resume.json), [화면 텍스트](evidence/persistence/native-codex-resume-screen.txt)
- [Claude 결과](evidence/persistence/native-claude-resume.json), [화면 텍스트](evidence/persistence/native-claude-resume-screen.txt)

재현기는 `experiments/persistence/native-resume.el`이며 제공자·대화 ID·새 기록 경로를
환경 변수로 명시해야 한다. 첫 시도는 프로브가 축약 경로 `~`를 관리자로 전달해 CLI 시작 전에
실패했다. 제품 시작 명령과 동일하게 절대 경로로 변환한 뒤 재시험했다. 프로브의 편집기 서버
정리도 이 프로세스가 실제 서버를 실행한 경우에만 수행하도록 제한했다.

이 검증은 명시적 native 재개 인자를 통한 실제 CLI 복원이다. `/resume` 선택 UI의 키 이동,
GUI 화면, 한글 조합을 검증한 것으로 간주하지 않는다. 새 사용자용 지속 재개 명령을 추가한
것도 아니다. 기존 지속 CLI에서의 `/resume` 사용과 Emacs 연결 복원은 서로 구분한다.

## 장애 상태와 명시적 종료 — 0.8.6

`experiments/persistence/failure-test.py`에서 소유한 가짜 CLI 세션만 사용했다.
기록기 TERM, 기록기 SIGKILL, 전용 tmux 서버 소실의 세 경우가 통과했다.
[시험 로그](evidence/persistence/failures-0.8.6.log).

- TERM/KILL 뒤 pane은 `exited`가 되고 재접속을 거부했다. 기록은 바뀌지 않았다.
- 서버 소실 뒤 상태와 종료 요청은 `unavailable`을 반환했다. 종료 성공 메타데이터를
  만들지 않았고 worktree 삭제 보호가 유지됐다. CLI를 자동 재시작하지 않았다.
- 각 가짜 자식 PID가 사라진 것을 확인했다. SIGKILL의 경우 기본 PTY hangup을 받는
  가짜 자식에 한정된 결과이며, 분리되거나 hangup을 무시하는 모든 자식의 종료 증거는 아니다.

M-x 종료 명령은 기존에 `unavailable`도 조용히 반환하며 읽기 위치 캐시를 제거했다.
0.8.6에서는 확인된 `stopped`일 때만 캐시를 제거한다. 그 외에는 종료 미확인 오류와
`M-x emacs-ai-persistent-status` 안내를 표시한다. 이 분기와 캐시 보존을 ERT로 검증했다.

0.8.6 설치본 58개 ERT, 별도 Emacs 복원, manifest·설치·삭제·기록 보존 검증이 통과했다.
[설치 로그](evidence/persistence/package-0.8.6.log), [패키지 정보](evidence/persistence/package-0.8.6.json).
실행: `bash ~/workspace/emacs-ai/var/package-profile-0.8.6/start.sh`.
준비 과정에서 GUI를 열거나 AI 요청을 제출하지 않았다.

서버 소실을 프로세스 전체 종료로 단정할 수 없으므로 `unavailable`에서 안전하게 수동 복구하고
삭제 보호를 해제하는 절차는 아직 남아 있다. 실제 GUI 검증도 별도 통과 조건이다.

## 0.8.8 별도 GUI 수명 검증

전용 가짜 CLI에서 최초 GUI의 PID와 한글 화면을 확인하고 시험 Emacs를 종료했다.
세션은 running/연결 수 0으로 유지됐고, 새 GUI에서 동일 PID 73442와 화면이 보였다.
명시적 stop 후 CLI PID 부재까지 확인했다.
[화면·프로세스 근거](evidence/gui-087/persistent02-lifecycle.json),
[재접속 화면](evidence/gui-087/persistent02-reconnected.png).
연결은 설치본 함수를 실행한 초기 fixture이며 수동 연결·분리 키 검증은 아니다.
