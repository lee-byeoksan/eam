# EAM — M-x와 키보드 사용 안내

2026-09-13 · 키보드 중심 버전 **0.1.0** · Ghostel 0.53.0. 모든 기본 흐름을 `M-x` 명령과 키로 사용한다. 메뉴 조작은 필요 없다. 아래 `C-c a e`는 Control+c, a, e를 차례로 누른다는 뜻이다.

사용 인터페이스의 기준은 **M-x 명령과 이에 연결한 키**다. 새 기능도 M-x로 실행할 수 있게 제공하고, 자주 사용하는 동작에는 키를 연결한다. 사용 안내에는 명령 이름, 키, 실행할 버퍼와 필요한 후속 동작을 함께 적는다. GUI 메뉴는 추가하지 않는다.


## 기본 M-x 명령 — 18개

기본 M-x 후보에는 아래 명령을 표시한다. `eam` 시작 화면은 없다. 나머지 보조 동작은 기존 키로 계속 사용할 수 있다. 아래 전체 키 표의 함수 이름은 동작 식별용이며, 이 목록 밖의 명령은 기본 M-x 자동완성에서 숨긴다. Emacs의 `read-extended-command-predicate`를 nil로 바꾼 설정에서는 숨긴 명령도 표시될 수 있다.

| 용도 | 명령 |
|---|---|
| 세션 | `eam-new`, `eam-sessions`, `eam-rename`, `eam-attach`, `eam-detach`, `eam-quit` |
| 재개·상태 | `eam-resume`, `eam-status` |
| 입력 편집 | `eam-edit-input`, `eam-draft`, `eam-paste` |
| 기록 | `eam-history`, `eam-history-open`, `eam-prompt` |
| 알림·리뷰 | `eam-notifications`, `eam-review-selection` |
| 설정·도움말 | `eam-caffeine-mode`, `eam-keys-mode`, `eam-help` |

방향키·Enter·Escape·Ctrl-C는 터미널에서 직접 입력한다. worktree 기능은 `C-c a w` 아래의 키를 쓴다. 가짜 스트림 명령은 테스트 전용 파일로 분리하여 배포 패키지에서 제외한다.

대문자 접두사는 제거했다. worktree는 `C-c a w …`, 알림은 `C-c a m`이다. `C-c a n`은 새 세션, 이전의 선택 영역 초안 추가는 충돌을 피하도록 `C-c a y`로 이동했다.

## 1. 실행과 키 활성화

```sh
bash ~/workspace/emacs-ai/eam/var/package-profile-monorepo-0.1.0/start.sh
```

새 독립 GUI가 열리고 키 바인딩이 활성화된다. 개인 Emacs 설정은 수정하지 않는다. 소스 실행은 `bash ~/workspace/emacs-ai/eam/scripts/start.sh`다. 두 방식의 기록 폴더는 다르다.

이미 설치한 패키지를 본인 Emacs에서 사용할 경우 `M-x eam-keys-mode`로 키를 켠다. 같은 명령으로 끄면 이전 바인딩이 다시 드러난다. 패키지를 로드하는 것만으로 전역 키를 활성화하지 않는다. 시작 참조 화면과 `eam` 명령은 제거했다. `M-x` 명령은 키 모드를 켜지 않아도 사용할 수 있다.

`M-x`는 Option+x이며 환경에 따라 Esc 다음 x도 사용할 수 있다. `RET`는 Return이다. **Ghostel char 모드에서는 `M-RET`(Option+Return)으로 기본 입력 모드에 돌아온 뒤 Emacs 명령을 실행한다.**

## 2. 프로젝트에서 Claude 또는 Codex 시작

- **`C-c a n` / `M-x eam-new`** → provider를 자동완성으로 선택 → 프로젝트 선택 → 이름 입력 (생략 가능)

`Project directory:`에 실제 작업 폴더를 입력한다. TAB으로 완성, RET으로 선택, `C-g`로 취소한다. 시작 화면은 호출한 버퍼의 작업 폴더를 기본값으로 유지한다. 패키지 설치 폴더를 프로젝트로 선택할 필요는 없다.

CLI 준비가 끝나면 입력란에 요청을 쓰고 RET으로 보낸다. 파일 수정·테스트·승인은 CLI가 처리한다. 폴더 신뢰나 실행 승인이 나오면 경로와 작업 내용을 확인한 뒤 선택한다. 앱은 자동 승인하지 않는다.

제공자별 시작 명령 대신 `eam-new`에서 provider와 프로젝트를 고른다. 프로젝트 입력란에는 호출한 버퍼의 현재 `default-directory`가 미리 채워진다. `eam-resume`도 동일하다. 이름은 비워도 되며 나중에 바꿀 수 있다.

모든 새 CLI는 지속 세션이다. Emacs를 껐다 켜면 **`C-c a a`**로 분리된 세션을 목록에서 골라 재접속한다. `C-c a d`는 화면만 닫고, **`C-c a q`**는 CLI를 종료한다.

## 3. CLI에 쓰던 입력 편집

1. CLI의 일반 요청 입력란에서 **`C-c a e`** (`M-x eam-edit-input`).
2. 열린 일반 Emacs 버퍼 상단의 저장·반환·제출 안내를 확인하고 내용을 수정한다. 한글·여러 줄·일반 undo를 사용한다.
3. **`C-c C-c`**로 저장하고 CLI에 반환한다. 취소는 **`C-c C-k`**: 중간에 저장했어도 편집 전 입력으로 복원한다.
4. CLI 입력란의 내용을 확인한 뒤 RET으로 제출한다.

승인 목록이나 실행 중 화면에서는 일반 입력란으로 돌아온 뒤 사용한다. 이 기능은 CLI의 외부 편집기 기능이다. 출력 화면을 읽어 입력 내용을 추측하지 않는다.

## 4. 새 보조 초안 작성·전송

- **`C-c a b`** / `eam-draft`: 해당 세션의 별도 초안 열기.
- **`C-c a p`** / `eam-paste`: 초안 전체를 CLI에 붙여넣기. Enter는 보내지 않음.
- **`C-c a t`** / `eam-focus`: 터미널 확인.

초안은 현재 CLI 입력을 자동으로 가져오지 않는다. 전송은 기존 CLI 입력·초안을 지우지 않으므로 반복하면 중복될 수 있다. 64KiB까지 전송하며 여러 줄은 CLI가 bracketed paste를 활성화한 경우에만 허용한다.

