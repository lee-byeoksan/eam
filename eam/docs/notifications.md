# 세션 알림 — F02 진행 기록

2026-09-10. 0.3.0 패키지에 포함됐다. 기존 0.2.1 프로필은 자동 변경하지 않는다.

## 공식 경로 확인

- [Codex 설정](https://learn.chatgpt.com/docs/config-file/config-reference):
  `tui.notifications`, `tui.notification_method = "osc9"`,
  `tui.notification_condition`을 제공한다. 기본 포커스 조건에 따라 이벤트가
  나오지 않을 수 있다. 설치된 CLI에서의 실제 발생은 아직 검증하지 않았다.
- [Claude 터미널 설정](https://code.claude.com/docs/en/terminal-config):
  완료·권한 요청 알림은 사용자가 터미널에서 떨어져 있다고 판단할 때 발생한다.
  지원 터미널 판별에 따라 기본 데스크톱 알림 전달이 달라진다.
- [Claude 훅](https://code.claude.com/docs/en/hooks): command hook의 JSON
  `terminalSequence`로 OSC 9/777을 터미널에 출력할 수 있다. 대화형 UI가
  표시되는 동안 적용되며 `-p`나 Agent SDK에서는 이 경로가 동작하지 않는다.
- 설치된 Ghostel 0.53.0의 `ghostel-notification-function`은 OSC 9/777을
  발신 버퍼 문맥에서 전달한다. 앱은 자체 터미널 버퍼에만 수신 함수를 연결한다.

공식 지원 확인과 실제 CLI 발생 검증은 별개다. 현재 CLI 옵션·계정 설정·훅을
변경하지 않았다. 앱의 시스템 프롬프트나 AI 상태 판별 호출은 추가하지 않는다.

## 소스 사용법

- `M-x emacs-ai-notifications` / `C-c a l`: 알림 목록.
- `M-x emacs-ai-notification-next` / `C-c a n`: 가장 오래된 미읽음 알림의
  열린 세션으로 이동하고 읽음 처리.
- 목록에서 `RET`: 해당 세션 이동, `r`: 읽음 토글, `g`: 갱신, `q`: 목록 닫기.

알림은 터미널이 명시적으로 보낸 제목·본문이며 완료나 승인 대기의 확정된
상태값이 아니다. 화면에 상태 미확인을 표시한다. 어떤 알림도 자동 승인하거나
입력을 보내지 않는다. 세션 버퍼가 닫힌 알림은 목록에 closed로 남고 다음 이동에서
건너뛴다. 일반 출력 증가·침묵·화면 문자열은 알림으로 취급하지 않는다.

## 제한과 검증

메모리에는 기본 100건, 제목·본문 각각 2048자만 보관한다. 동일 버퍼의 같은
미읽음 알림은 2초 내 중복이면 개수만 증가한다. 분량 제한 전 원문 해시도 비교해
잘린 앞부분이 같다는 이유로 다른 알림을 합치지 않는다. 원래 OSC 출력 바이트는
기존 세션 `.ansi` 디스크 기록에 보관되지만 읽음 상태는 Emacs 재실행 후 복원하지
않는다. 목록은 수동 갱신하며 출력 버퍼 undo는 비활성화한다.

`emacs-ai-notification-desktop`은 기본 nil. 켜면 Ghostel의 OS 알림 함수를
사용한다. Ghostel은 별도 alert 패키지가 없으면 Emacs message로 대체한다.
현재 격리 프로필에는 alert가 없으므로 옵션 활성화만으로 OS 배너가 표시되지는 않는다.
이후 별도 alert/osx-notifier 시험 GUI에서 가짜 알림 한 건의 실제 macOS 배너를
사용자가 확인했다. OS 권한 설정 변경은 하지 않았다.
[백엔드 반환](evidence/gui-087/desktop-probe-backend.json)과
[사용자 화면 확인](evidence/gui-087/desktop-probe-user.json)을 구분해 기록했다.

추가 확인: alert 자체도 기본 스타일이 message다. 설치만으로 OS 배너가 활성화되지
않는다. 공식 소스 `31fc56855289d0846e73d7ca9b84b628aeac16a0`을 var/deps/alert에
격리해 확인했다. macOS용 osx-notifier는 AppleScript 알림 API를 호출한다.
[공식 설정과 스타일 안내](https://github.com/jwiegley/alert)를 참고한다.
`tests/gui-desktop-notification-fixture.el`은 로드 시 알림을 보내지 않으며 명시적인
M-x emacs-ai-test-desktop-notification으로 가짜 알림 한 건을 시험하도록 준비했다.
백엔드 호출·반환은 var/desktop-notification-probe.json에 기록하되 실제 OS 배너
표시 여부는 별도의 화면 확인 전까지 false로 유지한다. 이 fixture는 제품 패키지에
포함하거나 개인 init에서 자동 로드하는 설정이 아니다.

`tests/notifications-test.el`의 로컬 배치 2개 통과: 세션별 귀속, 중복 억제,
읽음과 순서대로 이동, 닫힌 버퍼 처리, 건수·텍스트 제한, 목록 undo 비활성화.
기존 앱 회귀 15개도 통과했다. 실제 PTY OSC 파서부터 수신부까지의 통합,
제공자별 완료·확인 요청 이벤트, GUI 조작과 패키지 반영은 다음 검증 대상이다.

## 실제 PTY 통합 검증

`tests/notification-fixture.py`는 두 로컬 PTY에서 OSC 9와 OSC 777을 각각
출력한다. UTF-8을 3바이트 단위로 잘라 보내 문자·제어 시퀀스가 수신 경계에서
분리되는 경우도 검사했다. `tests/notifications-test.el` 전체 3개 통과.
[실행 로그](evidence/notifications/pty.log).

총 4건의 세션별 귀속, 한글 본문, 두 OSC 형식의 원시 바이트 디스크 저장,
입력 전송 기록이 0바이트임을 확인했다. 첫 실행은 콜백이 초안 버퍼에 잘못
연결돼 실패했으며, 실제 터미널 버퍼에 연결하도록 수정 후 같은 시험이 통과했다.
따라서 단위 콜백 호출만으로 실제 수신 경로가 검증됐다고 간주하지 않는다.

확인한 설치 버전은 Codex CLI 0.153.4, Claude Code 2.1.266이다.
이 버전에서 실제 작업 완료·승인 요청을 발생시켜 수신하는 검증은 남아 있다.

## 실제 구독 CLI 완료 알림 (2026-09-10)

[실행 결과와 해시](evidence/notifications/live-completion.json).
`tests/notification-live.el`은 실제 구독 CLI를 하나씩 실행하고 도구·파일 수정 없는
짧은 요청을 제출한다. 가짜 PTY 시험과 별개이며 AI 호출이 발생하는 시험이다.

- Codex 0.153.4: 실행 인수로 `tui.notifications=true`,
  `tui.notification_method="osc9"`, `tui.notification_condition="always"`를 지정했다.
  화면 응답과 수신 OSC 9 본문 모두 `NOTICE_PROBE_DONE`이었다.
- Claude Code 2.1.266: 실행 시 `--settings`로 시험 전용 command hook을 지정했다.
  `Stop` 훅이 JSON `terminalSequence`를 반환했고 Ghostel 수신부에는
  제목 `Claude hook`, 본문 `Stop`으로 도착했다. 화면 응답도 확인했다.

개인 CLI 설정 파일은 수정하지 않았다. 각 실행은 첫 이벤트를 수신한 뒤 시험
하네스가 종료했으므로 정상 `/exit`·`/quit` 수명 검증 결과로 사용하지 않는다.
Claude의 Stop은 해당 턴 종료 훅이며 전체 개발 목표 달성을 뜻하지 않는다.
기존 CLI 자체 설정·훅의 실행까지 앱이 제거하거나 무료라고 보장하지 않는다.

남은 작업: 실제 승인 요청 이벤트, 일반 시작·재개 경로에 옵션을 적용하는 제품
연결, 해당 연결의 회귀 테스트, 패키지 반영, GUI 키보드·읽음 상태 검증.
현재 일반 명령은 여전히 기존 CLI 옵션을 사용하므로 이 시험 인수가 기본 기능으로
배포됐다고 간주하지 않는다.

## 승인 요청과 제품 연결

[실제 승인 요청 결과](evidence/notifications/live-approval.json): 두 CLI 모두
시험 파일 쓰기의 승인 화면이 나타났고 알림을 수신했다. Claude는
`PermissionRequest`, Codex는 `Approval requested: ...` OSC 9 본문이었다.
승인 입력 없이 종료했으며 두 시험 대상 파일이 존재하지 않음을 확인했다.

일반 시작과 UUID/목록 재개 명령에도 알림 실행 인수를 연결했다.
`emacs-ai-cli-notifications`는 기본 t이며 nil이면 새 실행에 알림 옵션을 추가하지
않는다. 이미 실행 중인 CLI는 변경하지 않는다. Codex는 알림 활성화·OSC 9·포커스
무관 전달 옵션을 사용한다. Claude는 설치된 `bridge/notify_claude.py`를
Stop/Notification/PermissionRequest command hook으로 연결한다.

브리지는 이벤트 종류만 OSC로 반환한다. 컨텍스트, 승인 결정, 도구 인자나 알림의
사적인 본문을 반환하지 않는다. 기존 계정 설정 파일은 쓰지 않는다. 시작·재개 인수의
일관성과 비활성화 옵션 ERT, 훅 프로토콜 Python 테스트 2개가 통과했다.
이 구현은 0.2.1 패키지 이후 소스 변경이며 신규 패키지 설치 검증이 남아 있다.

## 0.3.0 설치 검증

[실행 로그](evidence/notifications/package-0.3.0.log),
[산출물 해시](evidence/notifications/package-0.3.0.json).
한글·공백 경로의 임시 패키지 설치본에서 앱 15개, 알림 4개, 터미널 6개
테스트가 통과했다. 설치된 알림 함수의 출처를 확인했고, 설치된 Python 훅을
실제로 실행해 출력 JSON까지 검사했다. 기록 검색과 삭제 후 기록 보존도 통과했다.

PTY 테스트의 첫 실행은 프로세스 종료와 종료 메시지 표시의 시점 차이를
기다리지 않아 실패했다. 종료 메시지가 나타날 때까지 제한 시간 안에서 기다리도록
테스트를 수정한 뒤 동일 패키지가 통과했다. 제품 동작을 테스트에 맞춰 변경하지 않았다.

사용: `bash ~/workspace/emacs-ai/var/package-profile-0.3.0/start.sh`.
이 명령은 새 GUI를 연다. 패키지 준비·검증 과정에서는 GUI를 열지 않았다.
Computer Use는 재시도에서도 `Sky Computer Use native pipe startup failed`로
실패했다. GUI의 `C-c a l`, `C-c a n`, 목록 `RET/r/g/q`, 긴 알림 표시와
한글 조합 검증은 미완료이며 배치 통과로 대체하지 않는다.

## 0.8.11 알림 원본 버퍼 보존

지속 세션 폴링처럼 다른 버퍼가 선택된 상태에서도 데스크톱 백엔드는 원래 세션
버퍼를 current-buffer로 실행한다. Ghostel이 빈 제목을 버퍼명으로 채울 때 잘못된
선택 버퍼명이 사용되던 문제를 수정했다. 기본 비활성·명시적 활성·중복 억제·백엔드
오류에도 내부 알림 보존을 테스트했고, 0.8.10에서 실패/수정 소스 통과를 확인했다.
이 테스트는 백엔드 대체 함수를 사용하며 실제 OS 배너의 증거는 아니다.
