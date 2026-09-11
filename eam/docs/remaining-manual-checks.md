# 직접 확인 완료 기록

**모든 요청 항목 확인 완료. 이 문서는 당시 절차의 이력이며 추가 시험 요청이 아니다.**
세션 선택의 한글 입력·수정도 사용자가 정상으로 확인했다. Space는 기본 completion
동작이었다. 리뷰 의견·배너·Claude/Codex worktree 시험도 완료했다.
[최종 판정](completion-audit.md)을 기준으로 한다.

## 1. 세션 선택 미니버퍼

1. `Claude reconnect GUI 0.8.11` 창을 선택한다.
2. `C-g`로 남아 있는 입력을 취소한 뒤 `M-x emacs-ai-app-switch`를 실행한다.
3. `CLI session:` 입력란에 붙여넣지 않고 `값과 꽃 abc 123`을 직접 입력한다.
4. 한글 받침을 Backspace로 지우고 재입력한다. 문장 중간으로 이동해 한영 전환과
   수정이 가능한지 확인한다. **안내 정정:** 기본 completion에서 Space는 단어 완성
   명령이다. 후보가 없는 임의 문장에 공백을 넣는 위 절차는 적절하지 않았다.
5. Enter 대신 `C-g`로 취소한다. CLI 실행이나 AI 요청은 발생하지 않는다.

`No CLI sessions open`이면 새 CLI를 시작하지 말고 그 문구만 알려준다.

## 2. 리뷰 의견 미니버퍼

1. 같은 창에서 `C-x C-f`로 `~/workspace/emacs-ai/var/manual-ime-review.diff`를 연다.
2. `C-x h`로 파일 전체를 선택하고 `M-x emacs-ai-review-selection`을 실행한다.
3. `Review comment (...)` 입력란에 `값과 꽃 abc 123`을 직접 입력한다.
4. 받침 삭제·재입력, 문장 중간 한영 전환과 수정이 가능한지 확인한다.
5. Enter 대신 `C-g`로 취소한다. 초안 생성·CLI 전송·AI 요청은 발생하지 않는다.

결과는 `세션 선택 정상 / 리뷰 의견 정상` 또는 실패한 단계와 현상으로 보고한다.
이 절차는 두 미니버퍼의 조합·수정·취소 확인이다. 전송, undo, 후보 선택 등 이미
별도로 확인한 동작을 재시험하는 절차가 아니다.

## 별도 남은 항목

업데이트: Claude와 Codex 모두 gui-created 입력 화면까지 확인했고 시험 프로세스도
정리했다. 아래는 당시 준비 기록이며 같은 시험을 다시 요청하는 안내가 아니다.
두 CLI가 올바른 worktree
경로에서 시작하고 삭제 보호가 동작하는 것까지는 확인했다. 이 문서의 한글 시험을
통과했다고 신뢰 이후 흐름까지 통과한 것으로 처리하지 않는다. 신뢰 대상과 화면을
구체적으로 확인하기 전에는 임의의 신뢰 승인이나 설정 변경을 요청하지 않는다.

### worktree 시험 대상 사전 점검

- 저장소: `~/workspace/emacs-ai/var/gui-worktree-087/repo/`
- 선택할 worktree: `~/workspace/emacs-ai/var/gui-worktree-087/gui-created/`
- 브랜치: `gui-created`, HEAD: `3b910a0eee6e6dbeed8e773c8e7b3f51ac094295`.
- 현재 파일은 `.git` 연결 파일과 `example.txt`뿐이다. example.txt 내용은
  `worktree GUI fixture` 한 줄이며 미커밋·미추적 변경은 없다.
- 저장소의 비표본 Git hook은 없고 로컬 config 키는 기본 core 설정뿐이다.
- [읽기 전용 점검 기록](evidence/gui-087/worktree-handoff.json).

한글 입력 확인 후 사용할 진입점은 `M-x emacs-ai-worktree-start`다.
Repository에 위 저장소를 지정하고 worktree 목록에서 gui-created 경로를 선택한 뒤
Provider를 선택한다. 신뢰 화면이 나오면 실제 표시 경로와 선택지를 먼저 확인한다.
이 사전 점검이나 실행 절차는 신뢰 승인으로 간주하지 않는다. CLI 설정이나 부모
디렉터리의 설정까지 없다고 주장하지 않으며, 기존 자동 승인 거절을 우회하지 않는다.