초안에 한정된 기존 키도 유지된다: `C-c C-c` 전송, `C-c C-z` 터미널 이동 후 Enter로 제출한다. **터미널에서 `C-c C-c`는 Ctrl-C, `C-c C-z`는 Ctrl-Z**이므로 공통 키 `C-c a …`와 구분한다.

## 5. 세션 전환·대화 재개

**`C-c a l` / `M-x eam-sessions`**는 실행 중인 CLI 세션을 보여준다. `eam-directory/persistent/`의 기록과 현재 연결된 세션 경로를 조회하므로 Emacs 화면을 닫았거나 Emacs를 재시작한 뒤에도 살아 있는 세션을 찾을 수 있다. 종료된 세션은 제외한다.

전용 `*EAM sessions*` 버퍼는 기본적으로 같은 디렉터리의 세션을 묶고 최근 PTY 입출력 순으로 표시한다. `attached`/`detached` 상태와 별도로 `Notice` 열은 `NEW`(안 읽음), `read`(확인함), `—`(알림 없음), `unknown`(알림 정보 없음)을 보여준다. 창을 열어 두었다는 이유만으로 확인 처리하지 않는다.

| 목록 키 | 동작 |
| --- | --- |
| `RET` | 선택한 세션으로 이동/재접속하고 목록에 표시된 시점까지의 알림 확인 |
| `g` | 실행 상태·활동 시각·알림 갱신 |
| `s` | 정렬 기준 자동완성 선택 |
| `r` | 이름 변경 (`M-x eam-rename`은 CLI·보조 입력 버퍼에서도 사용 가능) |
| `m` | 화면 이동 없이 선택한 세션의 알림 확인 |
| `t` | 디렉터리 그룹 켜기/끄기 |
| `f` | 전체 활성 / detached만 보기 전환 |
| `q` | 목록 닫기; CLI는 유지 |

정렬은 `activity`(최근 PTY 입출력), `created`(최근 생성), `directory`(경로), `name`(사용자 이름), `provider`, `state`(attached 우선), `unread`(안 읽은 알림 우선)를 제공한다. 그룹이 켜져 있으면 그룹 내 첫 세션의 순위로 그룹을 정렬한다. 다른 기준에서도 동률이면 최근 입출력, 관리 ID 순이다. 전체 세션을 한 줄의 순위로 비교하려면 `t`로 그룹을 끈다. 목록은 수동 갱신이며 `g` 또는 `C-c a l`로 최신 상태를 조회한다.

`Last PTY I/O`는 데몬이 실제 입력을 CLI에 전달하거나 출력을 받은 시각이다. 터미널 질의 응답·화면 갱신도 포함하므로 **마지막 사용자 요청/AI 응답의 의미상 시각과 같지는 않다**. 새 기능 이전부터 실행 중인 데몬의 활동 시각은 `—`로 표시하고 생성 시각으로 정렬한다. 생성 시각이 없는 구버전 기록은 메타데이터 파일 시각을 대용한다. 재접속 화면 재생과 크기 조정 자체는 활동으로 세지 않는다.

세션 이름은 한글 포함 최대 128자이며 제어 문자를 허용하지 않는다. 빈 이름은 자동 이름으로 복원한다. 중복 이름은 허용하고 관리 ID로 구분한다. 이름 변경은 CLI 프로세스나 대화 UUID를 바꾸지 않는다. `~/.eam/persistent/<관리 ID>/display.json`에 별도로 저장되므로 재접속·Emacs 재시작 후 유지된다 (`eam-directory` 설정에 따라 경로 변경).

읽음 위치는 같은 디렉터리의 `notification-read.json`에 저장한다. 분리된 세션도 데몬이 기록한 알림을 목록에서 확인할 수 있다. `RET`/`m`은 조회 당시 알림까지만 확인하므로 그 후 도착한 알림은 안 읽음으로 남는다. 목록 열기·단순 갱신·다른 Emacs에 연결되어 이동하지 못한 경우는 읽음 처리하지 않는다. `C-c a m`의 알림함에서 `RET`은 해당 알림까지 확인하고, `r`로 안 읽음으로 되돌리면 해당 알림 이후도 안 읽음이 된다. 확인은 사용자가 실제 내용을 읽었다는 자동 판정이 아니라 명시적인 이동/확인 기록이다.

기본값을 use-package에서 바꾸려면 기존 설정의 `:custom`에 추가한다.

```elisp
:custom
(eam-session-sort-order 'activity) ; unread, created, directory, name, provider, state
(eam-session-group-by-directory t)
```

설정은 목록 버퍼를 처음 만들 때 적용한다. 이미 열린 목록에서는 `s`와 `t`로 바꾼다.

분리된 세션만 보려면 **`C-u C-c a l`** 또는 **`C-u M-x eam-sessions`**를 실행한다. 현재 세션을 분리하려면 **`C-c a d`** (`eam-detach`)를 사용한다. CLI는 유지되지만 보조 초안은 닫히므로 미저장 초안은 먼저 보관한다.

`attached`는 연결 중, `detached`는 화면 없이 실행 중이다. 현재 Emacs에 연결된 항목은 화면으로 이동하고, detached 항목은 같은 CLI 프로세스에 재접속한다. 다른 Emacs에 연결된 항목은 그쪽에서 먼저 분리해야 한다. 선택을 취소하면 아무 연결도 바뀌지 않는다.

상태를 확인하지 못한 세션은 종료로 간주하지 않고 목록 상단의 `unverified` 개수로 알린다. 필요하면 `eam-status`로 확인한다. 조회는 최대 256개, 최대 10초이며 기다리는 동안 C-g로 취소할 수 있다. 사용자 이름·provider·프로젝트·관리 ID로 행을 구분한다. 조회만으로 AI 요청이나 CLI 재시작은 발생하지 않는다.

**`C-c a r` / `M-x eam-resume`** → 제공자 → 기존 프로젝트 → 이름 (생략 가능)을 입력하면 해당 CLI의 저장된 대화 목록이 열린다. 목록에서 이어갈 대화를 선택한다. UUID를 직접 입력하지 않는다. 이미 열린 CLI에서는 `/resume`을 사용할 수 있다.

동일 대화를 다른 CLI가 열고 있으면 그 CLI를 먼저 정상 종료한다. 앱이 모든 Emacs 인스턴스의 중복 writer를 감지하지는 않는다. 기록 파일 열기는 대화 재개와 다르다.

