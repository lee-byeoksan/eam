# EAM — Emacs AI Management

[emacs-packages 저장소](../README.md)의 EAM 패키지다. 이 문서의 상대 실행 경로는 `eam/` 디렉터리를 기준으로 한다.

**처음 사용한다면 [사용 매뉴얼](docs/user-guide.md)**에서 실행·입력 편집·대화 재개·기록 조회·종료 순서를 확인한다.

설치된 패키지로 시험하려면 [패키지 테스트 안내](docs/package-testing.md)를 따른다. 현재 Mac용 별도 프로필과 실행 스크립트를 준비했다.

Emacs for Mac OS X에서 Claude Code·Codex 공식 CLI를 기존 구독 계정으로 사용한다. 현재 주 사용 경로는 Ghostel 터미널이며, 기본은 터미널 직접 입력이고 필요한 경우 일반 Emacs 버퍼에서 입력을 편집한다. 앱은 독립적으로 작성했고 Ghostel을 원본 의존성으로 사용한다. 앱 자체의 시스템 프롬프트·스킬·선택 영역·프로젝트 파일·요약 자동 삽입은 없다. CLI 자신의 설정과 기능은 유지한다.

현재 버전은 **EAM 0.1.0**이다. 지금까지의 개발 스냅샷을 묶어 버전 번호를 새로 시작했다. `eam-new`에서 provider를 자동완성으로 선택하며, 모든 실제 CLI 시작·worktree 시작·대화 재개는 지속 세션을 사용한다. Emacs를 종료해도 CLI는 유지된다. 모든 세션은 PTY 데몬을 사용한다. PTY의 재연결 화면 복원에는 제한이 있으므로 [사용 안내](docs/user-guide.md)의 백엔드 설명을 확인한다. [변경 검증](docs/evidence/eam-010-release/result.json), [이전 기능 완료 근거](docs/completion-audit.md)를 참고한다.

실행 시 Python은 필요 없다. 첫 native 기능 사용 시 Rust/Cargo와 C 빌드 도구로 바이너리를 컴파일하고 `eam-directory/native/`에 캐시한다. `M-x eam-native-build`로 미리 빌드할 수도 있다. [최초 빌드·캐시 설정](docs/native-runtime.md)을 참고한다. [설치·업데이트·복구 안내](docs/setup-and-updates.md)에 지원 환경과 확인 절차를 정리했다.

긴 작업 중 자동 잠자기를 막으려면 `C-c a f` 또는 `M-x eam-caffeine-mode`를 사용한다. 기본은 꺼짐이며 화면 잠자기는 허용한다. [동작·검증 범위](docs/caffeine.md), [CLI 기능 호환표](docs/cli-compatibility.md).

## 시작하기 — M-x와 키보드

EAM 0.1.0 시험 창:

```sh
bash ~/workspace/emacs-ai/eam/var/package-profile-monorepo-0.1.0/start.sh
```

현재 소스를 별도 창에서 실행하려면 `bash scripts/start.sh`를 사용한다. 두 명령 모두 새 GUI를 연다. 시작 화면 없이 키 바인딩만 켜진다. `M-x eam-new`로 세션을 시작한다.

| 할 일 | 키 | M-x 명령 |
| --- | --- | --- |
| CLI 시작 (provider 자동완성 선택) | `C-c a n` | `eam-new` |
| 활성 세션 그룹·정렬·알림 확인 | `C-c a l` | `eam-sessions` |
| 대화 재개 | `C-c a r` | `eam-resume` |
| CLI 입력 편집 | `C-c a e` | `eam-edit-input` |
| 초안 열기 / 전송 | `C-c a b` / `C-c a p` | `eam-draft` / `eam-paste` |
| 프로젝트 저장 대화 | `C-c a h` | `eam-history` |
| 사용 안내 | `C-c a ?` | `eam-help` |

