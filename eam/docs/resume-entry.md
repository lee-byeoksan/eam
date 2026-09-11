# 대화 재개 진입점과 목록 검증

2026-09-09, G03. 시작 화면의 **이전 대화 재개** 또는 `M-x emacs-ai-terminal-resume`에서 제공자·프로젝트·UUID를 입력한다. UUID가 비면 네이티브 목록을 연다. 인자는 Claude `--resume [UUID]`, Codex `resume [UUID]`이며 설치된 CLI 도움말과 실제 실행으로 확인했다. UUID 입력란에 제목·프롬프트를 넣지 않는다.

## 사용과 한계

- 같은 대화를 다른 CLI에서 쓰고 있다면 그 CLI를 먼저 정상 종료한다. 목록의 제목·시간·미리보기로 찾거나 UUID를 지정한다. 빈 목록만으로 기록 삭제를 판단하지 않는다.
- 재개 후 CLI의 모델·권한 모드를 확인한다. 앱은 설정을 덮어쓰지 않는다. 이번 Claude 목록 재개는 auto mode였으며 이전 시험의 manual 모드를 앱이 강제하지 않았다.
- ANSI 기록 열기는 조회다. 파일명은 대화 UUID가 아니다. UUID는 네이티브 CLI 정보·종료 안내에서 확인한다.
- 같은 Emacs에서 **이 명령으로 UUID 재개한** 살아 있는 대화는 다시 지정하면 기존 버퍼로 이동한다. 실제 두 CLI에서 새 프로세스가 생기지 않는 것을 확인했다. 다른 Emacs·터미널, 새 대화에서 생긴 ID, 목록에서 선택된 ID는 추적하지 않으므로 전체 중복 writer 보호는 아니다.

## 실제 연결 검사

기존 두 한글 프로젝트 대화를 제품 명령으로 UUID 재개했다. Codex는 프로젝트 신뢰를 요구해 No로 종료했고, 후속 검증에만 프로세스 한정 trust 설정을 추가했다. 개인 설정 변경이나 제품의 신뢰 우회 옵션 추가는 하지 않았다.

Claude 대화 안의 `/resume`에서는 빈 목록이었으나 종료 후 새 `--resume` 목록에는 동일 대화가 나타났다. 현재 대화 제외와 일치하는 관찰이지만 내부 원인은 확정하지 않았다. Codex도 새 목록에서 cwd 필터의 해당 대화 1개를 확인했다. 각각 목록에서 선택한 뒤 “파일이나 도구를 사용하지 말고, 이전 명령의 출력 표식만 다시 답하세요.”를 제출했고 둘 다 `PATH_OK`를 답했다.

[Claude 목록](cli-validation-evidence/resume-entry/claude-picker.txt), [Codex 목록](cli-validation-evidence/resume-entry/codex-picker.txt), [Claude 후속 응답](cli-validation-evidence/resume-entry/claude-followup.txt), [Codex 후속 응답](cli-validation-evidence/resume-entry/codex-followup.txt).

명시적 모델 프롬프트 Claude 1회·Codex 1회이며 내부 통신 횟수와 같지 않다. 두 CLI exit 확인 후 배치 Emacs를 종료했다. 앱 회귀 4개 통과(잘못된 ID 실행 전 거절·명령 인자 검사 포함). 실제 PTY 결과이며 새 버튼의 GUI 선택은 G10/G01에 남는다.

X02 최근 대화 저장은 지금 추가하지 않는다. 네이티브 목록과 UUID로 복구 경로를 제공하며 앱이 CLI 데이터베이스를 별도 해석하거나 자동 프롬프트를 삽입하지 않는다. 제어 코드와 전체 기록은 `var/cli-validation/20260909-resume-entry/`에 있다.