## 6. CLI 명령·확인·중단

CLI 입력란의 `/model`은 모델 선택, `/resume`은 대화 재개다. 전체 제공 명령은 해당 CLI에서 `/`를 입력해 확인한다. CLI 버전에 따라 달라지며, 모델 선택의 적용 범위·저장 여부는 CLI 안내를 따른다.


Claude는 `/tasks`, Codex는 `/ps`로 작업을 확인하고 필요하면 Codex `/stop`을 사용한다. Claude `/goal`은 14절을 참고한다. 헤더의 `running`은 CLI 프로세스 생존을 뜻하며 모델 응답 생성 중이라는 뜻은 아니다.

## 터미널에서 긴 출력 스크롤

PTY 데몬은 CLI 출력을 Ghostel에 전달한다. 마우스 휠/트랙패드로 스크롤하며, CLI 자체의 mouse mode/alternate screen 사용 여부에 따라 Ghostel 또는 CLI가 처리한다. PTY 데몬은 이 모드를 강제로 켜지 않는다.

## 7. 기록 조회·복사

`C-c a h`는 현재 CLI의 프로젝트에서 저장된 대화를 선택하고, `C-c a o`는 provider부터 선택한다. CLI가 저장한 사용자 요청·응답을 일반 버퍼에서 읽는다. `p`/`n`은 페이지 이동, `g`는 최신 기록 갱신, `q`는 조회 종료다. `C-s`로 현재 페이지를 검색하고 `C-SPC`, `M-w`로 영역을 복사한다.

