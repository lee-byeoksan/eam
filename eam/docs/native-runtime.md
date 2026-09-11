# Native runtime — 2026-09-16

EAM 실행 경로의 Python 의존성을 제거했다. Rust 실행 파일 하나인 `eam-runtime`이 세션 관리자, PTY 데몬, attach 클라이언트, 외부 편집기 연결, Claude 알림, 대화 조회, 원시 기록 검색과 기존 읽기 전용 연결 어댑터를 담당한다. Emacs Lisp는 UI·키·버퍼·프로세스 시작을 담당한다. 버전은 0.1.0이다.

## 설치와 최초 실행

Ghostel은 별도 의존성으로 유지한다. EAM 소스 또는 설치 tar에는 Rust 소스와 Cargo.lock을 포함하고, tar에는 Python·테스트·가짜 CLI를 포함하지 않는다. VC 설치는 저장소 전체를 체크아웃하므로 역사적 Python 실험 파일이 남을 수 있지만 제품 실행에는 사용하지 않는다.

처음 사용할 머신에는 Rust/Cargo와 시스템 C 빌드 도구가 필요하다. macOS에서는 Xcode Command Line Tools가 C 컴파일러와 SDK를 제공한다. SQLite는 bundled 빌드이므로 별도 SQLite 설치는 필요 없다. Rust 설치는 [공식 안내](https://rust-lang.org/tools/install/)를 따른다. EAM이 컴파일러를 자동 설치하지는 않는다.

`eam-new` 등 native 기능의 첫 사용 시 `cargo build --release --locked`를 실행한다. 처음에는 Cargo 의존성 다운로드가 필요할 수 있다. 설치·autoload·require·바이트 컴파일만으로 빌드하거나 AI를 호출하지 않는다. 바이너리가 없으면 자동으로 백그라운드 빌드를 시작하고 `*EAM native build*`를 즉시 표시한다. 빌드 중에도 다른 Emacs 작업을 할 수 있으며 로그는 실시간으로 추가된다. 로그 버퍼의 `C-c C-k`로 취소한다. 완료 메시지가 나오면 원래 실행하려던 EAM 명령을 다시 실행한다. 재시도는 `M-x eam-native-build`이며 진행 중 다시 실행하면 같은 로그를 보여 주고 중복 빌드하지 않는다. Rust/Cargo·C 빌드 도구는 사용자가 설치해야 한다.

```elisp
(use-package eam
  :ensure nil
  :load-path "~/workspace/emacs-ai/eam/lisp"
  :demand t
  :init
  (setq eam-directory (expand-file-name "~/.eam/")
        eam-native-auto-build t
        eam-terminal-redraw-delay 0.016)
  :config
  (require 'eam-app)
  (eam-keys-mode 1))
```

Cargo는 PATH 또는 `~/.cargo/bin/cargo`에서 찾는다. 다른 위치는 `eam-native-cargo-executable`로 지정한다. 자동 빌드가 싫으면 `eam-native-auto-build`를 nil로 설정하고 직접 `eam-native-build`를 실행한다. 미리 빌드한 파일을 사용하려면 `eam-native-executable`에 절대 경로를 지정한다. helper의 protocol=1 호환성을 확인한다.

캐시는 `eam-directory/native/<플랫폼>/<소스 SHA256>/release/eam-runtime`이다. 위 설정이면 `~/.eam/native/` 아래다. 기본 eam-directory는 Emacs 프로필 아래이므로 독립 저장을 원하면 위처럼 설정해야 한다. 같은 소스·플랫폼의 캐시가 있으면 Rust/Cargo 없이 실행한다. 소스 변경 시 새 경로에 빌드해 실행 중인 구 바이너리를 덮어쓰지 않는다. 이전 캐시의 자동 정리는 아직 없다.

## 프로세스와 데이터

```text
Emacs/Ghostel → native attach → Unix socket → native daemon → CLI PTY
```

`eam-new`가 manager를 실행하고 manager가 백그라운드 데몬을 시작한다. Ghostel은 attach 클라이언트를 실행한다. detach 또는 Emacs 종료는 연결만 끊으며, CLI 종료나 `eam-quit`은 데몬과 소유한 세션을 정리한다. 기본 PTY 경로에서 중복 recorder PTY를 제거했고 데몬이 알림·선택적 원시 기록을 직접 처리한다.

SIGWINCH로 poll 대기를 깨워 CLI가 아무 출력도 하지 않는 동안에도 창 크기를 전달한다. 출력은 Emacs에서 기본 16ms로 묶어 갱신한다. 데몬은 시작 출력 최대 5MiB를 보관하고 16KiB 패킷으로 재생한다. attach 때 재생 전송용 사본 최대 약 5MiB가 일시적으로 추가된다. live 큐는 별도 256KiB이며 느린 연결이 이를 초과하면 그 연결을 끊고 CLI를 유지한다.

이는 완전한 터미널 화면 snapshot이 아니다. 시작 출력 상한을 넘으면 재생을 포기하며 최신 꼬리로 바꾸지 않는다. 연결 없는 동안의 terminal query·여러 크기 변경·복잡한 TUI 상태 복원은 한계가 있다. Ghostel 스크롤백 5MiB는 별도다. 이전 대화는 provider의 원본 저장소를 읽는 history로 조회한다.

기존 실행 중인 Python 데몬/CLI 프로세스를 강제로 바꾸지 않는다. 새 세션부터 전체 native 경로를 사용한다. 구 세션의 EDITOR 등 환경에는 예전 helper 경로가 남을 수 있으므로 Python 없는 환경으로 옮길 때는 CLI를 정상 종료한 뒤 새 세션 또는 `eam-resume`으로 대화를 재개한다. 실행 중인 프로세스를 유지하려면 구 실행 파일을 먼저 삭제하지 않는다.

## 검증과 한계

현재 Apple Silicon macOS + Emacs 30.1 + Ghostel 0.53.0에서 검증한다. Linux용 Unix 코드 경로는 있으나 이번에 빌드·실행 검증하지 않았다. Windows ConPTY는 미구현이다. 로컬 컴파일은 미리 모든 CPU 바이너리를 배포할 필요를 줄이지만 OS별 PTY 구현을 대신하지 않는다.

`bash scripts/test-native.sh`는 Rust 가짜 CLI와 실제 PTY·소켓, ERT를 사용한다. `bash scripts/test-native-package.sh`는 임시 프로필에 tar를 설치해 첫 release 빌드, Cargo 없는 캐시 재사용, 설치 코드의 PTY/history, 제거 후 기록·캐시 보존을 검사한다. 두 경로 모두 Python·실제 AI·GUI를 사용하지 않는다. 첫 Cargo 의존성 확보에는 네트워크가 필요할 수 있다.

한글 UTF-8·붙여넣기·외부 편집기 반환, detach/attach/quit, CLI 자연 종료, idle 창 너비 전달, 5MiB 재생 경계, 원시 기록 기본 OFF, JSONL/SQLite 읽기 전용 페이지, 알림 분할 입력을 자동 검증한다. 실제 GUI의 한글 조합·트랙패드 스크롤·실제 Claude/Codex 장시간 사용은 이번 배치 검사와 구분한다. Python 버전에서 측정한 RSS·지연 수치를 Rust 성능으로 사용하지 않는다. native 장기 메모리/GUI 지연 측정은 남아 있다.

저장소의 과거 Python 실험·측정 자료는 비교용으로 보관한다. 기본 테스트 진입점은 native suite이며 과거 테스트 전체가 그대로 이식됐다는 의미는 아니다.

## 이번 실행 결과

2026-09-16 macOS arm64, Rust/Cargo 1.97.0, Emacs 30.1에서 다음 결과를 얻었다.

| 검사 | 결과 |
| --- | --- |
| Rust OSC parser unit | 1/1 통과 |
| Rust runtime 통합 | 8/8 통과, 약 6.9초 |
| 소스 ERT | 51/51 통과, 약 3.7초 |
| 설치 tar의 release 바이너리 ERT | 15/15 통과, 약 1.6초 |
| 설치 최초 컴파일·캐시·제거 | 최초 release 빌드 성공, Cargo 경로가 없어도 캐시 재사용, 제거 후 기록·실행 파일 보존 |
| 제품 Python 참조 | lisp 내 python/`.py` 호출 없음, tar 내 `.py` 파일 없음 |

32MiB 출력 시험은 연결 클라이언트의 읽기를 멈춘 채 CLI 출력 완료, 256KiB 큐 초과 후 연결 해제, 데몬 상태 조회 및 새 연결의 한글 입력 왕복을 확인했다. 이는 전체 RSS나 장기 누수가 없다는 측정이 아니다. JSONL 프레임 초과·예기치 않은 EOF·API 키 계정 거절·SIGTERM 취소 후 부분 backend ID 재사용 방지와 자식 정리도 확인했다.

PTY master가 실행한 CLI로 상속되지 않도록 close-on-exec도 설정했다. 설치 로그에 기존 docstring/group/obsolete 함수 관련 비치명적 bytecomp 경고는 남으며 컴파일 오류는 없다.

2026-09-16 후속 수정: CLI 직접 자식의 종료도 확인한다. 하위 helper가 slave PTY를 유지해 EOF가 오지 않더라도 최대 1초 동안 잔여 출력을 전달한 뒤 터미널을 닫고 종료 상태를 저장한다. Emacs는 확인된 종료 상태에 따라 화면·세션 목록을 정리하며 수정한 미전송 초안은 보존한다. 실행 중인 구 데몬에는 소급 적용되지 않는다.

후속 검증 결과: Rust unit 1개·통합 9개, 소스 ERT 54개, 빌드 전용 ERT 3개(그중 2개는 소스 suite와 중복), 설치본 ERT 16개가 통과했다. 빌드 전용 검사는 즉시 반환·로그 표시·중복 빌드 방지·실패 후 재시도·Cargo 없는 캐시 재사용과 Cargo.toml/Cargo.lock/Rust 소스·파일명 변경 시 캐시 무효화를 포함한다. 종료 검사는 attached/detached 모두에서 HUP를 무시하는 3초짜리 가짜 helper가 PTY를 유지해도 CLI 세션이 2초 이내에 종료됨을 확인한다. helper에 추가 종료 신호를 보내지 않으며 helper는 스스로 종료한다. GUI와 실제 AI 호출은 하지 않았다.

2026-09-16 용량 조정: Ghostel 스크롤백과 PTY 시작 재생 기록을 모두 5MiB로 변경했다. live 큐는 256KiB다. Rust unit 1개·통합 9개(5MiB 정확한 경계·초과 포함), Emacs ERT 55개가 최종 실행에서 통과했다. 첫 전체 검사에서 기존 어댑터 취소 테스트의 일시적 타임아웃이 있었으며 단독 재실행과 최종 전체 검사에서는 통과했다. GUI에서 실제 resume 출력량·메모리·스크롤 체감은 별도 확인 대상이다.

그래픽 효과 차이 조사: 사용자가 Ghostel 직접 실행과 EAM PTY 실행 사이 효과 차이를 보고했다. 직접 실행은 Ghostel의 terminal-env를 CLI에 전달하지만 EAM 데몬은 Emacs 환경에서 시작해 TERM=xterm-256color를 고정한다. Ghostel 전용 COLORTERM/TERM_PROGRAM/TERMINFO 설정이 attach 프로세스에만 적용될 수 있다. 연결 전 터미널 질의 응답 지연도 후보이며 실제 효과 종류·원인은 아직 확정하지 않았다. 이번 용량 변경에서 환경·초기 연결 순서는 변경하지 않았다.

사용자 추가 관찰은 색상이 아니라 애니메이션·깜빡임·화면 갱신 차이다. 로컬 Ghostel 소스의 ghostel-term 설명은 xterm-ghostty terminfo의 DEC 2026 동기화 출력이 큰 TUI의 부분 갱신을 줄인다고 명시한다. 현재 EAM의 xterm-256color 고정과 초기 연결 전 질의 응답 차이가 우선 조사 대상이다. EAM의 16ms redraw와 Ghostel 기본 33ms 차이, 중계의 16KiB 분할도 비교 조건이다. 실제 CLI의 DEC 2026 사용 여부는 아직 추적하지 않았으므로 원인을 확정하지 않는다.

사용자는 위 동기화 출력/부분 갱신 설명이 자신이 말한 애니메이션과 다르다고 정정했고, 중요하지 않으므로 조사를 중단하도록 요청했다. 따라서 위 내용은 코드상 차이에 대한 가설일 뿐 사용자 관찰의 원인으로 채택하지 않으며, 이 이슈는 보류한다.

2026-09-16 PTY 전용 정리: 백엔드 선택 설정과 이전 서버 실행·attach·reaper·전용 알림 래퍼를 제거했다. 제품 실행 코드, 예제 설정, 현행 사용 안내는 PTY 경로만 제공한다. 과거 공통 PTY/편집기/알림 검증 자료는 보존하고 혼합 탐색 기록은 evidence로 이동했으며 제품 tar에는 포함하지 않는다. 기존 사용자 세션 파일과 실행 프로세스는 수정하지 않았다. 지원하지 않는 backend 메타데이터는 명시적으로 거절한다.

검증: Rust unit 1개·통합 10개, Emacs ERT 55개, 설치본 ERT 16개 통과. 제거한 native 연산과 알 수 없는 backend 요청이 실행 전에 거절됨을 확인했다. 비동기 빌드 테스트는 프로세스 종료 상태만 보지 않고 완료 sentinel까지 기다리도록 수정했다. 실제 GUI/AI 호출 없이 검증했다.

## 세션 목록·이름·읽음 상태 (2026-09-16)

`manager start`는 선택적인 `name`을 받고 생성 시각을 `session.json`에 저장한다. 이름 변경은 `manager rename`이 `display.json`만 원자적으로 교체한다. 데몬 종료 시 상태 기록과 이름 변경이 서로 덮어쓰지 않는다. 빈 이름은 자동 표시로 돌아간다. 이름은 128자 이내, 제어 문자 제외다.

데몬은 실제 PTY 쓰기/읽기 시각을 메모리에 갱신하고 상태 조회에 `last_input_at`/`last_output_at`을 반환한다. 종료 시 메타데이터에 남기며 매 바이트 디스크를 쓰지 않는다. 재접속 재생·resize 제어 메시지는 새 활동으로 기록하지 않는다. CLI가 resize에 반응해 출력하면 그 출력은 활동이다. 오래된 실행 데몬의 활동 시각은 알 수 없으므로 목록에서 `—`로 표시한다.

`manager inspect`/`list-live`는 이벤트 기록의 최신 64KiB 이하에서 완성된 마지막 알림 순서를 찾고, `notification-read.json`의 읽음 위치와 비교한다. 연결 클라이언트가 없어도 조회 가능하다. `manager acknowledge`는 호출자가 확인한 순서까지만 저장한다. 읽음 커서는 이름·데몬 종료 상태와 별도 파일에 저장한다. 알림함에서 안 읽음으로 돌리면 해당 순서 직전으로 되돌리므로 이후 알림도 안 읽음이다. 전체 이벤트 본문을 목록에 적재하지 않는다.

검증: Rust 단위 1개 + 통합 12개 통과, ERT 전체 61개 통과 후 읽음 복원 테스트를 추가한 목록 집중 검사 7개 통과. 별도 프로필의 배포 tar 설치·첫 자동 빌드·캐시·제거·PTY/기록/목록 검사 22개 통과. 설치 검사에서 새 목록 파일의 bytecomp 경고는 없었고 기존 파일의 경고는 남아 있다. 실제 AI 호출은 없었다.

목록 256개·디렉터리 32개·128자 한글 이름으로 10회 렌더: 전체 테스트 환경 0.044초, 독립 `-Q` 검사 0.308–0.314초. 렌더 후 버퍼 22,705자, undo 비활성화를 확인했다. Lisp/GC 환경 차이를 포함한 배치 버퍼 생성 시간이며 GUI redraw·키 입력 지연·RSS 측정값이 아니다. 실제 GUI의 한글 조합·스크롤 체감은 검증하지 않았다. 목록은 자동 폴링하지 않으며 `g`로 갱신한다.