세션 목록은 전용 버퍼다. `RET` 선택, `g` 갱신, `s` 정렬, `r` 이름 변경, `m` 알림 확인, `t` 디렉터리 그룹 전환, `f` detached 필터를 사용한다. 기본은 디렉터리 그룹 + 최근 PTY 입출력 순이다. 이름과 알림 읽음 상태는 세션 디렉터리에 저장된다. `eam-new`/`eam-resume`은 현재 디렉터리를 미리 채우고 선택적인 이름을 받는다. [상세 사용법과 정렬 설정](docs/user-guide.md#5-세션-전환대화-재개).

외부 편집기는 수정 후 `C-c C-c`로 저장·반환한다. `C-c C-k`는 편집 전 내용으로 복원하고 반환한다. CLI 정상 종료는 Claude `/exit`, Codex `/quit`다. `C-c a d`는 화면만 닫고 CLI를 유지한다. Emacs를 다시 열면 `C-c a a` (`eam-attach`)로 재접속한다. CLI를 확실히 종료하려면 `C-c a q` (`eam-quit`)를 사용한다.

별도 실행 스크립트는 `eam-keys-mode`를 활성화한다. 같은 모드를 끄면 기존 키 바인딩이 복원된다. 패키지 로딩만으로 전역 키를 켜지 않는다. Ghostel char 모드에서는 M-RET으로 기본 모드에 돌아온 뒤 Emacs 키를 쓴다.

대화 조회는 CLI가 저장한 로컬 기록을 직접 읽는다. `C-c a h`에서 대화를 선택하고 `p`/`n`으로 페이지 이동, `g`로 갱신한다. 원시 터미널 기록은 기본 OFF이며 새 진단 세션을 만들기 전에 `(setq eam-record-terminal t)`로 켤 수 있다. 자세한 내용은 [기록 사용법](docs/user-guide.md#13-cli-저장-대화-조회)을 참고한다.

[전체 명령과 모드 설명](docs/user-guide.md), [설치·기록 위치·제거](docs/package-testing.md), [현재 검증 범위](docs/completion-audit.md).

## 초기 일반 버퍼 실험 — 가짜 스트림

아래는 터미널과 비교하기 위해 보관한 초기 실험이다. `gui.sh`는 가짜 스트림, 과거 JSON 연결 실험은 읽기 전용이었다. 위 대화형 CLI의 기능 제한을 설명하는 부분이 아니다.

```sh
cd ~/workspace/emacs-ai/eam
bash scripts/gui.sh
```

macOS `open -n -a`로 `/Applications/Emacs.app`의 새 인스턴스를 전면 실행하고 `-Q`를 전달한다. 스크립트는 실행 요청 후 반환한다. 기존 init.el, early-init.el, 패키지 설정은 수정하지 않는다. 창 제목은 `eam — offline prototype`이며 `*AI input*`/`*AI output*` 버퍼가 열린다.

이 개발용 실행기는 tests/support/eam-demo.el을 명시적으로 로드한다. 가짜 스트림 명령은 배포 패키지에 없다.

입력 버퍼에 한글을 입력하고 `C-c C-c`로 전송한다. 약 5초간 한글·이모지·코드 블록·긴 로그 줄이 출력된다. 입력은 전송 후 그대로 남으며 일반 undo(`C-/`)를 사용할 수 있다. `M-x eam-demo-new`으로 세션을 추가한다.

전송·중지·기록 이동은 아래 키와 명령으로 실행한다. live 헤더는 디스크에 저장된 전체 바이트(`saved … B`)와 현재 출력 문자 수(`shown … chars`)를 각각 갱신한다.

| 위치 | 키/명령 | 동작 |
| --- | --- | --- |
| 입력 | `C-c C-c` | 해당 세션의 응답 시작; 진행 중 중복 전송 거절 |
| 입력 | `C-c C-k` | 응답 중지 |
| 출력 | `p` | 처음에는 최근 기록을 고정해서 표시; 다시 누르면 이전 페이지 |
| 출력 | `n` | 다음 페이지; 기록 끝에서 live로 복귀 |
| 출력 | `g` | 최신 기록으로 복귀하고 끝으로 이동 |
| 출력 | `i` / `s` | 입력 버퍼 / 응답 중지 |
| 출력 | `q` | 두 버퍼와 타이머 정리; 디스크 기록 유지 |
| 전체 | `M-x eam-history-open` | 저장된 txt를 제한된 페이지로 다시 열기 |

보관 기록을 다시 연 세션은 조회 전용이다. 개별 버퍼를 죽여도 해당 세션의 타이머는 중지된다. 남은 버퍼는 `q` 또는 일반 버퍼 종료로 정리할 수 있다.

## 초기 JSON 연결 실험의 위치

초기 읽기 전용 어댑터와 검증 기록은 보관한다. 새 실제 연결은 위의 `eam-new`를 사용한다. 과거 연결 조건과 측정은 [연결 검증 기록](docs/connections.md)을 참고한다.

## 일반 버퍼·JSON 경로의 기록과 상한

- 전송한 사용자 입력과 생성된 응답 전체는 `var/session-*.txt`에 UTF-8로 추가 기록한다. 미전송 초안은 기록하지 않는다.
- 출력 버퍼 상한은 기본 **65,536 문자**다. 바이트/RSS 상한이 아니며 한글·이모지의 실제 메모리 사용량은 더 크다.
- 가짜 출력은 50ms마다 코드와 로그 8줄을 쓴다. 실제 연결은 받은 출력을 먼저 디스크에 쓰고 50ms마다 제한된 최신 구간을 화면에 반영한다. 전체 응답이나 무제한 대기열을 메모리에 보관하지 않는다.
- 과거 페이지는 최대 `상한 - 4` 바이트에 UTF-8 경계용 최대 3바이트만 더 읽어 교체한다. 따라서 한글 기록은 live 버퍼보다 한 페이지의 문자 수가 적을 수 있다. 페이지들을 합치면 원문이 복원된다.
- 과거 페이지를 보는 동안 화면은 고정되고 새 출력은 디스크에만 쌓인다. live 화면에서 오래된 내용이 잘리는 것은 의도된 동작이다.
- 출력 undo는 비활성화한다. 입력 버퍼와 입력 undo는 제한하지 않으므로 큰 초안·반복 편집은 별도로 메모리를 늘릴 수 있다.
- 파일은 동기식 append다. 느린 디스크에서 UI가 멈출 수 있으며 전원 장애에 대한 fsync·트랜잭션 보장은 없다. 디스크 오류가 나면 생성기를 멈추고 오류를 표시한다.
- Markdown과 ANSI 해석, 도구 승인 UI, 검색 인덱스, Emacs 재시작 후 원격 세션 재개는 이번 범위 밖이다. 기록 파일의 외부 수정·동시 작성은 지원하지 않는다.

값을 바꿔 시험하려면 별도 인스턴스에서 `M-:`로 다음을 평가한다.

```elisp
(setq eam-response-batches 1000) ; 약 50초
```

이후 `M-x eam-demo-stress RET 12 RET`로 12개 가짜 스트림을 동시에 시작하고 입력·스크롤을 시험할 수 있다. 기존 세션에 추가되므로 부하 실험은 새 `-Q` 인스턴스에서 시작한다.

`eam-buffer-limit`, `eam-interval`, `eam-directory`는 세션 생성 전에 설정한다. 상한의 최소 실효값은 128 문자다.

## 재현과 다음 단계

```sh
bash scripts/test.sh # GUI·실제 AI 호출 없음; 첫 Cargo 빌드는 의존성 다운로드 가능
/Applications/Emacs.app/Contents/MacOS/Emacs --batch -Q -L lisp \
  -l tests/benchmark.el > docs/benchmark.json
```

테스트와 측정은 임시 디렉터리에 기록하고 종료 시 해당 임시 기록만 삭제한다. 일반 GUI 세션 기록은 유지된다.

[측정 결과와 GUI 검증표](docs/validation.md), [참고 개념과 실제 연결 전 확인 사항](docs/next-steps.md)을 참고한다. 자동 입력을 일반 text-mode와 대조하여 0/1/12 스트림에서 동일한 결과를 확인했다. Computer Use의 완성형 타이핑·붙여넣기 전달 문제도 분리했지만 실제 키보드 IME 검증은 남아 있다. 추가 재현이 필요하면 [키보드 확인 절차](docs/keyboard-handoff.md)를 사용한다. 사용자는 직접 확인 후 “별 문제 없는 것 같아”라고 보고했다. 세부 IME 항목을 모두 통과한 것으로 확대 해석하지 않는다. 실제 연결 결과는 [연결 검증 기록](docs/connections.md)을 참고한다.

Computer Use가 기존 Emacs 인스턴스만 선택하면 `python3 scripts/prepare-gui-test.py`로 테스트 전용 앱 ID를 준비한다. 출력된 앱 경로 또는 `local.eam.gui-test`를 지정한다. 설치된 Emacs 실행 파일 하나를 복사하고 나머지 리소스를 참조한다. 별도 `-Q` 인스턴스에만 `tests/gui-driver.el`을 로드하며 `AI Test` 메뉴를 추가한다. 앱은 `var/`에만 생성되고 개인 설정과 파일 연결은 변경하지 않는다. Emacs 설치본이 업데이트되면 테스트 앱도 새로 준비해야 한다.