EAM은 기본적으로 원시 터미널 기록을 저장하지 않는다. CLI 자체 저장 위치와 진단 기록을 켜는 방법은 [13절](#13-cli-저장-대화-조회)을 참고한다. 기존 진단 파일은 `C-u C-c a o`로 열 수 있다.

## 8. Caffeine·종료

**`C-c a f` / `M-x eam-caffeine-mode`**는 현재 Emacs의 유휴 잠자기 방지를 켜거나 끈다. 켜지면 모드라인에 `Caffeine`이 표시된다. 기본 꺼짐이며 화면 잠자기는 허용한다. Emacs 종료 시 해제되고 뚜껑 닫기·강제 잠자기까지 모두 막지는 않는다.

정상 종료는 Claude **`/exit`**, Codex **`/quit`**다. 남은 작업을 먼저 확인하고 CLI가 작업 정리 선택을 표시하면 원하는 종료 범위를 선택한다. CLI 종료가 확인되면 해당 백엔드의 데몬/서버와 터미널 버퍼를 자동 정리한다. 미저장 초안과 디스크 기록은 보존한다. 실행 중 **`C-c a d` / `eam-detach`**를 쓰면 화면만 분리하고 CLI는 유지하며 미저장 초안은 사라진다. 명시적으로 서버까지 종료하려면 `C-c a q`를 사용한다. 디스크 기록은 유지된다.

초안만 일반 Emacs 방식으로 닫으면 CLI는 계속 실행되며 새 초안을 열 수 있다. 터미널 버퍼 자체를 죽여도 지속 CLI는 유지된다. 창 표시만 닫는 `C-x 0`과 구분한다.

## 9. 도움말·키 확인·문제 해결

**`C-c a ?` / `M-x eam-help`**는 이 안내를 연다. 사용 안내는 읽기 전용으로 열리며 `q`로 이전 화면에 돌아온다. `C-h k` 다음 키를 누르면 현재 매핑, `C-h f` 다음 명령 이름을 넣으면 명령 설명, `C-h b`는 현재 키 바인딩을 보여준다.

`C-c a`가 다른 기능과 겹치면 `M-x eam-keys-mode`로 끄고 필요한 `M-x` 명령을 직접 사용한다. 자체 설정에서 `eam-command-map`을 원하는 접두사에 연결할 수도 있다. 개인 init을 앱이 자동 수정하지 않는다.

세션 관련 명령은 **해당 터미널 또는 초안 버퍼에서** 실행한다. 일반 파일 편집 중에는 `C-c a l`로 대상 세션을 고른다. 키가 CLI로 들어가면 char 모드인지 확인하고 M-RET으로 복귀한다. 초안 전송 뒤 응답이 없으면 CLI 내용과 Enter 제출 여부를 확인한다. 연결 실패는 CLI 안내에 따라 중단·재개하고, 기록 쓰기 오류는 디스크 공간과 기록 폴더 접근을 확인한다.

설치·제거는 [패키지 안내](package-testing.md), 기록 백업은 12절, 전체 검증 범위는 [현황](completion-audit.md)을 참고한다.

## 10. 터미널 모드·복사·스크롤·Emacs 창

**현재 포커스가 있는 버퍼에 따라 같은 키의 의미가 달라진다.** 터미널의 `C-c C-c`는 CLI에 Ctrl-C를 보내지만, 보조 초안의 `C-c C-c`는 초안을 전송한다. `C-c C-z`도 터미널에서는 Ctrl-Z이고 초안에서는 터미널로 이동한다. 일관된 동작이 필요하면 `C-c a` 아래의 키 또는 해당 `M-x` 명령을 사용한다.

Ghostel은 터미널 버퍼에서 다음 모드를 제공한다. 아래 키는 Ghostel 0.53.0 기본 매핑이며 개인 설정에 따라 달라질 수 있다.

| 모드 | 진입 | 용도와 돌아오는 방법 |
| --- | --- | --- |
| 기본 입력, semi-char | `C-c C-j` / `M-x ghostel-semi-char-mode` | 대부분의 키는 CLI에 보내고 `C-x`, `M-x` 등은 Emacs에 남긴다. 다른 모드에서 일반 CLI 입력으로 돌아올 때 사용 |
| 복사, copy | `C-c C-t` / `M-x ghostel-copy-mode` | 표시 갱신을 고정하고 읽기 전용으로 선택·검색. `C-c C-j`로 기본 입력 복귀. `q`도 기본 설정에서 복귀 |
| Emacs 읽기 모드 | `C-c C-e` / `M-x ghostel-emacs-mode` | 터미널 출력은 계속 갱신하면서 Emacs 탐색·검색·선택 사용. 커서 위치에서 벗어나면 화면 자동 추적을 멈춤. 같은 명령 또는 `C-c C-j`로 복귀 |
| 모든 키를 CLI에 전달, char | `C-c M-d` / `M-x ghostel-char-mode` | `M-x`·`C-c`까지 CLI에 전달. **`M-RET`(Option+Return) 또는 `C-M-m`으로 기본 입력 복귀**. 이 모드에서 `C-c C-j`는 탈출 키가 아님 |
| Ghostel 줄 편집, line | `C-c C-l` / `M-x ghostel-line-mode` | Ghostel 자체의 줄 편집 기능. 이 앱의 보조 초안과 다른 기능이며 AI CLI 사용 검증 범위에는 포함하지 않음. `C-c C-j`로 복귀 |

복사 모드의 기본 빠른 복귀 설정에서는 일반 문자를 누르면 모드를 빠져나가 그 키가 CLI로 전달될 수 있다. 복사 화면에서 시험 문장을 타이핑하지 말고, 입력하려면 먼저 기본 입력 모드로 돌아온다. 복사 모드의 표시 정지는 AI 작업을 취소하는 기능이 아니다.

**터미널에서 텍스트 복사하기:** 복사 또는 Emacs 읽기 모드 → `C-SPC`로 선택 시작 → 이동 → `M-w`로 복사. 선택한 내용을 보조 초안에 넣으려면 `C-c a y` (`M-x eam-draft-add-selection`)를 사용한다. 이 명령은 활성 선택 영역이 있는 터미널에서만 사용할 수 있다.

| 추가 Ghostel 기능 | 기본 키 / 명령 | 범위 |
| --- | --- | --- |
| 붙여넣기 | `C-y` / `ghostel-yank` | Emacs kill ring 내용을 CLI에 붙여넣기. 제출 여부는 화면 확인 |
| 시스템 클립보드 붙여넣기 | `C-c C-y` / `ghostel-paste` | CLI의 붙여넣기 처리에 따름 |
| 보유 중인 전체 터미널 텍스트 복사 | `C-c M-w` / `ghostel-copy-all` | 디스크의 전체 대화가 아니라 현재 터미널이 보유한 내용 |
| 다음 키를 CLI에 그대로 보내기 | `C-q` / `ghostel-send-next-key` | 기본 semi-char 모드에서 Emacs가 잡는 키 전달 |
| 터미널 스크롤백 비우기 | `C-c M-l` / `ghostel-clear-scrollback` | 터미널의 표시 기록 정리. 앱의 디스크 기록 삭제 기능은 아님 |
| 다음/이전 링크 이동 | `C-c C-n` / `C-c C-p` | 터미널이 인식한 링크 사이 이동, 모든 출력이 링크가 되는 것은 아님 |

마우스 휠로 이전 출력을 보고, 기본 입력 모드로 돌아가면 최신 입력 위치로 이동한다. CLI가 마우스 이벤트를 처리하면 휠 동작이 달라질 수 있다. 원하는 과거 내용이 터미널에 없으면 7절의 **Open raw archive**에서 검색한다. 이 앱은 Ghostel의 shell integration을 비활성화하므로 셸 프롬프트 단위 이동에 의존하지 않는다.

Emacs 창 조작은 기본 입력 또는 일반 편집 버퍼에서 사용한다. `C-x o`는 다음 창, `C-x 2`는 위아래 분할, `C-x 3`은 좌우 분할, `C-x 1`은 현재 창만 표시, `C-x 0`은 현재 창 표시를 닫는다. **창 표시를 닫는 것은 버퍼나 CLI를 종료하는 것과 다르다.** `C-x b`로 버퍼를 바꾸고 `M-x eam-sessions`로 세션을 선택한다.

## 11. 공통 키와 M-x 명령 전체

`C-c a` 접두사는 키 모드를 활성화한 Emacs 전체에서 사용할 수 있다. 세션 명령은 대상 터미널/초안에서 실행하며 char 모드에서는 먼저 M-RET으로 복귀한다.

| 키 | M-x 명령 | 기능 |
| --- | --- | --- |
| `C-c a n` | `eam-new` | provider 선택 후 지속 CLI 시작 |
| `C-c a l` | `eam-sessions` | 실행 중인 세션 선택·재접속 |
| `C-c a r` | `eam-resume` | 대화 재개 |
| `C-c a e` | `eam-edit-input` | CLI 미제출 입력 편집 |
| `C-c a b` | `eam-draft` | 보조 초안 |
| `C-c a y` | `eam-draft-add-selection` | 선택 영역을 초안에 추가 |
| `C-c a p` | `eam-paste` | 초안 붙여넣기, Enter 제외 |
| `C-c a t` | `eam-focus` | 터미널 포커스 |
| `C-c a u` | `eam-prompt` | 현재 세션의 최근 제출 프롬프트 조회 |
| `C-c a h` | `eam-history` | 현재 기록 조회 |
| `C-c a o` | `eam-history-open` | 저장 기록 선택 |
| `C-c a d` | `eam-detach` | 화면 분리, CLI 유지 |
| `C-c a f` | `eam-caffeine-mode` | Caffeine 토글 |
| `C-c a a` | `eam-attach` | 지속 CLI에 연결 |
| `C-c a q` | `eam-quit` | 지속 CLI 서버 종료 |
| `C-c a i` | `eam-status` | 지속 세션 상태 확인 |
| `C-c a w c` | `eam-worktree-create` | worktree 생성 |
| `C-c a w o` | `eam-worktree-open` | worktree 파일 탐색 |
| `C-c a w s` | `eam-worktree-start` | worktree에서 CLI 시작 |
| `C-c a w d` | `eam-worktree-remove` | worktree 삭제 |
| `C-c a v` | `eam-review-selection` | 선택한 diff의 리뷰 의견 작성 |
| `C-c a m` | `eam-notifications` | 알림 목록 |
| `C-c a ?` | `eam-help` | 사용 안내 |

키가 기억나지 않으면 `M-x eam-`까지 입력하고 `TAB`으로 명령을 찾는다. 위 표의 명령 이름을 그대로 입력해도 된다. 기록 조회·초안·알림 목록 등 버퍼 전용 키는 각 기능 설명에서 확인한다. `C-h m`은 현재 버퍼의 모드와 키를 보여준다.

개인 설정을 바꾸지 않고 접두사를 시험하려면 `M-:`에서 아래 식을 실행한다. 기존 `C-c a` 매핑은 비우고 이 패키지의 키 모드에만 `C-c i`를 연결한다. 키 모드를 끄면 이 매핑도 비활성화된다. 별도 실행 인스턴스를 종료하면 변경은 사라진다.

```elisp
(progn
  (require 'eam-app)
  (define-key eam-keys-mode-map (kbd "C-c a") nil)
  (define-key eam-keys-mode-map (kbd "C-c i") eam-command-map)
  (eam-keys-mode 1))
```

앱은 GUI 메뉴 항목을 추가하지 않는다. 실행할 명령은 `M-x` 또는 단축키로 선택한다.

## 12. 설정·저장 위치·백업

`M-x customize-group RET eam RET` 또는 `C-h v 변수이름`으로 설정과 설명을 확인한다. 현재 세션에만 시험하려면 Customize의 적용 기능을 사용하고 저장 여부를 구분한다. 제공한 `-Q` 실행은 개인 init을 읽지 않으므로 Customize에서 저장한 설정의 다음 실행 자동 적용을 가정하지 않는다.

| 변수 | 기본값 / 적용 범위 |
| --- | --- |
| `eam-keys-mode` | `M-x eam-keys-mode`로 켜짐. 단독 로딩은 꺼짐. 끄면 기존 키 복원 |
| `eam-directory` | 일반 패키지: `user-emacs-directory/eam/`. 독립 소스 런처: 프로젝트 `var/`. 새 세션 생성 전에 지정. 기존 기록 자동 이동 없음 |
| `eam-buffer-limit` | 일반 출력/과거 기록 페이지 65,536자, 최소 실효 128자. Ghostel 터미널 전체 메모리 상한은 아님 |
| `eam-record-terminal` | nil. 새 세션의 원시 터미널/명시적 입력 진단 기록만 선택적으로 켬 |
| `eam-terminal-redraw-delay` | 터미널 재표시 묶음 지연. 기본 0.016초(16ms) |
| `eam-interval` | 일반 버퍼 출력 갱신 0.05초. 터미널의 갱신 설정과 구분 |
| `eam-emacsclient-executable` | nil이면 현재 Emacs 설치와 PATH에서 탐색. 외부 편집기가 다른 Emacs에 연결되면 올바른 실행 파일 경로 확인 |
| `eam-native-auto-build` | t. 첫 native 사용 시 누락된 바이너리 빌드. require/설치 시 실행 안 함 |
| `eam-native-cargo-executable` | "cargo". 빌드용 Cargo 경로; 기본은 PATH 또는 ~/.cargo/bin/cargo |
| `eam-native-executable` | nil. 자동 빌드 캐시 사용. 미리 빌드한 파일은 절대 경로로 지정 |
| `eam-caffeine-mode` | 기본 nil. 수동 활성화·해제이며 CLI가 열린 것만으로 켜지지 않음 |

터미널은 기본 16ms의 재표시 묶음 지연과 Ghostel 스크롤백 예산 5MiB를 사용한다. `eam-terminal-redraw-delay`는 초 단위이며, 낮추면 응답성을 우선하고 높이면 재표시 작업량을 줄인다. 입력 직후 등에는 Ghostel이 더 일찍 그릴 수 있으므로 고정 프레임 속도를 보장하는 값은 아니다. 변경은 새 터미널 버퍼부터 적용된다. 기존 세션은 `eam-detach` 후 `eam-attach`하면 실행 중인 CLI를 유지하면서 적용할 수 있다. 이 값은 전체 Emacs 메모리나 화면의 문자 수 제한이 아니다. 입력 버퍼의 크기와 undo는 별도이며 긴 초안은 메모리를 사용할 수 있다.

진단 기록을 명시적으로 활성화했을 때 저장되는 `terminal-*.ansi`는 CLI에서 **수신한 터미널 출력**, `.ansi.input.jsonl`은 앱 명령·초안 등 명시적 입력 동작의 기록이다. 모든 직접 키 입력이나 CLI 자체 대화 DB의 완전한 사본은 아니다. CLI 종료 뒤 아래처럼 필요한 원본을 백업할 수 있다. `<...>` 부분은 실제 경로로 바꾸고 백업 폴더를 먼저 준비한다.

```sh
cd ~/workspace/emacs-ai/eam
cp -n '<원본 terminal 파일.ansi>' '<백업 폴더/대화.ansi>'
```

내보내기는 원본을 삭제하지 않고 기존 대상 파일을 덮어쓰지 않는다. 입력 JSONL도 필요하면 별도로 내보낸다. `cp -n`은 기존 대상 파일을 덮어쓰지 않는다.

바이너리가 없으면 자동으로 백그라운드 빌드를 시작하고 `*EAM native build*`에 로그를 표시한다. 빌드 중에도 Emacs를 사용할 수 있다. 완료 후 원래 EAM 명령을 다시 실행한다. 로그 버퍼에서 `C-c C-k`로 취소하며 `M-x eam-native-build`로 수동 빌드·재시도한다. 캐시는 `eam-directory/native/` 아래 플랫폼·소스 해시별로 저장한다. `~/.eam/` 설정과 use-package 예시는 [native runtime 안내](native-runtime.md)에 있다.

## 13. CLI 저장 대화 조회

| 실행 | 동작 |
| --- | --- |
| `C-c a h` / `eam-history` | 현재 CLI의 provider·프로젝트에 해당하는 저장 대화 목록에서 선택 |
| `C-c a o` / `eam-history-open` | provider를 고른 뒤 저장 대화 목록에서 선택 |
| `C-c a u` / `eam-prompt` | 현재 프로젝트의 대화를 고른 뒤 마지막 사용자 요청 표시 |
| `C-u C-c a o` | 디버깅용 기존 TXT/ANSI 파일을 직접 선택 |

목록에는 수정 시각·제목·프로젝트·ID를 표시한다. 같은 프로젝트에서 여러 CLI를 실행할 수 있으므로 가장 최근 대화를 현재 CLI의 대화로 단정하지 않는다. 최대 최근 300개를 제공한다.

대화 버퍼의 `p`/`n`은 이전/다음 페이지, `g`는 최신 스냅샷, `q`는 조회 닫기다. `C-s`는 현재 페이지 안에서 검색한다. 전체 대화 검색은 아직 지원하지 않는다. 사용자 요청과 assistant 텍스트만 표시하며 도구 호출·도구 결과·추론·터미널 상태 표시줄은 제외한다. Markdown 원문과 간단한 강조를 제공하며 터미널 화면/색상을 복제하지 않는다.

Claude의 `projects/*/*.jsonl`, Codex의 `state_5.sqlite` 및 `thread_history_1.sqlite` 또는 legacy JSONL을 읽는다. 원본을 수정하거나 EAM에 대화 사본을 저장하지 않는다. `eam-claude-history-directory`, `eam-codex-history-directory`로 CLI 저장 루트를 지정할 수 있다. 기본값은 각각 `CLAUDE_CONFIG_DIR` 또는 `~/.claude`, `CODEX_HOME` 또는 `~/.codex`다.

페이지는 `eam-buffer-limit` 이하이면서 최대 65,536자이며 undo는 꺼진다. 이전 기록도 동일한 제한을 지킨다. 조회는 로컬 worker로 실행하며 `C-g`로 취소할 수 있고 30초 제한이 있다. 자동 폴링은 하지 않으므로 CLI가 새 내용을 저장한 뒤 `g`를 누른다. 저장되지 않은 응답이나 첨부 원본은 표시할 수 없다. CLI 내부 저장 형식 변경, 분기·압축 대화의 해석에는 한계가 있다. 상세 범위는 [검증 기록](native-history.md)을 참고한다.

원시 기록은 기본적으로 꺼져 있다. 필요할 때 아래 설정을 적용하고 **새 세션을 생성**한다. 기존 세션의 recorder 설정은 바뀌지 않는다.

```elisp
(setq eam-record-terminal t)   ; 이후 새 세션의 진단 기록 활성화
;; 진단 세션을 만든 다음 기본값으로 되돌리기:
(setq eam-record-terminal nil)
```

활성화한 세션의 `eam-directory/persistent/<세션>/output.ansi`를 `C-u C-c a o`로 열면 ANSI 색상을 적용한다. 이 진단 뷰어에서 `/`는 파일 전체 검색, `M-n`은 다음 일치, `c`는 색상 코드 표시 전환이다. 기록을 끄더라도 기존 파일은 삭제하지 않는다. 알림 이벤트·세션 관리 정보는 계속 저장한다.

## 14. CLI 자체 기능의 사용 범위

이 앱의 명령은 입력·편집·세션 표시·기록 관리를 제공한다. CLI의 모든 slash 명령을 앱이 재구현하는 구조가 아니다. 현재 설치된 CLI에서 `/`를 입력해 표시되는 명령 목록과 해당 CLI의 도움말을 확인한다. CLI 버전에 따라 메뉴와 기능이 달라질 수 있다.

- `/model`: CLI가 제공하는 모델 선택과 적용 범위를 확인한다.
- `/resume`: CLI 자체 대화 선택·재개. 앱의 기록 페이지와 별개다.
- Claude `/goal <목표>`: 목표 설정 자체로 실행이 시작될 수 있다. `/goal`은 상태 조회, `/goal clear`는 활성 목표 해제다. Escape로 현재 작업을 중단해도 목표는 활성 상태로 남을 수 있다. 네이티브 목표 평가의 모델 호출은 앱의 추가 호출과 별개다.
- 파일 수정·테스트·명령 실행: 자연어 요청을 보내고 CLI가 제시한 변경·승인을 확인한다. 앱이 자동 승인하지 않는다.
- 이미지·첨부·파일 칩 등 비텍스트 기능: 현재 이 앱의 검증 범위에 포함되지 않는다. 텍스트 편집 왕복 성공이 이 기능의 성공을 뜻하지 않는다.

기본 기능이 잘 동작한다는 사용자 관찰은 기록했다. 개별 한글 조합·GUI 부하·CLI 버전별 모든 기능을 확인한 것으로 확대하지 않으며, 불편한 명령이나 누락된 동작은 해당 이름을 알려주면 추가로 확인한다.

## 15. 세션 알림 (0.3.0)

`M-x eam-notifications` / `C-c a m`로 명시적 터미널 알림 목록을 연다.
열린 세션으로 이동한다. 목록에서 `RET` 이동, `r` 읽음 토글, `g` 갱신,
`q` 목록 닫기를 사용한다. 닫힌 세션은 자동 재시작하지 않는다.

알림 목록에서 사용하는 M-x 명령:

| 명령 | 키 | 동작 |
| --- | --- | --- |
| `eam-notifications-visit` | `RET` | 현재 행의 세션으로 이동하고 읽음 처리 |
| `eam-notifications-toggle-read` | `r` | 현재 행의 읽음/미읽음 전환 |
| `eam-notifications-refresh` | `g` | 목록 표시 갱신 |

데스크톱 전달 옵션 `eam-notification-desktop`은 기본 꺼짐이다. 켜면 Ghostel의
알림 백엔드로 전달한다. 별도 `alert` 패키지가 없으면 Emacs 메시지로 표시하며,
현재 격리 프로필에는 `alert`를 설치하지 않았다. macOS 배너는 `alert` 백엔드와
OS 알림 허용 상태에 따라 달라진다. 별도 alert/osx-notifier 시험 GUI에서 실제 배너를
사용자가 확인했다([검증 기록](evidence/gui-087/desktop-probe-user.json)).
`alert`를 설치해도 기본 스타일은 `message`다. 배너를 사용하려면 별도 시험 설정에서
`alert-default-style`을 `osx-notifier`로 지정해야 한다. 이 스타일은 AppleScript의
알림 API를 사용한다. 개인 init이나 OS 설정을 앱이 자동 변경하지 않는다.

기본 최대 100건, 제목·본문 각각 2048자를 메모리에 유지한다. 원래 OSC 바이트는
세션 기록에 남는다. 알림 본문은 CLI 작업 상태를 보증하지 않으며 자동 승인은 없다.
두 CLI의 실제 완료·승인 요청 알림 수신을 확인했다. GUI 검증은 별도로 진행한다. [상세 범위와 설정](notifications.md)을 참고한다.

## 16. 선택한 diff에 리뷰 의견 쓰기 (0.4.0)

1. diff-mode에서 변경 줄을 영역으로 선택한다.
2. `M-x eam-review-selection` 또는 `C-c a v`를 실행한다.
3. 선택 문자 수를 확인하며 의견을 입력하고 대상 CLI 세션을 고른다.
4. 초안에서 파일 경로·변경 전후 줄 위치·선택한 diff·의견을 검토하고 편집한다.
5. `C-c a p`로 붙여넣고, 터미널 확인 후 `C-c a RET`로 제출한다.

1–4단계에서는 CLI에 입력을 보내지 않는다. 초안 작성은 일반 undo로 되돌릴 수 있다.
기본 선택 한도는 12,000자(`eam-review-selection-limit`)다. 넘으면 분량을
표시하고 중단하므로 영역을 줄여 재시도한다. 자동 잘라내기나 전체 diff 첨부는 없다.
문자 수는 토큰 수가 아니다. 위치는 diff에 기록된 줄 번호이므로 이후 파일 편집으로
달라질 수 있다. Magit 4.4.0 실제 diff 버퍼의 이름 변경·접기 처리를 배치 검증했다. 접힌 영역은
펼친 후 다시 선택한다. GUI의 영역 선택·한글 입력은 별도 검증 대상이다.

## 17. Worktree 사용 (0.5.0)

`C-c a w c` 생성, `C-c a w o` 파일 열기, `C-c a w s` CLI 시작,
`C-c a w d` 삭제. 각각 `M-x eam-worktree-create`, `-open`, `-start`,
`-remove` 명령이다. `w`는 소문자다.

생성 시 기준은 로컬 ref/SHA이며 fetch하지 않는다. 선택한 worktree에서 CLI를
명시적으로 시작하고 기존 `C-c a l`로 세션을 전환한다. 삭제는 경로를 확인하며
변경 파일·ignored 파일·실행 중인 앱 세션 등이 있으면 거절한다.
[명령과 검증 범위](worktrees.md)를 참고한다.

## 18. 프로젝트 실행 명령 제거

`eam-task-*`와 `C-c a T` 접두사를 제거했다. 이전 명령 등록 파일과 실행 로그는 자동 삭제하지 않는다.

## 19. Workspace 제거

Workspace 저장·복원 명령과 `C-c a s` 접두사를 제거했다. 기존 JSON 파일은 삭제하지 않는다. 살아 있는 CLI는 `C-c a a`, 종료된 저장 대화는 `C-c a r`로 재개한다.

## 20. Emacs 종료 후에도 CLI 유지하기


```elisp
(use-package eam
  :init
  (setq eam-directory (expand-file-name "~/.eam/")
        eam-terminal-redraw-delay 0.016)
  :config
  (require 'eam-app)
  (eam-keys-mode 1))
```

위 설정은 설치된 패키지 기준이다. 새 설치는 [VC 예시](../examples/eam-vc-init.el)를 참고한다. 모든 CLI는 Rust PTY 데몬으로 실행하며 첫 빌드에만 Rust/Cargo와 C 빌드 도구가 필요하다.

**PTY 재연결 한계:** 시작 출력 5MiB까지만 메모리에서 재생하며, 한도를 넘으면 화면 복원 불가 메시지를 표시한다. CLI 프로세스는 유지된다. 한도 이내여도 화면 크기 변경·terminal query·복잡한 TUI 상태의 정확한 복원은 보장하지 않는다. 이전 대화는 `C-c a h`로 확인한다. 연결 없는 동안 터미널 질의 응답을 기다리는 CLI의 동작은 아직 검증 범위에 포함되지 않았다.

PTY 재연결 기록은 16KiB 패킷으로 나눠 전송하며, 재연결 중에는 최대 약 5MiB의 전송용 복사본이 추가로 필요하다. 5MiB 한도는 새 데몬부터 적용된다. 기존 실행 세션의 PTY 한도를 바꾸려면 CLI를 종료한 뒤 `eam-resume` 또는 `eam-new`로 시작한다. 스크롤백 5MiB는 새 터미널 버퍼부터 적용되며, 이미 잘린 출력은 용량을 늘려도 복구되지 않는다.

창 너비 변경은 입력 대기 중에도 CLI로 전달한다. 기존 출력의 줄바꿈 재구성은 Ghostel과 CLI의 지원 범위에 따른다.

PTY 출력 큐는 256KiB로 제한된다. Emacs가 출력을 소비하지 못해 이 한도를 넘으면 연결을 끊고 CLI는 계속 실행한다. 이때 터미널 표시에는 누락이 생길 수 있으며 `eam-attach`로 다시 연결할 수 있다. 원시 기록은 계속 기본 OFF이고 `eam-record-terminal`을 켠 새 세션만 디스크 기록을 남긴다. 사용자 피드백을 위한 기본 전환이며 실제 GUI·Claude/Codex 재연결의 모든 상태를 검증 완료한 것은 아니다.

| M-x 명령 | 키 | 기능 |
| --- | --- | --- |
| `eam-new` | `C-c a n` | 제공자·프로젝트 선택 후 독립 CLI 시작·연결 |
| `eam-attach` | `C-c a a` | 기록 디렉터리 `persistent/` 아래의 세션 선택·재접속 |
| `eam-detach` | `C-c a d` | 표시만 닫기, CLI 유지 |
| `eam-quit` | `C-c a q` | 확인 후 해당 지속 서버 종료, 기록 유지 |
| `eam-status` | `C-c a i` | 로컬 프로세스 상태 확인 |

종료 명령이 `Stop could not be verified`를 표시하면 서버 종료를 확인하지 못한 것이다.
`C-c a i`로 상태를 확인한다. `unavailable`은 서버 조회 실패이며 종료 성공을 뜻하지 않는다.
이때 앱은 기존 기록·읽기 위치와 worktree 삭제 보호를 유지하고 CLI를 자동 재시작하지 않는다.

연결 문제는 먼저 `C-c a i`로 확인한다.

| 확인한 상태 | 다음 동작 |
| --- | --- |
| `running`, 연결 수 0 | `C-c a a`로 같은 지속 세션에 다시 연결 |
| `running`, 연결 수 1 이상 | 기존 Emacs에서 `C-c a d`로 분리한 뒤 다시 연결 |
| `exited` | `C-c a q`로 남은 서버를 명시적으로 종료. 기록은 `C-c a o`로 조회 |
| `stopped` | 같은 프로세스에 재접속할 수 없음. 저장 대화를 이어가려면 `C-c a r`로 CLI 대화 재개 |
| `unavailable` 또는 runtime/소유권 오류 | 종료 여부 미확인. 기록을 보존하고 원인을 확인. 같은 대화를 중복 실행하거나 메타데이터를 지워 삭제 보호를 우회하지 않음 |

서버가 사라졌을 때 분리된 자식까지 종료됐는지를 앱이 증명하거나 강제로 복구하는 명령은
현재 없다. `unavailable`은 재시도할 수 있는 조회 실패 상태이며 성공한 종료로 취급하지 않는다.
기록 파일 조회와 대화 재개는 원래 프로세스에 다시 붙는 기능과 다르다.

위 연결·분리 명령은 출력 버퍼에서 사용한다. 보조 초안 편집·전송 키는
기존과 같다. 지속 세션의 `C-c a d`도 표시만 닫는다. 독립 CLI 종료는
`persistent-stop` 또는 CLI 자체 종료를 사용한다. 작업 중인 AI는 분리 후에도 토큰을 사용할 수 있다.

동시에 한 Emacs 표시 연결만 허용한다. 다시 붙을 때 외부 편집기의 대상 Emacs도 바뀐다.
기록은 디스크에 유지되며 재접속 화면은 원본에 다시 추가하지 않는다. 알림은 최신의 제한된
분량을 읽는다. CLI의 UUID 대화 재개와는 별개다.

미종료 또는 상태 미확인 지속 세션이 있는 worktree는 삭제를 거절한다.

[구현·검증 범위와 한계](native-runtime.md)를 참고한다. GUI 한글 조합·장시간 다중
세션·장애 복구의 남은 검증이 있으므로 시험용 프로필에서 사용한다. 개인 init은 수정하지 않는다.


## 21. 기본 지속 세션과 버전 전환

EAM은 Emacs AI Management다. 명령·설정 접두사는 `eam`이며 패키지 루트는 `~/workspace/emacs-ai/eam`이다. `eam-new` / `C-c a n`에서 provider를 자동완성으로 고른다. Tab으로 후보를 보고 C-g로 취소할 수 있다. 이전의 중복 시작·분리·재개 명령은 각각 `eam-new`, `eam-detach`, `eam-resume`으로 통합했다.

| 할 일 | 명령 / 키 | 결과 |
|---|---|---|
| 시작 | `eam-new` / `C-c a n` | Emacs 밖의 PTY 데몬에서 CLI 실행 |
| 화면 닫기 | `eam-detach` / `C-c a d` | 화면만 분리, CLI 유지 |
| Emacs 종료 후 재접속 | `eam-attach` / `C-c a a` | 기존 세션 디렉터리를 선택해 같은 프로세스에 연결 |
| 명시적 종료 | `eam-quit` / `C-c a q` | 해당 서버 종료, 디스크 기록 유지 |

Emacs 재시작 시 자동으로 재접속하지 않는다. `attach`는 살아 있는 프로세스에 연결하며, `resume`은 저장된 대화를 새 지속 CLI에서 재개한다. Mac 재부팅 후 프로세스까지 복구하는 기능은 아니다. 작업 중인 AI는 화면 분리 후에도 작업을 계속한다. 가짜 스트림 실험은 실제 CLI가 아니므로 기존 로컬 버퍼 방식으로 유지한다.

기존 패키지·프로필·기록은 자동 삭제하거나 이동하지 않는다. 새 0.1.0 프로필은 별도로 생성한다. 이전 지속 세션은 `eam-attach`로 이전 세션 디렉터리를 지정한다. 이전 버전의 일반 CLI 프로세스가 자동으로 지속 세션으로 바뀌지는 않는다. 같은 GUI에 구버전과 신버전을 함께 로드하지 말고 새 격리 GUI로 시험한다.


## 22. 0.11.0 명령 이름 정리

자주 쓰는 세션 명령에서 구현 구분인 app·terminal·persistent를 뺐다. 중복 키 `C-c a P s`와 `C-c a P d`는 제거했다. 시작은 `C-c a n`, 분리는 `C-c a d`를 사용한다. 이전 명령 별칭은 두지 않으므로 M-x에서는 위 표의 새 이름을 사용한다.

시작 화면 명령 `eam`은 0.1.0에서 제거했다. 별도 실행 스크립트는 키만 켜며, 기존 Emacs에서는 `eam-keys-mode`로 켠다.



## CLI 자체 종료 감지

Emacs에 연결된 CLI를 `/exit`, `/quit` 등으로 종료하면 해당 백엔드가 종료를 확인해 데몬/서버를 정리한다. 터미널 화면과 세션 목록 항목도 자동 정리한다. 미저장 초안은 별도 버퍼로 보존한다. 연결이 끊겼거나 상태 확인에 실패한 경우는 `disconnected`로 표시하며 CLI 종료로 추정하지 않는다.

PTY 데몬은 분리된 상태에서도 CLI 종료 후 소켓과 상태를 자동 정리한다. 변경 전에 만들어진 세션은 기존 종료 방식이 유지되며, 연결 중에는 화면 정리를 적용할 수 있다.


지속 세션 전용 `C-c a P` 접두사는 제거했다. 재접속은 `C-c a a`, 종료는 `C-c a q`, 상태 확인은 `C-c a i`를 사용한다.

`eam-attach` (`C-c a a`)는 detached 세션 목록에서 선택한다. 내부 기록 디렉터리를 직접 입력할 필요가 없다.

`eam-resume`는 종료된 CLI의 저장 대화를 새 CLI로 재개한다. 살아 있는 프로세스로 돌아가려면 `eam-sessions` 또는 `eam-attach`를 쓴다. `eam-focus`는 초안에서 해당 CLI 버퍼로 이동할 뿐 새 세션을 만들지 않는다.

## 최근 저장된 프롬프트 확인

CLI 또는 초안에서 **C-c a u / M-x eam-prompt**를 실행하고 같은 프로젝트의 대화를 선택한다. CLI가 저장한 마지막 사용자 텍스트를 읽기 전용 버퍼로 표시한다. 긴 요청은 p/n으로 나누어 읽고 g로 새로 읽는다. 대기열·중간 지시·저장 지연 때문에 현재 처리 중인 요청과 다를 수 있다.

프롬프트 수집 훅은 새 CLI에 추가하지 않는다. 기존에 실행 중인 CLI에 전달했던 훅 설정은 CLI를 종료하고 새로 시작해야 없어지며, 이미 저장된 prompt 파일은 삭제하지 않는다. 알림 기능의 훅은 별개로 유지한다. 개인 CLI 설정은 수정하지 않는다.
