# Worktree와 CLI 연결 — F04

2026-09-10. 0.5.0 패키지 기능. Git 원본 worktree 명령을 argv로 호출하며
shell 문자열로 사용자 입력을 실행하지 않는다. 개인 설정·원격 fetch·자동 컨텍스트
삽입·의존성 설치는 수행하지 않는다.

| 명령 | 키 | 동작 |
|---|---|---|
| `emacs-ai-worktree-create` | `C-c a W c` | 저장소·새 경로·새 브랜치·로컬 기준 ref/SHA를 입력해 생성, Dired 열기 |
| `emacs-ai-worktree-open` | `C-c a W o` | 실제 등록 목록에서 경로·브랜치를 보고 선택, 파일 탐색 |
| `emacs-ai-worktree-start` | `C-c a W s` | 등록 worktree와 제공자를 골라 CLI 한 개 시작 |
| `emacs-ai-worktree-remove` | `C-c a W d` | 정확한 경로 확인 후 깨끗한 보조 worktree 제거, 브랜치 유지 |

생성과 파일 탐색만으로 AI를 호출하지 않는다. CLI 시작 후 요청은 사용자가 직접
입력한다. 열린 CLI는 기존 `C-c a s` 목록에서 프로젝트·브랜치로 찾아 이동한다.
등록 목록은 `git worktree list --porcelain -z`를 사용해 공백·한글·개행 경로를
구분한다. `.git`이 파일인 linked worktree와 bare 저장소도 처리한다.

## 삭제 보호

주 worktree, bare 경로, 잠긴/사라진 등록, 실행 중인 앱 CLI, 수정된 파일 방문 버퍼,
미커밋 변경, 미추적 파일, ignored 파일이 있으면 거절한다. 강제 삭제·브랜치 삭제·
stash·reset·clean·prune은 실행하지 않는다. 확인과 삭제 사이 외부 프로그램이
파일을 쓰는 경쟁 조건은 완전히 막지 못하며, 다른 Emacs나 외부 터미널에서 실행한
프로세스까지 탐지하는 기능은 아니다. Git의 추가 거절도 그대로 표시한다.

## 검증

`tests/worktree-test.el` 실제 임시 Git 저장소 테스트 3개 통과.
같은 저장소의 두 작업 분리, 중복 경로/브랜치 거절, 기존 등록 경로를 CLI 시작 함수에
전달, bare 저장소 생성·제거, NUL 구분 개행 경로, 변경·미추적·ignored 파일·
미저장 버퍼·살아 있는 프로세스·locked 상태 삭제 거절, 삭제 후 브랜치 보존 확인.
앱 회귀 15개도 통과했다. CLI 시작 함수는 이 테스트에서 대체했으므로 실제
구독 CLI 연결 검증이라고 부르지 않는다. 설치된 패키지 및 GUI 검증은 남아 있다.

새 브랜치는 사용자가 선택한 로컬 ref/SHA에서 생성한다. 원격 추적 ref를 입력해도
로컬에 저장된 커밋을 사용하며 최신 원격 상태라고 표시하지 않는다. checkout 실패가
부분 생성 상태를 남길 수 있으므로 강제 재실행·자동 삭제로 복구하지 않는다.

## 실제 CLI 시작과 0.5.0 패키지

[실제 시작 결과](evidence/worktrees/live-start.json): 별도 시험 저장소에서
`emacs-ai-worktree-start`로 Claude·Codex를 실행했다. 각 실제 프로세스의 cwd를
lsof로 읽어 선택한 한글·공백 경로와 일치함을 확인했다. 살아 있는 동안 삭제
명령은 거절됐고 앱 입력 기록은 0바이트였다.

두 CLI는 새 저장소 신뢰 확인 화면을 표시했다. 신뢰 선택·프롬프트 제출 없이
시험 프로세스를 닫았다. 실제 구독 AI 작업 완료나 신뢰 설정 승인을 검증한 결과는
아니다. 상세 화면은 `var/worktree-live-01/Claude-screen.txt`, `Codex-screen.txt`에
남아 있다. Codex는 신뢰가 공통 저장소 루트에 적용된다고 표시하므로 사용 시
CLI가 보여주는 적용 경로를 확인한다.

[설치 로그](evidence/worktrees/package-0.5.0.log),
[패키지 해시](evidence/worktrees/package-0.5.0.json).
0.5.0 설치본의 앱 15개, 리뷰 5개, worktree 3개, 알림 4개, PTY 6개,
Magit 1개 테스트가 통과했다. 설치 경로의 실제 함수를 검사했다.
실행: `bash ~/workspace/emacs-ai/var/package-profile-0.5.0/start.sh`.
GUI 작업은 별도 미검증이며 기존 프로필은 변경하지 않았다.

0.6.1부터 실행 중인 프로젝트 테스트·개발 서버 등 Emacs 프로세스 버퍼도 삭제
보호에 포함한다. 실제 프로젝트 명령이 실행되는 동안 삭제 거절, 취소 후 삭제
가능함을 검증했다. 외부 Emacs나 별도 터미널의 프로세스를 탐지한다는 뜻은 아니다.

## 0.8.10 저장소 경로 정규화

GUI에서 read-directory-name이 반환한 ~/... 경로를 Git -C에 그대로 전달하면
Git이 ~를 확장하지 않아 생성·조회가 실패했다. Git 호출 전에 저장소 경로를
절대 경로로 확장하도록 수정했다. 실제 Git 생성 테스트에 ~ 경로를 추가해
0.8.9 설치본에서 실패하고 수정 소스의 worktree 4개 테스트가 통과함을 확인했다.
GUI의 입력 전달 실패와는 별도로 실제로 재현된 제품 오류다.

0.8.10 설치본에서 M-x 생성 명령으로 ~ 저장소 경로, 상대 생성 경로, 브랜치, HEAD를
직접 입력해 Dired가 열리는 것을 확인했다. Git 상태·브랜치·커밋·파일 내용도 일치했다.
[GUI와 대조](evidence/gui-087/worktree-created-0810.json). 새 CLI 신뢰 승인은 수행하지 않았다.
