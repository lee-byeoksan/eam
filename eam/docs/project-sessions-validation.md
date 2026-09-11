# 같은 CLI의 여러 프로젝트 구분

2026-09-09. 실제 Claude Code의 기존 시험 대화 두 개를 서로 다른 프로젝트에서 같은 배치 Emacs/Ghostel에 재개했다. 변경 전 이름은 `*Claude terminal*`과 `*Claude terminal*<2>`였고 헤더에도 프로젝트가 없었다.

## 변경과 확인

- 터미널과 선택적 초안 이름에 제공자와 전체 프로젝트 경로를 표시한다. 홈 디렉터리만 `~`로 줄인다. 같은 말단 디렉터리 이름도 상위 경로로 구분하며, 같은 프로젝트를 두 번 열면 Emacs의 숫자 접미사가 구분한다.
- 헤더는 프로젝트와 프로세스 상태를 표시한다. 실제 Claude 한쪽 정상 종료 뒤 `exited (0)`, 다른 쪽 `running`을 동시에 확인했다. `running`은 모델 요청 처리 중인지와 무관하다.
- 모델 호출이나 별도 타이머 없이 헤더 표시 시 프로세스 상태를 읽는다. 네이티브 CLI 기능·설정은 변경하지 않았다.
- 수정 후 실제 두 세션을 20회 왕복해 세션 객체와 프로젝트 이름 유지를 확인했다. 초안에도 올바른 프로젝트·상태가 표시되고 undo가 활성화됐다.
- 터미널 회귀 검사 5개 통과. 시작 실패 검사에서 새 이름으로 버퍼 정리를 검사하도록 참조를 갱신했다. 실행 로그: `var/project-session-regression-20260909.log`.

[변경 전](cli-validation-evidence/project-sessions/before.el), [변경 후](cli-validation-evidence/project-sessions/after.el), [실행 헤더](cli-validation-evidence/project-sessions/headers.txt), [종료 헤더](cli-validation-evidence/project-sessions/exit-headers.txt), [초안](cli-validation-evidence/project-sessions/draft-check.txt).

## 재개 목록과 한계

두 프로젝트에서 `/resume`을 열었다. `workspace`는 목록 `1 of 5`와 첫 항목 `Exit probe validation`을 표시했다. `한글 프로젝트`는 `No conversations found in this project`를 표시했다. 후자는 현재 대화를 ID로 재개해 기존 내용을 확인한 상태였으므로 빈 목록만으로 저장 기록이 없다고 판단하지 않는다. 현재 대화 제외 여부 등 CLI 내부 원인은 확정하지 않았다. Escape로 목록을 취소했고 다른 대화 선택은 이번 검사에 포함하지 않았다. 화면은 `var/cli-validation/20260909-project-sessions/resume-menu-{0,1}.txt`에 있다.

명시적 모델 프롬프트는 0회다. 실제 로그인 CLI 시작·ID 재개·메뉴 동작으로 검증했으며, 네이티브 내부 호출까지 0회라는 주장은 아니다. 두 CLI와 배치 Emacs는 정상 종료했다. 첫 검증 스크립트 실행은 버퍼 전환 후 상대 경로 기준이 바뀌어 실패했으며, 스크립트의 루트를 고정한 뒤 재실행했다.

같은 제공자 검사는 Claude 두 개로 수행했다. 공통 구현이 Codex에도 적용되지만 이번 변경 후 실제 Codex 두 개 검사는 아직 하지 않았다. GUI의 긴 경로 표시와 물리 키보드 IME는 이번 배치 검사와 별개다. 좁은 창에서는 긴 헤더가 잘릴 수 있고, 이름만으로 대화 내용을 구분해 주지는 않는다. 기존 인스턴스를 자동 마이그레이션하지 않으며 새 Emacs로 시험한다.
